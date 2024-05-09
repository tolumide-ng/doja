mod bitboard;
mod squares;
mod pawn;
mod color;
mod board;
mod constants;
mod knight;
mod king;
mod piece_attacks;
mod bishop;
mod rook;
mod magic;
// mod attacks;
// mod magic;

use board::fen::FEN;
// use bishop::Bishop;
pub use bitboard::Bitboard;
// use crate::constants::

use crate::{bishop::Bishop, board::board_state::BoardState, color::Color, constants::{CMK_POSITION, KILLER_POSITION, TRICKY_POSITION}, magic::{attacks::DynamicAttacks, plain::PlainAttacks}, pawn::Pawn, rook::Rook, squares::{Square, BISHOP_RELEVANT_BITS, SQUARE_NAMES}};




fn main() {
    let board = BoardState::parse_fen("8/8/8/8/8/3p4/8/8 w - - ").unwrap();

    println!("{}", board.to_string());
    println!("{}", board.get_possible_destination_squares_for(Color::Black).to_string());
    // let point = board[Piece::BP as usize];
    // println!("{}", point.to_string());

    // let pa =Pawn::bitboard_pawn_attacks(Color::White, Square::B3.into());
    // println!("{}", pa.to_string());


    // let pw = u64::from(pa);
    // println!("{}", Bitboard::from(pw & u64::from(point)).to_string());

    // println!("{}", Bitboard::from(board.get_occupancy(Color::White)));
    // // println!("{}", Bitboard::from(board.get_occupancy(Color::Both)));git 
}
