use crate::{board::state::board_state::BoardState, move_type::MoveType::*};

use super::evaluation::Evaluation;

pub(crate) struct AlphaBeta {}


impl AlphaBeta {
    pub(crate) fn evaluate(board: &BoardState) -> i32 {
        Evaluation::evaluate(board)
    }

    pub(crate) fn alpha_beta(depth: usize, alpha: i32, beta: i32, board: &BoardState) -> i32 {
        if depth == 0 {
            return Self::evaluate(board);
        }

        let moves = board.gen_movement();
        for mv in moves {
            let Some(new_board) = board.make_move(mv, AllMoves) else {continue};
            let val = -Self::alpha_beta(depth-1, -beta, -alpha, &new_board);
            // unmake move, but since we never mutated the earlier one, we can just continue with `board`
        }
        0
    }
}