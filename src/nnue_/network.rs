use super::{accumulator::Accumualator, align64::Align64};

#[derive(Debug)] 
#[repr(C)]
pub(crate) struct NNUEParams<const M: usize, const N: usize, const P: usize, T: Copy> {
    // pub(crate) weight: [[T; M]; 2], // where U = 2(colors) * layer's size
    pub(crate) input_weight: Align64<[T; M]>,
    pub(crate) input_bias: Align64<[T; N]>,

    pub(crate) output_weights: [i16; P],
    pub(crate) output_bias: i16,
}

pub(crate) struct NNUEState {
    accumulator_stack: [Accumualator<>]
}