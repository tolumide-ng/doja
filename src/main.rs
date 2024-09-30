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
    
    thread::scope(|s| {
        let tt = Arc::new(Mutex::new(TTable::default()));
        // let tt = TTable::default();
        // let controller = Arc::new(Mutex::new(Control::default()));
        // let board = Position::with(Board::parse_fen(TRICKY_POSITION).unwrap());
        // let threads = std::thread::available_parallelism().unwrap_or(NonZero::<usize>::new(1).unwrap()).get();
        // let depth = 7;

        // let moves = board.gen_movement();
        // let moves_vec = moves.list[0..moves.count_mvs()].to_vec();
        // let chunks = (moves_vec.chunks(threads).collect::<Vec<_>>()).iter().map(|m| m.to_vec()).collect::<Vec<Vec<_>>>();
        // for mm in chunks {
        //     let bb = board.clone();
        //     let cc = Arc::clone(&controller);
        //     let tt = Arc::clone(&tt);
        //     s.spawn(move || {
        //         for mv in mm {
        //             let mut bb = bb.clone();
        //             bb.make_move_nnue(mv, MoveScope::AllMoves);
        //             NegaMax::run(cc.clone(), tt.clone(), depth, &mut bb);
        //         }
        //     });
        // }
    });
    
    // let mut threads = vec![];

    // for _ in 0..10 {
    //     let cc = Arc::clone(&controller);
    //     // let t = thread::spawn(move || {
    //     // });

    //     // threads.push(t);
    // }

    // let axx = [];

    // thread::scope(|s| {

    //     // let mut ttt = Vec::with_capacity(10);

    //     // let xtt = Arc::new(tt);

    //     let negamax = NegaMax::new(controller, &tt);
    //     // negamax.iterative_deepening(depth, board);
    //     let limit = 10;

    //     // for i in 0.. 6 {
    //     //     let mut bb = board.clone();
    //     //     // let cc = Arc::clone(&controller);

    //     //     let mut nn = negamax.clone();

    //     //     // ttt.push(s.spawn(move || NegaMax::run(cc, &tt, 7, &mut bb)));
    //     //     println!("########################################################################################################");
    //     //     s.spawn(move|| {
    //     //         // let mut ii = limit;
    //     //         // if i <= limit {
    //     //         //     ii = ii;
    //     //         // }
    //     //         println!("i is ------------------------------------------------------------------------>>>>>>>>>>>>>>>>>>>>>>>>>>>> {}", i);
    //     //         nn.iterative_deepening(i, &mut bb);
    //     //         println!("****************************");
    //     //     });
    //     // }
    // });

    // checkings();



    // println!()
}

// 8|4|2|1|
// 0|0|1|0|
// 1|0|0|1|
// 0|0|1|1|
// 1|0|0|0|
  
