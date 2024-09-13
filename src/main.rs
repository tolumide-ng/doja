mod nnue;
mod bitboard;
mod masks;
mod game_phase;
mod utils;
mod squares;
mod zobrist;
mod color;
mod search;
mod shift;
mod board;
mod command;
mod constants;
// mod magic;
mod moves;
// mod random_magic;
mod piece_attacks;
mod bit_move;
mod move_type;
mod perft;
mod kogge_stone;
mod uci;
mod tt;


// use std::io::Read;
// use std::sync::{Arc, Mutex};
// use std::{sync::mpsc, time::Instant};
// use std::{ptr, thread};

use std::sync::{Arc, Mutex};

use board::{fen::FEN, position::Position, state::board::Board};
use constants::{ALPHA, BETA, TRICKY_POSITION};
// use bit_move::BitMove;   
// use bitboard::Bitboard;
// use board::{state::board_state::Board, fen::FEN};
// use color::Color;
// use constants::{ALPHA, BETA, EMPTY_BOARD, REPETITIONS, START_POSITION, TRICKY_POSITION, ZOBRIST};
// use masks::EvaluationMasks
// use search::control::Control;
use search::control::Control;
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

    let controller = Arc::new(Mutex::new(Control::default()));
    let mut board = Position::with(Board::parse_fen(TRICKY_POSITION).unwrap());
    println!("**********************BEFORE*****************************");
    println!("{}", board.to_string());
    NegaMax::run(controller,6, &mut board);
    println!("**********************AFTER*****************************");
    println!("{}", board.to_string());



    // println!()
}

// 8|4|2|1|
// 0|0|1|0|
// 1|0|0|1|
// 0|0|1|1|
// 1|0|0|0|
  
