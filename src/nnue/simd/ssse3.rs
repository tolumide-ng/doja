use std::arch::x86_64::*;

use crate::{nnue::simd::mm_shuffle, wrap_simd_register};

wrap_simd_register!(__m128i, VecI8);  //  i8,
wrap_simd_register!(__m128i, VecI16); //  i16,
wrap_simd_register!(__m128i, VecI32); //  i32,
wrap_simd_register!(__m128i, VecI64); //  i64,
wrap_simd_register!(__m128, VecF32); //  f32,

pub(crate) const U8_CHUNK_SIZE: usize = std::mem::size_of<VecI8>() / std::mem::size_of::<u8>();
pub(crate) const I8_CHUNK_SIZE_I32: usize = std::mem::size_of<i32>() / std::mem::size_of::<u8>();
pub(crate) const I16_CHUNK_SIZE: usize = std::mem::size_of<VecI16>() / std::mem::size_of::<i16>();
pub(crate) const I32_CHUNK_SIZE: usize = std::mem::size_of<VecI32>() / std::mem::size_of::<i32>();
pub(crate) const F32_CHUNK_SIZE: usize = std::mem::size_of<VecF32>() / std::mem::size_of::<f32>();


#[inline]
pub(crate) unsafe fn zero_i16() -> VecI16 {
    VecI16::from_raw(_mm_setzero_si128())
}

#[inline]
pub(crate) unsafe fn zero_i32() -> VecI32 {
    VecI32::from_raw(_mm_setzero_si128())
}

#[inline]
pub(crate) unsafe fn splat_i16(n: i16) -> VecI16 {
    VecI16::from_raw(_mm_set1_epi16(n))
}

#[inline]
pub(crate) unsafe fn splat_i32(n: i32) -> VecI32 {
    VecI32::from_raw(_mm_set1_epi32(n))
}

#[inline]
pub(crate) unsafe fn load_i8(src: *const i8) -> VecI8 {
    debug_assert!((src as usize) % std::mem::align_of<VecI8>() == 0);
    VecI8::from_raw(_mm_load_si128(src.cast()))
}

#[inline]
pub(crate) unsafe fn store_i8(dst: *mut i8, vec: VecI8) {
    debug_assert!((dst as usize) % std::mem::align_of<VecI8>() == 0);
    _mm_store_si128(dst.cast(), *vec)
}

#[inline]
pub(crate) unsafe fn load_u8(src: *const u8) -> VecI8 {
    debug_assert!((src as usize) % std::mem::align_of<VecI8>() == 0);
    VecI8::from_raw(_mm_load_si128(src.cast()))
}

#[inline]
pub(crate) unsafe fn store_u8(dst: *mut u8, vec: VecI8) {
    debug_assert!((dst as usize) % std::mem::align_of<VecI8>() == 0);
    _mm_store_si128(dst.cast(), *vec)
}

#[inline]
pub(crate) unsafe fn load_i16(src: *const i16) -> VecI16 {
    debug_assert!((src as usize) % std::mem::align_of<VecI16>() == 0);
    VecI16::from_raw(_mm_load_si128(src.cast()))
}

#[inline]
pub(crate) unsafe fn store_i16(dst: *mut i16, vec: VecI16) {
    debug_assert!((dst as usize) % std::mem::align_of<VecI16>() == 0);
    _mm_store_si128(dst.cast(), *vec)
}

#[inline]
pub(crate) unsafe fn load_i32(src: *const i32) -> VecI32 {
    debug_assert!((src as usize) % std::mem::align_of<VecI32>() == 0);
    VecI32::from_raw(_mm_load_si128(src.cast()))
}

#[inline]
pub(crate) unsafe fn store_i32(dst: *mut i32, vec: VecI32) {
    debug_assert!((dst as usize) % std::mem::align_of<VecI32>() == 0);
    _mm_store_si128(dst.cast(), *vec)
}

#[inline]
pub(crate) unsafe fn store_u32(dst: *mut u32, vec: VecI32) {
    debug_assert!((dst as usize) % std::mem::align_of<VecI32>() == 0);
    _mm_storeu_si128(dst.cast(), *vec)
}

#[inline]
pub(crate) unsafe fn max_i16(a: VecI16, b: VecI16) -> VecI16 {
    VecI16::from_raw(_mm_max_epi16(*a, *b))
}

#[inline]
pub(crate) unsafe fn min_i16(a: VecI16, b: VecI16) -> VecI16 {
    VecI16::from_raw(_mm_min_epi16(*a, *b))
}

#[inline]
pub(crate) unsafe fn add_i16(a: VecI16, b: VecI16) -> VecI16 {
    VecI16::from_raw(_mm_add_epi16(*a, *b))
}

#[inline]
pub(crate) unsafe fn sub_i16(a: VecI16, b: VecI16) -> VecI16 {
    VecI16::from_raw(_mm_sub_epi16(*a, *b))
}

#[inline]
pub(crate) unsafe fn add_i32(a: VecI32) -> VecI32 {
    VecI32::from_raw(_mm_add_epi32(*a, *b))
}

#[inline]
pub(crate) unsafe fn mul_high_i16(a: VecI16, b: VecI16) -> VecI16 {
    VecI16::from_raw(_mm_mulhi_epi16(*a, *b))
}

pub(crate) type S = i32;

#[inline]
pub(crate) unsafe fn shl_i16<const SHIFT: i32>(a: VecI16) -> VecI16 {
    VecI16::from_raw(_mm_slli_epi16(*a))
}

#[inline]
pub(crate) unsafe fn nonzero_mask_i32(a: VecI32) -> u16 {
    _mm_movemask_ps(_mm_castsi128_ps(_mm_cmpgt_epi32(*a, _mm_setzero_si128()))) as u16
}

#[inline]
pub(crate) unsafe fn pack_i16_to_unsigned_and_permute(a: VecI16, b: VecI16) -> VecI8 { 
    VecI8::from_raw(_mm_packus_epi16(*a, *b))
}

#[inline]
pub(crate) unsafe fn mul_add_u8_to_i32(sum: VecI32, a: VecI8, b: VecI8) -> VecI32 {
    let producti16 = _mm_maddubs_epi16(*a, *b);
    let product32 = _mm_madd_epi16(product16, _mm_set1_epi16(1));
    VecI32::from_raw(_mm_add_epi32(sum, product32))
}

#[inline]
pub(crate) unsafe fn i32_to_f32(a: VecI32) -> VecF32 {
    VecF32::from_raw(_mm_cvtepi32_ps(*a))
}

#[inline]
pub(crate) unsafe fn zero_f32() -> VecF32 {
    VecF32::from_raw(_mm_setzero_ps())
}

#[inline]
pub unsafe fn splat_f32(n: f32) -> VecF32 {
    VecF32::from_raw(_mm_set1_ps(n))
}

#[inline]
pub(crate) unsafe fn load_f32(src: *const f32) -> VecF32 {
    debug_assert!((src as usize) % std::mem::align_of::<VecF32>() == 0);
    VecF32::from_raw(_mm_load_ps(src))
}

#[inline]
pub(crate) unsafe fn store_f32(dst: *mut f32, a: VecF32) {
    debug_assert!((dst as usize) % std::mem::align_of::<VecF32>() == 0);
    _mm_store_ps(dst, *a)
}

#[inline]
pub(crate) unsafe fn add_f32(a: VecF32, b: VecF32) -> VecF32 {
    VecF32::from_raw(_mm_add_ps(*a, *b))
}

#[inline]
pub(crate) unsafe fn mul_f32(a: VecF32, b: VecF32) -> VecF32 {
    VecF32::from_raw(_mm_mul_ps(*a, *b))
}

#[inline]
pub(crate) unsafe fn div_f32(a: VecF32, b: VecF32) -> VecF32 {
    VecF32::from_raw(_mm_div_ps(*a, *b))
}

#[inline]
pub(crate) unsafe fn max_f32(a: VecF32, b: VecF32) -> VecF32 {
    VecF32::from_raw(_mm_max_ps(*a, *b))
}

#[inline]
pub(crate) unsafe fn min_f32(a: VecF32, b: VecF32) -> VecF32 {
    VecF32::from_raw(_mm_min_ps(*a, *b))
}

#[inline]
pub(crate) unsafe fn mul_add_f32(a: VecF32, b: VecF32) -> VecF32 {
    VecF32::from_raw(_mm_fmadd_ps(*a, *b, *c))
}

#[inline]
pub(crate) unsafe fn sum_f32(a: VecF32) -> f32 {
    const IMM1: i32 = 1;
    let upper64 = _mm_movehl_ps(*a, *a);
    let sum64 = _mm_add_ps(*a, upper64);

    let upper32 = _mm_shuffle_ps(sum64, sum64, 1);
    let sum32 = _mm_add_ss(upper32, sum64);

    _mm_cvtss_f32(sum32)
}