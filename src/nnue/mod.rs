// The purpose of the ClippedReLu is to introduce non-linearity to the network.

use constants::custom_kp::*;
use feature_idx::FeatureIdx;
use network::NNUEParams;
use crate::color::Color::*;


// pub(crate) const FEATURES: usize = 768;
pub(crate) const HIDDEN: usize = 1024;

use crate::{board::piece::Piece, squares::Square};

pub mod quantmoid;
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



/// HalfKP: (our_king_square, piece_square, piece_type, color)
/// 64 * 64 * 5 * 2 = 40960
pub(crate) fn feature_index(piece: Piece, piece_sq: Square, king_sq: Square) -> usize {
    let p_idx = piece as usize * 2 + piece.color() as usize;
    let halfkp_idx = piece_sq as usize + (p_idx + king_sq as usize * 10) * 64;
    halfkp_idx
}

pub(crate) mod halfka {
    use super::*;

    const NUM_SQ: usize = 64;
    const NUM_PT: usize = 10;
    const NUM_PLANES: usize = NUM_SQ * NUM_PT + 1;

    

    pub(crate) fn halfka_index(p: Piece, king_sq: Square, sq: Square) -> usize {
        let pp = if p.color() == White { p as usize * 2} else { p as usize };

        let p_idx = pp + p.color() as usize;
        let xxx = 1 + orient(p, sq) + p_idx * NUM_SQ + king_sq as usize * NUM_PLANES;
        return xxx;
    }
    
    pub(crate) fn orient(piece: Piece, sq: Square) -> usize {
        ((56 * (!(piece.color() == White) as u64)) ^ sq as u64) as usize
    }
}
