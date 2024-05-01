mod bit_board;
mod squares;
mod pawn;
mod color;
mod constants;
mod knight;

pub use bit_board::BitBoard;
use knight::Knight;
use pawn::Pawn;

use crate::squares::Square;




fn main() {
    // println!("Hello, world!");
    // let mut bit_board = BitBoard::new();
    // println!("{:#?}", bit_board.to_string());
    // let e2: u64 = Square::E2.into();

    let mut bit_board = BitBoard::new();

    // for rank in 0..8 {
    //     for file in 0..8 {
    //         if file < 6{
    //             bit_board.set_bit((rank * 8) + file);
    //         }
                
    //     }
    // }

    // println!(":::::::::::::::::::::::::: {:#?}", bit_board.to_string());

    // let pawn = Pawn::mask_pawn_attacks(color::Color::Black, Square::E4.into());

    // let knight = Knight::mask_knight_attacks(Square::A1 as u64);
    let knights = Knight::init_leapers_attack();

    for i in knights {
        println!("{:#?} \n the board after ", i.to_string());
    }

    // let pawn_attacks = Pawn::init_leapers_attack();
    // for i in pawn_attacks {
    //     for j in i {
    //         println!("the board {:#}", j.to_string());
    //     }
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
