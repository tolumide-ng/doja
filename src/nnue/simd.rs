mod features {
    #[cfg(target_feature = "avx512f")]
    pub mod avx512;
    #[cfg(all(target_feature = "avx2", not(target_feature = "avx512f")))]
    pub mod avx2;
    #[cfg(all(target_feature = "ssse3", not(target_feature = "avx2"), not(target_feature = "avx512f")))]
    use ssse3::*;

    #[cfg(target_feature = "avx512f")]
    use avx512::*;
    #[cfg(all(target_feature = "avx2", not(target_feature = "avx512f")))]
    pub mod avx2;
    #[cfg(all(target_feature = "ssse3", not(target_feature = "avx2"), not(target_feature = "avx512f")))]
    use ssse3::*;
}

use features::*;

#[cfg(any(target_feature = "ssse3", target_feature = "avx2", target_feature = "avx512f"))]
#[inline]
pub(crate) fn reinterpret_i32s_as_i8s(a: VecI32) -> VecI8 {
    VecI8::from_raw(*a)
}


#[inline]
pub(crate) const fn mm_shuffle(z: i32, y: i32, x: i32, w: i32) -> i32 {
    ((z) << 6) | ((y) << 4) | ((x) << 2) | w
}


/// Given a regular type and a SIMD register type, and the new type name, create a new type that wraps the register type
#[allow(unused_macros)]
#[macro_export]
macro_rules! wrap_simd_register {
    // ($register_type:ty, $held_type:ty, $new_type:ident) => {
        ($register_type:ty, $new_type:ident) => {
        #[repr(transparent)]
        #[derive(Debug, Copy, Clone)]
        pub struct $new_type($register_type);
        impl $new_type {
            #[inline]
            pub const fn from_raw(value: $register_type) -> Self {
                Self(value)
            }
        }

        impl std::ops::Deref for $new_type {
            type Target = $register_type;
            
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}
