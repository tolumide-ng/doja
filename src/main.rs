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

use crate::{bishop::Bishop, board::board_state::BoardState, magic::attacks::DynamicAttacks, rook::Rook, squares::{Square, BISHOP_RELEVANT_BITS, SQUARE_NAMES}};




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


    // let mut chess_board = BoardState::new();
    // // chess_board[Piece::BK as usize].set_bit(Square::E4.into());
    // chess_board[Piece::WP as usize].set_bit(Square::A2.into());
    // chess_board[Piece::WP as usize].set_bit(Square::B2.into());
    // chess_board[Piece::WP as usize].set_bit(Square::C2.into());
    // chess_board[Piece::WP as usize].set_bit(Square::D2.into());
    // chess_board[Piece::WP as usize].set_bit(Square::E2.into());
    // chess_board[Piece::WP as usize].set_bit(Square::F2.into());
    // chess_board[Piece::WP as usize].set_bit(Square::G2.into());
    // chess_board[Piece::WP as usize].set_bit(Square::H2.into());
    
    // chess_board[Piece::WN as usize].set_bit(Square::G1.into());
    // chess_board[Piece::WN as usize].set_bit(Square::B1.into());

    // // bishops
    // chess_board[Piece::WB as usize].set_bit(Square::C1.into());
    // chess_board[Piece::WB as usize].set_bit(Square::F1.into());

    // chess_board[Piece::WR as usize].set_bit(Square::A1.into());
    // chess_board[Piece::WR as usize].set_bit(Square::H1.into());

    // chess_board[Piece::WQ as usize].set_bit(Square::D1.into());
    // chess_board[Piece::WK as usize].set_bit(Square::E1.into());

    // // println!("the bitboard is {}", chess_board[Piece::WP as usize].to_string());

    //  // chess_board[Piece::BK as usize].set_bit(Square::E4.into());
    // chess_board[Piece::BP as usize].set_bit(Square::A7.into());
    // chess_board[Piece::BP as usize].set_bit(Square::B7.into());
    // chess_board[Piece::BP as usize].set_bit(Square::C7.into());
    // chess_board[Piece::BP as usize].set_bit(Square::D7.into());
    // chess_board[Piece::BP as usize].set_bit(Square::E7.into());
    // chess_board[Piece::BP as usize].set_bit(Square::F7.into());
    // chess_board[Piece::BP as usize].set_bit(Square::G7.into());
    // chess_board[Piece::BP as usize].set_bit(Square::H7.into());
    
    // chess_board[Piece::BN as usize].set_bit(Square::G8.into());
    // chess_board[Piece::BN as usize].set_bit(Square::B8.into());

    // // bishops
    // chess_board[Piece::BB as usize].set_bit(Square::C8.into());
    // chess_board[Piece::BB as usize].set_bit(Square::F8.into());

    // chess_board[Piece::BR as usize].set_bit(Square::A8.into());
    // chess_board[Piece::BR as usize].set_bit(Square::H8.into());

    // chess_board[Piece::BQ as usize].set_bit(Square::D8.into());
    // chess_board[Piece::BK as usize].set_bit(Square::E8.into());
    // println!("{}", chess_board.to_string());

}
