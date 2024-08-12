use std::arch::x86_64::{__m128i, __m256, __m256i, _mm256_add_epi16, _mm256_add_epi32, _mm256_add_ps, _mm256_hadd_epi32, _mm256_madd_epi16, _mm256_maddubs_epi16, _mm256_max_epi16, _mm256_max_epi32, _mm256_mul_ps, _mm256_packus_epi16, _mm256_set1_epi16, _mm256_setzero_si256, _mm256_sra_epi16, _mm256_sub_epi16};

pub(crate) type VEPI8 = __m256i;
pub(crate) type VEPI16 = __m256i;
pub(crate) type VEPI32 = __m256i;

pub(crate) const VEPI8_CNT: usize = 32;
pub(crate) const VEPI16_CNT: usize = 16;
pub(crate) const VEPI32_CNT: usize = 8;
pub(crate) const VPS32_CNT: usize = 8;


pub(crate) unsafe fn vepi16_add(a: VEPI16, b: VEPI16 ) -> VEPI16 {
    _mm256_add_epi16(a, b)
}

pub(crate) unsafe fn vepi16_sub(a: VEPI16, b: VEPI16 ) -> VEPI16 {
    _mm256_sub_epi16(a, b)
}

pub(crate) unsafe fn vepi16_max(a: VEPI16, b: VEPI16 ) -> VEPI16 {
    _mm256_max_epi16(a, b)
}

pub(crate) unsafe fn vepi16_madd(a: VEPI16, b: VEPI16 ) -> VEPI16 {
    _mm256_madd_epi16(a, b)
}

pub(crate) unsafe fn vepi16_one() -> VEPI16 {
    _mm256_set1_epi16(1)
}

pub(crate) unsafe fn vepi16_zero() -> VEPI16 {
    _mm256_setzero_si256()
}

pub(crate) unsafe fn vepi16_srai(a: VEPI16, count: __m128i) -> VEPI16 {
    _mm256_sra_epi16(a, count)
}

pub(crate) unsafe fn vepi16_packu(a: VEPI16, b: VEPI16 ) -> VEPI16 {
    _mm256_packus_epi16(a, b)
}

pub(crate) unsafe fn vepi16_maubs(a: VEPI16, b: VEPI16 ) -> VEPI16 {
    _mm256_maddubs_epi16(a, b)
}


// VEPI32
pub(crate) unsafe fn vepi32_add(a: VEPI32, b: VEPI32) -> VEPI32 {
    _mm256_add_epi32(a, b)
}

pub(crate) unsafe fn vepi32_max(a: VEPI32, b: VEPI32) -> VEPI32 {
    _mm256_max_epi32(a, b)
}

pub(crate) unsafe fn vepi32_hadd(a: VEPI32, b: VEPI32) -> VEPI32 {
    _mm256_hadd_epi32(a, b)
}

pub(crate) unsafe fn vepi32_zero() -> VEPI32 {
    _mm256_setzero_si256()
}

