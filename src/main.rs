mod bitboard;
mod squares;
mod pawn;
mod color;
mod board;
mod constants;
mod knight;
mod king;
mod bishop;
mod rook;
mod magic;
// mod attacks;
// mod magic;

use board::{board::Board, fen::FEN, piece::Piece};
// use attacks::DynamicAttacks;
// use bishop::Bishop;
pub use bitboard::Bitboard;
// use crate::constants::

use crate::{bishop::Bishop, board::board_state::BoardState, color::Color, constants::{CMK_POSITION, KILLER_POSITION, TRICKY_POSITION}, magic::attacks::DynamicAttacks, rook::Rook, squares::{Square, BISHOP_RELEVANT_BITS, SQUARE_NAMES}};




fn main() {
    let board = BoardState::parse_fen("r2q1rk1/ppp2ppp/2n1bn2/2b1p3/3pP3/3P1NPP/PPP1NPB1/R1BQ1RK1 w Kq - 0 9 ").unwrap();
    println!("{}", board.to_string());

    // println!("{}", Bitboard::from(board.get_occupancy(Color::White)));
    // // println!("{}", Bitboard::from(board.get_occupancy(Color::Both)));
}
