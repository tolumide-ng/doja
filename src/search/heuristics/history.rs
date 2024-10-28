use crate::{board::piece::Piece, constants::{TOTAL_PIECES, TOTAL_SQUARES}, squares::Square};


// [i16; TOTAL_SQUARES]; TOTAL_PIECES]
/// History Heuristics
#[derive(Debug)]
pub(crate) struct HistoryHeuristic([i16; TOTAL_SQUARES * TOTAL_PIECES]);


impl HistoryHeuristic {
    const MAX_HISTORY: i16 = i16::MAX;

    pub(crate) fn new() -> Self {
        Self ([0; TOTAL_SQUARES * TOTAL_PIECES])
    }

    /// https://www.chessprogramming.org/History_Heuristic
    const fn bonus(depth: u8) -> i16 {
        300i16.saturating_mul(depth as i16) - 250
    }

    /// bonus is usually a multiple of depth
    /// fail-high -> Positive bonus, and all other moves -> Negative
    pub(crate) fn update(&mut self, piece: Piece, sq: Square, depth: u8) {
        let bonus = Self::bonus(depth);
        let clamped_bonus = i16::clamp(bonus, i16::MIN, i16::MAX);
        let history = unsafe { self.0.get_unchecked_mut((piece as usize * TOTAL_PIECES) + sq as usize) };
        *history += *history * clamped_bonus.abs() / Self::MAX_HISTORY;
    }
    
    pub(crate) fn get(&self, piece: Piece, sq: Square) -> i16 {
        *unsafe { self.0.get_unchecked((piece as usize * TOTAL_PIECES) + sq as usize) }
    }
}