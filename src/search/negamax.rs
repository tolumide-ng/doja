use crate::{bit_move::BitMove, board::board_state::BoardState, move_type::MoveType};

use super::evaluation::Evaluation;


/// this trait can be implemented by the planed search generic/struct
pub(crate) trait NegaMax {
    /// https://www.chessprogramming.org/Alpha-Beta#Negamax_Framework
    fn negamax(mut alpha: i32, beta: i32, mut played: i32, nodes: u16, depth: u16, board: &BoardState) -> (i32, Option<BitMove>) {
        if depth == 0 {
            return (Evaluation::evaluate(board) as i32, None);
        }

        // let mut best_move_so_far = None;
        let mut best_move = None;
        let old_alpha = alpha;

        // generate moves
        let moves = board.gen_movement();

        // loop through hte moves
        for mv in moves.into_iter() {
            played +=1;


            let play_move = board.make_move(mv, MoveType::AllMoves);
            if play_move.is_none() {
                played -=1;
                continue;
            }

            if let Some(new_board) = board.make_move(mv, MoveType::AllMoves) {
                let (score, _best_move) = Self::negamax(-beta, -alpha, played, nodes+1, depth-1, &new_board);
                let score = -score;
                
                played -=1;
        
                // fail-hard beta cutoff
                if score >= beta {
                    // node/move fails high
                    return (beta, best_move)
                }

                // best score so far
                if score > alpha {
                    alpha = score;
    

                    if played == 0 && alpha != old_alpha {
                        // associate best move with the best score
                        best_move = Some(mv); // the move where we got the best score so far
                    }
                }
            }
        }

        // println!("alpha -- {alpha}, old_alpha -- {old_alpha}");

        // if alpha != old_alpha {
        //     // iniitalize best move
        //     best_move = best_move_so_far;
        //     println!("Best Move {:?}", best_move);
        // }



        return (alpha, best_move)
    }
}