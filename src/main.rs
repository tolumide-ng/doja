mod bitboard;
mod squares;
mod color;
mod shift;
mod board;
mod constants;
mod magic;
mod moves;
mod random_magic;
mod piece_attacks;
mod bit_move;
mod move_type;
mod perft;
 mod kogge_stone;

use std::{borrow::Cow, io, ops::Shl, sync::Arc};

use board::fen::FEN;
// use bishop::Bishop;
pub use bitboard::Bitboard;
use kogge_stone::KoggeStone;
use perft::Perft;
// use crate::constants::

use crate::{bit_move::BitMove, board::{board_state::BoardState, piece::Piece}, color::Color, constants::{CMK_POSITION, KILLER_POSITION, PIECE_ATTACKS, RANK_4, START_POSITION, TRICKY_POSITION}, move_type::MoveType, moves::Moves, shift::Shift, squares::{Square, BISHOP_RELEVANT_BITS, SQUARE_NAMES}};




fn main() {
    Perft::start();

    // let no_attacks = !self.is_square_attacked(Square::E1.into(), !color) && !self.is_square_attacked(Square::F1.into(), !color);

    // let points = 1u64 << Square::D5 as u64 | 1u64 << Square::G4 as u64;

    // println!("the points are {}", Bitboard::from(points).to_string());

    // let lexd = Bitboard::from(1u64.shl(Square::D4 as u64));
    // println!("{}", lexd.to_string());
    // println!("{}", Bitboard::from(Square::D4 as u64).to_string());

    // let xo = Bitboard::from(PIECE_ATTACKS.nnbishop_attacks(points, 0));
    // println!("{}", xo.to_string());

    
    
}
