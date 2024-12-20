use crate::{board::{piece::Piece, position::{self, Position}}, move_logic::bitmove::Move, squares::Square};

use super::{history_bonus, malus, taper_bonus};

const LENGTH: usize = Piece::TOTAL * Square::TOTAL * Piece::TOTAL * Square::TOTAL;

/// The table is in the form of [prev_piece][prev_target][current_piece][current_target] = score;
/// Basically a 4D vector
/// We would turn this 4D vector into just 1D in this case.
#[derive(Debug, Clone)]
pub(crate) struct ContinuationHistory(Vec<i16>);
const MAX_HISTORY: i32 = i32::MAX/2;


impl ContinuationHistory {
    pub(crate) fn new() -> Self {
        // saves only the last 2-ply moves
        Self(vec![0; LENGTH])
    }

    /// Converts 4D index to 1D
    /// (a * bMax * cMax * dMax) + (b * cMax * dMax) + (c * dMax) + d
    /// a -> Previous Piece
    /// b -> Square target of the previous piece
    /// c -> Current Piece
    /// d -> Square target of the current piece
    const fn to_1_d_index(a: Piece, b: Square, c: Piece, d: Square) -> usize {
        ((a as usize) * Square::TOTAL * Piece::TOTAL * Square::TOTAL) +
        ((b as usize) * Piece::TOTAL * Square::TOTAL) + 
        ((c as usize) * Square::TOTAL) + (d as usize)
    }

    pub(crate) fn update(&mut self, prev_piece: Piece, prev_mv: Square, curr_piece: Piece, curr_mv: Square, bonus: i32) {
        let index = Self::to_1_d_index(prev_piece, prev_mv, curr_piece, curr_mv);
        let prev_value = unsafe{self.0.get_unchecked_mut(index)};
        // *prev_value = taper_bonus(*(prev_value) as i32, bonus);
        *prev_value = taper_bonus(*(prev_value) as i32, bonus);
    }

    /// 2-ply Continuation History
    const CONTINUATION_HISTORY: usize = 2;

    pub(crate) fn update_many(&mut self, pos: &Position, quiets: &Vec<Move>, depth: u8, best_mv: &Option<Move>) {
        let history_len = pos.history_len();
        // 2-ply Continuation history, so we need to get the last two moves
        for i in 0..Self::CONTINUATION_HISTORY {
            if let Some(idx) = history_len.checked_sub(i+1) {
                let Some(pos_history) = pos.history_at(idx) else {continue};
                let prev_tgt = pos_history.mv().tgt();
                let prev_piece = pos_history.board().piece_at(pos_history.mv().src()).unwrap();

                for mv in quiets {
                    let curr_piece = pos.piece_at(mv.get_src()).unwrap();
                    let curr_mv = mv.get_tgt();
                    let bonus = if Some(mv) == best_mv.as_ref() { history_bonus(depth) } else { malus(depth)};
                    self.update(prev_piece, prev_tgt, curr_piece, curr_mv, bonus);
                }
                
            } else {
                break;
            }
        }
    }

    pub(crate) fn get(&self, prev_piece: Piece, prev_tgt: Square, curr_piece: Piece, curr_tgt: Square) -> i16 {
        let idx = Self::to_1_d_index(prev_piece, prev_tgt, curr_piece, curr_tgt);
        *unsafe{ self.0.get_unchecked(idx) }
    }

    pub(crate) fn get_conth_at(&self, pos: &Position, mv: &Move, index: usize) -> i32 {
        let parent_exists = pos.history_len().checked_sub(index + 1);
        let Some(idx) = parent_exists else {return 0};
        // println!("the surfing index is >>>>> {idx}");
        if pos.history_at(idx).is_none() {
            // this only happens in the case of a null move
            return 0;
        }
        let prev = pos.history_at(idx).map(|history| {
            {
                let prev_mv = history.mv();
                let piece = history.board().piece_at(prev_mv.src()).unwrap();

                
                (prev_mv, piece)
            }
        }).unwrap();
        
        self.get(prev.1, prev.0.tgt(), pos.piece_at(mv.src()).unwrap(), mv.tgt()) as i32
    }
}
