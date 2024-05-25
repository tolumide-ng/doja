use std::borrow::Borrow;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use lazy_static::lazy_static;

use crate::bit_move::BitMove;
use crate::board::castling::Castling;
use crate::constants::{POSITION_4, POS_6, START_POSITION};
use crate::move_type::MoveType;
use crate::squares::Square;
use crate::{board::board_state::BoardState, constants::TRICKY_POSITION};
use crate::board::fen::FEN;

pub(crate) struct Perft;



lazy_static! {
    #[derive(Debug)]
    pub static ref CASTLES: Mutex<i16> = Mutex::new(0);
    pub static ref KING_CHECKS: Mutex<i16> = Mutex::new(0);
    pub static ref PROMOTIONS: Mutex<i16> = Mutex::new(0);
    pub static ref ENPASSANT: Mutex<i16> = Mutex::new(0);
    pub static ref CAPTURES: Mutex<i64> = Mutex::new(0);
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
        // println!("count for mv {}", move_list.count());

        for index in 0..move_list.count() {
            // println!("---------------------------------------------------------------------------------2222");
            let bmove = move_list.list[index];
            let legal_move = board.make_move(bmove, MoveType::AllMoves);
            if let Some(new_board) = legal_move {
                // println!("SRC---->>>> {}        TARGET----->>>>> {}          [[[[[[turn]]]]]] {:?}", bmove.get_src(), bmove.get_target(), board.turn);
                // println!("  {}", new_board.to_string());
                
                if bmove.get_capture() {
                    if let Ok(mut val) = CAPTURES.lock() {
                        *val +=1;
                    }
                }
                if bmove.get_promotion().is_some() {
                    if let Ok(mut val) = PROMOTIONS.lock() {
                        *val +=1;
                    }
                }
                if bmove.get_enpassant() {
                    if let Ok(mut val) = ENPASSANT.lock() {
                        *val +=1;
                    }
                }
                if bmove.get_castling() {
                    if let Ok(mut val) = CASTLES.lock() {
                       *val +=1;
                    }
                    if depth == 1 {
                        // println!()
                        // println!("{}", new_board.to_string());
                    }
                }
                
                Perft::driver(depth-1, nodes, new_board);
            }
        }
    }

    // pub(crate) fn cummulative_nodes() {}


    pub(crate) fn start(depth: usize) {
        println!("STARTED!!");
        let mut nodes = 0;
        let instant = Instant::now();
        let board = BoardState::parse_fen(START_POSITION).unwrap();
        println!("{}", board.to_string());
        Self::test(depth, &mut nodes, board);

        
        // let board = BoardState::parse_fen(TRICKY_POSITION).unwrap();
        // // let lt = BitMove::from(2099086);
        // let bm = BitMove::from(2098696);
        // let xot = BitMove::from(2099086);

        
        // let new_mask = d2c1.get_src().castling_mask() | d2c1.get_target().castling_mask();
        // println!("new mask ---- {0:04b}", new_mask);
        // let existing_rights = board.castling_rights.bits() & new_mask;
        // println!("existing_rights ---- {0:04b}", existing_rights);
        // let new_rights = board.castling_rights.bits() & !existing_rights;
        // println!("existing_rights ---- {0:04b}", new_rights);
        // let nr  = Castling::from(new_rights);
        // println!("NR {0:04b}", nr);
        
        
        
        
        // let d2c1 = BitMove::from(8331);
        // let board_00 = board.make_move(d2c1, MoveType::AllMoves).unwrap();
        // let depth = depth -1;
        // println!("d2c1");
        // println!("{}", board_00.to_string());


        // let h3g2 = BitMove::from(1074071);
        // let board_01 = board_00.make_move(h3g2, MoveType::AllMoves).unwrap();
        // let depth = depth -1;
        // println!("h3g2");
        // println!("{}", board_01.to_string());


        // let c1d2 = BitMove::from(8898);
        // let board_02 = board_01.make_move(c1d2, MoveType::AllMoves).unwrap();
        // let depth = depth -1;
        // println!("{}", board_02.to_string());

        // let b4b3 = BitMove::from(25689);
        // let board_03 = board_02.make_move(b4b3, MoveType::AllMoves).unwrap();
        // let depth = depth -1;

        // let a2a3 = BitMove::from(1032);
        // let board_00 = board.make_move(a2a3, MoveType::AllMoves).unwrap();
        // let depth = depth -1;

        // let h8h7 = BitMove::from(40447);
        // let board_01 = board_00.make_move(h8h7, MoveType::AllMoves).unwrap();
        // let depth = depth -1;

        // let c3b5 = BitMove::from(6226);
        // let board_02 = board_01.make_move(c3b5, MoveType::AllMoves).unwrap();
        // let depth = depth -1;

        
        // // let b2b3 = BitMove::from(1097);
        // // let board_01 = board.make_move(b2b3, MoveType::AllMoves).unwrap();
        // // let depth = depth -1;
        
        // // let h3g2 = BitMove::from(1074071);
        // // let board_02 = board_01.make_move(h3g2, MoveType::AllMoves).unwrap();
        // // let depth = depth -1;

        // // let a2a3 = BitMove::from(1032);
        // // let board_03 = board_02.make_move(a2a3, MoveType::AllMoves).unwrap();
        // // let depth = depth -1;
        
        // // println!("{} {}", a2a3.get_src(), a2a3.get_target());

        // println!("{}", board_03.to_string());
        // Self::test(depth, &mut nodes, board_03);
        // // println!("src={}  target={} \n src={}     target={} \n\n", bm.get_src(), bm.get_target(), xot.get_src(), xot.get_target());





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

        // println!("count ----->>>>|||| {}", move_list.count());
        // for i in 0..move_list.count() {
        //     let x = move_list.list[i];
        //     // println!("the board {}", board.to_string());
        //     println!("piecei={} . src=={} target=={} double={} promotion={:?} enpassant={} castling={} captures={}", x.get_piece(), x.get_src(), x.get_target(), x.get_double_push(), x.get_promotion(), x.get_enpassant(), x.get_castling(), x.get_capture());
        //     // println!("\n\n\n");
        //     // print!("{},", x.to_string());
        // }
        // println!("the board {}", board.to_string());


        // for index in 0..move_list.count() {
        for index in 0..move_list.count() {
            let bmove = move_list.list[index];
            let legal_move = board.make_move(bmove, MoveType::AllMoves);
            if let Some(new_board) = legal_move {
                
                // println!("piecei={} . src=={} target=={} double={} promotion={:?} enpassant={} castling={} captures={}", bmove.get_piece(), bmove.get_src(), bmove.get_target(), bmove.get_double_push(), bmove.get_promotion(), bmove.get_enpassant(), bmove.get_castling(), bmove.get_capture());
                // if bmove.get_src() == Square::B4 && bmove.get_target() == Square::B3 {
                //     // println!("{} {}" bmove.get_src() == Square::G2, bmove.get_src() != Square::G2)
                //     // continue;
                //     println!("{:?} -->> {}", bmove, bmove.to_string());
                // }
                // println!("xxxx {:?}", bmove);
                // println!("piecei={} . src=={} target=={} double={} promotion={:?} enpassant={} castling={} captures={}", bmove.get_piece(), bmove.get_src(), bmove.get_target(), bmove.get_double_push(), bmove.get_promotion(), bmove.get_enpassant(), bmove.get_castling(), bmove.get_capture());
                // println!("------------------------------------------------------------------------------------------------------------1111");

                // println!("the board {}", board.to_string());
                // println!("the board {}", new_board.to_string());
                // println!("\n\n\n\n");
                // println!("SRC---->>>> {}        TARGET----->>>>> {}          [[[[[[turn]]]]]] {:?}", bmove.get_src(), bmove.get_target(), board.turn);
                // println!("{}", new_board.to_string());
                if bmove.get_capture() {
                    if let Ok(mut val) = CAPTURES.lock() {
                        *val +=1;
                    }
                }
                if bmove.get_promotion().is_some() {
                    if let Ok(mut val) = PROMOTIONS.lock() {
                        *val +=1;
                    }
                }
                if bmove.get_enpassant() {
                    if let Ok(mut val) = ENPASSANT.lock() {
                        *val +=1;
                    }
                }
                if bmove.get_castling() {
                    if let Ok(mut val) = CASTLES.lock() {
                       *val +=1;
                    }
                }
                let cummulative_nodes = *nodes;
                Perft::driver(depth-1, nodes, new_board);
                let old_nodes = *nodes - cummulative_nodes;
                if let Some(p) = bmove.get_promotion() {
                    println!("{}{}{}: {:?},", bmove.get_src(), bmove.get_target(), p, old_nodes);
                } else {
                    // println!("SRC---->>>> {}        TARGET----->>>>> {}          [[[[[[CAPTURE]]]]]] {}", bmove.get_src(), bmove.get_target(), bmove.get_capture());
                    // println!("      move: {}{}     nodes: {:?}", bmove.get_src(), bmove.get_target(), old_nodes);
                    println!("{}{}: {:?},", bmove.get_src(), bmove.get_target(), old_nodes);
                }
            }
            // println!("\n\n\n\n");
        }
    }
}