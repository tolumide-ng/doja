// when the king moves, the accumulator is refreshed

use std::arch::x86_64::{__m256i, _mm256_add_epi16, _mm256_load_si256, _mm256_max_epi16, _mm256_max_epi8, _mm256_min_epi16, _mm256_mullo_epi16, _mm256_packs_epi16, _mm256_permute4x64_epi64, _mm256_set1_epi16, _mm256_setzero_si256, _mm256_store_si256, _mm256_sub_epi16};
use std::ops::{Index, IndexMut};


use crate::board::piece::Piece;
use crate::board::position::ACCUMULATOR_SIZE;
use crate::board::state::board::Board;
use crate::color::Color::*;
use crate::color::Color;
use crate::constants::PLAYERS_COUNT;
use crate::nnue::{halfka_idx00, INPUT, L1_SIZE};
use crate::squares::Square;

use super::align64::Align64;
use super::feature_idx::FeatureIdx;
use super::{halfka_idx, PARAMS};


//  // the size of bias is always the L1 size
//  // if U is 64(values), and Feature is _m256i(16 * i16), where mem::sizeof<m256i>() = 32, then that would be (32/2) * 64 = 1024 i.e. ((mem_size/2) * U)
//  // U = 1024, because Feature is i16, where std::mem::sizof::<i16>() = 2 then  (2/2) * 1024 = 1024
//  // 
//  let bias_len = (std::mem::size_of::<__m256i>()/2) * U;
//  println!("B len is {}", bias_len);
//  let mut accumulator = Self { white: Align64([_mm256_setzero_si256(); U]), black: Align64([_mm256_setzero_si256(); U]) };     
//  for color in [Color::White, Color::Black] {
//      for b in 0..bias_len {
//          // 
//      }
//  }


// A single AVX2 register can fit 16 i16 values, and there are 16AVX2 registers (32 since AVX512) (stockfish docs)

pub(crate) type Feature = __m256i;
pub(crate) const QA: i16 = 255;
pub(crate) const QAB: i32 = 255*64;

/// we're working with accumulator values as i16(16bits)
/// Where our Accumualtor has a data type of _m256i, each _m256i would contains 16 i16 values (i.e 16 * i16 = 256)
/// for a layer that should have 1024(L1_SIZE or U) neurons(values), we only need 1024/16 = 64;
/// so, the length of each side of our accumulator when using _m256i only needs to be 64
#[derive(Debug, Clone, Copy)]
#[repr(align(64))]
pub(crate) struct Accumulator<T, const U: usize> {
    pub(crate) white: Align64<[T; U]>,
    pub(crate) black: Align64<[T; U]>,
}



// In order to ensure that refresh must always be called first, I should remove this soon
impl<const U: usize> Default for Accumulator<Feature, U> {
    fn default() -> Self {
        unsafe { Self { white: Align64([_mm256_setzero_si256(); U]), black: Align64([_mm256_setzero_si256(); U]) } }
    }
}


/// U is the L1 Size (i.e number of first output)
impl<const U: usize> Accumulator<Feature, U> {
    const REGISTER_WIDTH: usize = 256/16; // 16

    const ACCUMULATOR_LEN: usize = U * 2;

    /// REGS_LEN: is the length of two accumualtors used here
    unsafe fn update_weights<const ON: bool, const REGS_LEN: usize>(regs: &mut [__m256i; REGS_LEN], idx: (FeatureIdx, FeatureIdx)) {
        let (white_idx, black_idx) = idx;

        // we can only load 16 i16(16bits) value at a time, so we must perform this operation 1024(size of U)/16(bits) i.e. => 64 times
        // let num_chunks = U / Self::REGISTER_WIDTH;  // 1024/16 = 64, 

        // there are U(in this case 1024) weights per idx
        // so, we load the 1024 weights(from feature_weights) related to this idx(white, black)
        // and depending on the value of ON (true or false), we add or remove the weights from the current accumulator
        for (color, color_idx) in [white_idx, black_idx].into_iter().enumerate() {
            for i in 0..U {
                // would load weighs @w_idx..w_idx+16 (up until we've read all 1024 of them, which means this would run 64 times)                
                let weights_idx = *color_idx + (i * Self::REGISTER_WIDTH);
                // because we know that REGS_LEN is basically U * 2, where the first half is white, and the second half is black
                let regs_idx = (color  * (REGS_LEN/2)) + i;
                println!("i is {}, weight idx {} register index is {}", i, *color_idx + i, regs_idx);
                // the interval for weights needs to be a multiple of 16, since weights is actually a bunch of i16 values, i.e 16 * i16 = 256
                // let weight_idx = i * 
                let weights_at_i = *(PARAMS.input_weight.as_ptr().add(weights_idx) as *const __m256i);
                // Get the value on the accumulator at this index
                let values_at_i = *regs.as_ptr().add(regs_idx);

                let result = _mm256_add_epi16(weights_at_i, values_at_i);
                _mm256_store_si256(regs.as_mut_ptr().add(regs_idx), result);
            }
        }
    }

    pub(crate) unsafe fn refresh(board: &Board) -> Self {
        println!("::::: the value of U is --->>>>>>>>> {U}");
        let mut acc = Accumulator::default();
        let num_chunks: usize = U / Self::REGISTER_WIDTH; // 1024/16 = 64 (U would usually be the L1SIZE)
        
        // const SIZE: usize = U * 2;
        // const BOTH: usize = U * 2;
        // U would normally be equal to ACCUMULATOR_SIZE, since rust doesn't allowing using generics in const operations
        const REGS_LEN: usize = ACCUMULATOR_SIZE * 2;
        let mut regs: [__m256i; REGS_LEN] = [std::mem::zeroed(); REGS_LEN];

        // Load the bias into the self
        for color in [Color::White, Color::Black] {
            for i in 0..U {
                let regs_idx = (ACCUMULATOR_SIZE * color as usize) + i;
                *regs.as_mut_ptr().add(regs_idx) = _mm256_load_si256(PARAMS.input_bias.as_ptr().add(i * Self::REGISTER_WIDTH) as *const __m256i);
            }
        }

        let bitmaps = &board.board;
        
        // identifies the pieces present on the board, and represents them on the the accumulator
        for (p, board) in (*bitmaps).into_iter().enumerate() {
            let mut sqs: u64 = *board;

            while sqs != 0 {
                let sq = Square::from(sqs.trailing_zeros() as u8);
                let piece = Piece::from(p as u8);
                // we need to update black, and white's perspective
                let (white, black) = halfka_idx(piece, sq);

                sqs &= sqs -1;
                Self::update_weights::<true, REGS_LEN>(&mut regs, (white, black));
            }
        }


        // println!("the length of the regs is {}", regs.len()); // 2048 for both accumulators

        // // Load bias into registers
        // for color in [White, Black] {
        //     for i in 0..num_chunks {
        //         // If there is a problem here, please confirm that halfa_idx works as it should, compare directly with the one from pytorch-NNUE
        //         // let color_idx = (color as usize * num_chunks) + i;

        //         let idx = (color as usize * L1_SIZE) + (i * Self::REGISTER_WIDTH);
        //         *regs.as_mut_ptr().add(idx) = _mm256_load_si256(PARAMS.input_bias.as_ptr().add(idx) as *const __m256i);
        //         // _mm256_store_si256(acc[color].as_mut_ptr().add(i) as *mut __m256i, *(MODEL.features_bias.as_ptr().add(i * REGISTER_WIDTH) as *const __m256i));
        //     }
        // }


        // for (a, color) in active_features {
        //     for i in 0..num_chunks {
        //         // the number of weights per feature, is equivalent to the L1_SIZE, in this case 1024figures represent the weight of a feature "a"
        //         // for any value of "a", by the end of this loop, we'd have loaded all weights that belong to "a"
        //         // this specifically updates all the values representing the feature "a" on the board
        //         // we can only load 16 i16s at a time, weil 16*16=256 (size of an AVX2 register)
        //         let idx =  *a + (i*Self::REGISTER_WIDTH);
        //         // let reg_idx = (color as usize * num_chunks) + i;
        //         let reg_idx = (color as usize * INPUT) + (i * Self::REGISTER_WIDTH);


        //         // there are 768's(input_size) per input size
        //         let xidx = *a;

        //         // println!("trying out 8************* {} {reg_idx}", xidx);
                
                
        //         // println!("inside params it is ((((({}))))", idx);
        //         // println!("input weight is::: _---- {}", PARAMS.input_weight.len());

        //         // let idx = ()

                
        //         let weights = *(PARAMS.input_weight.as_ptr().add(idx) as *const __m256i);
        //         // println!("the index for now is>>>>>>>>>>>>>>>>>>>>>>>>>>>* {}", reg_idx);
        //         // y = Ax + b (where A is the feature, x is the weight, and b is bias)
        //         *regs.as_mut_ptr().add(reg_idx) = _mm256_add_epi16(*(regs.as_ptr().add(reg_idx)), weights);
        //     }
        // }

        // // println!("the index for now is>>>>>>>>>>>>>>>>>>>>>>>>>>> {}", regs.len());

        // // println!("NUMBER OF CHUNKS IS {num_chunks}");

        // // println!("@ position xxxx {:?}", regs[100]);
        // // println!("@ position xxxx {:?}", PARAMS.input_bias[100]);

        // for i in 0..num_chunks {
        //     // println!("expected::::::::::::::::::::::::::::::::::::::::::::www {}", i*Self::REGISTER_WIDTH);
        //         let black_idx = num_chunks + i;
        //     _mm256_store_si256(acc[Color::White].as_mut_ptr().add(i*Self::REGISTER_WIDTH) as *mut __m256i, *regs.as_ptr().add(i));
        //     _mm256_store_si256(acc[Color::Black].as_mut_ptr().add(i*Self::REGISTER_WIDTH) as *mut __m256i, *regs.as_ptr().add(black_idx));
        // }

        // println!("acc len {}", acc.white.len());
        
        acc
    }

    pub(crate) unsafe fn update(&self, removed_features: &Vec<(Color, FeatureIdx)>, added_features: &Vec<(Color, FeatureIdx)>) -> Self {
        let num_chunks: usize = U / Self::REGISTER_WIDTH; // 1024/16 = 64 (U would usually be the L1SIZE)


        println!("the value of U is {}", U);

        let mut acc = Accumulator::default();
        let mut regs: Vec<__m256i> = Vec::with_capacity(num_chunks * PLAYERS_COUNT);

        
        // Load the previous values to registers and operate on registers only
        for i in 0..num_chunks {
            let black_idx = num_chunks + i;
            *regs.as_mut_ptr().add(i) = _mm256_load_si256(self.white.as_ptr().add(i * Self::REGISTER_WIDTH));
            *regs.as_mut_ptr().add(black_idx) = _mm256_load_si256(self.black.as_ptr().add(i * Self::REGISTER_WIDTH));
        }

        for (color, f_idx) in removed_features.iter() {
            for i in 0..num_chunks {
                // let model_idx = (**f_idx * U) + (i * Self::REGISTER_WIDTH);
                let model_idx = **f_idx + (i * Self::REGISTER_WIDTH);
                let regs_idx = (*color as usize * num_chunks) + i;
                
                let weights = *(PARAMS.input_weight.as_ptr().add(model_idx) as *const __m256i);
                *regs.as_mut_ptr().add(regs_idx) = _mm256_sub_epi16(*(regs.as_ptr().add(regs_idx)), weights);
            }
        }
        
       for (color, f_idx) in added_features.into_iter() {
            for i in 0..num_chunks {
                let model_idx = **f_idx+ (i * Self::REGISTER_WIDTH);
                let regs_idx = (*color as usize * num_chunks) + i;

                let weights = *(PARAMS.input_weight.as_ptr().add(model_idx) as *const __m256i);
                *regs.as_mut_ptr().add(regs_idx) = _mm256_add_epi16(*(regs.as_ptr().add(regs_idx)), weights);
            }
        }
        
        for i in 0..num_chunks {
            let black_idx = num_chunks + i;
            _mm256_store_si256(acc[Color::White].as_mut_ptr().add(i*Self::REGISTER_WIDTH) as *mut __m256i, *regs.as_ptr().add(i));
            _mm256_store_si256(acc[Color::Black].as_mut_ptr().add(i*Self::REGISTER_WIDTH) as *mut __m256i, *regs.as_ptr().add(black_idx));
        }
    
        acc
    }

    pub(crate) unsafe fn crelu16(&self, stm: Color) -> [[__m256i; U]; 2] {
        const IN_REGISTER_WIDTH: usize = 256/16;
        const OUT_REGISTER_WIDTH: usize = 256/8;
        assert_eq!(U % OUT_REGISTER_WIDTH, 0, "We're processing 32 elements at a time");
        let num_out_chunks = U/OUT_REGISTER_WIDTH;

        let input = if stm == Color::White {[self.white, self.black]} else {[self.black, self.white]};
        let mut output: [[__m256i; U]; 2] = [[_mm256_setzero_si256(); U]; 2];

        let zero = _mm256_setzero_si256();
        const CONTROL: i32 = 0b11011000; // 3, 1, 2, 0; lane 0 is the rightmost one

        for i in 0..num_out_chunks {
            for color in [Color::White, Color::Black] {
                let curr_input = *(input.as_ptr().add(color as usize));
                let in0 = _mm256_load_si256(curr_input.as_ptr().add((i * 2 + 0) * IN_REGISTER_WIDTH));
                let in1 = _mm256_load_si256(curr_input.as_ptr().add((i * 2 + 1) * IN_REGISTER_WIDTH));
                
                let result = _mm256_permute4x64_epi64(_mm256_max_epi8(_mm256_packs_epi16(in0, in1), zero), CONTROL);

                _mm256_store_si256(output.as_mut_ptr().add(i * OUT_REGISTER_WIDTH) as *mut __m256i, result);
            }
        }

        output
    }

    /// Loads input(16 i16 values), and 
    ///     1. Ensures that the max of the input is i8(127) (saturates the input)
    ///     2. And ensures that the min of the input is 0 (i8)
    /// Then multiplies the input(now 16 i8 values) with the weights(16 i16 values)
    /// to generates 16 i16 values (the output)
    pub(crate) unsafe fn sq_crelu16(&self, stm: Color) -> [Align64<[__m256i; U]>; 2] { // U is 1024
        const IN_REGISTER_WIDTH: usize = 256/16; // 16
        const OUT_REGISTER_WIDTH: usize = 256/16; // 16 (output would be in i16, because we would be squaring the clamped values(i8^2) squaredCReLU)
        let num_out_chunks = U/OUT_REGISTER_WIDTH; // 1024/16 = 64

        
        let input = if stm == Color::White {[self.white, self.black]} else {[self.black, self.white]};
        let mut output: [Align64<[__m256i; U]>; 2] = [Align64([_mm256_setzero_si256(); U]); 2];  // [[_; 1024]; 2];
        
        // let min = _mm256_set1_epi16(-128);
        // let max = _mm256_set1_epi16(127);

        let min = _mm256_set1_epi16(0);
        let max = _mm256_set1_epi16(255);

        
        // const CONTROL: i32 = 0b11011000; // 3, 1, 2, 0; lane 0 is the rightmost one
        
        for color in [Color::White, Color::Black] {
            for i in 0..num_out_chunks {
                let curr_input = *(input.as_ptr().add(color as usize)); // color
                let in0 = _mm256_load_si256(curr_input.as_ptr().add(i * IN_REGISTER_WIDTH)); // loads 16 i16 values from curr_input 

                let clamped_min = _mm256_max_epi16(in0, min);
                // despite being inside (i16 * 16) data structure, these are just (i8 * 16) values
                // the reason for this, is to still be capable of returning (i16 * 16) values after the squaring (i.e multiplication)
                let clamped_max = _mm256_min_epi16(clamped_min, max);
                let result = _mm256_mullo_epi16(clamped_max, clamped_max); // square

                _mm256_store_si256(output[color as usize].as_mut_ptr().add(i * OUT_REGISTER_WIDTH) as *mut __m256i, result);
            }
        }

        output
    }
}


impl<T, const U: usize> Index<Color> for Accumulator<T, U> {
    type Output = [T; U];

    fn index(&self, index: Color) -> &Self::Output {
        match index {
            Color::White => &self.white,
            Color::Black => &self.black,
            _ => panic!("unrecognized color")
        }
    }
}



impl<T, const U: usize> IndexMut<Color> for Accumulator<T, U> {
    fn index_mut(&mut self, index: Color) -> &mut Self::Output {
        match index {
            Color::White => &mut self.white,
            Color::Black => &mut self.black,
            _ => panic!("unrecognized color")
        }
    }
} 