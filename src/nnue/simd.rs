
#[cfg(target_feature = "avx512f")]
pub mod avx512;
pub mod avx2;

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
