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


use std::io::Read;
use std::{sync::mpsc, time::Instant};
use std::thread;

use bit_move::BitMove;   
use bitboard::Bitboard;
use board::{board_state::BoardState, fen::FEN, piece::Piece};
use constants::{START_POSITION, TRICKY_POSITION};
use perft::Perft;
use search::evaluation::Evaluation;
use squares::Square;
use uci::UCI;

use crate::{constants::CMK_POSITION, search::{negamax::NegaMax, zerosum::ZeroSum}};





// #[tokio::main]
fn main() {
    // Perft::start(5);
    // UCI::parse(String::from("e2c4"));
    // println!("{}", Bitboard::from(0xf0000).to_string())
    // let mv = BitMove::new(Square::A1 as u32, Square::B2 as u32, Piece::WB, None, false, false, false, false);

    // let board = BoardState::parse_fen(TRICKY_POSITION).unwrap();
    // UCI::search_position(5, &board);

    // // let bb = Evaluation::evaluate(&board);
    // // println!("BBBBBB {}", bb);
    // println!("{}", board.to_string());
    // let instant = Instant::now();
    // UCI::search_position(7, &board);
    // let elapsed = instant.elapsed();
    // println!("      Time: {}ms", elapsed.as_millis());
    let _ = UCI::default().reader();



    // let (sender, receiver) = mpsc::channel();

    // // Spawn a thread to read from stdin
    // thread::spawn(move || {
    //     // Read from stdin
    //     let mut input = String::new();
    //     if let Err(err) = std::io::stdin().read_line(&mut input) {
    //         // Send an error message to the main thread
    //         println!("||||||||||");
    //         sender.send(Err(err)).expect("Failed to send error");
    //         return;
    //     }
    //         println!("||||||||||");

    //     // Send the input to the main thread
    //     sender.send(Ok(input)).expect("Failed to send input");
    // });

    // // Receive the input from the stdin thread
    // match receiver.recv() {
    //     Ok(Ok(input)) => {
    //         println!("Input: {}", input);
    //     }
    //     Ok(Err(err)) => {
    //         eprintln!("Error reading from stdin: {}", err);
    //     }
    //     Err(_) => {
    //         println!("No input provided.");
    //     }
    // }
}

