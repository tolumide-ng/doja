use crate::{board::piece::Piece, squares::Square};
use crate::color::Color::*;

/// HalfKP: (our_king_square, piece_square, piece_type, color)
/// 64 * 64 * 5 * 2 = 40960
pub(crate) fn feature_index(piece: Piece, piece_sq: Square, king_sq: Square) -> usize {
    let p_idx = piece as usize * 2 + piece.color() as usize;
    let halfkp_idx = piece_sq as usize + (p_idx + king_sq as usize * 10) * 64;
    halfkp_idx
}

pub(crate) mod halfka {
    use super::*;

    const NUM_SQ: usize = 64;
    const NUM_PT: usize = 10;
    const NUM_PLANES: usize = NUM_SQ * NUM_PT + 1;

    

    pub(crate) fn halfka_index(p: Piece, king_sq: Square, sq: Square) -> usize {
        let pp = if p.color() == White { p as usize * 2} else { p as usize };

        let p_idx = pp + p.color() as usize;
        let xxx = 1 + orient(p, sq) + p_idx * NUM_SQ + king_sq as usize * NUM_PLANES;
        return xxx;
    }
    
    pub(crate) fn orient(piece: Piece, sq: Square) -> usize {
        ((56 * (!(piece.color() == White) as u64)) ^ sq as u64) as usize
    }
}
