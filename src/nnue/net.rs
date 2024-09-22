use crate::{board::piece::Piece, squares::Square};

use super::{align64::Align64, commons::{CR_MAX, CR_MIN, FEATURES, HIDDEN}};

/// Container for all network parameters
#[repr(C)]
pub(crate) struct NNUEParams {
    pub(crate) feature_weights: Align64<[i16; FEATURES * HIDDEN]>,
    pub(crate) features_bias: Align64<[i16; HIDDEN]>,
    pub(crate) output_weights: Align64<[i16; HIDDEN * 2]>,
    pub(crate) output_bias: i16,
}


/// NNUE model is initialized from binary values
pub(crate) static MODEL: NNUEParams = unsafe { std::mem::transmute(*include_bytes!("../../bins/net.bin")) };


/// Retrns white and black feature weight index for given features
pub(crate) fn nnue_index(piece: Piece, sq: Square) -> (usize, usize) {
    const COLOR_STRIDE: usize = 64 * 6; // number_of_squares * number of pieces per side
    const PIECE_STRIDE: usize = 64;
    let p = (piece as usize) % 6;
    let c = piece.color() as usize;

    println!("sqqqqq ::::: {}", sq as usize);

    let sqfv =sq.flipv() as usize;

    let white_idx = c * COLOR_STRIDE + p * PIECE_STRIDE + sqfv;
    let black_idx = (1 ^ c) * COLOR_STRIDE + p * PIECE_STRIDE + sq as usize;


    println!("|||||||||||| p =>> {p}, cc---- {c} -- white_idx {white_idx} ::::: black_idx ::: {black_idx}, sqqqq |||| {sqfv}");



    (white_idx * HIDDEN, black_idx * HIDDEN)
}



/// Squared Clipped ReLu activation function
pub(crate) fn squared_crelu(value: i16) -> i32 {
    let v = if value < CR_MIN {CR_MIN} else if value < CR_MAX {CR_MAX} else {value} as i32;
    // let v = value.clamp(CR_MIN, CR_MAX) as i32;
    v * v
}