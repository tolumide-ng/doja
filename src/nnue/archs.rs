use std::{arch::x86_64::{__m128i, __m256}, usize};

pub type vepi8 = __m128i;
pub type vepi16 = __m128i;
pub type vepi32 = __m128i;
pub type vps32 = __m256;

pub const VEPI8_CNT: usize = 16;
pub const VEPI16_CNT: usize = 8;
pub const VEPI32_CNT: usize = 4;
pub const VPS32_CNT: usize = 8;


// type vepi16_add = mmaddepi

#[cfg(target_feature = "avx")]
pub(crate) mod arch {
    use std::arch::x86_64::{_mm_add_epi16, _mm_add_epi32, _mm_hadd_epi32, _mm_madd_epi16, _mm_maddubs_epi16, _mm_max_epi32, _mm_packus_epi16, _mm_set1_epi16, _mm_setzero_si128, _mm_sra_epi16, _mm_srai_epi16, _mm_sub_epi16};

    use super::{vepi16, vepi8};

    pub(crate) unsafe fn vepi16_add(a: vepi16, b: vepi16) -> vepi16 {
        _mm_add_epi16(a, b)
    }

    pub(crate) unsafe fn vepi16_sub (a: vepi16, b: vepi16) -> vepi16 {
        _mm_sub_epi16(a, b)
    }

    pub(crate) unsafe fn vepi16_max(a: vepi16, b: vepi16) -> vepi16 {
        _mm_max_epi32(a, b)
    }

    pub(crate) unsafe fn vepi16_madd (a: vepi16, b: vepi16) -> vepi16 {
        _mm_madd_epi16(a, b)
    }

    pub(crate) unsafe fn vepi16_one() -> vepi16 {
        _mm_set1_epi16(1)
    }

    pub(crate) unsafe fn vepi16_zero() -> vepi16 {
        _mm_setzero_si128()
    }

    pub(crate) unsafe fn vepi16_srai(a: vepi16, count: vepi16) -> vepi16 {
          _mm_sra_epi16(a, count)
    }

    pub(crate) unsafe fn vepi16_packu(a: vepi16, b: vepi16) -> vepi16 {
        _mm_packus_epi16(a, b)
    }

    pub(crate) unsafe fn vepi16_maubs(a: vepi16, b: vepi16) -> vepi16 {
        _mm_maddubs_epi16(a, b)
    }

    pub(crate) unsafe fn vepi32_add(a: vepi16, b: vepi16) -> vepi16 {
        _mm_add_epi32(a, b)
    }

    pub(crate) unsafe fn vepi32_max(a: vepi16, b: vepi16) -> vepi16 {
        _mm_max_epi32(a, b)
    }

    pub(crate) unsafe fn vepi32_hadd(a: vepi16, b: vepi16) -> vepi16 {
        _mm_hadd_epi32(a, b)
    }

    pub(crate) unsafe fn vepi32_zero() -> vepi16 {
        _mm_setzero_si128()
    }
}


#[cfg(target_feature = "avx")]
pub(crate) use arch::*;