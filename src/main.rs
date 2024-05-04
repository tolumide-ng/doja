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
mod magic;

use attacks::DynamicAttacks;
// use bishop::Bishop;
pub use bit_board::BitBoard;
use magic::Magic;
// use king::King;
// use knight::Knight;
// use pawn::Pawn;
// use rook::Rook;

use crate::{bishop::Bishop, rook::Rook, squares::{Square, SQUARE_NAMES}};




fn main() {
    // println!("Hello, world!");
    // let mut bit_board = BitBoard::new();
    // println!("{:#?}", bit_board.to_string());
    // let e2: u64 = Square::E2.into();

    let mut block = BitBoard::new();
    // block.set_bit(Square::B6.into());
    // block.set_bit(Square::G7.into());
    // block.set_bit(Square::F4.into());
    // block.set_bit(Square::B2.into());
    // block.set_bit(Square::D5.into());
    // println!("{:#?} :::: XOVVVVTY :::: \n\n ", block.to_string());

    // block.set_bit(Square::D7.into());
    // block.set_bit(Square::D2.into());
    // block.set_bit(Square::D1.into());
    // block.set_bit(Square::B4.into());
    // block.set_bit(Square::G4.into());

    // // println!("counting the bits {:#?}", block.count_bits());
    // println!("counting the bits {:#?}", block.count_bits());
    // println!(">>>>>>>>>> {:#?}", SQUARE_NAMES[block.get_lsb1().unwrap() as usize]);
    // let x = (block.0 as i64 & -(block.0 as i64)) -1;
    // let mut oo = BitBoard::new();
    // oo.0 = x as u64;
    // println!(">>>>>>>>>> {:#?}", oo.to_string());


    // mask piece at given square
    
    
    // // init occupancy
    // for rank in 0..8 {
    //     for file in 0..8 {
    //         let square = (rank * 8) + file;
    //         let data = Rook::mask_rook_attacks(square).count_bits();
    //         print!(" {data},");
    //     }
    //     println!("\n")
    // }

    let mut magic = Magic::new();
    let x = magic.random_u32() as u64;
    let ox = x & 0xFFFF;

    let sx = BitBoard::from(magic.random_u32() as u64);
    let sox = BitBoard::from(*sx & 0xFFFF);

    println!("magic number >>>> {:064b}", x);
    println!("magic number >>>> {:064b}", ox);

    println!("-------------xxx-----------–-------------xxx-----------–-------------xxx-----------–");

    // println!("magic number >>>> {} \n\n\n\n", sx.to_string());
    // println!("magic number >>>> {} \n\n", sox.to_string());

    println!("------|||||||||:::::::::::::: {:#?}", BitBoard::from(Magic::new().random_u64()).to_string());

    // println!("magic number >>>> {}", BitBoard::from(magic.get_random_u32_number() as u64 & 0xFFFF).to_string()); // slice upprt (from MS1B side) 16bits
    // println!("magic number >>>> {}", BitBoard::from(magic.get_random_u32_number() as u64).to_string());

}
