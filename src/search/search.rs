use crate::{bit_move::Move, bitboard::Bitboard, board::{piece::Piece, position::{self, Position}}, color::Color, constants::{params::MAX_DEPTH, INFINITY, MAX_PLY, MVV_LVA, PLAYERS_COUNT, TOTAL_SQUARES, VAL_WINDOW}, move_scope::MoveScope, moves::Moves, squares::Square, tt::{flag::HashFlag, tpt::TPT}};

use crate::search::heuristics::pv_table::PVTable;

use super::heuristics::{history::HistoryHeuristic, killer_moves::KillerMoves};

/// The number of nodes you can actually cut depends on:
/// 1. How well written your alpha-beta program is
/// 2. How well ordered your game-tree is (i.e the next moves on the board) --- GOOD MOVE ORDERING IS IMPORTANT
///     If you always put the best possible move first, you elimiate the most nodes.
/// 
/// GOALS:
///  1. [-] AlphaBeta 
///  2. [-] Quiescence Search
///         a. [x] Standing Pat
///         b. [-] Delta Pruning
///             i. https://www.chessprogramming.org/Delta_Pruning
///             ii. https://www.chessprogramming.org/CPW-Engine_quiescence (check how CPW implements its beta pruning)
///  3. [-] Late Move Reduction
///  4. [-] MVV_LVA - (Most Viable Victim -- Least Viable Attacker)
///  5. [-] Move Ordering
///  6. [-] Transposition Table
///  7. [-] Null move forward Prunning
///  8. [-] Principal Variation Node
///  9. [x] Killer Moves
/// 10. [x] History Moves 
/// 11. [-] Aspiration Window
/// 12. [-] Iterative Deepening
/// 13. [x] PV-Table
/// 14. [x] Repetitions https://www.chessprogramming.org/Repetitions
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
    tt: TPT<'a>
}


impl<'a> Search<'a> {
    pub(crate) fn new(tt: TPT<'a>) -> Self {
        Self { nodes: 0, ply: 0, pv_table: PVTable::new(), killer_moves: KillerMoves::new(),
            history_table: HistoryHeuristic::new(), tt }
    }

    pub(crate) fn iterative_deepening(&mut self, limit: u8, position: &mut Position) {
        let mut alpha = -INFINITY;
        let mut beta = INFINITY;

        for depth in 1..=limit {
            println!("RUNNING :::::::::::::::::: {depth}");
            let score = self.alpha_beta(alpha, beta, depth, position);
            println!("the score is {score} and nodes {}", self.nodes);
            if score <= alpha || score >= beta {
                // We fell outside the window, so try agin with a full-width window (and the same depth)
                alpha = -INFINITY; beta = INFINITY;
                continue;
            }

            alpha = score - VAL_WINDOW;
            beta = score + VAL_WINDOW;

            println!("MOVES ARE ::::");
            for i in self.pv_table.get_pv(depth as usize) {
                print!("-->> {}", Move::from(*i));
            }
        }
    }

    fn get_sorted_moves(&self, board: &Position) -> Vec<Move> {
        let mvs = board.gen_movement();
        let mut mvs = (mvs.collect::<Vec<_>>())[0..mvs.count_mvs()].to_vec();
        //  now sort those moves and return the sorted moves
        // PV-nodes and expected Cut-nodes must be searched (give them more priority)

        
        let mut sorted_mvs = Vec::with_capacity(mvs.len());
        if let Some(pv_mv) = self.pv_table.get_pv(self.ply).get(0) {
            if let Some(pos) = mvs.iter().position(|&m| *m == *pv_mv) {
                sorted_mvs.push(mvs[pos]);
                mvs.remove(pos);
                // mvs.swap(0, pos);
                // start_idx = 1;
            }
        }
        
        // Sort by MVV-LVA table
        let [mut captures, mut non_captures] = mvs.iter().fold([Vec::new(), Vec::new()], |mut acc, mv| {
            if mv.get_capture() { acc[0].push(*mv) } else { acc[1].push(*mv) }
            [acc[0].clone(), acc[1].clone()]
        });
        
        captures.sort_by_key(|mv| {
            let src = board.get_piece_at(mv.get_src(), board.turn).unwrap();
            let tgt = board.get_piece_at(mv.get_target(), !board.turn).unwrap();
            
            return MVV_LVA[src][tgt]
        });
        
        sorted_mvs.append(&mut captures);

        non_captures.sort_by_key(|mv| {
            if self.killer_moves.is_killer(self.ply, mv) {
                return 1
            } 
            0
        });

        sorted_mvs.append(&mut non_captures);

        return sorted_mvs
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

        let in_check = Self::in_check(&position, position.turn);
        // conditions
            // 1. is king in check
                //  if the stm is in check, the position is not quiet, and there is a threat that needs to be resolved. In that case, 
                // all evastions to the check are searched. Stand pat is not allowed if we are in check.
                // So, if the king of the stm is in check - WE MUST SEARCH EVERY MOVE IN THE POSITION, RATHER THAN ONLY CAPTURES.
                    // - LIMIT THE GENERATION OF CHECKS TO THE FIRST X PLIES OF QUIESCENCE (AND USE "DELTA PRUNNING" TO AVOID LONG FRUITLESS SEARCHES TO GET OUT OF BEEN IN CHECK)
        
        // beta cutoff
        if !in_check && stand_pat >= beta { return beta }
        if !in_check && stand_pat > alpha { alpha = stand_pat }
        
        // Probe the Transposition Table here
        let moves = self.get_sorted_moves(&position);
        let captures_only = !in_check;
        let mut best_score = -INFINITY;

        for mv in moves.into_iter() {
            if captures_only && !mv.get_capture() { continue; }

            if position.make_move_nnue(mv, MoveScope::AllMoves) {
                self.ply += 1;
                // if position.make_move_nnue(mv, MoveScope::AllMoves) {}
                let score = -self.quiescence(-beta, -alpha, position);
                position.undo_move(true);

                self.ply -= 1;
    
                if score > best_score {
                    best_score = score;
                    
                    if score > alpha { best_score = score; alpha = score; }
                    if score >= beta {alpha = beta; break}
                }
            }


        }


        if in_check && best_score == -INFINITY {
            return  - 5000;
        }
        
        alpha
    }

    fn in_check(position: &Position, color: Color) -> bool {
        let king_square = u64::from(position[Piece::king(color)].trailing_zeros());
        position.is_square_attacked(king_square, !color)
    }

    /// NB: This method is not pure, and would update the provided Move if the conditions are satisified
    fn probe_tt(&self, key: u64, depth: Option<u8>, alpha: i32, beta: i32, best_move: &mut Option<Move>) -> Option<i32>  {
        if let Some(entry) = self.tt.probe(key) {
            if depth.is_some_and(|d| d != entry.depth) { return None };
            best_move.replace(entry.mv?);
            let entry_score = entry.score(self.ply);
            let value = match entry.flag {
                HashFlag::Exact => Some(entry_score),
                HashFlag::LowerBound if entry_score >= beta => Some(beta),
                HashFlag::UpperBound if entry_score <= alpha => Some(alpha),
                _ => None
            };
            return value
        }
        None
    }


    pub(crate) fn alpha_beta(&mut self, mut alpha: i32, beta: i32, depth: u8, mut position: &mut Position) -> i32 {
        let mut hash_flag = HashFlag::UpperBound;

        if Self::is_repetition(&position, position.hash_key) || position.fifty.iter().any(|&s| s >= 50) {
            return 0; // daw
        }

        let mut best_mv: Option<Move> = None;

        let tt_hit = self.probe_tt(position.hash_key, Some(depth), alpha, beta, &mut best_mv);
        // When beta - alpha > 1, it indicates that there is a significant gap between the two bounds. This gap suggests that there are possible values for the evaluation score that have not yet been fully explored or are still uncertain.
        // The search can continue to explore more moves because the values returned by the evaluated moves could potentially fall within this range, providing room for a better evaluation.
        let explore_more_moves = (beta - alpha) > 1;

        if self.ply > 0 && tt_hit.is_some() && !explore_more_moves { return tt_hit.unwrap() }

        if depth == 0 { return self.quiescence(alpha, beta, position) }

        if self.ply > MAX_PLY - 1 { return position.evaluate() }

        self.nodes += 1;

        let stm_in_check = Self::in_check(position, position.turn);
        let depth = if stm_in_check { depth + 1} else { depth };

        let mut best_score = -INFINITY;
        let mvs = self.get_sorted_moves(&position);

        for (_count, mv) in mvs.iter().enumerate() {            
            // println!("ababababababab");
            if position.make_move_nnue(*mv, MoveScope::AllMoves) {
                self.ply += 1;
                
                let score = -self.alpha_beta(-beta, -alpha, depth -1, &mut position);
                
                position.undo_move(true);
                let moved_piece = position.piece_at(mv.get_src()).unwrap();
                
                if score > best_score {
                    best_score = score;
    
                    if score > alpha {
                        best_mv = Some(*mv);
                        hash_flag = HashFlag::Exact;
                        self.pv_table.store_pv(depth as usize, mv);
                        alpha = score;
                        
                        if score >= beta {
                            self.tt.record(position.hash_key, depth, beta, INFINITY, self.ply, hash_flag, 0, best_mv); 
                            if !mv.get_capture() {
                                self.history_table.update(moved_piece, mv.get_src(), depth);
                                self.killer_moves.store(depth as usize, mv);
                            }
                            return beta;
                        }
                    }
                }
            }
        }

        self.tt.record(position.hash_key, depth, best_score, INFINITY, depth as usize, hash_flag, 0, best_mv);
        alpha
    }
}
