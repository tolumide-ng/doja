pub(crate) const MAX_DEPTH: usize = 127;
/// type returned by the network
pub(crate) type Eval = i32;

/// 768
pub(crate) const FEATURES: usize = 768;
/// 1024
pub(crate) const HIDDEN: usize = 1024;


// Clipped ReLu bounds
pub(crate) const CR_MIN: i16 = 0;
pub(crate) const CR_MAX: i16 = 255;


// Quantization factors
pub(crate) const QA: i32 = 255;
pub(crate) const QAB: i32 = 255 * 64;

/// Eval scaling factor
pub(crate) const SCALE: i32 = 400;