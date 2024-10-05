// The purpose of the ClippedReLu is to introduce non-linearity to the network.

use constants::custom_kp::*;
use feature_idx::FeatureIdx;
use network::NNUEParams;

// pub(crate) const FEATURES: usize = 768;
pub(crate) const HIDDEN: usize = 1024;

use crate::{board::piece::Piece, squares::Square};

pub mod quantmoid;
pub(crate) mod calc;
pub(crate) mod accumulator;
pub(crate) mod feature_idx;
#[allow(dead_code)]
pub(crate) mod constants;
pub(crate) mod network;
pub(crate) mod relu;
pub(crate) mod align64;
pub(crate) mod accumulator_ptr;

// All layers are linear, and all hidden neurons use ClippedReLU activation function

// HalfKP is just P taken 64 times, once for each king square


pub(crate) static PARAMS: NNUEParams<{INPUT * L1_SIZE}, L1_SIZE, {L1_SIZE*2}, i16> = unsafe {
    let bytes: &[u8] = include_bytes!("../../bin/net.bin");
    std::ptr::read_unaligned(bytes.as_ptr() as *const NNUEParams<{INPUT*L1_SIZE}, {L1_SIZE}, {L1_SIZE * 2}, i16>)
};

/// NNUE model is initialized from binary values
// pub(crate) static MODEL: NNUEParamz = unsafe { std::mem::transmute(*include_bytes!("../../bin/net.bin")) };

pub(crate) fn halfka_idx(piece: Piece, sq: Square) -> FeatureIdx {
    // const COLOR_STRIDE: usize = 64 * 6; // number_of_squares * number of pieces per side
    const PIECE_STRIDE: usize = 64;
    let p = (piece as usize) % 6;
    let c = piece.color() as usize;

    let sqfv =sq.flipv() as usize;

    let white_idx = p * PIECE_STRIDE + sqfv;
    let black_idx = p * PIECE_STRIDE + sq as usize;

    let idx = (1 - c) * white_idx + c * black_idx; // Choose based on the color
    FeatureIdx::from(idx * HIDDEN)
}


pub(crate) fn checkings() {
    // 
}

