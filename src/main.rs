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

use std::{borrow::Cow, io, sync::Arc};

use board::fen::FEN;
// use bishop::Bishop;
pub use bitboard::Bitboard;
use kogge_stone::KoggeStone;
use perft::Perft;
// use crate::constants::

use crate::{bit_move::BitMove, board::{board_state::BoardState, piece::Piece}, color::Color, constants::{CMK_POSITION, KILLER_POSITION, RANK_4, START_POSITION, TRICKY_POSITION}, move_type::MoveType, moves::Moves, shift::Shift, squares::{Square, BISHOP_RELEVANT_BITS, SQUARE_NAMES}};




fn main() {
    // let board = BoardState::parse_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBbPPP/R3K2R w KQkq - 0 1 ").unwrap();

    // // let xx = Arc::new(&board);
    // println!("{}", board.to_string());
    // let move_list =board.gen_movement();

    // for index in 0..move_list.count() {
    //     let bmove = move_list.list[index];
    //     let resulting_board = board.make_move(bmove, MoveType::AllMoves);
    //     println!(" making the move {}", bmove.to_string());
    //     if let Some(new_board) = resulting_board {
    //         println!("{:?}", new_board.to_string());
    //         let mut input = String::new();
    //         io::stdin().read_line(&mut input).unwrap();
    //     }
    // }


    // println!("{}", move_list);

    // Perft::start();

    // let board = Bitboard::from(0xfefefefefefefe00);
    // println!("{}", board.to_string());

    // let wqueens = 0x2000000120000u64;
    let wqueens = 0x8000000u64;
    let bopponents = 0x10840500800088u64;
    // let empty = 0xffed7bfaff6dff77u64;
    let empty = !(bopponents|wqueens);
    println!("{0:x}", !(bopponents|wqueens));

    println!("{}", Bitboard::from(wqueens).to_string());
    println!("{}", Bitboard::from(bopponents).to_string());

    let xwqueens = Bitboard::from(0x2000000120000u64);
    // println!("{}", bqueens.to_string());
    let north = KoggeStone::sliding_attacks(wqueens, empty, Shift::North);
    let south = KoggeStone::sliding_attacks(wqueens, empty, Shift::South);
    let east = KoggeStone::sliding_attacks(wqueens, empty, Shift::East);
    let west = KoggeStone::sliding_attacks(wqueens, empty, Shift::West);

    let north_east = KoggeStone::sliding_attacks(wqueens, empty, Shift::NorthEast);
    let south_east = KoggeStone::sliding_attacks(wqueens, empty, Shift::SouthEast);
    let south_west = KoggeStone::sliding_attacks(wqueens, empty, Shift::SouthWest);
    let north_west = KoggeStone::sliding_attacks(wqueens, empty, Shift::NorthWest);


    println!("{}", Bitboard::from(north | north_east | east | south_east | south | south_west | west | north_west))


    // let x = (0b00000010 as u64).to_le();
    // let y = (0b01000000 as u64).to_le();


    
    
}
