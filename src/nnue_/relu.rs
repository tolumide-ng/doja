use std::arch::x86_64::{__m256i, _mm256_load_si256, _mm256_max_epi8, _mm256_packs_epi16, _mm256_packs_epi32, _mm256_permute4x64_epi64, _mm256_permutevar8x32_epi32, _mm256_set_epi32, _mm256_setzero_si256, _mm256_store_si256};

pub struct Crelu;

impl Crelu {
    /// Convert i16 -> i8
    /// https://disservin.github.io/stockfish-docs/nnue-pytorch-wiki/docs/nnue.html#int16-int8
    pub(crate) unsafe fn crelu16<const M: usize>(size: usize, input: [i16; M]) -> [i8; M] {
        const INPUT_REGISTER_WIDTH: usize = 256/16; // 16
        const OUTPUT_REGISTER_WIDTH: usize = 256/8; // 32
        assert!(M % OUTPUT_REGISTER_WIDTH == 0, "We're processing 32 elements at a time");
        
        let num_out_chunks: usize = size / OUTPUT_REGISTER_WIDTH; // if size=32, then this is = 1;

        let mut output: [i8; M] = [0; M]; // where M=32

        let zero = _mm256_setzero_si256();
        const CONTROL: i32 = 0b11011000; // 3, 1, 2, 0; lane 0 is the rightmost lane

        for i in 0..num_out_chunks {
            // where M(or size) is 32, @0, this would be ((0*2+0)*16)=0, @1 it would be ((1*2+0)*16)=32
            let in0 = _mm256_load_si256(input.as_ptr().add((i * 2 + 0) * INPUT_REGISTER_WIDTH) as *const __m256i); 
            let in1 = _mm256_load_si256(input.as_ptr().add((i * 2 + 1) * INPUT_REGISTER_WIDTH) as *const __m256i);

            let result = _mm256_permute4x64_epi64(_mm256_max_epi8(_mm256_packs_epi16(in0, in1), zero), CONTROL);

            _mm256_store_si256(output.as_mut_ptr().add(i * OUTPUT_REGISTER_WIDTH) as *mut __m256i, result);

            // _mm256_store_si256(&output.as_mut_ptr().add(i * OUTPUT_REGISTER_WIDTH), a);
        }

        output
    }


    /// i32 -> i8
    /// https://disservin.github.io/stockfish-docs/nnue-pytorch-wiki/docs/nnue.html#int32-int8
    pub(crate) unsafe fn crelu32<const M: usize>(size: usize, input: [i16; M]) -> [i8; M] {
        let mut output: [i8; M] = [0; M];

        const INPUT_REGISTER_WIDTH: usize = 256/32; // 8
        const OUTPUT_REGISTER_WIDTH: usize = 256 / 8; // 32
        assert!(M == size);
        assert!(size % OUTPUT_REGISTER_WIDTH == 0, "We're processing 32 elements at a time");
        
        let num_out_chunks = size / OUTPUT_REGISTER_WIDTH;

        let zero = _mm256_setzero_si256();
        let control = _mm256_set_epi32(7, 3, 6, 2, 5, 1, 4, 0);

        for i in 0..num_out_chunks {
            let inp0 = _mm256_packs_epi32(*(input.as_ptr().add((1 * 4 + 0) * INPUT_REGISTER_WIDTH) as * const __m256i), 
                *(input.as_ptr().add((i*4 + 1) * INPUT_REGISTER_WIDTH) as *const __m256i));
            
            let inp1 = _mm256_packs_epi32(*(input.as_ptr().add((1 * 4 + 2) * INPUT_REGISTER_WIDTH) as * const __m256i), 
            *(input.as_ptr().add((i*4 + 3) * INPUT_REGISTER_WIDTH) as *const __m256i));

            let result = _mm256_permutevar8x32_epi32(_mm256_max_epi8(_mm256_packs_epi16(inp0, inp1), zero), control);

            _mm256_store_si256(output.as_mut_ptr().add(i * OUTPUT_REGISTER_WIDTH) as *mut __m256i, result);
        }


        output
    }
}

