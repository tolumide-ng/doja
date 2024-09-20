// The purpose of the ClippedReLu is to introduce non-linearity to the network.

use accumulator::{Accumualator, Feature};
use feature_idx::FeatureIdx;
use network::LinearLayer;

use crate::color::Color;


pub mod quantmoid;
pub(crate) mod calc;
pub(crate) mod accumulator;
pub(crate) mod network;
pub(crate) mod feature_idx;
pub(crate) mod constants;

// All layers are linear, and all hidden neurons use ClippedReLU activation function

// HalfKP is just P taken 64 times, once for each king square

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
}