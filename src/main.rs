mod bitboard;
mod masks;
mod pesto;
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
    // let _ = UCI::default().reader();



    // let board = EvaluationMasks::init_eval_masks();

    // println!("{}", board.to_string());

    // println!("{}", Bitboard::from(Bitboard::from(0x420880000).isolanis()).to_string());

    // Perft::start(6);


    // let mut hash_key = 0u64;
    // hash_key ^= ZOBRIST.piece_keys[Piece::WP][Square::A2];

    // println!("{}", board.to_string());
    // println!("{}", new_board.to_string());
    // println!("the new key should be {0:x}", new_board.hash_key());

    // board.hash_key();
    // println!("mbs {}", HASH_SIZE);

    // let mut rtt = TTable::default();r
    // // rtt.set(0x2938, BitMove::from(0), 0, 12, HashFlag::Exact);
    // let result = rtt.probe(0, 0, 10, -10);
    // println!("the result is {:?}", result);

    // let mut x: u8 = 254;
    // let xx = x.wrapping_shr(100);
    // println!("{}", xx);

    // let board = BoardState::parse_fen("6k1/5p1p/8/8/8/8/5P1P/6K1 w - - ").unwrap();
    // let board = BoardState::parse_fen("8/8/8/8/8/8/8/8 w - - ").unwrap();
    let board = BoardState::parse_fen(START_POSITION).unwrap();
    println!("{}", board.to_string());
    Evaluation::evaluate(&board);
    Evaluation::get_game_phase_score(&board);
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
  
