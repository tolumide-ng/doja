use std::ops::Neg;

use crate::{bit_move::BitMove, board::{board::Board, board_state::BoardState, piece::Piece}, constants::{MAX_PLY, SQUARES, TOTAL_PIECES}, move_type::MoveType, moves::Moves};

use super::evaluation::Evaluation;

pub struct NegaMax {
    killer_moves: [[u32; 64]; 2],
    history_moves: [[u32; SQUARES]; TOTAL_PIECES],
    pv_length: [usize; 64],
    pv_table: [[i32; 64]; MAX_PLY],
    nodes: u32,
    /// ply: is the distance to the root, see: https://www.chessprogramming.org/Root
    ply: usize,
    follow_pv: bool,
    score_pv: u32,
}


impl NegaMax {
    fn new() -> Self {
        Self {
            killer_moves: [[9; 64]; 2], history_moves: [[0; 64]; 12], pv_length: [0; 64], pv_table: [[0; 64]; 64], nodes: 0, ply: 0, follow_pv: false, score_pv: 0
        }
    }

    fn iterative_deepening(&mut self, limit: usize, alpha: i32, beta: i32, board: &BoardState) {
        for depth in 1..=limit {
            self.follow_pv = true;
            self.negamax(alpha, beta, depth, board);
        }
    }
    
    pub(crate) fn run(alpha: i32, beta: i32, depth: usize, board: &BoardState) {
        let mut negamax = Self::new();
        negamax.iterative_deepening(depth, alpha, beta, board);
        println!("I am now done >>>>>");
        // println!("{:?}", negamax.pv_table[0]);
        // let rr = Self::iterative_deepening(&mut self, limit, alpha, beta, board);

        for count in 0..negamax.pv_length[0] as usize {
            println!("PV TABLE {}", BitMove::from(negamax.pv_table[0][count] as u32))
        }
        println!("number of nodes is {}", negamax.nodes)
        // (r.0, Some(BitMove::from(nega_max.pv_table[0][0]as u32)))
    }

    fn enable_pv_scoring(&mut self, moves: &Moves) {
        // disable following pv
        self.follow_pv = false;

        for mv in moves .into_iter(){
            if self.pv_table[0][self.ply] == (*mv) as i32 {
                // enable scoring
                self.score_pv = 1;
                // enable following pv
                self.follow_pv = true;
            }
        }
    }


      /// mv: Move (please remove the mut later, and find a abtter way to write this)
    pub(crate) fn score_move(&mut self, board: &BoardState, mv: BitMove) -> u32 {
        if self.score_pv != 0 {
            if self.pv_table[0][self.ply] == (*mv) as i32 {
                self.score_pv = 0;
                return 20_000;
            }
        }
        if let Some(victim) = board.get_move_capture(mv, !board.turn) {
            // println!("{} >>>>>> |||| captured piece is {}", mv, victim.to_string());
            // score move by MVV LVA lookup
            let attacker = mv.get_piece();
            let score = attacker.get_mvv_lva(&victim)  + 10_000;
            return score;
        } else {
            if let Some(kill_move) = self.killer_moves[0].get(self.ply) {
                // score 1st killer move
                if *kill_move == *mv {
                    return 9_000
                }
            }
            
            // score  2nd killer move
            if let Some(kill_move) = self.killer_moves[1].get(self.ply) {
                 if *kill_move == *mv {
                     return 8_000
                 }
            }
            
            // score history move
            return self.history_moves[mv.get_piece()][mv.get_target()];

        }

        // return 0
    }

    /// todo! add target on the BitMove, so that this cmp method can be implenented directly on Moves(MvList), that way
    /// we wouldn't need this one anymore
    pub(crate) fn sort_moves(&mut self, board: &BoardState, mv_list: Moves) -> Vec<BitMove> {
        let mut sorted_moves: Vec<BitMove> = Vec::with_capacity(mv_list.count());
        sorted_moves.extend_from_slice(&mv_list.list[..mv_list.count()]);
        sorted_moves.sort_by(|b, a| self.score_move(board, *a).cmp(&self.score_move(board, *b)));
        return sorted_moves
    }
}

/// this trait can be implemented by the planed search generic/struct
impl NegaMax {
    /// https://www.chessprogramming.org/Quiescence_Search
    fn quiescence(&mut self, mut alpha: i32, beta: i32, board: &BoardState) -> i32 {
        self.nodes += 1;
        
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

        let sorted_moves = self.sort_moves(board, board.gen_movement().into_iter());

        for mv in sorted_moves {
            if let Some(new_board) = board.make_move(mv, MoveType::AllMoves) {
                self.ply += 1;
                let score = -self.quiescence(-beta, -alpha, &new_board);
                self.ply -=1;
                
                if score >= beta { return beta }
                if score > alpha { alpha = score; }
            }
        }

        return alpha
    }

    
    /// https://www.chessprogramming.org/Alpha-Beta#Negamax_Framework
    fn negamax(&mut self, mut alpha: i32, beta: i32, depth: usize, board: &BoardState) -> i32 {
        // println!("original depth is {}", depth);
        self.pv_length[self.ply] = self.ply;
        if depth == 0 {
            return self.quiescence(alpha, beta, board);
        }

        if self.ply > MAX_PLY -1{
            return Evaluation::evaluate(board) as i32;
        }

        self.nodes+=1;

        let king_square = u64::from(board[Piece::king(board.turn)].trailing_zeros());
        // is king in check
        let king_in_check = board.is_square_attacked(king_square, !board.turn);
        let mut legal_moves = 0;

        let moves = board.gen_movement().into_iter();
        if self.follow_pv {
            self.enable_pv_scoring(&moves);
        }
        let generated_moves = self.sort_moves(board, moves);

        // https://www.chessprogramming.org/Principal_Variation_Search#Pseudo_Code
        let mut b_search_pv = true;

        // loop through hte moves
        for mv in generated_moves {
            self.ply +=1;


            let play_move = board.make_move(mv, MoveType::AllMoves);
            if play_move.is_none() {
                self.ply -=1;
                continue;
            }

            if let Some(new_board) = play_move {
                legal_moves += 1;


                let score = match b_search_pv {
                    true => -self.negamax(-beta, -alpha, depth-1, &new_board),
                    false => {
                        let mut score = -self.negamax(-alpha-1, -alpha, depth-1, &new_board);
                        if score > alpha && score < beta {
                            score = -self.negamax(-beta, -alpha, depth-1, &new_board);
                        }
                        score
                    }
                };

                
                self.ply -=1;
        
                // fail-hard beta cutoff
                if score >= beta {
                    if !mv.get_capture() { // quiet move (non-capturing quiet move that beats the opponent)
                        self.killer_moves[1][self.ply] = self.killer_moves[0][self.ply];
                        self.killer_moves[0][self.ply] = *mv;
                    }
                    // node/move fails high
                    return beta
                }

                // best score so far
                if score > alpha {
                    // store history moces
                    if !mv.get_capture() {
                        if let Some(history_move) = self.history_moves[mv.get_piece()].get_mut(mv.get_target() as usize) {
                            *history_move += depth as u32;
                        }
                    }

                    alpha = score; // alpha acts like max in Minimax

                    self.pv_table[self.ply][self.ply] =  *mv as i32;
                    for next_ply in (self.ply+1)..self.pv_length[self.ply+1] {
                        // copy move from deeper ply into current ply's line
                        self.pv_table[self.ply][next_ply] = self.pv_table[self.ply+1][next_ply];
                    }

                    self.pv_length[self.ply] = self.pv_length[self.ply + 1];

                    b_search_pv = false;
                }
            }

        }


        // we don't have any legal moves to make in the current position
        if legal_moves == 0 {
            // is king in check
            if king_in_check {
                return -49_000 + (self.ply) as i32;
            }
            // king is not in check and there are not legal moves
            return 0 // stalemate | draw
        }

        return alpha
    }

}