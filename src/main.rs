mod nnue;
mod bitboard;
mod masks;
mod game_phase;
mod utils;
mod squares;
mod zobrist;
mod color;
mod search;
mod nnue_;
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



// use std::io::Read;
// use std::sync::{Arc, Mutex};
// use std::{sync::mpsc, time::Instant};
// use std::{ptr, thread};

use std::{clone, num::NonZero, sync::{Arc, Mutex}, thread::{self, Thread}};

use board::{fen::FEN, position::Position, state::board::Board};
use constants::TRICKY_POSITION;
use move_scope::MoveScope;
use search::control::Control;
use tt::table::TTable;
// use zobrist::Zobrist;

use crate::search::alpha_beta::NegaMax;





// #[tokio::main]
fn main() {
    // let _ = UCI::default().reader();

    // // let board = Board::parse_fen("6k1/5p1p/8/8/8/8/5P1P/6K1 w - - ").unwrap();
    // // let board = Board::parse_fen("8/8/8/8/8/8/8/8 w - - ").unwrap();
    
    // // let board = Board::parse_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1 ").unwrap();
    // let board = Board::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 ").unwrap();
    // // let board = Board::parse_fen("r3k2r/p1ppqpb1/1n2pnp1/3PN3/1p2P3/2N2Q1p/PPPB1PPP/R3K2R w KQkq - 0 1 ").unwrap();
    // println!("{}", board.to_string());
    // println!("the score {}", Evaluation::evaluate(&board));

    // println!("STARTING>>>>>");

    // Perft::start(6);


    
    // println!("{}", board.to_string());
    // Evaluation::evaluate(&board);
    // Evaluation::get_game_phase_score(&board);

    // let score = Evaluation::evaluate(&board);
    // println!("the scoer now ius >>>> {}", score)::::
    // EvaluationMasks::init();

    // NegaMax::run(controller, &tt, 7, &mut board);
    // println!("{}", board.to_string());
    println!("**********************AFTER*****************************");
    
    let board = Position::with(Board::parse_fen(TRICKY_POSITION).unwrap());
    // println!("**********************BEFORE*****************************");
    println!("{}", board.to_string());
    
    let bb = board.clone();
    // NegaMax::run(controller, &tt, 1, &mut board);
    
    println!("num of cpus {:?}", std::thread::available_parallelism().unwrap_or(NonZero::<usize>::new(1).unwrap()));
    // let tt = TTable::default();
    let controller = Arc::new(Mutex::new(Control::default()));
    let board = Position::with(Board::parse_fen(TRICKY_POSITION).unwrap());
    let threads = std::thread::available_parallelism().unwrap_or(NonZero::<usize>::new(1).unwrap()).get();
    let depth = 5;
    // let mut bb = board.clone();
    let table = TTable::default();

    let mut negamax_thread = (0..threads).map(|i| NegaMax::new(controller.clone(), table.get(), i)).collect::<Vec<_>>();
    

    thread::scope(|s| {
        for td in negamax_thread.iter_mut() {
            let mut bb = board.clone();
            s.spawn(move || {
                td.iterative_deepening(depth, &mut bb);
            });
        }
    });
}

// 8|4|2|1|
// 0|0|1|0|
// 1|0|0|1|
// 0|0|1|1|
// 1|0|0|0|
  
