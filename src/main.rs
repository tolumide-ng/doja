mod bitboard;
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


use std::io::Read;
use std::{sync::mpsc, time::Instant};
use std::thread;

use bit_move::BitMove;   
use bitboard::Bitboard;
use board::{board_state::BoardState, fen::FEN, piece::Piece};
use color::Color;
use constants::{START_POSITION, TRICKY_POSITION, ZOBRIST};
use move_type::MoveType;
use perft::Perft;
use search::evaluation::Evaluation;
use squares::Square;
use uci::UCI;
use zobrist::Zobrist;

use crate::{constants::CMK_POSITION, search::{negamax::NegaMax, zerosum::ZeroSum}};





// #[tokio::main]
fn main() {
    // Perft::start(5);
    // UCI::parse(String::from("e2c4"));
    // println!("{}", Bitboard::from(0xf0000).to_string())
    // let mv = BitMove::new(Square::A1 as u32, Square::B2 as u32, Piece::WB, None, false, false, false, false);

    let board = BoardState::parse_fen(TRICKY_POSITION).unwrap();

    // let mvs = board.get_castling(Color::White);
    // for x in mvs {
    //     println!("the mv {x} {:#?}", x);
    // }
    
    
    let mv = BitMove::new(Square::E1 as u32, Square::C1 as u32, Piece::WK, None, false, false, false, true);
    // let mv = BitMove::from(8409220);
    // println!("src={}, target={}, castling={}, piece={}, promotion={:?}", mv.get_src(), mv.get_target(), mv.get_castling(), mv.get_piece(), mv.get_promotion());
    let new_board = board.make_move(mv, MoveType::AllMoves).unwrap();
    // let new_board = board.clone();


    // let tricky = BoardState::parse_fen("r3k2r/p1ppqpb1/bn1Ppnp1/4N3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R b KQkq - 0 1 ").unwrap();
    // println!("the trikcy board is \n {}", tricky.to_string());
    // println!("expected tricky should be {0:x}", tricky.hash_key);




    // UCI::search_position(5, &board);

    // // let bb = Evaluation::evaluate(&board);
    // // println!("BBBBBB {}", bb);
    // println!("{}", board.to_string());
    // let instant = Instant::now();
    // UCI::search_position(7, &board);
    // let elapsed = instant.elapsed();
    // println!("      Time: {}ms", elapsed.as_millis());
    
    
    // let _ = UCI::default().reader();

    let result = Perft::start(1);


    // let mut hash_key = 0u64;
    // hash_key ^= ZOBRIST.piece_keys[Piece::WP][Square::A2];

    println!("{}", board.to_string());
    println!("{}", new_board.to_string());
    println!("the new key should be {0:x}", new_board.hash_key());

    // board.hash_key();

}