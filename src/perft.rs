use std::borrow::Borrow;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use lazy_static::lazy_static;

use crate::bit_move::BitMove;
use crate::constants::START_POSITION;
use crate::move_type::MoveType;
use crate::{board::board_state::BoardState, constants::TRICKY_POSITION};
use crate::board::fen::FEN;

pub(crate) struct Perft;



lazy_static! {
    #[derive(Debug)]
    pub static ref CASTLES: Mutex<i16> = Mutex::new(0);
    pub static ref KING_CHECKS: Mutex<i16> = Mutex::new(0);
    pub static ref PROMOTIONS: Mutex<i16> = Mutex::new(0);
    pub static ref ENPASSANT: Mutex<i16> = Mutex::new(0);
    pub static ref CAPTURES: Mutex<i16> = Mutex::new(0);
}


impl Perft {
    // #[inline(always)]
    pub(crate) fn driver(depth: usize, nodes: &mut usize, board: BoardState) {
        // println!("((((((((((((((((((****************************************))))))))))))))))))");
        if depth == 0 {
            *nodes += 1;
            return;
        }

        // println!("{}", board.to_string());
        let move_list =board.gen_movement();

        for index in 0..move_list.count() {
            // println!("---------------------------------------------------------------------------------2222");
            let bmove = move_list.list[index];
            let legal_move = board.make_move(bmove, MoveType::AllMoves);
            if let Some(new_board) = legal_move {
                // println!("SRC---->>>> {}        TARGET----->>>>> {}          [[[[[[turn]]]]]] {:?}", bmove.get_src(), bmove.get_target(), board.turn);
                // println!("  {}", new_board.to_string());
                Perft::driver(depth-1, nodes, new_board);
            }
        }
    }

    // pub(crate) fn cummulative_nodes() {}


    pub(crate) fn start(depth: usize) {
        println!("STARTED!!");
        let mut nodes = 0;
        let instant = Instant::now();
        let board = BoardState::parse_fen(TRICKY_POSITION).unwrap();
        Self::test(depth, &mut nodes, board);
        
        // let board = BoardState::parse_fen(TRICKY_POSITION).unwrap();
        // println!("{}", board.to_string());
        // // let lt = BitMove::from(2099086);
        // let bm = BitMove::from(2098696);
        // let xot = BitMove::from(2099086);
        // println!("src={}  target={} \n src={}     target={} \n\n", bm.get_src(), bm.get_target(), xot.get_src(), xot.get_target());
        // let board = board.make_move(bm, MoveType::AllMoves).unwrap();
        // Self::test(depth-1, &mut nodes, board);

        let elapsed = instant.elapsed();
        println!("\n\n");
        println!("      Depth: {depth}");
        println!("      Nodes: {nodes}");
        println!("      Time: {}ms", elapsed.as_millis());
        println!("      Done!!!");
        println!("      castles??? {:?}", CASTLES.lock().unwrap());
        println!("      PROMOTIONS =>> {:?}", PROMOTIONS.lock().unwrap());
        println!("      ENPASSANT =>> {:?}", ENPASSANT.lock().unwrap());
        println!("      CAPTURES =>> {:?}", CAPTURES.lock().unwrap());
        
    }

    pub(crate) fn test(depth: usize, nodes: &mut usize, board: BoardState) {
        // println!("{}", board.to_string());
        // println!("{}", board.to_string());
        let move_list = board.gen_movement();


        for index in 0..move_list.count() {
            // println!("------------------------------------------------------------------------------------------------------------1111");
            let bmove = move_list.list[index];
            let legal_move = board.make_move(bmove, MoveType::AllMoves);
            if let Some(new_board) = legal_move {
                // println!("SRC---->>>> {}        TARGET----->>>>> {}          [[[[[[turn]]]]]] {:?}", bmove.get_src(), bmove.get_target(), board.turn);
                // println!("{}", new_board.to_string());
                let cummulative_nodes = *nodes;
                Perft::driver(depth-1, nodes, new_board);
                let old_nodes = *nodes - cummulative_nodes;
                if let Some(p) = bmove.get_promotion() {
                    println!("      move: {}{}{}2     nodes: {:?}", bmove.get_src(), bmove.get_target(), p, old_nodes);
                } else {
                    // println!("SRC---->>>> {}        TARGET----->>>>> {}          [[[[[[CAPTURE]]]]]] {}", bmove.get_src(), bmove.get_target(), bmove.get_capture());
                    println!("      move: {}{}     nodes: {:?}", bmove.get_src(), bmove.get_target(), old_nodes);
                }
            }
            // println!("\n\n\n\n");
        }
    }
}