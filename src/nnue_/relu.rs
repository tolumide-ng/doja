use std::arch::x86_64::{__m256i, _mm256_load_si256, _mm256_max_epi8, _mm256_packs_epi16, _mm256_permute4x64_epi64, _mm256_setzero_si256, _mm256_store_si256};

pub struct Crelu;

impl Crelu {
    /// Convert i16 -> i8
    pub(crate) unsafe fn crelu16(size: usize, input: &Vec<i16>) {
        const INPUT_REGISTER_WIDTH: usize = 256/16; // 16
        const OUTPUT_REGISTER_WIDTH: usize = 256/8; // 32
        assert!(size % OUTPUT_REGISTER_WIDTH == 0, "We're processing 32 elements at a time");
        
        let num_out_chunks = size / OUTPUT_REGISTER_WIDTH;

        let zero = _mm256_setzero_si256();
        const CONTROL: i32 = 0b11011000; // 3, 1, 2, 0; lane 0 is the rightmost lane

        let mut output: Vec<i8> = vec![];
        for i in 0..num_out_chunks {
            let in0 = _mm256_load_si256(input.as_ptr().add((i * 2 + 0) * INPUT_REGISTER_WIDTH) as *const __m256i);
            let in1 = _mm256_load_si256(input.as_ptr().add((i * 2 + 1) * INPUT_REGISTER_WIDTH) as *const __m256i);

            let result = _mm256_permute4x64_epi64(
                _mm256_max_epi8(_mm256_packs_epi16(in0, in1), zero), CONTROL);

            _mm256_store_si256(output.as_mut_ptr().add(i * OUTPUT_REGISTER_WIDTH) as *mut __m256i, result);
        }
    }
}

