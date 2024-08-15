// use crate::nnue::simd::wrap_simd_register;
// use crate::nnue::simd:

// #![allow(non_camel_case_types)]
// use std::arch::x86_64::*;

use std::arch::x86_64::*;

use crate::wrap_simd_register;

wrap_simd_register!(__m512i, i8, VecI8);
wrap_simd_register!(__m512i, i16, VecI16);
wrap_simd_register!(__m512i, i32, VecI32);
wrap_simd_register!(__m512i, i64, VecI64);
wrap_simd_register!(__m512i, f32, VecF32);

#[inline]
pub unsafe fn zero_i16() -> VecI16 {
    VecI16::from_raw(_mm512_setzero_si512())
}

#[inline]
pub unsafe fn zero_i32() -> VecI32 {
    VecI32::from_raw(_mm512_setzero_si512())
}

#[inline]
pub unsafe fn splat_i16(n: i16) -> VecI16 {
    VecI16::from_raw(_mm512_set1_epi16(n))
}

#[inline]
pub unsafe fn splat_i32(n: i32) -> VecI32 {
    VecI32::from_raw(_mm512_set1_epi32(n))
}

#[inline]
pub unsafe fn load_i8(src: *const i8) -> VecI8 {
    // check alignment in debug mode
    debug_assert!((src as usize) % std::mem::align_of::<VecI8>() = 0);
    _mm512_store_si512(dst.cast(), vec.inner());
}