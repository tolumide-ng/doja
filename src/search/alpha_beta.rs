use std::{sync::{Arc, Mutex}, time::Instant};

use crate::{bit_move::BitMove, board::{state::board_state::BoardState, piece::Piece}, constants::{ALPHA, BETA, DEPTH_REDUCTION_FACTOR, FULL_DEPTH_MOVE, MATE_SCORE, MATE_VALUE, MAX_PLY, NODES_2047, REDUCTION_LIMIT, TOTAL_PIECES, TOTAL_SQUARES, VAL_WINDOW, ZOBRIST}, move_type::MoveType, moves::Moves, tt::{HashFlag, TTable}};
use super::{evaluation::Evaluation, time_control::TimeControl};


#[derive(Debug, Clone)]
pub struct NegaMax<T: TimeControl> {
    killer_moves: [[u32; 64]; 2],
    history_moves: [[u32; TOTAL_SQUARES]; TOTAL_PIECES],
    pv_length: [usize; 64],
    pv_table: [[i32; 64]; MAX_PLY],
    nodes: u64,
    ply: usize,
    /// pv - principal variation
    follow_pv: bool,
    score_pv: bool,
    controller: Arc<Mutex<T>>,
    /// Transposition table
    tt: TTable,
    repetition_index: usize,
    repetition_table: [u64; 500],
}


impl<T> NegaMax<T> where T: TimeControl {
    fn new(controller: Arc<Mutex<T>>) -> Self {
        let x = Self {
            killer_moves: [[9; 64]; 2], 
            history_moves: [[0; 64]; 12], 
            pv_length: [0; 64], 
            pv_table: [[0; 64]; 64], 
            nodes: 0, ply: 0, follow_pv: false, score_pv: false, controller,
            tt: TTable::default(),
            repetition_index: 0,
            repetition_table: [0; 500],
        };

        x
    }

    fn iterative_deepening(&mut self, limit: u8, alpha: i32, beta: i32, board: &BoardState) {
        let mut alpha = alpha;
        let mut beta = beta;

        for depth in 1..=(limit) {
            let start_time = Instant::now();
            // return 0 if time is up
            if self.controller.as_ref().lock().unwrap().stopped() { break; }

            self.follow_pv = true;
            println!("************************************************************depth is {depth}");
            let score = self.negamax(alpha, beta, depth, board);
            if (score <= alpha) || (score >= beta) {
                alpha = ALPHA; // We fell outside the window, so try again with a
                beta = BETA; //  full-width window (and the same depth).
                continue;
            }
            
            alpha = score - VAL_WINDOW;
            beta = score + VAL_WINDOW;
            // println!("at depth {depth}, with alpha as {alpha}, and bet as {beta}");
            

            if score > -MATE_VALUE && score < -MATE_SCORE {
                println!("info score mate {} depth {} nodes {} time {}ms pv", (-(score + MATE_VALUE)/2) -1, depth, self.nodes, start_time.elapsed().as_millis())
            } else if (score > MATE_SCORE) && score < MATE_VALUE {
                println!("info score mate {} depth {} nodes {} time {}ms pv", ((MATE_VALUE - score)/2) + 1, depth, self.nodes, start_time.elapsed().as_millis())
            } else {
                println!("info score cp {} depth {} nodes {} time {}ms pv", score, depth, self.nodes, start_time.elapsed().as_millis())
            }

            for count in 0..self.pv_length[0] as usize {
                print!("{}, ", BitMove::from(self.pv_table[0][count] as u32))
            }
            // println!("");
            println!("\n--------------------------");
        }
    }
    
    // This method is currently VERY SLOW once the depth starts approaching 8, please work to improve it
    pub(crate) fn run(controller: Arc<Mutex<T>>, alpha: i32, beta: i32, depth: u8, board: &BoardState) {
        let mut negamax = Self::new(controller);
        negamax.iterative_deepening(depth, alpha, beta, board);
        // println!("{:?}", negamax.pv_table[0]);
        // println!("{:?}\n\n", negamax.pv_length);
        // let rr = Self::iterative_deepening(&mut self, limit, alpha, beta, board);

        // for count in 0..negamax.pv_length[0] as usize {
        //     println!("PV TABLE {}", BitMove::from(negamax.pv_table[0][count] as u32))
        // }
        // if negamax.pv_length[0] == 0 {
        //     println!("PV table is none");
        // }
        // println!("number of nodes is {}", negamax.nodes)
        // (r.0, Some(BitMove::from(nega_max.pv_table[0][0]as u32)))
    }

    
    fn enable_pv_scoring(&mut self, moves: &Moves) {
        // disable following pv
        self.follow_pv = false;

        for mv in moves .into_iter(){
            if self.pv_table[0][self.ply] == (*mv) as i32 {
                // enable scoring
                self.score_pv = true;
                // enable following pv
                self.follow_pv = true;
            }
        }
    }


    /// mv: Move (please remove the mut later, and find a abtter way to write this)
    pub(crate) fn score_move(&mut self, board: &BoardState, mv: BitMove) -> u32 {
        if self.score_pv {
            if self.pv_table[0][self.ply] == (*mv) as i32 {
                self.score_pv = false;
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
        let mut sorted_moves: Vec<BitMove> = Vec::with_capacity(mv_list.count_mvs());
        // println!("the count is {}", mv_list.count_mvs());
        sorted_moves.extend_from_slice(&mv_list.list[..mv_list.count_mvs()]);
        sorted_moves.sort_by(|a, b| self.score_move(board, *b).cmp(&self.score_move(board, *a)));
        return sorted_moves
    }




  /// https://www.chessprogramming.org/Quiescence_Search
    fn quiescence(&mut self, mut alpha: i32, beta: i32, board: &BoardState) -> i32 {
        // this action will be performed every 2048 nodes
        if (self.nodes & NODES_2047) == 0 {
            self.controller.as_ref().lock().unwrap().communicate();
        }


        self.nodes+=1;
        // println!("NODES====== {:?}", self.nodes);
        
        // evaluate position
        let evaluation = Evaluation::evaluate(board);
        // fail head beta cutoff
        if evaluation >= beta { return beta; } // node (move) fails high
        if evaluation > alpha { alpha = evaluation; } // found a better score

        let sorted_moves = self.sort_moves(board, board.gen_movement().into_iter());

        for mv in sorted_moves {
            if let Some(new_board) = board.make_move(mv, MoveType::CapturesOnly) {
                self.ply += 1;
                self.repetition_index+=1;
                self.repetition_table[self.repetition_index] = new_board.hash_key;
                // print!("{}, ", self.ply);
                let score = -self.quiescence(-beta, -alpha, &new_board);
                self.ply -=1;
                self.repetition_index-=1;

                // return 0 if time is up
                if self.controller.as_ref().lock().unwrap().stopped() { return 0}
                
                if score > alpha { 
                    alpha = score; 
                    if score >= beta { return beta }
                }
            }
        }

        return alpha
    }

    fn is_repetition(&self, board: &BoardState) -> bool {
        for i in 0..self.repetition_index {
            if self.repetition_table[i] == board.hash_key {
                return true
            }
        }
        return false;
    }


    
    /// https://www.chessprogramming.org/Alpha-Beta#Negamax_Framework
    fn negamax(&mut self, mut alpha: i32, beta: i32, depth: u8, board: &BoardState) -> i32 {
        self.pv_length[self.ply] = self.ply;

        let mut hash_flag = HashFlag::UpperBound;
        if self.ply > 0 && self.is_repetition(board) || board.fifty.iter().any(|&p| p >= 100) {
            return 0 // draw
        }

        let pv_node = (beta - alpha) > 1;
        // if we had cached the score for this move before, we return it, and confirm that the current node is not a PV node(principal variation)
        if (self.ply > 0) && pv_node == false {
            // read hash entry if we're not in a root ply and hash enret is available and current node is not a principal variation node
            if let Some(score) =  self.tt.probe(board.hash_key, depth, alpha, beta, self.ply) {
                return score
            }
        }
        // this action will be performed every 2048 nodes
        if (self.nodes & NODES_2047) == 0 {
            self.controller.as_ref().lock().unwrap().communicate();
        }

        // println!("ply is {}", self.ply);
        if depth == 0 {
            let score = self.quiescence(alpha, beta, board);
            return score;
        }

        if self.ply > MAX_PLY -1 {
            return Evaluation::evaluate(board);
        }

        self.nodes+=1;
        // println!("::::::: {depth}");

        let king_square = u64::from(board[Piece::king(board.turn)].trailing_zeros());
        // is king in check
        let king_in_check = board.is_square_attacked(king_square, !board.turn);
        let depth = if king_in_check {depth +1} else {depth};
        let mut legal_moves = 0;

        // Null-Move Forward Pruning
        let null_move_forward_pruning_conditions = depth >= (DEPTH_REDUCTION_FACTOR + 1) && !king_in_check && self.ply> 0;
        // added 1 to the depth_reduction factor to be sure, there is atleast one more depth that would be checked
        if null_move_forward_pruning_conditions {
            let mut nmfp_board = board.clone();
            self.ply += 1;
            self.repetition_index+=1;
            self.repetition_table[self.repetition_index] = nmfp_board.hash_key;

            // update the zobrist hash accordingly, since this mutating actions do not direcly update the zobrist hash
            if let Some(enpass_sq) = nmfp_board.enpassant {
                // we know that we're going to remove the enpass if it's available (see 4 lines below), so we remove it from the hashkey if it exists here
                nmfp_board.hash_key ^= ZOBRIST.enpassant_keys[enpass_sq];
            }
            nmfp_board.set_turn(!board.turn);
            nmfp_board.set_enpassant(None);
            nmfp_board.hash_key ^= ZOBRIST.side_key;
            let score = -self.negamax(-beta, -beta+1, depth-1-DEPTH_REDUCTION_FACTOR, &nmfp_board);

            self.ply -= 1;
            self.repetition_index-=1;

            // return 0 if time is up
            if self.controller.as_ref().lock().unwrap().stopped() { return 0}

            if score >= beta {
                return beta
            }
        }

        let moves = board.gen_movement().into_iter();
        if self.follow_pv {
            self.enable_pv_scoring(&moves);
        }
        let sorted_moves = self.sort_moves(board, moves);
        // https://www.chessprogramming.org/Principal_Variation_Search#Pseudo_Code
        let mut moves_searched = 0;

        // loop through hte moves
        for mv in sorted_moves {
            let play_moves = board.make_move(mv, MoveType::AllMoves);
            
            if let Some(new_board) = play_moves {
                self.ply +=1;
                self.repetition_index+=1;
                self.repetition_table[self.repetition_index] = new_board.hash_key;
                legal_moves += 1;


                // Null-move forward pruning is a step you perform prior to searching any of the moves.  You ask the question, "If I do nothing here, can the opponent do anything?"


                // https://www.chessprogramming.org/Principal_Variation_Search#Pseudo_Code
                let score = match moves_searched {
                    0 => {
                        // full depth search
                        -self.negamax(-beta, -alpha, depth-1, &new_board)
                    },
                    _ => {
                        // https://web.archive.org/web/20150212051846/http://www.glaurungchess.com/lmr.html
                        // condition for Late Move Reduction
                        let ok_to_reduce = !king_in_check && mv.get_promotion().is_none() && !mv.get_capture();

                        let mut value =  if (moves_searched >= FULL_DEPTH_MOVE) && (depth >= REDUCTION_LIMIT) && ok_to_reduce {
                            -self.negamax(-alpha - 1, -alpha, depth-2, &new_board)
                            // -self.negamax(-(alpha + 1), -alpha, depth-2, &new_board)
                        } else {
                            alpha +1
                        };

                        if value > alpha {
                            value = -self.negamax(-alpha-1, -alpha, depth-1, &new_board);
                            // value = -self.negamax(-(alpha+1), -alpha, depth-1, &new_board);
                            if (value > alpha) && (value < beta) {
                                value = -self.negamax(-beta, -alpha, depth-1, &new_board);
                            }
                        }
                        value
                    }
                };


                
                self.ply -=1;
                self.repetition_index-=1;
                moves_searched += 1;
                // return 0 if time is up
                if self.controller.as_ref().lock().unwrap().stopped() { return 0}


                // fail-hard beta cutoff
                if score >= beta {
                    self.tt.record(board.hash_key, depth, beta, self.ply, HashFlag::LowerBound);
                    // println!("ply @3 is {}", self.ply);
                    if !mv.get_capture() { // quiet move (non-capturing quiet move that beats the opponent)
                        self.killer_moves[1][self.ply] = self.killer_moves[0][self.ply];
                        self.killer_moves[0][self.ply] = *mv;
                    }
                    // node/move fails high
                    return beta
                }
                
                // best score so far
                if score > alpha {


                    hash_flag = HashFlag::Exact;
                    
                    if !mv.get_capture() {
                        // store history moves
                        *self.history_moves[mv.get_piece()].get_mut(mv.get_target() as usize).unwrap() += depth as u32;
                    }
                    alpha = score; // PV move (position)

                    // println!("ply @2 is {}", self.ply);
                    // write PV move
                    self.pv_table[self.ply][self.ply] =  *mv as i32;

                    for next_ply in (self.ply+1)..self.pv_length[self.ply+1] {
                        // copy move from deeper ply into current ply's line
                        self.pv_table[self.ply][next_ply] = self.pv_table[self.ply+1][next_ply];
                    }
                    self.pv_length[self.ply] = self.pv_length[self.ply + 1];
                    

                   
                } 


            }

        }


        // we don't have any legal moves to make in the current position
        if legal_moves == 0 {
            // is king in check
            if king_in_check {
                return -MATE_VALUE + (self.ply) as i32;
            }
            // king is not in check and there are not legal moves
            return 0 // stalemate | draw
        }

        self.tt.record(board.hash_key, depth, alpha, self.ply, hash_flag);
        return alpha
    }








}