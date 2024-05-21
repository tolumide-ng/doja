use std::sync::Arc;
use std::time::Instant;

use crate::constants::START_POSITION;
use crate::move_type::MoveType;
use crate::{board::board_state::BoardState, constants::TRICKY_POSITION};
use crate::board::fen::FEN;

pub(crate) struct Perft;


impl Perft {
    // #[inline(always)]
    pub(crate) fn driver(depth: usize, nodes: &mut usize, board: BoardState) {
        if depth == 0 {
            *nodes += 1;
            return;
        }


        // println!("{}", board.to_string());
        let move_list =board.gen_movement();

        for index in 0..move_list.count() {
            let bmove = move_list.list[index];
            let legal_move = board.make_move(bmove, MoveType::AllMoves);
            if let Some(new_board) = legal_move {
                Perft::driver(depth-1, nodes, new_board);
            }
        }
    }

    // pub(crate) fn cummulative_nodes() {}


    pub(crate) fn start(depth: usize) {
        println!("STARTED!!");
        let mut nodes = 0;
        let instant = Instant::now();
        Self::test(depth, &mut nodes, BoardState::parse_fen(START_POSITION).unwrap());
        let elapsed = instant.elapsed();
        println!("\n\n");
        println!("      Depth: {depth}");
        println!("      Nodes: {nodes}");
        println!("      Time: {}ms", elapsed.as_millis());
        println!("      Done!!!");
    }

    pub(crate) fn test(depth: usize, nodes: &mut usize, board: BoardState) {
        if depth == 0 {
            *nodes += 1;
            return;
        }


        // println!("{}", board.to_string());
        let move_list =board.gen_movement();

        for index in 0..move_list.count() {
            let cummulative_nodes = *nodes;
            let bmove = move_list.list[index];
            let legal_move = board.make_move(bmove, MoveType::AllMoves);
            if let Some(new_board) = legal_move {
                Perft::driver(depth-1, nodes, new_board);
                let old_nodes = *nodes - cummulative_nodes;
                println!("      move: {}{}     nodes: {:?}", bmove.get_src(), bmove.get_target(), old_nodes);
            }
        }
    }
}