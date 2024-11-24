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
mod move_logic;
#[allow(dead_code)]
mod constants;
// mod random_magic;
mod piece_attacks;
mod move_scope;
mod perft;
mod kogge_stone;
mod uci;
mod tt;
mod syzygy;




use std::{num::NonZero, sync::{atomic::AtomicBool, Arc, Mutex}, thread};

// use move_logic::bitmove::{Move, MoveType};
use board::{position::Position, state::board::Board};
use color::Color;
use constants::{CMK_POSITION, TRICKY_POSITION};
use nnue::halfka_idx;
use search::{control::Control, search::Search, threads::Thread};
use squares::Square;
use syzygy::probe::TableBase;
use tt::table::TTable;
use uci::{clock::Clock, UCI};
// use zobrist::Zobrist;

// use crate::search::alpha_beta::NegaMax;





// #[tokio::main]
fn main() {
    // let _ = UCI::default().reader();
    // println!("num of cpus {:?}", std::thread::available_parallelism().unwrap_or(NonZero::<usize>::new(1).unwrap()));
    // // let tt = TTable::default();
    // let controller = Arc::new(Mutex::new(Control::default()));
    // // let mut board = Position::with(Board::fromunwrap());
    // // let threads = std::thread::available_parallelism().unwrap_or(NonZero::<usize>::new(1).unwrap()).get();
    // let threads = 1;
    // let depth = 4;
    // // let mut bb = board.clone();
    // let table = TTable::default();

    // // let mut negamax_thread = (0..threads).map(|i| NegaMax::new(controller.clone(), table.get(), i)).collect::<Vec<_>>();
    
    // let tb = TableBase::default();



    // // let mut board = Position::from(Board::try_from("r2Rk2r/p1ppqpb1/bn2pnp1/3PN3/4P3/2p2Q1p/PPPBBPPP/R3K21 b KQkq - 0 1 ").unwrap());
    // let mut board = Position::from(Board::try_from(CMK_POSITION).unwrap());
    // let clock = Clock::new(&AtomicBool::new(false));
    // let mut negamax_thread = (0..threads).map(|i| Search::new(table.get(), clock.clone())).collect::<Vec<_>>();
    
    // // let mut board = Position::from(Board::try_from(TRICKY_POSITION).unwrap());
    // let thread = Thread::new(30, table.get(), 0);
    // println!("{}", board.to_string());
    // // let mut search = Search::new(table.get());
    // // search.iterative_deepening(8, &mut board, &mut thread);

    // thread::scope(|s| {
    //     for td in negamax_thread.iter_mut() {
    //         let mut bb = board.clone();
    //         let mut th = thread.clone();
    //         s.spawn(move || {
    //             td.iterative_deepening(depth, &mut bb, &mut th);
    //         });
    //     }
    // });


    let _ = UCI::default().reader();

}

// 8|4|2|1|
// 0|0|1|0|
// 1|0|0|1|
// 0|0|1|1|
// 1|0|0|0|
