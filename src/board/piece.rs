use std::ops::{Index, IndexMut};

use crate::color::Color;

#[derive(Debug, Clone, Copy, derive_more::Display)]
pub enum Piece {
    /// white pawn
    #[display(fmt="P")]
    WP = 0, 
    /// white knight
    #[display(fmt="N")]
    WN = 1,
    /// white bishop
    #[display(fmt="B")]
    WB = 2,
    /// white rook
    #[display(fmt="R")]
    WR = 3,
    /// white queen
    #[display(fmt="Q")]
    WQ = 4,
    /// white king
    #[display(fmt="K")]
    WK = 5,
    /// black pawn
    #[display(fmt="p")]
    BP = 6, 
    /// black knight
    #[display(fmt="n")]
    BN = 7, 
    /// black bishop
    #[display(fmt="b")]
    BB = 8, 
    /// black rook
    #[display(fmt="r")]
    BR = 9,
    /// black queen
    #[display(fmt="q")]
    BQ = 10,
    /// black king
    #[display(fmt="k")]
    BK = 11,
}

impl From<u8> for Piece {
    fn from(value: u8) -> Self {
        match value {
            0 => Piece::WP,
            1 => Piece::WN,
            2 => Piece::WB,
            3 => Piece::WR,
            4 => Piece::WQ,
            5 => Piece::WK,
            6 => Piece::BP,
            7 => Piece::BN,
            8 => Piece::BB,
            9 => Piece::BR,
            10 => Piece::BQ,
            11 => Piece::BK,
            _ => panic!("Unexpected Piece value")
        }
    }
}

impl<T> Index<Piece> for [T] {
    type Output = T;

    fn index(&self, index: Piece) -> &Self::Output {
        match index {
            Piece::WP => &self[0],
            Piece::WN => &self[1],
            Piece::WB => &self[2],
            Piece::WR => &self[3],
            Piece::WQ => &self[4],
            Piece::WK => &self[5],
            Piece::BP => &self[6],
            Piece::BN => &self[7],
            Piece::BB => &self[8],
            Piece::BR => &self[9],
            Piece::BQ => &self[10],
            Piece::BK => &self[11],
        }
    }
}

impl<T> IndexMut<Piece> for [T] {
    fn index_mut(&mut self, index: Piece) -> &mut Self::Output {
        match index {
            Piece::WP => &mut self[0],
            Piece::WN => &mut self[1],
            Piece::WB => &mut self[2],
            Piece::WR => &mut self[3],
            Piece::WQ => &mut self[4],
            Piece::WK => &mut self[5],
            Piece::BP => &mut self[6],
            Piece::BN => &mut self[7],
            Piece::BB => &mut self[8],
            Piece::BR => &mut self[9],
            Piece::BQ => &mut self[10],
            Piece::BK => &mut self[11],
        }
    }
}


impl From<Piece> for usize {
    fn from(value: Piece) -> Self {
        match value {
            Piece::WP => 0,
            Piece::WN => 1,
            Piece::WB => 2,
            Piece::WR => 3,
            Piece::WQ => 4,
            Piece::WK => 5,
            Piece::BP => 6,
            Piece::BN => 7,
            Piece::BB => 8,
            Piece::BR => 9,
            Piece::BQ => 10,
            Piece::BK => 11,
        }
    }
}

impl From<char> for Piece {
    fn from(value: char) -> Self {
        match value {
            'P' => Piece::WP,
            'N' => Piece::WN,
            'B' => Piece::WB,
            'R' => Piece::WR,
            'Q' => Piece::WQ,
            'K' => Piece::WK,
            'p' => Piece::BP,
            'n' => Piece::BN,
            'b' => Piece::BB,
            'r' => Piece::BR,
            'q' => Piece::BQ,
            'k' => Piece::BK,
            _ => panic!("Invalid Piece character provide {value}")
        }
    }
}

impl From<Piece> for char {
    fn from(value: Piece) -> Self {
        match value {
            Piece::WP => '\u{2659}',
            Piece::WN => '\u{2658}',
            Piece::WB => '\u{2657}',
            Piece::WR => '\u{2656}',
            Piece::WQ => '\u{2655}',
            Piece::WK => '\u{2654}',
            Piece::BP => '\u{265F}',
            Piece::BN => '\u{265E}',
            Piece::BB => '\u{265D}',
            Piece::BR => '\u{265C}',
            Piece::BQ => '\u{265B}',
            Piece::BK => '\u{265A}',
        }
    }
}


impl Piece {
    pub fn ascii_pieces() -> [Piece; 12] {
        [
            Piece::WP, Piece::WN, Piece::WB, Piece::WR, Piece::WQ, Piece::WK, Piece::BP, Piece::BN, Piece::BB, Piece::BR, Piece::BQ, Piece::BK, 
        ]
    }

    pub fn unicode_pieces() -> [char; 12] {
        [
            '\u{2659}',
            '\u{2658}',
            '\u{2657}',
            '\u{2656}',
            '\u{2655}',
            '\u{2654}',
            '\u{265F}',
            '\u{265E}',
            '\u{265D}',
            '\u{265C}',
            '\u{265B}',
            '\u{265A}',
        ]
    }

    pub(crate) fn white_pieces() -> [Piece; 6] {
        [Piece::WP, Piece::WN, Piece::WB, Piece::WR, Piece::WQ, Piece::WK]
    }

    pub(crate) fn black_pieces() -> [Piece; 6] {
        [ Piece::BP, Piece::BN, Piece::BB, Piece::BR, Piece::BQ, Piece::BK ]
    }

    pub(crate) fn queen(color: Color) -> Piece {
        if color == Color::Black { return Piece::BQ }
        return Piece::WQ;
    }

    pub(crate) fn knight(color: Color) -> Piece {
        if color == Color::Black { return Piece::BN }
        return Piece::WN;
    }

    pub(crate) fn bishop(color: Color) -> Piece {
        if color == Color::Black { return Piece::BB }
        return Piece::WB;
    }

    pub(crate) fn rook(color: Color) -> Piece {
        if color == Color::Black { return Piece::BR }
        return Piece::WR;
    }

    pub(crate) fn pawn(color: Color) -> Piece {
        if color == Color::Black { return Piece::BP }
        return Piece::WP;
    }

    pub(crate) fn king(color: Color) -> Piece {
        if color == Color::Black { return Piece::BK }
        return Piece::WK;
    }
}