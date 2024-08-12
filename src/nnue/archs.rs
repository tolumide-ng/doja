#[cfg(any(target_feature = "avx2", target_feature = "sse3"))]
pub mod avx2;
#[cfg(target_feature = "avx")]
pub mod avx;


use std::{arch::x86_64::{__m256, _mm256_add_ps, _mm256_hadd_ps, _mm256_max_ps, _mm256_mul_ps, _mm256_setzero_ps}, usize};


pub(crate) type VPS32 = __m256;



#[inline]
pub(crate) unsafe fn vps32_add(a: VPS32, b: VPS32) -> VPS32 {
    _mm256_add_ps(a, b)
}

#[inline]
pub(crate) unsafe fn vps32_mul(a: VPS32, b: VPS32) -> VPS32 {
    _mm256_mul_ps(a, b)
}

#[inline]
pub(crate) unsafe fn vps32_max(a: VPS32, b: VPS32) -> VPS32 {
    _mm256_max_ps(a, b)
}

#[inline]
pub(crate) unsafe fn vps32_hadd(a: VPS32, b: VPS32) -> VPS32 {
    _mm256_hadd_ps(a, b)
}

#[inline]
pub(crate) unsafe fn vps32_zero() -> VPS32 {
    _mm256_setzero_ps()
}


#[inline]
pub(crate) unsafe fn vps32_fma(a: VPS32, b: VPS32, c: VPS32) -> VPS32 {
    vps32_add(vps32_mul(a, b), c)
}