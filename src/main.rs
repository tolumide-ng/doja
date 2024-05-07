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

use board::{board::Board, piece::Piece};
// use attacks::DynamicAttacks;
// use bishop::Bishop;
pub use bitboard::Bitboard;
use magic::plain::PlainAttacks;

use crate::{bishop::Bishop, magic::attacks::DynamicAttacks, rook::Rook, squares::{Square, BISHOP_RELEVANT_BITS, SQUARE_NAMES}};




fn main() {

    // // // init all// init all// init all// init all// init all
    // let mut occupancy = Bitboard::new();
    // occupancy.set_bit(Square::C5.into());
    // occupancy.set_bit(Square::F2.into());
    // occupancy.set_bit(Square::G7.into());
    // occupancy.set_bit(Square::B2.into());
    // occupancy.set_bit(Square::G5.into());
    // occupancy.set_bit(Square::E2.into());
    // occupancy.set_bit(Square::E7.into());
    // println!("{:#?}", occupancy.to_string());


    // let bishop = PlainAttacks::init_sliders_attacks(true).get_bishop_attacks(Square::E5, occupancy.into());
    // println!("{:#?}", Bitboard::from(bishop).to_string());
    // // let rook = PlainAttacks::init_sliders_attacks(false);

    // let rookie = PlainAttacks::init_sliders_attacks(false).get_rook_attacks(Square::D4, occupancy.into());
    // println!("{:#?}", Bitboard::from(rookie).to_string());


    let mut chess_board = Board::new();
    chess_board[Piece::WP as usize].set_bit(Square::E2.into());

    println!("the bitboard is {}", chess_board[Piece::WP as usize].to_string());
    println!("{}", chess_board.to_string());
}
