// Sigmoid--->>>> y = 1/(1+e^(-kx))
// where k is the paramter that determines how stretched the shaped is. However Sigmoid is too expensive.
// The reason for the choice of the upper range bneing defined as 126 is that this is the largest even 8-bit integer.

use std::arch::x86_64::{__m256i, _mm256_abs_epi16, _mm256_blendv_epi8, _mm256_load_si256, _mm256_mulhi_epi16, _mm256_packs_epi16, _mm256_permute4x64_epi64, _mm256_set1_epi16, _mm256_set1_epi8, _mm256_slli_epi16, _mm256_store_si256, _mm256_subs_epi8, _mm256_subs_epu16};


/// https://disservin.github.io/stockfish-docs/nnue-pytorch-wiki/docs/nnue.html#quantmoid4
fn quantmoid(x: i32) -> i32{
    let sign = (x > 0) as i32; // x > 0 ? 1 : 0 

    let abs_x = i32::min(x.abs(), 127) - 127;
    let abs_sq = (abs_x * abs_x)/256;

    (sign * abs_sq) + ((1-sign) * (126-abs_sq))
}

pub(crate) unsafe fn quantmoid4(size: usize, input: &Vec<i16>) -> Vec<i8> {
    const INPUT_REGISTER_WIDTH: usize = 256/16; // 16
    const OUTPUT_REGISTER_WIDTH: usize = 256/8; // 32
    assert!(size % OUTPUT_REGISTER_WIDTH == 0);
    
    let num_out_chunks = size / OUTPUT_REGISTER_WIDTH;

    let cst_127_epi16 = _mm256_set1_epi16(127);
    let cst_126_epi8 = _mm256_set1_epi8(126);

    const CONTROL: i32 = 0b11011000; // [0, 2, 1, 3];
    let mut output: Vec<i8> = Vec::with_capacity(input.len());
    
    for i in 0..num_out_chunks {
        let v0 = _mm256_load_si256(input.as_ptr().add((i*2+0) * INPUT_REGISTER_WIDTH) as *const __m256i);
        let v1 = _mm256_load_si256(input.as_ptr().add((i*2+1) * INPUT_REGISTER_WIDTH) as *const __m256i);

        let sign = _mm256_packs_epi16(v0, v1);

        // v0 = i32::min(input[i].abs(), 127)-127
        let v0 = _mm256_subs_epu16(cst_127_epi16, _mm256_abs_epi16(v0));
        let v1 = _mm256_subs_epu16(cst_127_epi16, _mm256_abs_epi16(v1));

        let v0 = _mm256_slli_epi16(v0, 4);
        let v1 = _mm256_slli_epi16(v1, 4);

        let v0 = _mm256_mulhi_epi16(v0, v0);
        let v1 = _mm256_mulhi_epi16(v1, v1);

        // Now we can convert to i8 after that for the rest
        let v0 = _mm256_packs_epi16(v0, v1);

        let v0 = _mm256_blendv_epi8(_mm256_subs_epi8(cst_126_epi8, v0), v0, sign);

        // Deinterleave the output due to AVX2 semantics.
        _mm256_store_si256(output.as_mut_ptr().add(i * OUTPUT_REGISTER_WIDTH) as *mut __m256i, _mm256_permute4x64_epi64(v0, CONTROL));
    }

    output
}