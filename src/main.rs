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
use tt::{HashFlag, TTable, TT};
use uci::UCI;
use zobrist::Zobrist;

use crate::{constants::CMK_POSITION, search::{negamax::NegaMax, zerosum::ZeroSum}};





// #[tokio::main]
fn main() {
    let _ = UCI::default().reader();


    // let board = BoardState::parse_fen("6k1/5p1p/8/8/8/8/5P1P/6K1 w - - ").unwrap();
    // let board = BoardState::parse_fen("8/8/8/8/8/8/8/8 w - - ").unwrap();
    
    // let board = BoardState::parse_fen(START_POSITION).unwrap();
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
  
