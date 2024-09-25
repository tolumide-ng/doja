// The purpose of the ClippedReLu is to introduce non-linearity to the network.

use accumulator::Feature;
use calc::halfka::halfka_index;
use constants::customKA0::*;
use network::{NNUEParams, NNUEState};

use crate::{board::{fen::FEN, piece::Piece, state::board::Board}, constants::TRICKY_POSITION, nnue::{commons::HIDDEN, net::nnue_index}, squares::Square};

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

    use crate::board::piece::Piece::*;
    
    let network = &PARAMS;
    let b = Board::parse_fen(TRICKY_POSITION).unwrap();
    println!("BEFORE IT IS ");
    let nn = NNUEState::<Feature, 1024>::from(b);

    println!("AFTER IT WAS ");

    println!("the nn is {:#?}", nn);
    // println!("((((((((((((((((((moddd:::::::::::::::::::::: {:?}
    //     {}, {}, {}, {}, {}", PARAMS.input_weight[2], 
    // PARAMS.input_weight[70000],
    // PARAMS.input_weight[76],
    // PARAMS.input_weight[28],
    // PARAMS.input_weight[201],
    // PARAMS.input_weight[99],);

    // println!("linear layer xxx {:?}", network.input_weight.len());
    // // let xxo = halfka_index(Piece::BK, Square::E8, Square::A5);
    // let xxo = nnue_index(WB    , Square::A4);

    // println!(">>>> xxxx w={:#?} \t b={}", xxo.0, xxo.1);

    // println!("{}", 63 + (64 * (5 + 6 * 1)));
    // let params = &
}

