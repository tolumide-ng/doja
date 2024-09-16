use crate::{board::piece::Piece, squares::Square};


/// HalfKP: (our_king_square, piece_square, piece_type, color)
/// 64 * 64 * 5 * 2 = 40960
pub(crate) fn feature_index(piece: Piece, piece_sq: Square, king_sq: Square) -> usize {
    let p_idx = piece as usize * 2 + piece.color() as usize;
    let halfkp_idx = piece_sq as usize + (p_idx + king_sq as usize * 10) * 64;
    halfkp_idx
}