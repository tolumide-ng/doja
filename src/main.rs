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




use std::{num::NonZero, sync::{Arc, Mutex}};

use move_logic::bitmove::{Move, MoveType};
use board::{position::Position, state::board::Board};
use color::Color;
use constants::TRICKY_POSITION;
use nnue::halfka_idx;
use search::{control::Control, search::Search};
use squares::Square;
use syzygy::probe::TableBase;
use tt::table::TTable;
// use zobrist::Zobrist;

// use crate::search::alpha_beta::NegaMax;





// #[tokio::main]
fn main() {
    // let _ = UCI::default().reader();

    // println!("**********************AFTER*****************************");
    
    // let mut board = Position::with(Board::fromunwrap());
    // println!("**********************BEFORE*****************************");
    // println!("{}", board.to_string());
    

    println!("num of cpus {:?}", std::thread::available_parallelism().unwrap_or(NonZero::<usize>::new(1).unwrap()));
    // let tt = TTable::default();
    let controller = Arc::new(Mutex::new(Control::default()));
    // let mut board = Position::with(Board::fromunwrap());
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

    use crate::squares::Square::*;
    use crate::board::piece::Piece::*;
    // use crate::move_logic::bitmoveMoveType::*;

    // println!("before {}", board.evaluate());

    // let mm = board.make_move_nnue(Move::new(F3 as u8, F6 as u8, Capture), move_scope::MoveScope::AllMoves);

    // println!("{mm} mmmmm is {}", board.evaluate());
    
    // println!("{}", board.to_string());

    // let mm = board.make_move_nnue(Move::new(E7 as u8, F6 as u8, Capture), move_scope::MoveScope::AllMoves);

    // println!("xxxxx {mm} {}", board.to_string());
    // board.set_turn(Color::Black);

    // let fen: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1 "; // white
    // let fen: &str = "r3k2r/p1ppqpb1/bn2pQp1/3PN3/1p2P3/2N4p/PPPBBPPP/R3K2R b KQkq - 0 1"; // black after queen attack
    
    // let fen: &str = "8/8/2k5/8/4B1P1/8/5K2/8 w ---- 0 1";

    // let fen = "8/3n4/q3q3/5k2/8/8/3K4/8 w - - 0 1"; // black to win (-64)
    // // let fen = "8/1k3p2/8/8/3P4/2Q2N2/3K4/8 w - - 0 1"; // white would win (3257)
    // // let fen = TRICKY_POSITION;
    // let board = Position::from(Board::fromunwrap());
    // println!("board {}", board.to_string());
    // board.get_all_attacks(Square::C3);
    // println!("eval now ::::: {}", board.evaluate());


    // // println!("H8 is {}", H8 as usize);

    // let bk_63 = halfka_idx(BK, H8);
    // let wk_63 = halfka_idx(WK, H8);
    // let bk_56 = halfka_idx(BQ, A8);
    // let wp_01 = halfka_idx(WP, A1);

    // // println!("black king on 63 --->>>> {:?}", bk_63);
    // // println!("white king on 63 --->>>> {:?}", wk_63);
    // // println!("black queen on 56 --->>>> {:?}", bk_56);
    // // println!("white pawn on 01 --->>>> {:?}", wp_01);

    // println!("H8 index is {}", H8 as usize);

    // // let mut search = Search::new(table.get());
    // // search.iterative_deepening(5, &mut board);


    // // let xx0 = Search::see(&board, &Move::new(F3 as u8, F6 as u8, Capture));
    // // let xx0 = Search::see(&board, &Move::new(D5 as u8, E6 as u8, Capture));

    // println!("the xx0 is {}", xx0);


    // let mut board = Position::from(Board::try_from("r3k2r/p1ppqpb1/bn2pnp1/3PN3/4P3/2p2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1 ").unwrap());
    let mut board = Position::from(Board::try_from(TRICKY_POSITION).unwrap());
    println!("{}", board.to_string());
    let mut search = Search::new(table.get());
    search.iterative_deepening(6, &mut board);
    // board.get_all_attacks(Square::A6);
    // Search::see(&board, &Move::new(F3 as u8, H3 as u8, MoveType::Capture), 0);

    // println!("----------------------->>>>>>>>>         {}", board.evaluate());


    // let s0 = Position::from(Board::try_from("r3k2r/p1ppqpb1/Bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPB1PPP/R3K2R w KQkq - 0 1 ").unwrap()).evaluate();
    // println!("white bishop captures black bishop>>>>> {s0}",);

    // let board = Position::from(Board::try_from("r3k2r/p1ppqpb1/Bn21np1/3pN3/1p2P3/2N2Q1p/PPPB1PPP/R3K2R b KQkq - 0 1 ").unwrap());
    // println!("{}", board.to_string());
    // let s01 = board.evaluate();
    // println!("white bishop captures black bishop>>>>> {s01}",);

    // let board = Position::from(Board::try_from("r3k2r/p1ppqpb1/Bn2pnp1/3PN3/4P3/2p2Q1p/PPPB1PPP/R3K2R b KQkq - 0 1 ").unwrap());
    // println!("{}", board.to_string());
    // let s02 = board.evaluate();
    // println!("white bishop captures black bishop>>>>> {s02}",);
    

}

// 8|4|2|1|
// 0|0|1|0|
// 1|0|0|1|
// 0|0|1|1|
// 1|0|0|0|
  
