mod bitboard;
mod masks;
mod game_phase;
mod utils;
mod squares;
mod zobrist;
mod nnue;
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


use std::io::Read;
use std::sync::{Arc, Mutex};
use std::{sync::mpsc, time::Instant};
use std::{ptr, thread};

use bit_move::BitMove;   
use bitboard::Bitboard;
use board::{board_state::BoardState, fen::FEN, piece::Piece};
use color::Color;
use constants::{ALPHA, BETA, EMPTY_BOARD, REPETITIONS, START_POSITION, TRICKY_POSITION, ZOBRIST};
use masks::EvaluationMasks;
use move_type::MoveType;
use perft::Perft;
use search::control::Control;
use search::evaluation::Evaluation;
use squares::Square;
use tt::{HashFlag, TTable};
use uci::UCI;
use zobrist::Zobrist;

use crate::{constants::CMK_POSITION, search::{negamax::NegaMax, zerosum::ZeroSum}};





// #[tokio::main]
fn main() {
    // let _ = UCI::default().reader();

    // let board = BoardState::parse_fen("6k1/5p1p/8/8/8/8/5P1P/6K1 w - - ").unwrap();
    // let board = BoardState::parse_fen("8/8/8/8/8/8/8/8 w - - ").unwrap();
    
    // let board = BoardState::parse_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1 ").unwrap();
    let board = BoardState::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 ").unwrap();
    // let board = BoardState::parse_fen("r3k2r/p1ppqpb1/1n2pnp1/3PN3/1p2P3/2N2Q1p/PPPB1PPP/R3K2R w KQkq - 0 1 ").unwrap();
    println!("{}", board.to_string());
    println!("the score {}", Evaluation::evaluate(&board));
    // println!("{}", board.to_string());
    // Evaluation::evaluate(&board);
    // Evaluation::get_game_phase_score(&board);

    // let score = Evaluation::evaluate(&board);
    // println!("the scoer now ius >>>> {}", score)
    // EvaluationMasks::init();

    // println!()
}

// 8|4|2|1|
// 0|0|1|0|
// 1|0|0|1|
// 0|0|1|1|
// 1|0|0|0|
  
