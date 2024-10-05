mod bitboard;
mod masks;
mod game_phase;
mod utils;
mod squares;
mod zobrist;
mod color;
mod search;
mod nnue;
mod shift;
mod board;
mod command;
mod constants;
mod moves;
// mod random_magic;
mod piece_attacks;
mod bit_move;
mod move_scope;
mod perft;
mod kogge_stone;
mod uci;
mod tt;
mod syzygy;




use std::{num::NonZero, sync::{Arc, Mutex}, thread};

use board::{fen::FEN, position::Position, state::board::Board};
use constants::TRICKY_POSITION;
use search::control::Control;
use syzygy::probe::TableBase;
use tt::table::TTable;
// use zobrist::Zobrist;

use crate::search::alpha_beta::NegaMax;





// #[tokio::main]
fn main() {
    // let _ = UCI::default().reader();

    println!("**********************AFTER*****************************");
    
    let board = Position::with(Board::parse_fen(TRICKY_POSITION).unwrap());
    // println!("**********************BEFORE*****************************");
    println!("{}", board.to_string());
    

    println!("num of cpus {:?}", std::thread::available_parallelism().unwrap_or(NonZero::<usize>::new(1).unwrap()));
    // let tt = TTable::default();
    let controller = Arc::new(Mutex::new(Control::default()));
    let board = Position::with(Board::parse_fen(TRICKY_POSITION).unwrap());
    // let threads = std::thread::available_parallelism().unwrap_or(NonZero::<usize>::new(1).unwrap()).get();
    let threads = 1;
    let depth = 10;
    // let mut bb = board.clone();
    let table = TTable::default();

    let mut negamax_thread = (0..threads).map(|i| NegaMax::new(controller.clone(), table.get(), i)).collect::<Vec<_>>();
    
    let tb = TableBase::default();

    // thread::scope(|s| {
    //     for td in negamax_thread.iter_mut() {
    //         let mut bb = board.clone();
    //         s.spawn(move || {
    //             td.iterative_deepening(depth, &mut bb, &tb);
    //         });
    //     }
    // });

    let mut bb = board.clone();
    negamax_thread[0].iterative_deepening(depth, &mut bb, &tb);

}

// 8|4|2|1|
// 0|0|1|0|
// 1|0|0|1|
// 0|0|1|1|
// 1|0|0|0|
  
