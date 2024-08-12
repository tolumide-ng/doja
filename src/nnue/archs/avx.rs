use std::arch::x86_64::{__m128i, __m256, _mm256_add_ps, _mm_add_epi16, _mm_add_epi32, _mm_hadd_epi32, _mm_madd_epi16, _mm_maddubs_epi16, _mm_max_epi16, _mm_max_epi32, _mm_packus_epi16, _mm_set1_epi16, _mm_setzero_si128, _mm_sra_epi16, _mm_srai_epi16, _mm_sub_epi16};



pub(crate) type VEPI8 = __m128i;
pub(crate) type VEPI16 = __m128i;
pub(crate) type VEPI32 = __m128i;

pub(crate) const VEPI8_CNT: usize = 16;
pub(crate) const VEPI16_CNT: usize = 8;
pub(crate) const VEPI32_CNT: usize = 4;
pub(crate) const VPS32_CNT: usize = 8;

#[inline]
pub(crate) unsafe fn vepi16_add(a: VEPI16, b: VEPI16) -> VEPI16 {
    _mm_add_epi16(a, b)
}

#[inline]
pub(crate) unsafe fn vepi16_sub (a: VEPI16, b: VEPI16) -> VEPI16 {
    _mm_sub_epi16(a, b)
}

#[inline]
pub(crate) unsafe fn vepi16_max(a: VEPI16, b: VEPI16) -> VEPI16 {
    _mm_max_epi16(a, b)
}

#[inline]
pub(crate) unsafe fn vepi16_madd (a: VEPI16, b: VEPI16) -> VEPI16 {
    _mm_madd_epi16(a, b)
}

#[inline]
pub(crate) unsafe fn vepi16_one() -> VEPI16 {
    _mm_set1_epi16(1)
}

#[inline]
pub(crate) unsafe fn vepi16_zero() -> VEPI16 {
    _mm_setzero_si128()
}

#[inline]
pub(crate) unsafe fn vepi16_srai(a: VEPI16, count: VEPI16) -> VEPI16 {
        _mm_sra_epi16(a, count)
}

#[inline]
pub(crate) unsafe fn vepi16_packu(a: VEPI16, b: VEPI16) -> VEPI16 {
    _mm_packus_epi16(a, b)
}

#[inline]
pub(crate) unsafe fn vepi16_maubs(a: VEPI16, b: VEPI16) -> VEPI16 {
    _mm_maddubs_epi16(a, b)
}

#[inline]
pub(crate) unsafe fn vepi32_add(a: VEPI16, b: VEPI16) -> VEPI16 {
    _mm_add_epi32(a, b)
}

#[inline]
pub(crate) unsafe fn vepi32_max(a: VEPI16, b: VEPI16) -> VEPI16 {
    _mm_max_epi32(a, b)
}

#[inline]
pub(crate) unsafe fn vepi32_hadd(a: VEPI16, b: VEPI16) -> VEPI16 {
    _mm_hadd_epi32(a, b)
}

#[inline]
pub(crate) unsafe fn vepi32_zero() -> VEPI16 {
    _mm_setzero_si128()
}
