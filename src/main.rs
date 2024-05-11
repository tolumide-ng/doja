mod bitboard;
mod squares;
mod color;
mod board;
mod constants;
mod magic;
mod moves;
mod random_magic;
mod piece_attacks;

use board::fen::FEN;
// use bishop::Bishop;
pub use bitboard::Bitboard;
use constants::PIECE_ATTACKS;
// use crate::constants::

use crate::{board::board_state::BoardState, color::Color, constants::{CMK_POSITION, KILLER_POSITION, START_POSITION, TRICKY_POSITION}, squares::{Square, BISHOP_RELEVANT_BITS, SQUARE_NAMES}};




fn main() {
    let board = BoardState::parse_fen(TRICKY_POSITION).unwrap();

    println!("{}", board.to_string());

    println!("{}", board.get_possible_destination_squares_for(Color::White));

    // println!("{}", board.to_string());
    // println!("{}", board.get_possible_destination_squares_for(Color::Black).to_string());


    // let point = board[Piece::BP as usize];
    // println!("{}", point.to_string());
    
    
    // let mut occ = Bitboard::new();
    // occ.set_bit(Square::G4.into());

    // let x = PIECE_ATTACKS.get_queen_attacks(Square::D4.into(), occ.into());
    // println!("{}", Bitboard::from(x));

    // let pa =Pawn::bitboard_pawn_attacks(Color::White, Square::B3.into());
    // println!("{}", pa.to_string());


    // let pw = u64::from(pa);
    // println!("{}", Bitboard::from(pw & u64::from(point)).to_string());

    // println!("{}", Bitboard::from(board.get_occupancy(Color::White)));
    // // println!("{}", Bitboard::from(board.get_occupancy(Color::Both)));git 
}
