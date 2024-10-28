use crate::{board::{piece::{Piece, PieceType}, position::Position, state::board::Board}, color::Color, constants::{params::MAX_DEPTH, DEPTH_REDUCTION_FACTOR, FULL_DEPTH_MOVE, INFINITY, MATE_VALUE, PIECE_ATTACKS, REDUCTION_LIMIT, VAL_WINDOW, ZOBRIST}, move_logic::{bitmove::{Move, MoveType}, move_picker::MovePicker, move_stack::MoveStack}, move_scope::MoveScope, squares::Square, tt::{flag::HashFlag, tpt::TPT}};
use crate::board::piece::Piece::*;
use crate::color::Color::*;
use crate::search::heuristics::pv_table::PVTable;

use super::heuristics::{history::HistoryHeuristic, killer_moves::KillerMoves};

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
    tt: TPT<'a>,
    limit: u8,
}


impl<'a> Search<'a> {
    pub(crate) fn new(tt: TPT<'a>) -> Self {
        Self { nodes: 0, ply: 0, pv_table: PVTable::new(), killer_moves: KillerMoves::new(),
            history_table: HistoryHeuristic::new(), tt, limit: 0, }
    }

    // fn aspiration_window(&mut self) {

    // }

    pub(crate) fn iterative_deepening(&mut self, limit: u8, position: &mut Position) {
        let mut alpha = -INFINITY;
        let mut beta = INFINITY;
        let mut delta = VAL_WINDOW;
        let mut depth = 1;

        const BIG_DELTA: usize = 975;

        loop {
            if depth > limit { break; }
            self.limit = depth;
            println!("RUNNING ::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::<<>>::::::::::: {depth}");
            println!("{}", position.to_string());
            self.ply = 0;
            let score = self.negamax(alpha, beta, depth, position);
            println!("the score is {score} and nodes {}", self.nodes);
            depth += 1;

            if score <= alpha {
                beta = (alpha + beta) / 2;
                alpha = (-INFINITY).max(alpha - beta);
                // depth = limit;
            } else if score >= beta {
                beta = (INFINITY).min(beta + delta);
            } else {
                // break
            }
            delta += delta/2;
            if delta >= BIG_DELTA as i32 { alpha = -INFINITY; beta = INFINITY } 

        }

            println!("MOVES ARE :::: with length of {}", self.pv_table.len(0));
            let mvs = self.pv_table.get_pv(0);
            for i in 0..limit {
                print!("-->> {}", Move::from(mvs[i as usize]));
            }
            println!("\n");
    }


 
    fn get_sorted_moves<const T: u8>(&self, board: &Position) -> Vec<Move> {
        let mut mvs = MoveStack::<Move>::new();
        board.gen_movement::<T, Move>(&mut mvs);
        let mut mvs = mvs.to_vec();

        let mut sorted_mvs = Vec::with_capacity(mvs.len());


        if let Some(pv_mv) = self.pv_table.get_pv(self.ply).get(0) {
            if let Some(pos) = mvs.iter().position(|&m| *m == *pv_mv) {
                sorted_mvs.push((mvs[pos], i32::MAX));
                mvs.remove(pos);
            }
        }

        if let Some(tt_data) = self.tt.probe(board.hash_key) {
            if let Some(tt_mv) = tt_data.mv {
                if let Some(pos) = mvs.iter().position(|&m| *m == *tt_mv) {
                    sorted_mvs.push((tt_mv, i32::MAX - 10));
                    mvs.remove(pos);
                }
            }
        }

        
        let [mut captures, mut non_captures] = mvs.iter().fold([Vec::new(), Vec::new()], |mut acc, mv| {
            if mv.get_capture() {
                let r = if board.see(mv, 0) {20_000} else{7_000} ;
                acc[0].push((*mv, r)) 
            } else { acc[1].push((*mv, 0)) }
            [acc[0].clone(), acc[1].clone()]
        }); // use Self::see at this level, and filter out bad moves already
        
        captures.sort_by_key(|mv| { return mv.1 }); captures.reverse();
        sorted_mvs.append(&mut captures);

        non_captures.sort_by_key(|(mv, score)| {
            if let Some(promoted_to) = mv.get_promotion() { if promoted_to == PieceType::Q { return 12_000 } else { return 11_000 }}
            let killers = self.killer_moves.get_killers(self.ply);
            if killers[0] == **mv { return 9_000 }
            if killers[1] == **mv { return 8_000 }

            let piece = board.piece_at(mv.get_src()).unwrap();
            return self.history_table.get(piece, mv.get_target());
        });

        non_captures.reverse();

        sorted_mvs.append(&mut non_captures);



        // if board.turn == Color::Black {
        //     for (mv, score) in &sorted_mvs {
        //         print!("-->> {} {}-->>", mv.to_string(), score);
        //     }
        // }

        // println!("\n\n");
        
        return sorted_mvs.iter().map(|mv| mv.0).collect::<Vec<_>>()
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
        let (tt_score, _tt_mv) = self.probe_tt(position.hash_key, None, alpha, beta);
        if tt_score.is_some() { return tt_score.unwrap() }

        // conditions
        // 1. is king in check: If the stm is in check, the position is not quiet, and there is a threat that needs to be resolved. In that case, 
        // all ways to evade the check are searched. Stand pat is not allowed if we are in check.
        // So, if the king of the stm is in check - WE MUST SEARCH EVERY MOVE IN THE POSITION, RATHER THAN ONLY CAPTURES.
        // - LIMIT THE GENERATION OF CHECKS TO THE FIRST X PLIES OF QUIESCENCE (AND USE "DELTA PRUNNING" TO AVOID LONG FRUITLESS SEARCHES TO GET OUT OF BEEN IN CHECK)
        let in_check = Self::in_check(&position, position.turn);
        // beta cutoff
        if !in_check && stand_pat >= beta { 
            // println!("{}", position.to_string());
            // println!("RETURNING BETA************************************* stand_pat={stand_pat} alpha==>>{alpha}, and beta is {beta}",);
            return beta }
        if !in_check && stand_pat > alpha { alpha = stand_pat }

        let mut best_move: Option<Move> = None;
    
        // let captures_only = !in_check;
        let moves = self.get_sorted_moves::<{MoveScope::CAPTURES}>(&position);
        let mut best_score = -INFINITY;

        for mv in moves.into_iter() {
            // if captures_only && !mv.get_capture() { continue; }

            if position.make_move_nnue(mv, MoveScope::AllMoves) {
                self.ply += 1;
                let score = -self.quiescence(-beta, -alpha, position);
                position.undo_move(true);
                
                self.ply -= 1;
                

                // if position.turn == Color::Black {
                //     println!(":::::::::::::::::::::::::::::::::::::::::::: mm-->>{} alpha={alpha}, beta={beta}, score={score} ply==>>{} eval-->>{}", mv.to_string(), self.ply, position.evaluate());
                // }
                if score > best_score {
                    best_score = score;
                    
                    if score > alpha { best_move = Some(mv); alpha = score; }
                    if score >= beta {alpha = beta; break}
                }
            }


        }

        if in_check && best_score == -INFINITY {
            return  - 5000;
        }

        let tt_flag = if best_score >= beta { HashFlag::LowerBound } else if best_score > alpha { HashFlag::Exact } else { HashFlag::UpperBound };
        self.tt.record(position.hash_key, 0, alpha, 0, self.ply, tt_flag, 0, best_move);

        
        alpha
    }

    fn in_check(position: &Position, color: Color) -> bool {
        let king_square = u64::from(position[Piece::king(color)].trailing_zeros());
        position.is_square_attacked(king_square, !color)
    }

    fn probe_tt(&self, key: u64, depth: Option<u8>, alpha: i32, beta: i32) -> (Option<i32>, Option<Move>)  {
        if let Some(entry) = self.tt.probe(key) {
            if depth.is_some_and(|d| d != entry.depth) { return (None, None) };
            let entry_score = entry.score(self.ply);
            let value = match entry.flag {
                HashFlag::Exact => (Some(entry_score), None),
                HashFlag::LowerBound if entry_score >= beta => (Some(beta), None),
                HashFlag::UpperBound if entry_score <= alpha => (Some(alpha), None),
                _ => (None, entry.mv)
            };
            return value
        }
        (None, None)
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

        let score = -self.negamax(-beta, -beta+1, depth-1-DEPTH_REDUCTION_FACTOR, &mut position);

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

    pub(crate) fn negamax(&mut self, mut alpha: i32, beta: i32, depth: u8, mut position: &mut Position) -> i32 {
        if Self::is_repetition(&position, position.hash_key) || position.fifty.iter().any(|&s| s >= 50) {
            return 0; // draw
        }
        
        let stm_in_check = Self::in_check(position, position.turn);
        let depth = if stm_in_check && depth < MAX_DEPTH as u8 { depth + 1 } else { depth };
        
        if depth == 0 || self.ply >= MAX_DEPTH {
            return self.quiescence(alpha, beta, position);
        }

        let (tt_score, tt_mv) = self.probe_tt(position.hash_key, Some(depth), alpha, beta);
        let mut best_mv: Option<Move> = tt_mv;
        
        // When beta - alpha > 1, it indicates that there is a significant gap between the two bounds. This gap suggests that there are possible values for the evaluation score that have not yet been fully explored or are still uncertain.
        // The search can continue to explore more moves because the values returned by the evaluated moves could potentially fall within this range, providing room for a better evaluation.
        let explore_more_moves = (beta - alpha) > 1;
        if self.ply > 0 && tt_score.is_some() && !explore_more_moves { return tt_score.unwrap() }
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
        let killer_mvs = self.killer_moves.get_killers(self.ply).map(|m| if m == 0 {None} else {Some(Move::from(m))});
        // let mvs = self.get_sorted_moves::<{ MoveScope::ALL }>(&position);
        let mut mvs = MovePicker::<{MoveScope::ALL}>::new(0, tt_mv, killer_mvs);
        // println!("the total moves generated are >>>>>>{}, so that the ones after is now {}", mvs);
        // println!("tt => {:?}, and pv={:?}", tt_mv, self.pv_table.get_pv(self.ply).get(0));
        while let Some(mv) = mvs.next(&position, &self.history_table) {
            println!("generating >>>>>>>>>>>>>>>>>>>>>>>>>>>> {}", mv);
        // for mv in mvs.iter() {
            // let mv = &mv;
            // let mv = *mv;
            // println!("running {} at {i}", mv.to_string());
            if position.make_move_nnue(mv, MoveScope::AllMoves) {
                self.ply += 1;

                let score = match mvs_searched {
                    0 => -self.negamax(-beta, -alpha, depth -1, &mut position),
                    _ => {
                        // https://web.archive.org/web/20150212051846/http://www.glaurungchess.com/lmr.html
                        // condition for Late Move Reduction
                        let non_tatcital_mv = !stm_in_check && mv.get_promotion().is_none() && !mv.get_capture();

                        let mut value = if (mvs_searched as u8 >= FULL_DEPTH_MOVE) && (depth >= REDUCTION_LIMIT) && non_tatcital_mv {
                            -self.negamax(-(alpha + 1), -alpha, depth-2, position)
                        } else {
                            alpha + 1 // Hack to ensure that full-depth search is done
                        };

                        if value > alpha {
                            value = -self.negamax(-(alpha - 1), -alpha, depth-1, position);

                            if (value > alpha) && value < beta {
                                value = -self.negamax(-beta, -alpha, depth-1, position);
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

                // if score >= beta {
                //     self.tt.record(zobrist_key, depth, beta, INFINITY, self.ply, HashFlag::LowerBound, 0, best_mv); 
                //     if !mv.get_capture() {
                //         self.killer_moves.store(depth as usize, &mv);
                //     }
                //     return beta;
                // }

                // if score > alpha {
                //     best_score = score;
                    
                //     best_mv = Some(mv);
                //     // hash_flag = HashFlag::Exact;
                //     self.pv_table.store_pv(self.ply, &mv);
                //     alpha = score;

                //     if !mv.get_capture() {
                //         self.history_table.update(moved_piece, mv.get_src(), depth);
                //     }
                // }


                if score > best_score {
                    best_score = score;
                    if score > alpha {
                        best_mv = Some(mv);
                        self.pv_table.store_pv(self.ply, &mv);
                        alpha = score;
    
                        if !mv.get_capture() {
                            self.history_table.update(moved_piece, mv.get_src(), depth);
                        }
                    }

                    if alpha >= beta {
                        self.tt.record(zobrist_key, depth, beta, INFINITY, self.ply, HashFlag::LowerBound, 0, best_mv); 
                    if !mv.get_capture() {
                        self.killer_moves.store(depth as usize, &mv);
                    }
                    }
                    
                }
            }
        }

        if mvs_searched == 0 {
            if stm_in_check {
                return -MATE_VALUE + self.ply as i32;
            }
            // king is not in check, but there are no legal moves
            return 0; // stalemate/draw
        }
        
        let tt_flag = if best_score >= beta { HashFlag::LowerBound } else if best_score > alpha { HashFlag::Exact } else { HashFlag::UpperBound };
        self.tt.record(position.hash_key, depth, best_score, INFINITY, self.ply, tt_flag, 0, best_mv);
        alpha
    }




}
