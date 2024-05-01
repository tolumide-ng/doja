mod bit_board;
mod squares;
mod pawn;
mod color;
mod constants;
mod knight;
mod king;
mod bishop;
mod rook;
mod attacks;

use attacks::DynamicAttacks;
// use bishop::Bishop;
pub use bit_board::BitBoard;
// use king::King;
// use knight::Knight;
// use pawn::Pawn;
use rook::Rook;

use crate::squares::Square;




fn main() {
    // println!("Hello, world!");
    // let mut bit_board = BitBoard::new();
    // println!("{:#?}", bit_board.to_string());
    // let e2: u64 = Square::E2.into();

    let mut block = BitBoard::new();
    block.set_bit(Square::B6.into());
    block.set_bit(Square::G7.into());
    block.set_bit(Square::E3.into());
    block.set_bit(Square::B2.into());
    println!("{:#?} :::: XOVVVVTY :::: \n\n ", block.to_string());

    let rookie = DynamicAttacks::dynamic_bishpp_attacks(Square::D4 as u64, block.0);
    println!(":::: this is the rookie :::: {:#?}", rookie.to_string());

    // let rooks = Rook::init_leapers_attack();
    // // let pawn_attacks = Pawn::init_leapers_attack();
    // for i in rooks {
    //     println!("the board {:#}", i.to_string());
    // }

    
    // bit_board.set_bit(Square::C7); 
    // bit_board.set_bit(Square::C6);

    // println!("{:#?}", bit_board.to_string());


    // bit_board.pop_bit(Square::C7);
    // println!("{:#?}", bit_board.to_string());


    // for i in (1..=8).rev() {
    //     println!("\"A{i}\", \"B{i}\", \"C{i}\", \"D{i}\", \"E{i}\", \"F{i}\", \"G{i}\", \"H{i}\",");
    // }
}
