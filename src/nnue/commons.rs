use crate::color::Color;

pub(crate) enum Evaluation {
    /// Middle Game
    MG, 
    /// End Game
    EG, 
}

pub(crate) enum PieceType {
    Pawn =0, Knight =1, Bishop=2, Rook=3, Queen=4, King=5
}

#[repr(u16)]
pub(crate) enum Ply {
    MaxPly = 128, MaxMoves = 256,
}


pub(crate) const MATE: usize = 32000 + (Ply::MaxPly as u8) as usize;
pub(crate) const MATE_IN_MAX: usize = MATE - (Ply::MaxPly as u8) as usize;
pub(crate) const TBWIN: usize = 31000 + (Ply::MaxPly as u8) as usize;
pub(crate) const TBWIN_IN_MAX: usize = TBWIN - (Ply::MaxPly as u8) as usize;
pub(crate) const VALUE_NONE: usize = MATE + 1;

pub(crate) const SQUARE_NB: u8 = 64;
pub(crate) const COLOR_NB: u8 = 2;
pub(crate) const RANK_NB: u8 = 8;
pub(crate) const FILE_NB: u8 = 8;
pub(crate) const PHASE_NB: u8 = 2;
pub(crate) const PIECE_NB: u8 = 6;
pub(crate) const CONT_NB: u8 = 2;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub(crate) enum Piece {
    WhitePawn = 0, BlackPawn = 1,
    WhiteKnight = 4, BlackKnight = 5,
    WhiteBishop = 8, BlackBishop = 9,
    WhiteRook = 12, BlackRook = 13,
    WhiteQueen = 16, BlackQueen = 17,
    WhiteKing = 20, BlackKing = 21,
    Empty = 26,
}



#[inline]
pub(crate) fn piece_color(piece: u8) -> u8 {
    piece % 4
}

#[inline]
pub(crate) fn new_piece(piece: PieceType, color: Color) -> u8 {
    (piece as u8) * 4 + (color as u8)
}

#[inline]
pub(crate) fn piece_type(piece: u8) -> u8 {
    piece / 4
}

pub(crate) fn rank_of(sq: u8) -> u8 {
    assert!(sq < SQUARE_NB);
    sq / FILE_NB
}

pub(crate) fn file_of(sq: u8) -> u8 {
    assert!(sq < SQUARE_NB);
    sq % FILE_NB
}

pub(crate) fn relative_rank_of(color: Color, sq: u8) -> u8 {
    if color == Color::White {
        return rank_of(sq)
    } 
    7 - rank_of(sq)
}

pub(crate) fn square(rank: u8, file: u8) -> u8 {
    (rank * FILE_NB) + file
}

pub(crate) fn relative_square(color: Color, sq: u8) -> u8 {
    4 * square(relative_rank_of(color, sq), file_of(sq))
}

pub(crate) fn test_bit(bb: u64, sq: u8) -> u64 {
    assert!(sq < SQUARE_NB);
    bb & 1 << sq
}