use std::{sync::{Arc, Mutex}, time::Instant};

use crate::{bit_move::Move, board::{piece::Piece, position::Position, state::board::Board}, constants::{params::MAX_DEPTH, ALPHA, BETA, DEPTH_REDUCTION_FACTOR, FULL_DEPTH_MOVE, MATE_SCORE, MATE_VALUE, MAX_PLY, NODES_2047, REDUCTION_LIMIT, TOTAL_PIECES, TOTAL_SQUARES, VAL_WINDOW, ZOBRIST}, move_scope::MoveScope, moves::Moves, tt::{flag::HashFlag, tpt::TPT}};
use super::{search_entry::SearchE, time_control::TimeControl};


/// Sometimes you can figure out what kind of node you are dealing with early on. If the first move you search fails high (returns a score greater than or equal to beta).
/// you've vlearly got a beta node. If the first move fails low(returns a score lesser than or equal to alpha), assuming that your move ordering is pretty good, you
/// probably have an alpha mode. If the first move returns a score between alpha and beta, you probably have a PV node.
/// Ofcourse, you could be wrong in two of tyhe case. Once you fail high, you return beta, so you can't make a mistake about that, 
#[derive(Debug)]
pub struct NegaMax<'a, T: TimeControl> {
    nodes: u64,
    ply: usize,
    follow_pv: bool,
    score_pv: bool,
    controller: Arc<Mutex<T>>,
    /// Transposition table
    tt: TPT<'a>,
    repetition_index: usize,
    repetition_table: [u64; 500],

    // / Search Stack
    ss: [SearchE; MAX_DEPTH],
    
    // MOVE ORDERING HEURISTICS
    killer_moves: [[u32; 64]; 2],
    history_moves: [[u32; TOTAL_SQUARES]; TOTAL_PIECES], //[[target_sq; 64]; moving_piece];
    /// The Principal variation (PV) is a sequence of moves that programs consider best and therefore expect to be played. All the nodes included by the PV are PV-nodes
    /// [Principal Variation](https://www.chessprogramming.org/Principal_Variation)
    pv_table: [[Move; MAX_PLY]; MAX_PLY],
    pv_length: [usize; MAX_PLY],
    name: usize
}


impl<'a, T> NegaMax<'a, T> where T: TimeControl {
    pub(crate) fn new(controller: Arc<Mutex<T>>, tt: TPT<'a>, name: usize) -> Self {
        let x = Self {
            killer_moves: [[0; 64]; 2], 
            history_moves: [[0; 64]; 12], 
            pv_length: [0; 64], 
            pv_table: [[Move::default(); 64]; 64], 
            nodes: 0, ply: 0, follow_pv: false, score_pv: false, controller,
            tt,
            repetition_index: 0,
            repetition_table: [0; 500],
            name,
            ss: [SearchE::default(); MAX_DEPTH],
        };

        x
    }

    pub(crate) fn iterative_deepening(&mut self, limit: u8, board: &mut Position) {
        let mut alpha = ALPHA;
        let mut beta = BETA;


        for depth in 1..=(limit) {
            let start_time = Instant::now();
            // return 0 if time is up
            if self.controller.as_ref().lock().unwrap().stopped() { break; }

            self.follow_pv = true;
            // println!("!!!!<<<before>>");
            let score = self.negamax(alpha, beta, depth, board);
            // println!("READY>>");
            if (score <= alpha) || (score >= beta) {
                // println!("potentially bad move :::: {:#?}", score);
                alpha = ALPHA; // We fell outside the window, so try again with a
                beta = BETA; //  full-width window (and the same depth).
                continue;
            }
            
            alpha = score - VAL_WINDOW;
            beta = score + VAL_WINDOW;

            if depth == limit {
                if score > -MATE_VALUE && score < -MATE_SCORE {
                    println!("info score mate {} depth {} nodes {} time {}ms pv", (-(score + MATE_VALUE)/2) -1, depth, self.nodes, start_time.elapsed().as_millis());
                    println!("MATE IN {}", (MATE_VALUE - (score + 1)/2));
                } else if (score > MATE_SCORE) && score < MATE_VALUE {
                    println!("info score mate {} depth {} nodes {} time {}ms pv", ((MATE_VALUE - score)/2) + 1, depth, self.nodes, start_time.elapsed().as_millis());
                    println!("MATED IN {}", (MATE_VALUE + score)/2);
                } else {
                    println!("info score cp->{} depth===>{} nodes {} time {}ms pv", score, depth, self.nodes, start_time.elapsed().as_millis());
                }
    
                for count in 0..self.pv_length[0] as usize {
                    print!("-->>> {}, ", Move::from(self.pv_table[0][count]))
                }
    
                // println!("");
                println!("\n----index {}---------------------- {:#?}ms", self.name, start_time.elapsed().as_millis());
                println!("=======------------------- {:#?}s \n", start_time.elapsed().as_secs());
                // println!("{:?}", self.pv_table);
            }
            

        }

    }
    
    // This method is currently VERY SLOW once the depth starts approaching 8, please work to improve it
    pub(crate) fn run(controller: Arc<Mutex<T>>, tt: TPT<'a>, depth: u8, board: &mut Position, name: usize) {
        let mut negamax = Self::new(controller, tt, name);
        negamax.iterative_deepening(depth, board);
    }

    
    fn enable_pv_scoring(&mut self, moves: &Moves) {
        // disable following pv
        self.follow_pv = false;

        for mv in moves.into_iter() {
            // if this move is the best move at that specific ply(self.ply), then enable `score_pv`, and `follow_pv`
            if self.pv_table[0][self.ply] == mv {
                self.score_pv = true;
                self.follow_pv = true;
            }
        }
    }


    /// mv: Move (please remove the mut later, and find a abtter way to write this)
    pub(crate) fn score_move(&mut self, board: &Board, mv: Move) -> u32 {
        if self.score_pv {
            if self.pv_table[0][self.ply] == mv {
                self.score_pv = false;
                return 20_000;
            }
        }
        let Some(piece) = board.piece_at(mv.get_src()) else {return 0};
        if let Some(victim) = board.get_move_capture(mv, !board.turn) {
            let attacker = piece;
            let score = attacker.get_mvv_lva(&victim)  + 10_000;
            return score;
        } else {
            if let Some(kill_move) = self.killer_moves[0].get(self.ply) {
                if *kill_move == (*mv).into() {
                    return 9_000
                }
            }
            
            // score  2nd killer move
            if let Some(kill_move) = self.killer_moves[1].get(self.ply) {
                 if *kill_move == (*mv).into() {
                     return 8_000
                 }
            }
            
            // score history move
            return self.history_moves[piece][mv.get_target()];

        }

        // return 0
    }

    /// todo! add target on the Move, so that this cmp method can be implenented directly on Moves(MvList), that way
    /// we wouldn't need this one anymore
    pub(crate) fn sort_moves(&mut self, board: &Board, mv_list: Moves) -> Vec<Move> {
        let mut sorted_moves: Vec<Move> = Vec::with_capacity(mv_list.count_mvs());
        // println!("the count is {}", mv_list.count_mvs());
        sorted_moves.extend_from_slice(&mv_list.list[..mv_list.count_mvs()]);
        sorted_moves.sort_by(|a, b| self.score_move(board, *b).cmp(&self.score_move(board, *a)));
        return sorted_moves
    }




  /// https://www.chessprogramming.org/Quiescence_Search
    fn quiescence(&mut self, mut alpha: i32, beta: i32, mut board: &mut Position, d: usize) -> i32 {
        // this action will be performed every 2048 nodes
        if (self.nodes & NODES_2047) == 0 { self.controller.as_ref().lock().unwrap().communicate(); }
        self.nodes+=1;

        if self.ply > MAX_PLY - 1 {
            return board.evaluate();
        }
        
        if self.ply > 0 && (self.is_repetition(board) || board.fifty.iter().any(|&p| p >= 50)) {
            return 0 // draw
        }

        // let king_square = u64::from(board[Piece::king(board.turn)].trailing_zeros());
        // let king_in_check = board.is_square_attacked(king_square, !board.turn);

        let mut best_move: Option<Move> = None;

        if let Some(data) =  self.tt.probe(board.hash_key) {
            let score = data.score(self.ply);

            match data.flag {
                HashFlag::Exact => return score,
                HashFlag::LowerBound if score >= beta => return beta,
                HashFlag::UpperBound if score <= alpha => return alpha, 
                _ => best_move = data.mv,
            }
        }


        let evaluation = board.evaluate();
        if evaluation >= beta { return beta; } // node (move) fails high
        if evaluation > alpha { alpha = evaluation; } // found a better score

        let sorted_moves = self.sort_moves(board, board.gen_movement().into_iter());
        for mv in sorted_moves {
            if mv.get_capture() {
                if board.make_move_nnue(mv, MoveScope::CapturesOnly) {
                    self.ply += 1;
                    self.repetition_index += 1;
                    self.repetition_table[self.repetition_index] = board.hash_key;
                    
                    
                    let score = -self.quiescence(-beta, -alpha, &mut board, d);
                    self.ply -=1;
                    self.repetition_index-=1;
                    board.undo_move(true);
        
                    // return 0 if time is up
                    if self.controller.as_ref().lock().unwrap().stopped() { return 0}
                    
                    if score >= beta { return beta }
                    if score > alpha { alpha = score; }
                }

            }
        }

        return alpha
    }

    fn is_repetition(&self, board: &Board) -> bool {
        for i in 0..self.repetition_index {
            if self.repetition_table[i] == board.hash_key {
                return true
            }
        }
        return false;
    }

    /// nmfp: Null Move forward prunning
    /// https://web.archive.org/web/20040427014629/http://brucemo.com/compchess/programming/nullmove.htm
    /// "If I do nothing here, can the opponent do anything?"
    /// Returns the score, only if the score is greater than beta.
    /// This means that even if we "skip" our play, and allow the opponent to play (instead of us),
    /// They still won't be better off than they were before we skipped our play
    fn make_null_move(&mut self, mut alpha: i32, beta: i32, depth: u8, board: &Position) -> Option<i32> {
            // nmfp: null-move forward prunning (board)
            let mut nmfp_board = Position::with((**board).clone());
            self.ply += 1;
            self.repetition_index+=1;
            self.repetition_table[self.repetition_index] = nmfp_board.hash_key;

            // update the zobrist hash accordingly, since this mutating actions do not direcly update the zobrist hash
            if let Some(enpass_sq) = nmfp_board.enpassant {
                // we know that we're going to remove the enpass if it's available (see 4 lines below), so we remove it from the hashkey if it exists here
                nmfp_board.set_zobrist(nmfp_board.hash_key ^ ZOBRIST.enpassant_keys[enpass_sq]);
            }
            nmfp_board.set_turn(!board.turn);
            nmfp_board.set_enpassant(None);
            nmfp_board.set_zobrist(nmfp_board.hash_key ^ ZOBRIST.side_key);
            nmfp_board.nnue_push();
            
            let score = -self.negamax(-beta, -beta+1, depth-1-DEPTH_REDUCTION_FACTOR, &mut nmfp_board);

            self.ply -= 1;
            self.repetition_index-=1;
            nmfp_board.nnue_pop();
            // return 0 if time is up
            if self.controller.as_ref().lock().unwrap().stopped() { return None}

            if score >= beta {
                return Some(beta)
            }
            
            return None;
    }


    /// Detect Extension conditions(Situations that warrant a deeper search): check, captures, promotion, best move whose evaluation is greater than every other move 
    fn search_extension() {}


    
    /// https://www.chessprogramming.org/Alpha-Beta#Negamax_Framework
    fn negamax(&mut self, mut alpha: i32, beta: i32, depth: u8, mut board: &mut Position) -> i32 {
        self.pv_length[self.ply] = self.ply;
        
        let mut hash_flag = HashFlag::UpperBound; // alpha
        if self.ply > 0 && (self.is_repetition(board) || board.fifty.iter().any(|&p| p >= 50)) {
            return 0 // draw
        }

        let mut singularity_search = false; 

        let pv_node = (beta - alpha) > 1;
        // if we had cached the score for this move before, we return it, and confirm that the current node is not a PV node(principal variation)
        if (self.ply > 0) && pv_node == false {
            // read hash entry if we're not in a root ply and hash entry is available, current node is not a principal variation node
            if let Some(entry) =  self.tt.probe(board.hash_key) {
                if !singularity_search {
                    if entry.depth >= depth {
                        let entry_value = entry.score(self.ply);
                        match entry.flag {
                            HashFlag::Exact => return entry_value,
                            HashFlag::LowerBound if entry_value >= beta => return beta,
                            HashFlag::UpperBound if entry_value <= alpha =>  return alpha,
                            _ => {}
                        }
                    }
                }
            }
        }

        
        // this action will be performed every 2048 nodes
        if (self.nodes & NODES_2047) == 0 {
            self.controller.as_ref().lock().unwrap().communicate();
        }
        // println!("ply is {}", self.ply);
        if depth == 0 || self.ply >= MAX_DEPTH {
            return self.quiescence(alpha, beta, board, depth as usize);
        }

        
        if self.ply > MAX_PLY -1 {
            let evaluation = board.evaluate();
            // let evaluation = Evaluation::evaluate(board);
            return evaluation;
        }

        self.nodes+=1;
        // println!("::::::: {depth}");
        
        let king_square = u64::from(board[Piece::king(board.turn)].trailing_zeros());
        // is king in check
        let king_in_check = board.is_square_attacked(king_square, !board.turn);
        let depth = if king_in_check {depth +1} else {depth};
        let mut legal_moves = 0;
        
        let static_eval = board.evaluate();
        
        // Null-Move Forward Pruning
        // Null-move forward pruning is a step you perform prior to searching any of the moves.  You ask the question, "If I do nothing here, can the opponent do anything?"
        // In order to test this, we allow the opponent play this turn(even though its ours), if they play and we're not in harms way (greater than beat), then we're good.
        // -- "Null-move forward pruning is not used, at least in endgames.  If you do try to use it in endgames, very bad things will happen very often."
        let null_move_forward_pruning_conditions = depth >= (DEPTH_REDUCTION_FACTOR + 1) && !king_in_check && self.ply> 0;
        // added 1 to the depth_reduction factor to be sure, there is atleast one more depth that would be checked
        
        if null_move_forward_pruning_conditions {
            if let Some(beta) = self.make_null_move(alpha, beta, depth, board) {
                return beta;
            };
        }



        
        // [Strelka's Razoring](https://www.chessprogramming.org/Razoring)
        if !pv_node && !king_in_check && depth <= 3 {
            let mut value = static_eval + 125;

            if value < beta {
                if depth == 1 {
                    let new_value = self.quiescence(alpha, beta, board, depth as usize);
                    return i32::max(value, new_value);
                }
                value += 175;
                if value < beta && depth <= 32 {
                    let new_value = self.quiescence(alpha, beta, board, depth as usize);
                    if new_value < beta {
                        return i32::max(new_value, value);
                    }
                }
            }
        }

        
        
        let moves = board.gen_movement().into_iter();
        if self.follow_pv {
            self.enable_pv_scoring(&moves);
        }
        
        // for mv in moves 
        let sorted_moves = self.sort_moves(board, moves);
        
        // https://www.chessprogramming.org/Principal_Variation_Search#Pseudo_Code
        let mut moves_searched = 0;
        
        // loop through hte moves
        for mv in sorted_moves {
            let legal_move = board.make_move_nnue(mv, MoveScope::AllMoves);
            
        

            // let Some(new_board) = play_moves else {continue};
            if !legal_move { continue; }
            
            self.ply +=1;
            self.repetition_index+=1;
            self.repetition_table[self.repetition_index] = board.hash_key;
            legal_moves += 1;


            // https://www.chessprogramming.org/Principal_Variation_Search#Pseudo_Code
            let score = match moves_searched {
                0 => {
                    // full depth search
                    -self.negamax(-beta, -alpha, depth-1, &mut board)
                },
                _ => {
                    // https://web.archive.org/web/20150212051846/http://www.glaurungchess.com/lmr.html
                    // condition for Late Move Reduction
                    let ok_to_reduce = !king_in_check && mv.get_promotion().is_none() && !mv.get_capture();

                    let mut value =  if (moves_searched >= FULL_DEPTH_MOVE) && (depth >= REDUCTION_LIMIT) && ok_to_reduce {
                        -self.negamax(-alpha-1, -alpha, depth-2, &mut board)
                    } else {
                        alpha +1
                    };

                    if value > alpha {
                        value = -self.negamax(-alpha-1, -alpha, depth-1, &mut board);
                        if (value > alpha) && (value < beta) {
                            value = -self.negamax(-beta, -alpha, depth-1, &mut board);
                        }
                    }
                    value
                }
            };

            board.undo_move(true);


            
            self.ply -=1;
            self.repetition_index-=1;
            moves_searched += 1;
            // return 0 if time is up
            if self.controller.as_ref().lock().unwrap().stopped() { return 0}

            
            
            // fail-hard beta cutoff
            if score >= beta {
                // if mv.to_string() == String::from("e2a6x") {
                //     println!("<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<< score={score:10} alpha={alpha:10}, and beta={beta:10} >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
                // }
                self.tt.record(board.hash_key, depth, beta, self.ply, HashFlag::LowerBound, 0, Some(mv));
                // println!("ply @3 is {}", self.ply);
                if !mv.get_capture() { // quiet move (non-capturing quiet move that beats the opponent)
                    self.killer_moves[1][self.ply] = self.killer_moves[0][self.ply];
                    self.killer_moves[0][self.ply] = (*mv).into();
                }
                // node/move fails high
                return beta
            }
            
            // best score so far
            if score > alpha {
                hash_flag = HashFlag::Exact;
                
                if !mv.get_capture() {
                    // store history moves
                    self.history_moves[board.piece_at(mv.get_src()).unwrap()][mv.get_target() as usize] += depth as u32;
                    // *self.history_moves[mv.get_piece()].get_mut(mv.get_target() as usize).unwrap() += depth as u32;
                }
                alpha = score; // PV move (position)

                // println!("ply @2 is {}", self.ply);
                // write PV move
                // Traingular PV-Table
                self.pv_table[self.ply][self.ply] =  mv;

               for j in (self.ply+1)..self.pv_length[self.ply+1] {
                    // copy move from deeper ply into current ply's line
                    self.pv_table[self.ply][j] = self.pv_table[self.ply+1][j];
                }
                self.pv_length[self.ply] = self.pv_length[self.ply + 1];

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

        self.tt.record(board.hash_key, depth, alpha, self.ply, hash_flag, 0, None);
        return alpha
    }
}