use std::{ops::Neg, sync::atomic::{AtomicBool, Ordering}};

use crate::{board::{piece::Piece, state::board_state::BoardState}, move_type::MoveType::*, squares::Square};

use super::evaluation::Evaluation;

const MOVE_MAX: usize = 100;

/// "MVV/LVA" stands for "Most Valuable Victim/Least Valuable Attacker".
pub(crate) struct AlphaBeta {
    /// pv means principal variation
    found_pv: bool,

    // line
    cmove: usize, // Number of moves in the line
    arg_move: [u32; MOVE_MAX], // The line.
    timeout: AtomicBool,
}


impl AlphaBeta {
    pub(crate) fn evaluate(board: &BoardState) -> i32 {
        Evaluation::evaluate(board)
    }

    pub(crate) fn iterative_deepening(&mut self, limit: usize, alpha: i32, beta: i32, board: &BoardState) {
        let mut alpha = alpha; let mut beta = beta;
        
        for depth in 1..=limit {
            let val = &self.alpha_beta(depth, &mut alpha, &mut beta, board);

            if self.timeout.load(Ordering::Relaxed) { break; }

            if *val <= alpha || *val >= beta {
                // alpha 
            }
        }
    }

    pub(crate) fn alpha_beta(&mut self, depth: usize, alpha: &mut i32, beta: &mut i32, board: &BoardState) -> i32 {
        self.found_pv = false;
        if depth == 0 {
            return Self::evaluate(board);
        }

        let moves = board.gen_movement();
        for mv in moves {
            // make next move
            let Some(new_board) = board.make_move(mv, AllMoves) else {continue};
            
            let val = match  self.found_pv {
                true => {
                    // If we found any pv, we search with (alpha, alpha_1)
                    let mut score = -self.alpha_beta(depth-1, &mut (alpha.neg()-1), &mut alpha.neg(), &new_board);

                    if (score > *alpha) && (score < *beta) { // check for failure
                        score = -self.alpha_beta(depth-1, &mut beta.neg(), &mut alpha.neg(), &new_board);
                    }
                    score
                }
                false => {
                    // if no pv has been found, "AlphaBeta()" is recursed normally.
                    -self.alpha_beta(depth-1, &mut beta.neg(), &mut alpha.neg(), &new_board)
                }
            };

            // unmake move, but since we never mutated the earlier one, we can just continue with `board`
            if val >= *beta {
                return *beta;
            }
            if val > *alpha {
                *alpha = val;
                self.found_pv = true;
            }
        }
        
        *alpha
    }

    /// A quiescent seasrch is an evaluation function that takes into account some dynamic possibilities
    pub(crate) fn quiescence(&mut self, alpha: &mut i32, beta: &mut i32, board: &BoardState) -> i32 {
        let king_sq = board[Piece::king(board.turn)].trailing_zeros() as u64;
        let king_in_check = board.is_square_attacked(king_sq, !board.turn);
        if king_in_check {
            return self.alpha_beta(1, alpha, beta, board);
        }

        let val = Evaluation::evaluate(board);
        if val >= *beta {
            return *beta
        }
        if val > *alpha {
            *alpha = val;
        }

        // generate good captures
        let moves = board.gen_movement();
        for mv in moves {
            let Some (new_board) = board.make_move(mv, CapturesOnly) else {continue};
            let val = -self.quiescence(&mut beta.neg(), &mut alpha.neg(), &new_board);
            // unmake move
            if val >= *beta {
                return *beta;
            }
            if val > *alpha {
                *alpha = val;
            }
        }
        *alpha
    }
}