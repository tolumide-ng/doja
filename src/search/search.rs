use crate::{board::{piece::Piece, position::Position}, color::Color, constants::{params::MAX_DEPTH, DEPTH_REDUCTION_FACTOR, FULL_DEPTH_MOVE, INFINITY, LONGEST_TB_MATE, MATE_IN_MAX_PLY, MATE_VALUE, MAX_PLY, RAZOR_MARGIN, REDUCTION_LIMIT, SE_LOWER_LIMIT, ZOBRIST}, move_logic::{bitmove::Move, move_picker::{MovePicker, Stage}}, move_scope::MoveScope, search::constants::Root, tt::{entry::TTData, flag::HashFlag, tpt::TPT}, utils::lmr::reduction};
use crate::board::piece::Piece::*;
use crate::color::Color::*;

use super::{constants::{NodeType, NotPv, Pv}, heuristics::{capture_history::CaptureHistory, continuation_history::ContinuationHistory, countermove::CounterMove, history::HistoryHeuristic, killer_moves::KillerMoves, pv::PVTable}, stack::{Stack, StackItem}, threads::Thread};

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
/// 16. [-] [Need to add implementation for detecting tactical moves](https://www.chessprogramming.org/Tactical_Moves)
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
    ss: [StackItem; MAX_PLY + 10],
    depth: usize,
    limit: usize,
    eval: i32,
    last_move_was_null: bool,
}


impl<'a> Search<'a> {
    pub(crate) fn new(tt: TPT<'a>) -> Self {
        Self { nodes: 0, ply: 0, pv_table: PVTable::default(), killer_moves: KillerMoves::new(), last_move_was_null: false,
            history_table: HistoryHeuristic::new(), tt, caphist: CaptureHistory::default(), conthist: ContinuationHistory::new(),
                counter_mvs: CounterMove::new(), ss: [StackItem::default(); MAX_PLY + 10], depth: 0, limit: 0, eval: 0 }
    }

    fn aspiration_window(&mut self, position: &mut Position, t: &mut Thread) -> i32 {
        let mut alpha = -INFINITY;
        let mut beta = INFINITY;
        let mut delta = -INFINITY;
        let mut depth = t.depth + 1;
        let mut pv = PVTable::default();

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
            // self.ply = 0;
            let score = self.negamax::<Root>(alpha, beta, depth  as u8, position, &mut pv, false, t);
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
                self.pv_table = pv.clone();
                // if score.abs() < LONGEST_TB_MATE && depth > 1 {
                //     depth -= 1;
                // }
            } else {
                self.pv_table = pv.clone();
                println!("4444---->>>4444");
                return score;
            }
            delta += delta/2;
            if delta >= BIG_DELTA as i32 { alpha = -INFINITY; beta = INFINITY } 
        }
    }

    pub(crate) fn iterative_deepening(&mut self, limit: usize, position: &mut Position, t: &mut Thread) {
        self.limit = limit;
        while self.depth < MAX_DEPTH && self.depth < self.limit {
            // println!("\n\n\n RUNNING ::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::<<>>::::::::::: {}", self.depth);
            let eval = self.aspiration_window(position, t);
            
            self.eval = eval;
            t.eval = eval;
            self.depth += 1;
            t.depth += 1;

        }


        println!("Originally ||||||||||||||||||||||||| {}", position.to_string());


        let length = self.pv_table.length;
        println!("MOVES ARE :::: with length of {}", length);
        let mvs = &self.pv_table.mvs()[0..length];
        for i in 0..length {
            print!("|||-->> {}", Move::from(mvs[i as usize]));
            // println!("in check???? {}", position);
        }
        println!("\n\n");
    }

    fn is_repetition(position: &Position, key: u64) -> bool {
        let len = position.history_len();
        if len == 0 { return false }

        // subtracting 1 from len because we don't care about the opponent's (the person who played last's) game
        // stepping by 2 because we don't care about the opponent's key positional history in this case
        for index in (0..len-1).rev().step_by(2) {
            if let Some(history) = position.history_at(index) { 
                if history.board().hash_key == key { return true }
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
        let mut best_value = eval;

        let killer_mvs = self.killer_moves.get_killers(self.ply).map(|m| if m == 0 {None} else {Some(Move::from(m))});
        let mut mvs = MovePicker::<{MoveScope::CAPTURES}>::new(0, tt_move, killer_mvs);
        while let Some(mv) = mvs.next(&position, &self.history_table, &self.caphist, &self.conthist, &self.counter_mvs) {
            if position.make_move(mv, MoveScope::AllMoves) {
                self.ply += 1;
                let value = -self.quiescence(-beta, -alpha, position);
                position.undo_move(true);
                
                self.ply -= 1;

                if value > best_value {
                    best_value = value;

                    if value > alpha {
                        best_move = Some(mv);
                        alpha = value;
                    }

                    if value >= beta {
                        alpha = beta;
                        break;
                    }
                }
            }
        }

        if in_check && best_value == -INFINITY {
            return  -5000;
        }

        let tt_flag = if best_value >= beta { HashFlag::LowerBound } else if best_value > old_alpha { HashFlag::Exact } else { HashFlag::UpperBound };
        self.tt.record(position.hash_key, 0, alpha, self.ss[self.ply].eval, self.ply, tt_flag, 0, best_move);

        
        alpha
    }

    fn in_check(position: &Position, color: Color) -> bool {
        let king_square = u64::from(position[Piece::king(color)].trailing_zeros());
        position.is_square_attacked(king_square, !color)
    }

    // fn probe_tt(&self, key: u64, depth: Option<u8>, alpha: i32, beta: i32) -> Option<TTData> {
    //     if let Some(entry) = self.tt.probe(key) {
    //         if depth.is_some_and(|d| entry.depth < d) { return None };
    //         let entry_score = entry.score;
    //         let value = match entry.flag {
    //             HashFlag::Exact => Some(entry),
    //             HashFlag::LowerBound if entry_score >= beta => Some(entry),
    //             HashFlag::UpperBound if entry_score <= alpha => Some(entry),
    //             _ => None
    //         };
    //         return value;
    //     }
    //     None
    // }

    /// nmfp: Null Move forward prunning
    /// https://web.archive.org/web/20040427014629/http://brucemo.com/compchess/programming/nullmove.htm
    /// "If I do nothing here, can the opponent do anything?"
    fn make_null_move(&mut self, beta: i32, depth: u8, mut position: &mut Position, pv: &mut PVTable, cutnode: bool, t: &mut Thread,) -> i32 {
        self.ss[self.ply].moved = None;
        self.ss[self.ply].mv = None;
        self.ply += 1;
        
        let old_hashkey = position.hash_key;
        let old_enpassant = position.enpassant;

        if let Some(enpass_sq) = position.enpassant {
                // we know that we're going to remove the enpass if it's available (see 4 lines below), so we remove it from the hashkey if it exists here
            position.set_zobrist(position.hash_key ^ ZOBRIST.enpassant_keys[enpass_sq]);
        }

        position.set_enpassant(None);  // because whatever move they made (enpassant or not) would have resulted in them losing the enpassant anyway
        position.set_turn(!position.turn);
        position.set_zobrist(position.hash_key ^ ZOBRIST.side_key); // side about to move
        position.nnue_push();

        self.last_move_was_null = true;
        let score = -self.negamax::<NotPv>(-beta, -beta+1, depth, &mut position, pv, cutnode, t);
   
        // reverse all actions, after we're done
        self.ply -= 1;
        position.nnue_pop();

        // recent change for undo move in order to avoid cloning the board
        position.set_turn(!position.turn);
        position.set_enpassant(old_enpassant);
        position.set_zobrist(old_hashkey);
        self.last_move_was_null = false;

        score
    }

    pub(crate) fn negamax<NT: NodeType>(&mut self, mut alpha: i32, mut beta: i32, depth: u8, mut position: &mut Position, pv: &mut PVTable, cutnode: bool, t: &mut Thread) -> i32 {
        let mut depth = depth;
        
        let stm_in_check = Self::in_check(position, position.turn);
        // Check extension
        // https://www.chessprogramming.org/Check_Extensions
        if stm_in_check && depth < MAX_DEPTH as u8 { depth +=1; };
        
        if depth == 0 || self.ply >= MAX_DEPTH {
            return self.quiescence(alpha, beta, position);
        }
        
        if Self::is_repetition(&position, position.hash_key) || position.fifty.iter().any(|&s| s >= 50) {
            return 0; // draw
        }
        let pv_node = alpha != beta -1;
        let mut old_pv = PVTable::default();
        let opv = &mut old_pv;
        let hash_key = position.hash_key;
        pv.length = 0;
        

        if !NT::ROOT {
            let ply = self.ply as i32;
            // Mate distance pruning
            alpha = alpha.max(-MATE_VALUE + ply);
            beta = beta.min(MATE_VALUE - ply - 1);
            if alpha >= beta { return alpha }

            if ply > 0 && (Self::is_repetition(&position, hash_key) || position.fifty.iter().any(|&p| p >= 50)) {
                return 0 // draw
            }

            // if alpha < 0 && self.re/
        }

        // Transposition table lookup
        // Singular extension: If this move was already excluded, aboid pruning on this node
        let excluded = self.ss[self.ply].excluded;
        let tt_entry = self.tt.probe(hash_key);
        let mut tt_move: Option<Move> = None;
        
        let in_signular_search = excluded.is_some();
        let mut possibly_singular = false;

        // if let Some(entry) = tt_entry {
        if let Some(entry) = tt_entry {
            // if !pv_node && self.ply >= (depth  as usize) && !(position.fifty.iter().sum::<u8>() >= 80) 
            // && (matches!(entry.flag, HashFlag::Exact) || 
            //     matches!(entry.flag, HashFlag::LowerBound if entry.score >= beta) ||
            //     matches!(entry.flag, HashFlag::UpperBound if entry.score <= alpha)) {
            //         if let Some(mv) = entry.mv {
            //             if !mv.is_tactical() && entry.score >= beta {
            //                 self.conthist.update_many(&position, &vec![mv], depth, &Some(mv));
            //             }
            //         }
            //         // tt_move = entry.mv;
            //         return entry.score;
            //     }
                // Some(entry)
            // Don't use the tt result at the root of a singular search
            if !in_signular_search {
                let tt_depth = entry.depth;
                let tt_flag = entry.flag;
                let tt_value = entry.score;

                if !pv_node && tt_depth >= depth {
                    tt_move = entry.mv;
                    match entry.flag {
                        HashFlag::Exact => return tt_value,
                        HashFlag::LowerBound if tt_value >= beta => return beta,
                        HashFlag::UpperBound if tt_value <= alpha => return alpha,
                        _ => ()
                    };
                }

                // println!("xxxxxxxxxxxxxxxxxxxxxxxxxxxxx-------------------------------------------------------------------->>");
                // if !pv_node && tt_depth >= depth && position.fifty.iter().sum() >= 80 && matches!()

                possibly_singular = !NT::ROOT && depth >= SE_LOWER_LIMIT
                && matches!(tt_flag, HashFlag::LowerBound | HashFlag::UpperBound)
                && tt_value.abs() < LONGEST_TB_MATE
                && tt_depth >= depth -3;
            }
        };

        // Probe Tablebase (Skipped for now)

        // Static evaluation of this position
        let eval = if stm_in_check {
            -INFINITY
        } else if in_signular_search {
            self.ss[self.ply].eval
        } else if !stm_in_check {
            if let Some(entry) = tt_entry {
                let tt_value = entry.score;
                let tt_eval = entry.eval as i32;
                

                self.ss[self.ply].eval = if tt_eval == -INFINITY {
                    position.evaluate()
                } else {
                    tt_eval
                };

                match entry.flag {
                    HashFlag::Exact => tt_value,
                    HashFlag::LowerBound if tt_value > self.ss[self.ply].eval => tt_value,
                    HashFlag::UpperBound if tt_value < self.ss[self.ply].eval => tt_value,
                    _ => self.ss[self.ply].eval,
                }
            } else {
                self.ss[self.ply].eval = position.evaluate();
                self.ss[self.ply].eval
            }
        } else {
            self.ss[self.ply].eval = -INFINITY;
            self.ss[self.ply].eval
        };


        // Setup the `improving` flag (from carp)
        // https://www.chessprogramming.org/Improving
        // let improving = if self.ply >= 2 && self.ss[self.ply - 2].eval != -INFINITY {
        //     self.ss[self.ply].eval > self.ss[self.ply - 2].eval
        // } else if self.ply >= 4 && self.ss[self.ply - 4].eval != -INFINITY {
        //     self.ss[self.ply].eval > self.ss[self.ply - 4].eval
        // } else {
        //     true
        // };
        let improving = !stm_in_check && ((self.ply >= 4 && eval >= self.ss[self.ply -4].eval) || (self.ply >= 2 && eval >= self.ss[self.ply -2].eval));

        // if !pv_node && !NT::ROOT && !stm_in_check && !in_signular_search {
            if !pv_node && !NT::ROOT && !stm_in_check && !in_signular_search {
            // Razoring: If evaluation + margin isn't better than alpha at the lowest depth, Go straight to quiescence search.
            // if eval < alpha - 392 - 297 * (depth * depth) as i32 {
            //     let value = self.quiescence(alpha -1, alpha, position);
            //     if value < alpha { return value; }
            // }
            // if depth < 3 && eval <= alpha - 494 - 290 * (depth * depth) as i32 {
            // // if depth < 3 && eval <= alpha - RAZOR_MARGIN[depth as usize] {
            //     let value = self.quiescence(alpha -1, alpha, position);
            //     if value < alpha && value.abs() < MATE_IN_MAX_PLY {
            //         return value;
            //     }
                    
            //         // let r_alpha = alpha - (depth >= 2) as i32 * RAZOR_MARGIN[depth as usize];
            //         // let value = self.quiescence(r_alpha, r_alpha + 1, position);
            //         // if depth < 2 || value <= r_alpha {
            //             //     return value;
            //             // }
            // }
            

            // Reverse futility pruning
            // https://www.chessprogramming.org/Reverse_Futility_Pruning
            let rfp_margin = 80 * depth as i32 - 55 * (improving as i32);
            if depth <= 8 && eval - rfp_margin >= beta {
                return beta;
            }
            
            
            // Null move search
            // let null_move_forward_pruning_conditions = depth >= (DEPTH_REDUCTION_FACTOR + 1) && !stm_in_check && self.ply > 0;
            let null_move_forward_pruning_conditions = depth > 3 && self.ply > 0 && ((eval + 70 * (improving as i32)) >= beta) && !position.possibly_zugzwang() && !self.last_move_was_null;
            if null_move_forward_pruning_conditions {
                // Null move dynamic reduction based on depth
                let r = (4 + depth/4).min(depth);
                // let r = if depth > 6 {4} else {3};
                // let r = ((eval - beta)/202).min(6) + (depth as i32)/3 + 5;
                let value = self.make_null_move(beta, depth-r as u8, position, opv, !cutnode, t);
                if value >= beta {
                    return beta;
                }
            }
        }

        let mut depth = depth;
        
        // Internal Iterative Reduction (IIR)
        // https://www.chessprogramming.org/Internal_Iterative_Reductions
        // if !NT::ROOT && depth > 5 && tt_entry.is_some_and(|entry| entry.mv.is_none()) {
            // depth -= 3;
        // if !NT::ROOT && depth >= 4 && !in_signular_search && tt_entry.is_none() {
        //     depth -= 1;
        // }

        
        // When beta - alpha > 1, it indicates that there is a significant gap between the two bounds. This gap suggests that there are possible values for the evaluation score that have not yet been fully explored or are still uncertain.
        // The search can continue to explore more moves because the values returned by the evaluated moves could potentially fall within this range, providing room for a better evaluation.
        // let explore_more_moves = (beta - alpha) > 1;
        // if self.ply > 0 && tt_value.is_some() && !explore_more_moves { return tt_value.unwrap() }
        // if self.ply > MAX_PLY - 1 { return position.evaluate() }

        self.nodes += 1;

        let mut mvs_searched = 0;

        // if tt_entry.is_none() && !!stm_in_check && excluded.is_none() {
        //     self.tt.record(hash_key, depth, -INFINITY, eval, self.ply, HashFlag::UpperBound, 0, None);
        // }


        let mut best_value = -INFINITY;
        let mut best_mv: Option<Move> = None;
        let original_alpha = alpha;
        let killer_mvs = self.killer_moves.get_killers(self.ply).map(|m| if m == 0 {None} else {Some(Move::from(m))});
        let mut mvs = MovePicker::<{MoveScope::ALL}>::new(0, tt_move, killer_mvs);

        if mvs.stage == Stage::Done {
            if stm_in_check { return  -INFINITY + self.ply as i32 }
            else { return 0 }
        }

        // 30 is a practical maximum number of quiet moves that can be generated in a chess position (MidGame)
        let mut quiet_mvs: Vec<Move> = Vec::with_capacity(20);
        let mut captures: Vec<(Move, HashFlag)> = Vec::with_capacity(16);
        while let Some(mv) = mvs.next(&position, &self.history_table, &self.caphist, &self.conthist, &self.counter_mvs) {
            // println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>--{}", mv.to_string());
            if excluded == Some(mv) {
                // mvs_searched += 1;
                continue;
            }
            let mut extension = 0;
            // if possibly_singular  && tt_move.is_some_and(|tt_mv| tt_mv == mv) {
            if possibly_singular && tt_move.is_some_and(|tt_mv| tt_mv == mv) {
                let tt_value = tt_entry.unwrap().score;
                let se_beta = (tt_value - 2 * depth as i32).max(-INFINITY); // singular extension beta
                let se_depth = (depth-1)/2;
            
                self.ss[self.ply].excluded = Some(mv);
                let value = self.negamax::<NotPv>(se_beta -1, se_beta, se_depth, position, opv, cutnode, t);
                self.ss[self.ply].excluded = None;
            
                if value < se_beta {
                    extension = 1;
                }
            }

            let moved_piece = position.piece_at(mv.get_src()).unwrap();
            if position.make_move(mv, MoveScope::AllMoves) {

                self.ss[self.ply].moved = Some(moved_piece);
                self.ss[self.ply].mv = Some(mv);
                self.ply += 1;
                // self.ss[self.ply]. 

                let is_killer_mv = killer_mvs.iter().any(|km| km.is_some_and(|kmv| kmv == mv));
                let new_depth = depth + extension;
                let mut value = -INFINITY;

                // // Determines whether or not we should perform a full-depth search
                // // https://www.chessprogramming.org/Late_Move_Reductions
                // let full_depth_search = if depth >= 2 && mvs_searched >= (2 + pv_node as usize) && !NT::ROOT {
                //     let r = if mv.is_quiet() {
                //         // println!("mv --->>> {}, and depth --->> {}", mv, depth);
                //         // let mut r = new_depth as i16 - reduction::<Pv>(improving, depth as usize, mvs_searched);
                //         let mut r = reduction::<Pv>(improving, depth as usize, mvs_searched) as i32;
                //         // if tt_move.is_some_and(|m| m.is_capture()) { r += 1; } // Reduce when tt_mv is a capture
                //         // if stm_in_check {r -=1;} // reduce less if we're in check 
                //         // // side has changed now since the new move has been applied
                //         // if position.stm_in_check() {r-=1;} // reduce depth less, if this would put the opponent in check (gives check)
                //         // if is_killer_mv { r -=1; }
                //         // if improving { r -=1; }
                //         // // println!("-XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX-");
                //         // if !improving { r -=1; }
                //         // lmr_depth.clamp(0, depth as i16 -1);

                //         // carp
                //         r += !pv_node as i32;
                //         r += cutnode as i32;
                //         r -= stm_in_check as i32;
                //         r -= position.stm_in_check() as i32;
                //         // r.clamp(1, (depth-1) as i32) as usize
                //         0.max((depth as i32) - r)
                //     } else { 1 };
                    
                //     value = -self.negamax::<NotPv>((-alpha)-1, -alpha, new_depth - (r as u8), position, opv, !cutnode, false);
                //     // let reduced_depth = new_depth - r as u8;
                //     // If value is greater than alpha (potentially better), search deeper
                //     value > alpha && r > 1
                // } else {
                //     !pv_node || mvs_searched > 0
                // };

                
                // if full_depth_search {
                //         // println!("the values here are >>>>>> alpha---->>>> {alpha}  ** score --> {value} ** beta==>> {beta} bestValue ==>>> {best_value}");
                //     value = -self.negamax::<NotPv>((-alpha)-1, -alpha, new_depth -1 , position, opv, !cutnode, false);
                // }

                // // if pv_node && (mvs_searched == 1 || (value > alpha && (NT::ROOT || value < beta)) ) {
                // if pv_node && (mvs_searched == 0 || value > alpha) {
                //     value = -self.negamax::<NotPv>(-beta, -alpha, new_depth-1, position, opv, false, false);
                // }

                
                let value = match mvs_searched {
                    0 => -self.negamax::<NotPv>(-beta, -alpha, new_depth -1, &mut position, opv, false, t),
                    _ => {
                        // https://web.archive.org/web/20150212051846/http://www.glaurungchess.com/lmr.html
                        // condition for Late Move Reduction
                        let non_tatcital_mv = !stm_in_check && mv.is_quiet();

                        let mut result = if (mvs_searched as u8 >= FULL_DEPTH_MOVE) && (depth >= REDUCTION_LIMIT) && non_tatcital_mv {
                            -self.negamax::<NotPv>(-(alpha + 1), -alpha, depth-2, position, opv, false, t)
                        } else {
                            alpha + 1 // Hack to ensure that full-depth search is done
                        };
                        
                        if result > alpha {
                            result = -self.negamax::<NotPv>(-(alpha - 1), -alpha, depth-1, position, opv, false, t);
                            
                            if (result > alpha) && result < beta {
                                result = -self.negamax::<NotPv>(-beta, -alpha, depth-1, position, opv, false, t);
                            }
                        }
                        
                        result
                    }
                };
                                
                // let zobrist_key = position.hash_key;
                position.undo_move(true);
                self.ply -= 1;
                
                let mut flag = HashFlag::UpperBound;
                if value > best_value {
                    best_value = value;
                    
                    if value > alpha {
                        // println!("---------------------------------------------------------------- alpha -->> {alpha}, beta -->> {beta}, and value -->> {value}");
                        best_mv = Some(mv);
                        alpha = value;
                        pv.update(mv, &opv);
                    }

                    if value >=beta {
                        if mv.is_quiet() {
                            self.killer_moves.store(depth as usize, &mv);
                        }
                        self.update_logs(&position, &best_mv, &quiet_mvs, &captures, depth);
                        t.update_stats(&position, &best_mv, &quiet_mvs, &captures, depth);
                        alpha = beta;
                        flag = HashFlag::LowerBound;
                        break;
                    }
                }

                if mv.is_quiet() {
                    quiet_mvs.push(mv);
                    self.history_table.update(moved_piece, mv.get_src(), depth);
                    t.history_table.update(moved_piece, mv.get_src(), depth);
                } else if mv.is_capture() {
                    captures.push((mv, flag));
                }
                mvs_searched +=  1;
            }
        }
        
        let tt_flag = if best_value >= beta { HashFlag::LowerBound } else if best_value > original_alpha { HashFlag::Exact } else { HashFlag::UpperBound };
        self.tt.record(hash_key, depth, best_value, self.ss[self.ply].eval, self.ply, tt_flag, 0, best_mv);
        self.ss[self.ply].best_move = best_mv;
        alpha
    }

    pub(crate) fn update_logs(&mut self, position: &Position, best_mv: &Option<Move>, quiets: &Vec<Move>, captures: &Vec<(Move, HashFlag)>, depth: u8) {
        self.caphist.update_many(&position, depth,captures);

        if best_mv.is_some_and(|m| m.is_quiet()) {
            self.conthist.update_many(&position, &quiets, depth, best_mv);
            self.counter_mvs.add_many(&position, quiets);
        }
    }




}
