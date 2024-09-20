use std::arch::x86_64::{__m128i, __m256i, _mm256_add_epi32, _mm256_castsi256_si128, _mm256_extracti128_si256, _mm256_hadd_epi32, _mm256_load_si256, _mm256_maddubs_epi16, _mm256_set1_epi16, _mm256_setzero_si256, _mm_add_epi32, _mm_load_si128, _mm_srai_epi32, _mm_store_si128};

use crate::{color::Color, nnue_::constants::halfKA::LOG2_WEIGHT_SCALE};

use super::accumulator::{Accumualator, Feature};
// / U: is the size of the layer e.g. L1_size can be 518, or 768 e.t.c
// / M: is the size of the each column in the layer, e.g. on AVX-2 with 256 register 
// / if we are using i32, then M = 8 (i.e, 8 x 32 = 256)
// / if we're using i16, then M = 16 (i.e, 16 x 16 = 256)

const COLUMNS: usize = 8;

/// Column-Major layout: Access to the individual elements in the following form
/// L0.weight[column_index][row_index]
// #[repr(align(32))]
/// M = INPUT_SIZE * OUTPUT_SIZE
/// N = OUTPUT_SIZE 
pub(crate) struct LinearLayer<const M: usize, const N: usize, T: Copy> {
    pub(crate) weight: [[T; M]; 2], // where U = 2(colors) * layer's size
    pub(crate) bias: [T; N],
    pub(crate) num_inputs: usize,
    pub(crate) num_outputs: usize,
}


impl<const M: usize, const N: usize, T: Copy> LinearLayer<M, N, T> {
    pub(crate) fn new(weight: [[T; M]; 2], bias: [T; N]) -> Self {
        Self { weight, bias, num_inputs: weight[0].len()/bias.len(), num_outputs : bias.len() }
    }

    pub(crate) fn run(&self, input: Vec<i8>, output: &mut Vec<__m128i>, color: Color) {
        // Assuming the expected output is size i32
        const REGISTER_WIDTH: usize = 256/8; // 32
        assert!((self.num_inputs % REGISTER_WIDTH ) == 0, "We're processing 32 elements at a time");
        assert!(self.num_outputs % 4 == 0, "We unroll by 4");
        let num_in_chunks: usize = self.num_inputs / REGISTER_WIDTH; // 768/32 = 24
        let num_out_chunks: usize = self.num_outputs / 4; // 4

        for i in 0..num_out_chunks {
            // Prepare weight offsets. One offset for one row of weights
            // This is a simple index into a 2D array
            // for the next four output chunks
            let offset_0 = (i * 4 + 0) * self.num_inputs;
            let offset_1 = (i * 4 + 1) * self.num_inputs;
            let offset_2 = (i * 4 + 2) * self.num_inputs;
            let offset_3 = (i * 4 + 3) * self.num_inputs;
            
            // Accumulation starts from 0, we add the bias only at the end
            let mut sum0 = unsafe { _mm256_setzero_si256() };
            let mut sum1 = unsafe { _mm256_setzero_si256() };
            let mut sum2 = unsafe { _mm256_setzero_si256() };
            let mut sum3 = unsafe { _mm256_setzero_si256() };
            
            // Each innermost loop processes a 32*4 chunk of weights, so 128 weights at a time!
            for j in 0..num_in_chunks {
                // remember the input is u8(8 bits = 1 byte), and we have a 256bits wide hardware register
                // so, we can process 256bits/8bits(u8) = 32inputs(u8) on each register
                // with this appraoch of loading 4 of them at a time, we would have 32 * 4 = 128

                // We unroll by 4 so that we can reuse this value, reducing the number of memory operations required
                unsafe { 
                    let chunk = input.as_ptr().add(j * REGISTER_WIDTH) as *const __m256i;
                    let inp = _mm256_load_si256(chunk);

                    // This function processes a 32*1 chunk of i8, and produces 8*1 chunk of i32
                    // For definition see below
                    let mem_addr0 = (*self.weight.as_ptr().add(color as usize)).as_ptr().add(offset_0 + j * REGISTER_WIDTH) as *const __m256i;
                    sum0 = Self::m256_add_dpbusd_epi32(sum0, inp, _mm256_load_si256(mem_addr0));
                    
                    let mem_addr1 = (*self.weight.as_ptr().add(color as usize)).as_ptr().add(offset_1 + j * REGISTER_WIDTH) as *const __m256i;
                    sum1 = Self::m256_add_dpbusd_epi32(sum1, inp, _mm256_load_si256(mem_addr1));

                    let mem_addr2 = (*self.weight.as_ptr().add(color as usize)).as_ptr().add(offset_2 + j * REGISTER_WIDTH) as *const __m256i;
                    sum1 = Self::m256_add_dpbusd_epi32(sum2, inp, _mm256_load_si256(mem_addr2));

                    let mem_addr3 = (*self.weight.as_ptr().add(color as usize)).as_ptr().add(offset_3 + j * REGISTER_WIDTH) as *const __m256i;
                    sum2 = Self::m256_add_dpbusd_epi32(sum3, inp, _mm256_load_si256(mem_addr3));
                };
            }

            unsafe {
                let bias = _mm_load_si128(self.bias.as_ptr().add(i * 4) as *const __m128i);
                // This function adds horizontally 8 values from each sum together, producing 4 i32 values
                let mut outval = Self::m256_haddx4(sum0, sum1, sum2, sum3, bias);
                // Weight scaling
                outval = _mm_srai_epi32(outval, LOG2_WEIGHT_SCALE);
                _mm_store_si128(output.as_mut_ptr().add(i * 4), outval);
            }

            // return output.as_ptr().add(self.num_outputs)
        }
    }

    unsafe fn m256_add_dpbusd_epi32(result: __m256i, a: __m256i, b: __m256i) -> __m256i {
        
        // Mutiply a * b, with the output in i16
        let product0 = _mm256_maddubs_epi16(a, b);

        let one = _mm256_set1_epi16(1);
        let product0 = _mm256_maddubs_epi16(product0, one);

        // Add to the main i32 result (accumulator)
        let acc = _mm256_add_epi32(result, product0);

        acc
    }

    /// This function takes 4 _m256i registers containing 8 i32 values each, accumulates them horizontally and produces one _m128i register
    /// containing 4 i32 values, each corresponding to one input sum. 
    /// https://disservin.github.io/stockfish-docs/nnue-pytorch-wiki/docs/nnue.html#m256-haddx4
    unsafe fn m256_haddx4(sum0: __m256i, sum1: __m256i, sum2: __m256i, sum3: __m256i, bias: __m128i) -> __m128i {
        let sum0 = _mm256_hadd_epi32(sum0, sum1);
        let sum2 = _mm256_hadd_epi32(sum2, sum3);

        let sum0 = _mm256_hadd_epi32(sum0, sum2);

        let sum_128lo = _mm256_castsi256_si128(sum0);
        let sum_128hi = _mm256_extracti128_si256(sum0, 1);
        
        _mm_add_epi32(_mm_add_epi32(sum_128lo, sum_128hi), bias)
    }
}