// use crate::nnue::simd::wrap_simd_register;
// use crate::nnue::simd:

// #![allow(non_camel_case_types)]
// use std::arch::x86_64::*;

use std::arch::x86_64::*;

use crate::wrap_simd_register;

wrap_simd_register!(__m512i, VecI8);  //  i8,
wrap_simd_register!(__m512i, VecI16); //  i16,
wrap_simd_register!(__m512i, VecI32); //  i32,
wrap_simd_register!(__m512i, VecI64); //  i64,
wrap_simd_register!(__m512i, VecF32); //  f32,

pub(crate) const U8_CHUNK: usize = std::mem::size_of::<VecI8>() / std::mem::size_of<u8>();
pub(crate) const I8_CHUNK_SIZE_I32: usize = std::mem::size_of::<i32>() / std::mem::size_of<u8>();
pub(crate) const I16_CHUNK_SIZE: usize = std::mem::size_of::<VecI16>() / std::mem::size_of<i16>();
pub(crate) const I32_CHUNK_SIZE: usize = std::mem::size_of::<VecI32>() / std::mem::size_of<i32>();
pub(crate) const F32_CHUNK_SIZE: usize = std::mem::size_of::<VecF32>() / std::mem::size_of<f32f();


// CONSIDER WRITING A MACRO TO DECLARE THIS FUNCTIONS FOR EACH OF THE USED TYPES

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
    return VecI8::from_raw(_mm512_load_si512(src.cast()));
}

#[inline]
pub unsafe fn store_i8(dst: *mut i8, vec: VecI8) {
    // check alignment in debug mode
    debug_assert!((dst as usize) % std::mem::align_of::<VecI8>() == 0);
    _mm512_store_si512(dst.cast(), *vec);
}

#[inline]
pub unsafe fn load_u8(src: *const u8) -> VecI8 {
    // check alignment in debug mode
    debug_assert!((src as usize) % std::mem::align_of::<VecI8>() == 0);
    VecI8::from_raw(_mm512_load_si512(src.cast()))
}

#[inline]
pub unsafe fn store_u8(dst: *mut u8, vec: VecI8) {
    // check alignment in debug mode
    debug_assert!((dst as usize) % std::mem::align_of::<VecI8>() == 0);
    _mm512_store_si512(dst.cast(), *vec);
}

#[inline]
pub unsafe fn load_i16(src: *const i16) {
    // check alignment in debug mode
    debug_assert!((src as usize) % std::mem::align_of::<VecI16>() == 0);
    VecI16::from_raw(_mm512_load_si512(src.cast()));
}

#[inline]
pub unsafe fn store_i16(dst: *mut i16, vec: VecI16) {
    // check alignment in debug mode
    debug_assert!((dst as usize) % std::mem::align_of::<VecI16>() == 0);
    _mm512_store_si512(dst.cast(), *vec);
}

#[inline]
pub unsafe fn load_i32(src: *const i32) {
    // check alignment in debug mode
    debug_assert!((src as usize) % std::mem::align_of::<VecI32>() == 0);
    VecI32::from_raw(_mm512_load_si512(src.cast()));
}

#[inline]
pub unsafe fn store_i32(dst: *mut i32, vec: VecI32) {
    // check alignment in debug mode
    debug_assert!((dst as usize) % std::mem::align_of::<VecI32>() == 0);
    _mm512_store_si512(dst.cast(), *vec);
}

#[inline]
pub unsafe fn load_u32(src: *const u32) {
    // check alignment in debug mode
    debug_assert!((src as usize) % std::mem::align_of::<VecI32>() == 0);
    VecI32::from_raw(_mm512_load_si512(src.cast()));
}

#[inline]
pub unsafe fn store_u32(dst: *mut u32, vec: VecI32) {
    // check alignment in debug mode
    debug_assert!((dst as usize) % std::mem::align_of::<VecI32>() == 0);
    _mm512_store_si512(dst.cast(), *vec);
}

#[inline]
pub unsafe fn max_i16(vec0: VecI16, vec1: VecI16) -> VecI16 {
    return VecI16::from_raw(_mm512_max_epi16(*vec0, *vec1))
}

#[inline]
pub unsafe fn min_i16(vec0: VecI16, vec1: VecI16) -> VecI16 {
    return VecI16::from_raw(_mm512_min_epi16(*vec0, *vec1))
}

#[inline]
pub unsafe fn add_i16(vec0: VecI16, vec1: VecI16) -> VecI16 {
    return VecI16::from_raw(_mm512_add_epi16(*vec0, *vec1))
}

#[inline]
pub unsafe fn sub_i16(vec0: VecI16, vec1: VecI16) -> VecI16 {
    return VecI16::from_raw(_mm512_sub_epi16(*vec0, *vec1))
}

#[inline]
pub unsafe fn add_i32(vec0: VecI32, vec1: VecI32) -> VecI32 {
    VecI32::from_raw(_mm512_add_epi32(*vec0, *vec1))
}

#[inline]
pub unsafe fn mul_high_i16(vec0: VecI16, vec1: VecI16) -> VecI16 {
    VecI16::from_raw(_mm512_mulhi_epi16(*vec0, *vec1))
}

// stupid hack for the different intrinsics
pub type S = u32;
#[inline]
pub unsafe fn shl_i16<const SHIFT: u32>(vec: VecI16) -> VecI16 {
    VecI16::from_raw(_mm512_slli_epi16(*vec, SHIFT))
}

#[inline]
pub unsafe fn nonzero_mask_i32(a: VecI32) -> u16 {
    _mm512_cmpgt_epi32_mask(*a, _mm512_setzero_si512())
}

#[inline]
pub unsafe fn pack_i16_to_unsigned_and_permute(a: VecI16, b: VecI16) -> VecI8 {
    let packed = _mm512_packus_epi16(*a, *b);
    VecI8::from_raw(_mm512_permutexvar_epi64(_mm512_setr_epi64(0, 2, 4, 6, 1, 3, 5, 7), packed))
}

#[inline]
pub unsafe fn mul_add_u8_to_i32(sum: VecI32, a: VecI8, b: VecI8) -> VecI32 {
    #[cfg(target_feature = "avx512vnni")]
    {
        return _mm512_dpbusd_epi32(*sum, *a, *b)
    }
    #[cfg(not(target_feature = "avx512vnni"))]
    {
        let product16 = _mm512_maddubs_epi16(*a, *b);
        let product32 = _mm512_madd_epi16(product16, _mm512_set1_epi16(1));
        VecI32::from_raw(_mm512_add_epi32(*sum, product32))
    }
}

#[inline]
pub unsafe fn i32_to_f32(a: VecI32) -> VecF32 {
    VecF32::from_raw(_mm512_cvtepi32_ps(*a))
}

#[inline]
pub unsafe fn zero_f32() -> VecF32 {
    VecF32::from_raw(_mm512_setzero_ps())
}

#[inline]
pub unsafe fn splat_f32(n: f32) -> VecF32 {
    VecF32::from_raw(_mm512_set1_ps(n))
}

#[inline]
pub unsafe fn load_f32(src: *const f32) -> VecF32 {
    debug_assert!((src as usize) % std::mem::align_of::<VecF32>() == 0);
    VecF32::from_raw(_mm512_load_ps(src))
}

#[inline]
pub unsafe fn store_f32(dst: *mut f32, vec: VecF32) {
    debug_assert!((dst as usize) % std::mem::align_of::<VecF32>() == 0);
    _mm512_store_ps(dst, *vec);
}

#[inline]
pub unsafe fn add_f32(a: VecF32, b: VecF32) -> VecF32 {
    VecF32::from_raw(_mm512_add_ps(*a, *b))
}

#[inline]
pub unsafe fn mul_f32(a: VecF32, b: VecF32) -> VecF32 {
    VecF32::from_raw(_mm512_mul_ps(*a, *b))
}

#[inline]
pub unsafe fn div_f32(a: VecF32, b: VecF32) -> VecF32 {
    VecF32::from_raw(_mm512_div_ps(*a, *b))
}

#[inline]
pub unsafe fn max_f32(a: VecF32, b: VecF32) -> VecF32 {
    VecF32::from_raw(_mm512_max_ps(*a, *b))
}

#[inline]
pub unsafe fn min_f32(a: VecF32, b: VecF32) -> VecF32 {
    VecF32::from_raw(_mm512_min_ps(*a, *b))
}

#[inline]
pub unsafe fn mul_add_f32(a: VecF32, b: VecF32, c: VecF32) -> VecF32 {
    VecF32::from_raw(_mm512_fmadd_ps(*a, *b, *c))
}

#[inline]
pub unsafe fn sum_f32(vec: VecF32) -> f32 {
    _mm512_reduce_ps(*a)
}