use crate::{board::{piece::{Piece, PieceType}, position::Position, state::board::Board}, color::Color, constants::{params::MAX_DEPTH, DEPTH_REDUCTION_FACTOR, FULL_DEPTH_MOVE, INFINITY, MATE_VALUE, PIECE_ATTACKS, REDUCTION_LIMIT, VAL_WINDOW, ZOBRIST}, move_logic::{bitmove::{Move, MoveType}, move_picker::MovePicker, move_stack::MoveStack}, move_scope::MoveScope, search::constants::Root, squares::Square, tt::{entry::TTData, flag::HashFlag, tpt::TPT}};
use crate::board::piece::Piece::*;
use crate::color::Color::*;
use crate::search::heuristics::pv_table::PVTable;

use super::{constants::{NodeType, NotPv}, heuristics::{capture_history::CaptureHistory, continuation_history::ContinuationHistory, countermove::CounterMove, history::HistoryHeuristic, history_bonus, killer_moves::KillerMoves}, stack::{Stack, StackItem}};

/// The number of nodes you can actually cut depends on:
/// 1. How well written your alpha-beta program is
/// 2. How well ordered your game-tree is (i.e the next moves on the board) --- GOOD MOVE ORDERING IS IMPORTANT
///     If you always put the best possible move first, you elimiate the most nodes.
/// 
/// GOALS:
///  1. [x] AlphaBeta 
///  2. [-] Quiescence Search
///         a. [x] Standing Pat
///         b. [-] Delta Pruning
///             i. https://www.chessprogramming.org/Delta_Pruning
///             ii. https://www.chessprogramming.org/CPW-Engine_quiescence (check how CPW implements its beta pruning)
///  3. [x] Late Move Reduction
///  4. [x] MVV_LVA - (Most Viable Victim -- Least Viable Attacker)
///  5. [x] Move Ordering
///  6. [x] Transposition Table
///  7. [x] Null move forward Prunning
///  8. [x] Principal Variation Node
///  9. [x] Killer Moves
/// 10. [x] History Moves 
/// 11. [x] Aspiration Window
/// 12. [x] Iterative Deepening
/// 13. [x] PV-Table
/// 14. [x] Repetitions https://www.chessprogramming.org/Repetitions
/// 15. [-] Singular extensions
/// This implementation is a fail-soft implementation (meaning we have to keep track of the best score)[XX] 
/// - fail-hard for now
pub(crate) struct Search<'a> {
    nodes:  usize,
    ply: usize,
    pv_table: PVTable,
    /// The Killer Move is a quiet move which caused a beta-cutoff in a sibling Cut-node,
    killer_moves: KillerMoves,
    /// Previosyly successful moves in a particular positioin that resulted in a beta-cutoff
    history_table: HistoryHeuristic,
    caphist: CaptureHistory,
    conthist: ContinuationHistory,
    counter_mvs: CounterMove,
    // continuation_hist
    tt: TPT<'a>,
    ss: Stack,
    depth: usize,
    limit: usize,
    eval: i32,
}


/// Defines a margin to decide how "bad" a position must be to be considered for razoring. The margin can depend on the search depth and should be empirically tuned.
/// For instance, at depth 1, the margin might be a small value (like half a pawn), whereas at depth 2, you might use a larger margin.
/// Should still be further tuned
const RAZORING_MARGIN: [i32; 3] = [0, 293, 512];


impl<'a> Search<'a> {
    pub(crate) fn new(tt: TPT<'a>) -> Self {
        Self { nodes: 0, ply: 0, pv_table: PVTable::new(), killer_moves: KillerMoves::new(),
            history_table: HistoryHeuristic::new(), tt, caphist: CaptureHistory::default(), conthist: ContinuationHistory::new(),
                counter_mvs: CounterMove::new(), ss: Stack::default(), depth: 0, limit: 0, eval: 0 }
    }

    fn aspiration_window(&mut self, position: &mut Position, depth: usize) -> i32 {
        let mut alpha = -INFINITY;
        let mut beta = INFINITY;
        let mut delta = -INFINITY;
        let mut depth = depth + 1;

        const BIG_DELTA: usize = 975;

        println!("||||||||||||||||||||||||||||||||||||||||||||||||| alpha-> {alpha}, and beta-->> {beta} ---------------- ((((({}))))))", depth);
        println!("XXXXXXXXXXXXXXX eval => {}, delta==> {}, for al-->> {}, and for beta-->> {}", self.eval, delta, self.eval - delta, self.eval + delta);

        if depth >= 5 {
            delta = 20;
            alpha = (-INFINITY).max(self.eval - delta);
            beta = (INFINITY).min(self.eval + delta);
        }

        println!("proceeding with alpha-> {alpha}, and beta-->> {beta}");

        loop {
            self.ply = 0;
            let score = self.negamax::<Root>(alpha, beta, depth  as u8, position);
            // println!("the score is {score}, alpha={alpha}, beta={beta} and nodes {}", self.nodes);
            
            // if depth > self.limit { break; }
            // self.depth = depth;
            // println!("{}", position.to_string());
            depth += 1;

            if score <= alpha {
                beta = (alpha + beta) / 2;
                alpha = (-INFINITY).max(alpha - beta);
                depth = self.depth + 1;
                println!("00000");
            } else if score >= beta {
                println!("currently alpha={alpha}, beta={beta}, and score==>>{score}");
                beta = (INFINITY).min(beta + delta);
                println!("1111-1111 {}", beta);
                // if score.abs() < LONGEST_TB_MATE && depth > 1 {
                //     depth -= 1;
                // }
            } else {
                println!("4444---->>>4444");
                return score;
            }
            delta += delta/2;
            if delta >= BIG_DELTA as i32 { alpha = -INFINITY; beta = INFINITY } 

            // let mvs = self.pv_table.get_pv(0);
            // println!("dddd is {depth}");
            // for i in 0..mvs.len() {
            //     print!("-->> {}", Move::from(mvs[i as usize]));
            // }
            // println!("\n\n");

        }

        // println!("MOVES ARE :::: with length of {}", self.pv_table.len(0));
        // let mvs = self.pv_table.get_pv(0);
        // for i in 0..self.limit {
        //     print!("-->> {}", Move::from(mvs[i as usize]));
        //     // println!("in check???? {}", position);
        // }
        // println!("\n");
    }

    pub(crate) fn iterative_deepening(&mut self, limit: usize, position: &mut Position) {
        self.limit = limit;
        while self.depth < MAX_DEPTH && self.depth < self.limit {
            // println!("\n\n\n RUNNING ::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::<<>>::::::::::: {}", self.depth);
            let eval = self.aspiration_window(position, self.depth);
            
            self.eval = eval;
            self.depth += 1;
        }


        println!("MOVES ARE :::: with length of {}", self.pv_table.len(0));
        let mvs = self.pv_table.get_pv(0);
        for i in 0..limit {
            print!("|||-->> {}", Move::from(mvs[i as usize]));
            // println!("in check???? {}", position);
        }
        println!("\n");
    }


    #[inline]
    fn futility_margin(depth : u8, improving: bool) -> i32 {
        (depth as i32) * (175 - 50 * improving as i32)
    }

    fn is_repetition(position: &Position, key: u64) -> bool {
        let len = position.history_len();
        if len == 0 { return false }

        // subtracting 1 from len because we don't care about the opponent's (the person who played last's) game
        // stepping by 2 because we don't care about the opponent's key positional history in this case
        for index in (0..len-1).rev().step_by(2) {
            if let Some(history) = position.history_at(index) { 
                if history.hash() == key { return true }
             } else { continue }
        } 

        false
    }


    // In addition, we a score to return in case there are no captures available to be played. -->> static evaluation
    /// At the beginning of quiescence, the position's evaluation is used to establish a lower-bound on the score.
    /// If the lower bound from the stand pat(static evaluation) is always greater than or equal to beta, we can return the stand-pat(fail-soft)
    /// or beta(fail-hard) as a lower bound. Otherwise, the search continues
    /// https://www.chessprogramming.org/Quiescence_Search
    fn quiescence(&mut self, mut alpha: i32, beta: i32, position: &mut Position) -> i32 {
        self.nodes+=1;
        
        let stand_pat = position.evaluate();
        if self.ply >= MAX_DEPTH { return stand_pat }
        // check if it's a draw
        if self.ply > 0 && (Self::is_repetition(&position, position.hash_key) || position.fifty.iter().any(|&p| p >= 50)) {
            return 0 // draw
        }

        
        // Probe the Transposition Table here
        // let tt_hit = self.probe_tt(position.hash_key, None, alpha, beta);
        // if tt_score.is_some() { return tt_score.unwrap() }

        // conditions
        // 1. is king in check: If the stm is in check, the position is not quiet, and there is a threat that needs to be resolved. In that case, 
        // all ways to evade the check are searched. Stand pat is not allowed if we are in check.
        // So, if the king of the stm is in check - WE MUST SEARCH EVERY MOVE IN THE POSITION, RATHER THAN ONLY CAPTURES.
        // - LIMIT THE GENERATION OF CHECKS TO THE FIRST X PLIES OF QUIESCENCE (AND USE "DELTA PRUNNING" TO AVOID LONG FRUITLESS SEARCHES TO GET OUT OF BEEN IN CHECK)
        let in_check = Self::in_check(&position, position.turn);
        let mut tt_move: Option<Move> = None;

        if let Some(entry) = self.tt.probe(position.hash_key) {
            let tt_value = entry.score;
            match entry.flag {
                HashFlag::Exact => return tt_value,
                HashFlag::LowerBound if tt_value >= beta => return beta,
                HashFlag::UpperBound if tt_value <= alpha => return alpha,
                _ => tt_move = entry.mv,
            }
        }
        
        let eval = if in_check {
            self.ss[self.ply].eval = -INFINITY;
            self.ss[self.ply].eval
        } else {
            if let Some(entry) = self.tt.probe(position.hash_key) {
                let tt_eval = entry.eval as i32; let tt_value = entry.score;

                self.ss[self.ply].eval = if tt_eval == -INFINITY {
                    position.evaluate() } else { tt_eval };
                match entry.flag {
                    HashFlag::Exact => return tt_value,
                    HashFlag::LowerBound if tt_value > self.ss[self.ply].eval => return tt_value,
                    HashFlag::UpperBound if tt_value < self.ss[self.ply].eval => return tt_value,
                    _ => self.ss[self.ply].eval,
                }
            } else {
                self.ss[self.ply].eval = position.evaluate();
                self.ss[self.ply].eval
            }
        };

        let old_alpha = alpha;
        alpha = alpha.max(eval);
        // standing pat
        if eval >= beta { return beta }
    
        // let captures_only = !in_check;
        // let moves = self.get_sorted_moves::<{MoveScope::CAPTURES}>(&position);
        // let mut best_score = -INFINITY;
        let mut best_move: Option<Move> = None;
        let best_value = eval;

        let killer_mvs = self.killer_moves.get_killers(self.ply).map(|m| if m == 0 {None} else {Some(Move::from(m))});

        if self.depth == 5 {
        }
        let mut mvs = MovePicker::<{MoveScope::CAPTURES}>::new(0, tt_move, killer_mvs);
        while let Some(mv) = mvs.next(&position, &self.history_table, &self.caphist, &self.conthist, &self.counter_mvs) {
            if  mv.to_string() == "d5c6x" {
                println!("quiescence--quiescence--quiescence--quiescence--quiescence--quiescence--quiescence--");
                // let victim = position.get_piece_at(mv.get_tgt(), !position.turn);
                // if victim.is_none() {
                //     if let Some(x) = position.last_history() {
                //         println!("{} -->>\n the board here is {}", mv.to_string(), position.to_string());
                //         println!("<<<<<<<<<THE PREVIOUS MOVE IS>>>>>>>>> {} ---<<<<<{}", x.mv(), x.mv().get_double_push());
                //     }
                //     // println!("NEGAMAX--NEGAMAX--NEGAMAX--, but tt is {:?}", tt_hit.and_then(|x| x.mv));
                // }
            }
            if position.make_move_nnue(mv, MoveScope::AllMoves) {
                self.ply += 1;
                let value = -self.quiescence(-beta, -alpha, position);
                position.undo_move(true);
                
                self.ply -= 1;

                if value > best_value {
                    if value > alpha {
                        best_move = Some(mv);
                        alpha = value;
                    }

                    if value >= beta {
                        alpha = beta;break;
                    }
                }
                

                // if position.turn == Color::Black {
                //     println!(":::::::::::::::::::::::::::::::::::::::::::: mm-->>{} alpha={alpha}, beta={beta}, score={score} ply==>>{} eval-->>{}", mv.to_string(), self.ply, position.evaluate());
                // }
                // if score > best_score {
                //     best_score = score;
                    
                //     if score > alpha { best_move = Some(mv); alpha = score; }
                //     if score >= beta {alpha = beta; break}
                // }
            }
        }

        // for mv in moves.into_iter() {
        //     // if captures_only && !mv.get_capture() { continue; }

        //     if position.make_move_nnue(mv, MoveScope::AllMoves) {
        //         self.ply += 1;
        //         let score = -self.quiescence(-beta, -alpha, position);
        //         position.undo_move(true);
                
        //         self.ply -= 1;
                

        //         // if position.turn == Color::Black {
        //         //     println!(":::::::::::::::::::::::::::::::::::::::::::: mm-->>{} alpha={alpha}, beta={beta}, score={score} ply==>>{} eval-->>{}", mv.to_string(), self.ply, position.evaluate());
        //         // }
        //         if score > best_score {
        //             best_score = score;
                    
        //             if score > alpha { best_move = Some(mv); alpha = score; }
        //             if score >= beta {alpha = beta; break}
        //         }
        //     }


        // }

        if in_check && best_value == -INFINITY {
            return  - 5000;
        }

        let tt_flag = if best_value >= beta { HashFlag::LowerBound } else if best_value > alpha { HashFlag::Exact } else { HashFlag::UpperBound };
        self.tt.record(position.hash_key, 0, alpha, self.ss[self.ply].eval, self.ply, tt_flag, 0, best_move);

        
        alpha
    }

    fn in_check(position: &Position, color: Color) -> bool {
        let king_square = u64::from(position[Piece::king(color)].trailing_zeros());
        position.is_square_attacked(king_square, !color)
    }

    fn probe_tt(&self, key: u64, depth: Option<u8>, alpha: i32, beta: i32) -> Option<TTData> {
        if let Some(entry) = self.tt.probe(key) {
            if depth.is_some_and(|d| entry.depth < d) { return None };
            let entry_score = entry.score;
            let value = match entry.flag {
                HashFlag::Exact => Some(entry),
                HashFlag::LowerBound if entry_score >= beta => Some(entry),
                HashFlag::UpperBound if entry_score < alpha => Some(entry),
                _ => None
            };
            return value;
        }
        None
    }

    /// nmfp: Null Move forward prunning
    /// https://web.archive.org/web/20040427014629/http://brucemo.com/compchess/programming/nullmove.htm
    /// "If I do nothing here, can the opponent do anything?"
    fn make_null_move(&mut self, beta: i32, depth: u8, mut position: &mut Position) -> Option<i32> {
        self.ply += 1;
        
        let old_hashkey = position.hash_key;
        let old_enpassant = position.enpassant;

        if let Some(enpass_sq) = position.enpassant {
                // we know that we're going to remove the enpass if it's available (see 4 lines below), so we remove it from the hashkey if it exists here
            position.set_zobrist(position.hash_key ^ ZOBRIST.enpassant_keys[enpass_sq]);
        }

        position.set_turn(!position.turn);
        position.set_enpassant(None);
        position.set_zobrist(position.hash_key ^ ZOBRIST.side_key); // side about to move
        position.nnue_push();

        let score = -self.negamax::<NotPv>(-beta, -beta+1, depth-1-DEPTH_REDUCTION_FACTOR, &mut position);

        // reverse all actions, after we're done
        self.ply -= 1;
        position.nnue_pop();

        // recent change for undo move in order to avoid cloning the board
        position.set_turn(!position.turn);
        position.set_enpassant(old_enpassant);
        position.set_zobrist(old_hashkey);

        if score > beta {
            return Some(beta);
        }

        None
    }

    pub(crate) fn negamax<NT: NodeType>(&mut self, mut alpha: i32, beta: i32, depth: u8, mut position: &mut Position) -> i32 {
        if Self::is_repetition(&position, position.hash_key) || position.fifty.iter().any(|&s| s >= 50) {
            return 0; // draw
        }
        
        let stm_in_check = Self::in_check(position, position.turn);
        let depth = if stm_in_check && depth < MAX_DEPTH as u8 { depth + 1 } else { depth };
        
        if depth == 0 || self.ply >= MAX_DEPTH {
            return self.quiescence(alpha, beta, position);
        }

        let improving = if self.ply >= 2 && self.ss[self.ply - 2].eval != -INFINITY {
            self.ss[self.ply].eval > self.ss[self.ply - 2].eval
        } else if self.ply >= 4 && self.ss[self.ply - 4].eval != -INFINITY {
            self.ss[self.ply].eval > self.ss[self.ply - 4].eval
        } else {
            true
        };


        let tt_hit = self.probe_tt(position.hash_key, Some(self.ply as u8), alpha, beta);
        // let (tt_score, tt_mv) = self.probe_tt(position.hash_key, Some(self.ply as u8), alpha, beta);
        
        if let Some(tt_data) = tt_hit {
            if let Some(mv) = tt_data.mv {
                let quiet = mv.get_capture() && mv.get_promotion().is_none();
                if quiet {
                    self.killer_moves.store(depth as usize, &mv);
                    self.conthist.update_many(&position, vec![mv], depth, mv);
                    self.counter_mvs.add(&position, mv);
                } else {
                    self.conthist.update_many(&position, vec![mv], depth, Move::from(0));
                }
            }

            return tt_data.score;
        }

        if !NT::ROOT && depth > 7 {}
        

        // Pausing razoring for now
        if !NT::ROOT && !NT::PV && !stm_in_check {
            // let eval = position.evaluate();
            // // currently uses 'Stockfishs\'' value, this need to be fine-tuned more later 
            // if eval < (alpha - 490 - 290 * depth as i32 * depth as i32) {
            //     let value = self.quiescence(alpha-1, alpha, position);
            //     // println!("the value here is {value}, and the alpha here is {alpha}");
            //     if value < alpha {
            //         return eval;
            //     }

            // }
        //     let eval = position.evaluate();
        //     // currently uses 'Stockfishs\'' value, this need to be fine-tuned more later 
        //     // if eval < (alpha - 392 - 267 * depth as i32 * depth as i32) {
                
        //     // }
        //     if depth < 3 && eval <= alpha - RAZORING_MARGIN[depth as usize] {
        //         let r_alpha = alpha - (depth >= 2) as i32 * RAZORING_MARGIN[depth as usize];
        //         let value = self.quiescence(alpha-1, alpha, position);
        //         // println!("the value here is {value}, and the alpha here is {alpha}");
        //         if depth < 2 || value < r_alpha { return eval; }
        //     }
        }
        
        // When beta - alpha > 1, it indicates that there is a significant gap between the two bounds. This gap suggests that there are possible values for the evaluation score that have not yet been fully explored or are still uncertain.
        // The search can continue to explore more moves because the values returned by the evaluated moves could potentially fall within this range, providing room for a better evaluation.
        // let explore_more_moves = (beta - alpha) > 1;
        // if self.ply > 0 && tt_value.is_some() && !explore_more_moves { return tt_value.unwrap() }
        // if self.ply > MAX_PLY - 1 { return position.evaluate() }

        self.nodes += 1;

        let mut mvs_searched = 0;

        let null_move_forward_pruning_conditions = depth >= (DEPTH_REDUCTION_FACTOR + 1) && !stm_in_check && self.ply > 0;

        if null_move_forward_pruning_conditions {
            if let Some(beta) = self.make_null_move(beta, depth, position) {
                return beta;
            }
        }

        let mut best_score = -INFINITY;
        let mut best_mv: Option<Move> = None;
        let original_alpha = alpha;
        let killer_mvs = self.killer_moves.get_killers(self.ply).map(|m| if m == 0 {None} else {Some(Move::from(m))});
        // let mvs = self.get_sorted_moves::<{ MoveScope::ALL }>(&position);
        let mut mvs = MovePicker::<{MoveScope::ALL}>::new(0, tt_hit.and_then(|tt| tt.mv), killer_mvs);
        // println!("the total moves generated are >>>>>>{}, so that the ones after is now {}", mvs);
        // println!("tt => {:?}, and pv={:?}", tt_mv, self.pv_table.get_pv(self.ply).get(0));

        // 30 is a practical maximum number of quiet moves that can be generated in a chess position (MidGame)
        let mut quiet_mvs: Vec<Move> = Vec::with_capacity(30);
        if self.depth == 5 {
        }
        while let Some(mv) = mvs.next(&position, &self.history_table, &self.caphist, &self.conthist, &self.counter_mvs) {
            // if mv.to_string() == "b4a3x" && mv.get_enpassant() && position.enpassant.is_none() {
                if  mv.to_string() == "d5c6x" {
                println!("negamax-->>negamax-->>negamax_->>negamax-->>negamax-->>negamax-->>negamax-->> {} {}", mv.to_string(), self.depth);
                // println!("\n\n");
                // let victim = position.get_piece_at(mv.get_tgt(), !position.turn);
                // if victim.is_none() {
                //     if let Some(x) = position.last_history() {
                //         println!("THE PREVIOUS MOVE IS {} ---<<<<<{}", x.mv(), x.mv().get_double_push());
                //     }
                //     println!("NEGAMAX--NEGAMAX--NEGAMAX--, but tt is {:?}", tt_hit.and_then(|x| x.mv));
                // }
            }
            if position.make_move_nnue(mv, MoveScope::AllMoves) {
                self.ply += 1;

                let score = match mvs_searched {
                    0 => -self.negamax::<NotPv>(-beta, -alpha, depth -1, &mut position),
                    _ => {
                        // https://web.archive.org/web/20150212051846/http://www.glaurungchess.com/lmr.html
                        // condition for Late Move Reduction
                        let non_tatcital_mv = !stm_in_check && mv.get_promotion().is_none() && !mv.get_capture();

                        let mut value = if (mvs_searched as u8 >= FULL_DEPTH_MOVE) && (depth >= REDUCTION_LIMIT) && non_tatcital_mv {
                            -self.negamax::<NotPv>(-(alpha + 1), -alpha, depth-2, position)
                        } else {
                            alpha + 1 // Hack to ensure that full-depth search is done
                        };

                        if value > alpha {
                            value = -self.negamax::<NotPv>(-(alpha - 1), -alpha, depth-1, position);

                            if (value > alpha) && value < beta {
                                value = -self.negamax::<NotPv>(-beta, -alpha, depth-1, position);
                            }
                        }

                        value
                    }
                };
                
                mvs_searched += 1;
                
                let zobrist_key = position.hash_key;
                position.undo_move(true);
                self.ply -= 1;
                let moved_piece = position.piece_at(mv.get_src()).unwrap();

                // if position.turn == Color::Black && i < 4 {
                //     println!("THE MOVE: {} at depth -->> {depth} ----- the score {score}, the alpha==>> {alpha}, and the beta==>>{beta} \n\n\n", mv.to_string());
                // }
                
                if score >= beta {
                    self.tt.record(zobrist_key, depth, beta, INFINITY, self.ply, HashFlag::LowerBound, 0, best_mv); 
                    if mv.get_capture() {
                        self.caphist.update(depth, HashFlag::LowerBound, moved_piece, mv.get_target(), PieceType::from(position.piece_at(mv.get_target()).unwrap()));
                    } else {
                        self.killer_moves.store(depth as usize, &mv);
                        self.conthist.update_many(&position, quiet_mvs, depth, mv);
                        self.counter_mvs.add(&position, mv);
                    }
                    return beta;
                }

                if score > alpha {
                    best_score = score;
                    
                    best_mv = Some(mv);
                    // hash_flag = HashFlag::Exact;
                    self.pv_table.store_pv(self.ply, &mv);
                    alpha = score;

                    if mv.get_capture() {
                        self.caphist.update(depth, HashFlag::UpperBound, moved_piece, mv.get_target(), PieceType::from(position.piece_at(mv.get_target()).unwrap()));
                    } else {
                        self.history_table.update(moved_piece, mv.get_src(), depth);
                        quiet_mvs.push(mv);
                    }
                }

                // if score > best_score {
                //     best_score = score;

                //     if score > alpha {
                //         best_mv = Some(mv);
                //         alpha = score;
                //         self.pv_table.store_pv(self.ply, &mv);
                //     }

                //     if score >= beta {
                //         if mv.get_capture() {
                //             self.caphist.update(depth, HashFlag::LowerBound, moved_piece, mv.get_target(), PieceType::from(position.piece_at(mv.get_target()).unwrap()));
                //         } else {
                //             self.killer_moves.store(depth as usize, &mv);
                //             self.conthist.update_many(&position, quiet_mvs, depth, mv);
                //             self.counter_mvs.add(&position, mv);
                //         }

                //         break;
                //     }
                // }

                // if !mv.get_capture() {
                //     quiet_mvs.push(mv);
                // } else {
                //     // 
                // }
            }
        }

        // if mvs_searched == 0 {
        //     if stm_in_check {
        //         return -MATE_VALUE + self.ply as i32;
        //     }
        //     // king is not in check, but there are no legal moves
        //     return 0; // stalemate/draw
        // }

        // if alpha != original_alpha {
        //     if best_mv.is_some_and(|mv| !mv.get_capture()) {
        //         self.conthist.update_many(&position, quiet_mvs, depth, best_mv.unwrap());
        //     }
        // }
        
        let tt_flag = if best_score >= beta { HashFlag::LowerBound } else if best_score > alpha { HashFlag::Exact } else { HashFlag::UpperBound };
        self.tt.record(position.hash_key, depth, best_score, self.ss[self.ply].eval, self.ply, tt_flag, 0, best_mv);
        self.ss[self.ply].best_move = best_mv;
        alpha
    }




}
