use crate::{bit_move::Move, board::{piece::Piece, position::Position}, constants::{params::MAX_DEPTH, MAX_PLY, PLAYERS_COUNT, TOTAL_SQUARES}, move_scope::MoveScope, moves::Moves};

use crate::search::heuristics::pv_table::PVTable;

use super::heuristics::killer_moves::KillerMoves;

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
/// 10. [-] History Moves 
/// 11. [-] Aspiration Window
/// 12. [-] Iterative Deepening
/// 13. [x] PV-Table
/// This implementation is a fail-soft implementation (meaning we have to keep track of the best score)[XX] 
/// - fail-hard for now
pub(crate) struct Search {
    nodes:  usize,
    ply: usize,
    pv_table: PVTable,
    killer_moves: KillerMoves,
}


impl Search {
    pub(crate) fn new() -> Self {
        Self { nodes: 0, ply: 0, pv_table: PVTable::new(), killer_moves: KillerMoves::new() }
    }

    pub(crate) fn sort_moves(&self, mvs: &Moves, board: &Position) -> Vec<Move> {
        unimplemented!()
    }

    pub(crate) fn is_repetition(&self, board: &Position) -> bool {
        unimplemented!()
    }

    pub(crate) fn get_moves(&self, board: &Position) -> Vec<Move> {
        let mvs = board.gen_movement();
        //  now sort those moves and return the sorted moves
        // PV-nodes and expected Cut-nodes must be searched (give them more priority)
        

        // order of moves ordering
        // 1. (Good) Captures
        // 3. Killer moves (Should be just below the MVV-LVA captures)
        // 2. Hash move
        unimplemented!()
    }


    // In addition, we a score to return in case there are no captures available to be played. -->> static evaluation
    /// At the beginning of quiescence, the position's evaluation is used to establish a lower-bound on the score.
    /// If the lower bound from the stand pat(static evaluation) is always greater than or equal to beta, we can return the stand-pat(fail-soft)
    /// or beta(fail-hard) as a lower bound. Otherwise, the search continues
    fn quiescence(&mut self, mut alpha: i32, beta: i32, mut position: &mut Position) -> i32 {
        self.nodes+=1;
        
        let stand_pat = position.evaluate();

        if self.ply >= MAX_DEPTH { return stand_pat }
        // check if it's a draw
        if self.ply > 0 && (self.is_repetition(&position) || position.fifty.iter().any(|&p| p >= 50)) {
            return 0 // draw
        }

        let king_square = u64::from(position[Piece::king(position.turn)].trailing_zeros());
        let king_in_check = position.is_square_attacked(king_square, !position.turn);
        // conditions
            // 1. is king in check
                //  if the stm is in check, the position is not quiet, and there is a threat that needs to be resolved. In that case, 
                // all evastions to the check are searched. Stand pat is not allowed if we are in check.
                // So, if the king of the stm is in check - WE MUST SEARCH EVERY MOVE IN THE POSITION, RATHER THAN ONLY CAPTURES.
                    // - LIMIT THE GENERATION OF CHECKS TO THE FIRST X PLIES OF QUIESCENCE (AND USE "DELTA PRUNNING" TO AVOID LONG FRUITLESS SEARCHES TO GET OUT OF BEEN IN CHECK)
        // beta cutoff
        if stand_pat >= beta { return beta }

        if stand_pat > alpha { alpha = stand_pat }

        // Probe the Transposition Table here


        let moves = self.sort_moves(&position.gen_movement(), &position);
        let captures_only = !king_in_check;

        for (i, mv) in moves.into_iter().enumerate() {
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
}
