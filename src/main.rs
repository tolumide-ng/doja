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
#[allow(dead_code)]
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




use std::{num::NonZero, sync::{Arc, Mutex}};

use bit_move::Move;
use board::{fen::FEN, position::Position, state::board::Board};
use constants::TRICKY_POSITION;
use search::{control::Control, search::Search};
use syzygy::probe::TableBase;
use tt::table::TTable;
// use zobrist::Zobrist;

use crate::search::alpha_beta::NegaMax;





// #[tokio::main]
fn main() {
    // let _ = UCI::default().reader();

    println!("**********************AFTER*****************************");
    
    let mut board = Position::with(Board::parse_fen(TRICKY_POSITION).unwrap());
    // println!("**********************BEFORE*****************************");
    println!("{}", board.to_string());
    

    println!("num of cpus {:?}", std::thread::available_parallelism().unwrap_or(NonZero::<usize>::new(1).unwrap()));
    // let tt = TTable::default();
    let controller = Arc::new(Mutex::new(Control::default()));
    // let mut board = Position::with(Board::parse_fen(TRICKY_POSITION).unwrap());
    // let threads = std::thread::available_parallelism().unwrap_or(NonZero::<usize>::new(1).unwrap()).get();
    let threads = 1;
    let depth = 4;
    // let mut bb = board.clone();
    let table = TTable::default();

    // let mut negamax_thread = (0..threads).map(|i| NegaMax::new(controller.clone(), table.get(), i)).collect::<Vec<_>>();
    
    let tb = TableBase::default();

    // thread::scope(|s| {
    //     for td in negamax_thread.iter_mut() {
    //         let mut bb = board.clone();
    //         s.spawn(move || {
    //             td.iterative_deepening(depth, &mut bb, &tb);
    //         });
    //     }
    // });

    // let mut bb = board.clone();
    // negamax_thread[0].iterative_deepening(depth, &mut bb, &tb);
    // NegaMax::run(controller, table.get(), depth, &mut bb, 1, &tb);

    let mut search = Search::new(table.get());
    search.iterative_deepening(5, &mut board);
    // use crate::squares::Square::*;
    // use crate::bit_move::MoveType::*;

    // let xx0 = Search::see(&board, &Move::new(F3 as u8, F6 as u8, Capture));
    // let xx0 = Search::see(&board, &Move::new(D5 as u8, E6 as u8, Capture));

    // println!("the xx0 is {}", xx0);
    

}

// 8|4|2|1|
// 0|0|1|0|
// 1|0|0|1|
// 0|0|1|1|
// 1|0|0|0|
  
