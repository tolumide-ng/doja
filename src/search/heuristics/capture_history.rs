use crate::{board::piece::{Piece, PieceType}, squares::Square, tt::flag::HashFlag};

const MAX_HISTORY: i32 = i32::MAX/2;

// type CaptureHistoryTable = [[i16; Piece::COUNT]; Square::TOTAL];
/// It is a history table indexed by moved piece, target square, and captured piece type. 
/// The history table receives a bonus for captures that failed high, and maluses for all capture moves that did not fail high. 
/// The history values is used as a replacement for LVA in MVV-LVA.
/// https://www.chessprogramming.org/History_Heuristic
// pub(crate) struct CaptureHistory([CaptureHistoryTable; Piece::COUNT * 2]);
#[derive(Debug)]
pub(crate) struct CaptureHistory(Vec<i16>);

impl Default for CaptureHistory {
    fn default() -> Self {
        Self(vec![0; Piece::COUNT * Square::TOTAL * Piece::TOTAL])
    }
}

impl CaptureHistory {
    /// From 'Stockfish'
    pub(crate) const fn malus(depth: u8) -> i32 {
        if depth < 4 { return 736 * (depth+1) as i32} else {2044}
    }

    /// From 'Stockfish'
    pub(crate) const fn bonus(depth: u8) -> i32 {
        let value = 190 * (depth as i16) - 298;
        if value < 20 { return 20; }
        if value > 1596 { return 1596 }
        return value as i32;
    }

    /// Convert 3D index to 1D
    /// (z * yMax * xMax) + (y * xMax) + x;
    /// https://stackoverflow.com/a/34363187/9347459
    const fn to_1_d_index(x: Piece, y: Square, z: PieceType) -> usize {
        ((z as usize) * Square::TOTAL * Piece::TOTAL) + ((y as usize) * Piece::COUNT) + x as usize
    }

    fn taper_bonus(prev: i32, value: i32) -> i16 {
        (prev + value - (prev * value.abs()) / MAX_HISTORY) as i16
    }

    pub(crate) fn update(&mut self, depth: u8, flag: HashFlag, attacker: Piece, tgt_sq: Square, victim: PieceType) {
        let index = Self::to_1_d_index(attacker, tgt_sq, victim);
        let prev_value = self.0.get_mut(index).unwrap();
        let bonus = if flag == HashFlag::LowerBound {Self::bonus(depth)} else {Self::malus(depth)};
        *prev_value = Self::taper_bonus((*prev_value) as i32, bonus);
    }

    pub(crate) fn get(&self, attacker: Piece, tgt_sq: Square, victim: PieceType) -> i16 {
        let index = Self::to_1_d_index(attacker, tgt_sq, victim);
        return *(unsafe { self.0.get_unchecked(index) })
    }
}