use std::sync::Arc;
use std::time::Instant;

use crate::move_type::MoveType;
use crate::{board::board_state::BoardState, constants::TRICKY_POSITION};
use crate::board::fen::FEN;

pub(crate) struct Perft;


impl Perft {
    pub(crate) fn run(depth: usize, nodes: &mut usize, board: BoardState) {
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
                let nodes = Perft::run(depth-1, nodes, new_board);
            }
        }
    }


    pub(crate) fn start() {
        println!("STARTED!!");
        let mut nodes = 0;
        let instant = Instant::now();
        Self::run(1, &mut nodes, BoardState::parse_fen(TRICKY_POSITION).unwrap());
        let elapsed = instant.elapsed();
        println!("{nodes} nodes in time: {}ms", elapsed.as_millis());
        println!("done!!!");
    }
}