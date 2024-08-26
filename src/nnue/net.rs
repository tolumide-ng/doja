use super::{align64::Align64, commons::{FEATURES, HIDDEN}};

/// Container for all network parameters
#[repr(C)]
pub(crate) struct NNUEParams {
    pub(crate) feature_weights: Align64<[i16; FEATURES * HIDDEN]>,
    pub(crate) features_bias: Align64<[i16; HIDDEN]>,
    pub(crate) output_weights: Align64<[i16; HIDDEN * 2]>,
    pub(crate) output_bias: i16,
}


/// NNUE model is initialized from binary values
pub(crate) static MODEL: NNUEParams = unsafe { std::mem::transmute(*include_bytes!("../../bins/net.bin")) };