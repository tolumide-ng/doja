use super::L1_SIZE;

pub mod generic;
pub mod ssse3;

#[cfg(feature = "nnz-counts")]
pub static NNZ_COUNTS: [std::sync::atomic::AtomicU64; L1_SIZE] =
    { unsafe { std::mem::transmute([0u64; L1_SIZE]) } };
    