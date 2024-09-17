// The purpose of the ClippedReLu is to introduce non-linearity to the network.


pub mod quantmoid;
pub(crate) mod calc;
pub(crate) mod accumulator;
pub(crate) mod network;
pub(crate) mod feature_idx;


// All layers are linear, and all hidden neurons use ClippedReLU activation function

// HalfKP is just P taken 64 times, once for each king square

