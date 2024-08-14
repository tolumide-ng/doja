use crate::{board::board::Board, color::Color};

use super::{commons::{piece_color, piece_type, relative_square, test_bit, Piece}, constants::{NNUEAccumulator, LEFT_FLANK}};

const MIRROR: [u8; 8] = [3, 2, 1, 0, 0, 1, 2, 3];

pub(crate) const fn sq64_to_sq32(sq: u8) -> u8 {
    (sq >> 1) & !0x3 + MIRROR[(sq & 0x7) as usize]
}

pub(crate) fn nnue_index(piece: u8, relksq: u8, color: Color, sq: u8) -> usize {
     let piece_type = piece_type(piece);
     let piece_color = piece_color(piece);
     let relpsq = relative_square(color, sq);

     let mksq = if test_bit(LEFT_FLANK, relksq) > 0 {relksq ^ 0x7} else {relksq};
     let mpsq = if test_bit(LEFT_FLANK, relksq) > 0 {relpsq ^ 0x7} else {relpsq};

    //  let color_u8 = piece_color as 
     let player_turn = if (color as u8) == piece_color { 1u8 } else { 0u8 };

     640 * (sq64_to_sq32(mksq) + (64 * (5 * player_turn + piece_type)) + mpsq) as usize
}


pub(crate) fn nnue_can_update(accum: *mut NNUEAccumulator, board: Board, color: Color) {}