mod bitboard;
mod squares;
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


use std::time::Instant;

use bit_move::BitMove;   
use bitboard::Bitboard;
use board::{board_state::BoardState, fen::FEN, piece::Piece};
use constants::{START_POSITION, TRICKY_POSITION};
use perft::Perft;
use search::evaluation::Evaluation;
use squares::Square;
use uci::UCI;

use crate::{constants::CMK_POSITION, search::{negamax::NegaMax, zerosum::ZeroSum}};




fn main() {
    // Perft::start(5);
    // UCI::parse(String::from("e2c4"));
    // println!("{}", Bitboard::from(0xf0000).to_string())
    // let mv = BitMove::new(Square::A1 as u32, Square::B2 as u32, Piece::WB, None, false, false, false, false);

    let board = BoardState::parse_fen(CMK_POSITION).unwrap();
    // let bb = Evaluation::evaluate(&board);
    // println!("BBBBBB {}", bb);
    println!("{}", board.to_string());
    let instant = Instant::now();
    // UCI::search_position(5, &board);
    UCI::search_position(6, &board);
    let elapsed = instant.elapsed();
    println!("      Time: {}ms", elapsed.as_millis());

    // let score = Evaluation::evaluate(&board);

    // println!("score is {score}");
    

    // if let Some(n_board) = UCI::parse(&board, String::from("b7b8q")) {
    //     println!("move works");
    //     println!("{}", n_board.to_string());
    // } else {
    //     println!("Illegal move");
    // }

    let _ = UCI::reader();
}

