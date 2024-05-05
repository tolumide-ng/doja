mod bit_board;
mod squares;
mod pawn;
mod color;
mod constants;
mod knight;
mod king;
mod bishop;
mod rook;
mod magic;
// mod attacks;
// mod magic;

// use attacks::DynamicAttacks;
// use bishop::Bishop;
pub use bit_board::BitBoard;
use magic::{plain::PlainAttacks};

use crate::{bishop::Bishop, magic::attacks::DynamicAttacks, rook::Rook, squares::{Square, BISHOP_RELEVANT_BITS, SQUARE_NAMES}};




fn main() {

    // // init all// init all// init all// init all// init all
    // let bishop = PlainAttacks::init_sliders_attacks(true);
    // let mut occupancy = BitBoard::new();
    // occupancy.set_bit(Square::C5.into());
    // println!("{:#?}", BitBoard::from(bishop.get_bishop_attacks(Square::D4, 0)).to_string());
    // // // let rook = PlainAttacks::init_sliders_attacks(false);



    let rookie = PlainAttacks::init_sliders_attacks(false);
    let mut occupancy = BitBoard::new();
    occupancy.set_bit(Square::C5.into());
    println!("{:#?}", BitBoard::from(rookie.get_rook_attacks(Square::D4, 0)).to_string());
}
