use std::usize;
// / U: is the size of the layer e.g. L1_size can be 518, or 768 e.t.c
// / M: is the size of the each column in the layer, e.g. on AVX-2 with 256 register 
// / if we are using i32, then M = 8 (i.e, 8 x 32 = 256)
// / if we're using i16, then M = 16 (i.e, 16 x 16 = 256)

const COLUMNS: usize = 8;

/// Column-Major layout: Access to the individual elements in the following form
/// L0.weight[column_index][row_index]
// #[repr(align(32))]
/// M = INPUT_SIZE * OUTPUT_SIZE
/// N = OUTPUT_SIZE 
pub(crate) struct LinearLayer<const M: usize, const N: usize> {
    pub(crate) weight: [[i16; M]; 2], // where U = 2(colors) * layer's size
    pub(crate) bias: [i16; N],
    pub(crate) num_inputs: usize,
    pub(crate) num_outputs: usize,
}


impl<const M: usize, const N: usize> LinearLayer<M, N> {
    pub(crate) fn new(weight: [[i16; M]; 2], bias: [i16; N]) -> Self {
        Self { weight, bias, num_inputs: weight[0].len()/bias.len(), num_outputs : bias.len() }
    }

    pub(crate) fn run(&self, input: Vec<i8>) {
        // Assuming the expected output is size i32
        const REGISTER_WIDTH: usize = 256/8;
        assert!((self.num_inputs % REGISTER_WIDTH ) == 0, "We're processing 32 elements at a time");
        assert!(self.num_outputs % 4 == 0, "We unroll by 4");
        let num_in_chunks: usize = self.num_inputs / REGISTER_WIDTH;
        let num_out_chunks: usize = self.num_outputs / 4;

        for i in 0..num_out_chunks {}
    }
}