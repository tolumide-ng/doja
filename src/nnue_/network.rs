use std::usize;


/// Column-Major layout: Access to the individual elements in the following form
/// L0.weight[column_index][row_index]
pub(crate) struct LinearLayer<T, const U: usize> {
    pub(crate) weight: [T; U], // where U = 2(colors) * layer's size
    pub(crate) bias: [T; U],
    pub(crate) num_inputs: usize,
    pub(crate) num_outputs: usize,
}


// impl 