use crate::{bit_move::BitMove, board::{board_state::BoardState, piece::Piece}, move_type::MoveType};

use super::evaluation::Evaluation;


/// this trait can be implemented by the planed search generic/struct
pub(crate) trait NegaMax {
    /// https://www.chessprogramming.org/Quiescence_Search
    fn quiescence(mut alpha: i32, beta: i32, mut nodes: u16, board: &BoardState) -> i32 {
        nodes += 1;
        // evaluate position
        let evaluation = Evaluation::evaluate(board) as i32;
        // fail head beta cutoff
        if evaluation >= beta {
            // node (move) fails high
            return beta;
        }
        if evaluation > alpha { // found a better score
            alpha = evaluation;
        }

        for mv in board.gen_movement().into_iter() {
            if let Some(new_board) = board.make_move(mv, MoveType::AllMoves) {
                let score = -Self::quiescence(-beta, -alpha, nodes,&new_board);
                
                if score >= beta { return beta }
                if score > alpha { alpha = score; }
            }
        }

        return alpha
    }

    /// https://www.chessprogramming.org/Alpha-Beta#Negamax_Framework
    fn negamax(mut alpha: i32, beta: i32, mut played: i32, nodes: u16, depth: u16, board: &BoardState) -> (i32, Option<BitMove>) {
        if depth == 0 {
            return (Self::quiescence(alpha, beta, nodes, board), None);
        }

        // let mut best_move_so_far = None;
        let mut best_move = None;
        let old_alpha = alpha;

        let king_square = u64::from(board[Piece::king(board.turn)].trailing_zeros());
        // is king in check
        let king_in_check = board.is_square_attacked(king_square, !board.turn);
        let mut legal_moves = 0;

        // legal moves counter


        // loop through hte moves
        for mv in board.gen_movement().into_iter() {
            played +=1;


            let play_move = board.make_move(mv, MoveType::AllMoves);
            if play_move.is_none() {
                played -=1;
                continue;
            }

            if let Some(new_board) = board.make_move(mv, MoveType::AllMoves) {
                legal_moves +=1 ;
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

        // we don't have any legal moves to make in the current position
        if legal_moves == 0 {
            // is king in check
            if king_in_check {
                return (-49_000 + played, None);
            }
            // king is not in check and there are not legal moves
            return (0, None) // stalemate | draw
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