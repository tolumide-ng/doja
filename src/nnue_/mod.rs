// The purpose of the ClippedReLu is to introduce non-linearity to the network.

use constants::customKA0::*;
use linear_layer::LinearLayer;
use network::NNUEParams;

pub mod quantmoid;
pub(crate) mod calc;
pub(crate) mod accumulator;
pub(crate) mod linear_layer;
pub(crate) mod feature_idx;
pub(crate) mod constants;
pub(crate) mod network;
pub(crate) mod relu;
pub(crate) mod align64;

// All layers are linear, and all hidden neurons use ClippedReLU activation function

// HalfKP is just P taken 64 times, once for each king square

// pub(crate) static MODEL: LinearLayer<{INPUT*L1_SIZE}, L1_SIZE, i16> = unsafe {
//     let bytes: &[u8] = include_bytes!("../../bins/net.bin");
//     // const _: () = assert_eq!(BYTES.len(), std::mem::size_of::<LinearLayer<{768*1024}, 1024, i16>>());
//     std::ptr::read_unaligned(bytes.as_ptr() as *const LinearLayer<{INPUT*L1_SIZE}, L1_SIZE, i16>)
// };

pub(crate) static PARAMS: NNUEParams<{INPUT * L1_SIZE}, L1_SIZE, {L1_SIZE*2}, i16> = unsafe {
    let bytes: &[u8] = include_bytes!("../../bins/net.bin");
    std::ptr::read_unaligned(bytes.as_ptr() as *const NNUEParams<{INPUT*L1_SIZE}, {L1_SIZE}, {L1_SIZE * 2}, i16>)
};


pub(crate) fn checkings() {
    // let linear = LinearLayer {
    //     weight: [[10, 20, 304, 100, 32, 43, 99, 22, 243, 4354, 3786, 452, 4956, 289, 100, 32, 43, 99, 22, 243,],
    //              [10, 20, 304, 100, 32, 43, 99, 22, 243, 4354, 3786, 452, 4956, 289, 100, 32, 43, 99, 22, 243,],
    //              [10, 20, 304, 100, 32, 43, 99, 22, 243, 4354, 3786, 452, 4956, 289, 100, 32, 43, 99, 22, 243,],
    //              [10, 20, 304, 100, 32, 43, 99, 22, 243, 4354, 3786, 452, 4956, 289, 100, 32, 43, 99, 22, 243,],
    //              [10, 20, 304, 100, 32, 43, 99, 22, 243, 4354, 3786, 452, 4956, 289, 100, 32, 43, 99, 22, 243,],
    //              [10, 20, 304, 100, 32, 43, 99, 22, 243, 4354, 3786, 452, 4956, 289, 100, 32, 43, 99, 22, 243,],
    //              [10, 20, 304, 100, 32, 43, 99, 22, 243, 4354, 3786, 452, 4956, 289, 100, 32, 43, 99, 22, 243,],
    //              [10, 20, 304, 100, 32, 43, 99, 22, 243, 4354, 3786, 452, 4956, 289, 100, 32, 43, 99, 22, 243,],
    //              [10, 20, 304, 100, 32, 43, 99, 22, 243, 4354, 3786, 452, 4956, 289, 100, 32, 43, 99, 22, 243,],
    //              [10, 20, 304, 100, 32, 43, 99, 22, 243, 4354, 3786, 452, 4956, 289, 100, 32, 43, 99, 22, 243,],
    //              ],
    //     bias: [4, 32, 12, 1, 3, 4, 5, 6, 8, 90, 3, 5, 32, 5, 6, 5, 6, 8, 89, 90],
    //     num_inputs: 20,
    //     num_outputs: 20,
    // };

    // let mut acc = Accumualator::default();
    // Accumualator::refresh_accumulator(linear, &mut acc,
    //      &vec![FeatureIdx::new(3), FeatureIdx::new(4), FeatureIdx::new(5), FeatureIdx::new(18)],
    //      Color::White);
    
    let network = &PARAMS;
    println!("linear layer xxx {:?}", network);
    // let params = &
}

