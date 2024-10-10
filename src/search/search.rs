use crate::{bit_move::Move, board::{piece::Piece, position::Position}, constants::{params::MAX_DEPTH, MAX_PLY, PLAYERS_COUNT, TOTAL_SQUARES}, move_scope::MoveScope, moves::Moves, tt::{flag::HashFlag, tpt::TPT}};

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
    killer_moves: KillerMoves,
    history_table: HistoryHeuristic,
    tt: TPT<'a>
}


impl<'a> Search<'a> {
    pub(crate) fn new(tt: TPT<'a>) -> Self {
        Self { nodes: 0, ply: 0, pv_table: PVTable::new(), killer_moves: KillerMoves::new(),
            history_table: HistoryHeuristic::new(), tt }
    }

    pub(crate) fn sort_moves(&self, mvs: &Moves, board: &Position) -> Vec<Move> {
        unimplemented!()
    }


    pub(crate) fn get_sorted_moves(&self, board: &Position) -> Vec<Move> {
        let mvs = board.gen_movement();
        //  now sort those moves and return the sorted moves
        // PV-nodes and expected Cut-nodes must be searched (give them more priority)
        

        // order of moves ordering
        // 1. (Good) Captures
        // 3. Killer moves (Should be just below the MVV-LVA captures)
        // 2. Hash move
        unimplemented!()
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

        let in_check = self.stm_in_check(&position);
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

        for mv in moves.into_iter() {
            if captures_only && !mv.get_capture() { continue; }

            self.ply += 1;
            position.make_move_nnue(mv, MoveScope::AllMoves);
            let score = -self.quiescence(-beta, -alpha, position);
            position.undo_move(true);
            self.ply -= 1;

            if score >= beta {return beta}
            if score > alpha { alpha = score; }
        }
        
        alpha
    }

    fn stm_in_check(&self, position: &Position) -> bool {
        let king_square = u64::from(position[Piece::king(position.turn)].trailing_zeros());
        position.is_square_attacked(king_square, !position.turn)
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

        let stm_in_check = self.stm_in_check(position);
        let depth = if stm_in_check { depth + 1} else { depth };
        
        let mut searched_mvs = 0;
        let mvs = self.get_sorted_moves(&position);

        for (count, mv) in mvs.iter().enumerate() {
            self.ply += 1;
            // https://www.chessprogramming.org/Repetitions
            // self.is_repetition(&position);
        }


        
        
        0
    }
}
