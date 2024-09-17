use std::usize;

const COLUMNS: usize = 8;

/// Column-Major layout: Access to the individual elements in the following form
/// L0.weight[column_index][row_index]
/// U: is the size of the layer e.g. L1_size can be 518, or 768 e.t.c
/// M: is the size of the each column in the layer, e.g. on AVX-2 with 256 register 
/// if we are using i32, then M = 8 (i.e, 8 x 32 = 256)
/// if we're using i16, then M = 16 (i.e, 16 x 16 = 256)
pub(crate) struct LinearLayer<T, const U: usize, const M: usize> {
    pub(crate) weight: [[T; M]; U], // where U = 2(colors) * layer's size
    pub(crate) bias: [T; U],
    pub(crate) num_inputs: usize,
    pub(crate) num_outputs: usize,
}

