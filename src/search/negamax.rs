use crate::{board::board_state::BoardState, move_type::MoveType};

use super::evaluation::Evaluation;

// lazy_static! {
//     static ref completes_moves_for_now = 0;
// }

/// this trait can be implemented by the planed search generic/struct
pub(crate) trait NegaMax {
    /// https://www.chessprogramming.org/Alpha-Beta#Negamax_Framework
    fn negamax(mut alpha: i32, beta: i32, mut played: u16, nodes: u16, depth: u16, board: &BoardState) -> i32 {
        if depth == 0 {
            return Evaluation::evaluate(board) as i32;
        }

        let mut best_move_so_far = None;
        let mut old_alpha = alpha;

        // generate moves
        let moves = board.gen_movement();


        // let mut play_count = 0;

        // loop through hte moves
        for mv in moves.into_iter() {
            if let Some(_) = board.make_move(mv, MoveType::AllMoves) {
                played +=1;
            }
            
            let score = - Self::negamax(-alpha, -beta, played, nodes+1, depth-1, board);
            played -= 1;
    
            // fail-hard beta cutoff
            if score >= beta {
                // node/move fails high
                return beta
            }
    
            if score > alpha {
                alpha = score;

                if played == 0 {
                    // associate best move with the best score
                    best_move_so_far = Some(mv);
                }
            }
        }

        if alpha != old_alpha {
            // iniitalize best move
        }


        return alpha
    }
}