#[derive(Debug, Clone, Copy, derive_more::Display)]
pub enum Piece {
    /// white pawn
    #[display(fmt="P")]
    WP, 
    /// white knight
    #[display(fmt="N")]
    WN, 
    /// white bishop
    #[display(fmt="B")]
    WB, 
    /// white rook
    #[display(fmt="R")]
    WR,
    /// white queen
    #[display(fmt="Q")]
    WQ,
    /// white king
    #[display(fmt="K")]
    WK,
    /// black pawn
    #[display(fmt="p")]
    BP, 
    /// black knight
    #[display(fmt="n")]
    BN, 
    /// black bishop
    #[display(fmt="b")]
    BB, 
    /// black rook
    #[display(fmt="r")]
    BR,
    /// black queen
    #[display(fmt="q")]
    BQ,
    /// black king
    #[display(fmt="k")]
    BK,
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

impl From<&str> for Piece {
    fn from(value: &str) -> Self {
        match value {
            "P" => Piece::WP,
            "N" => Piece::WN,
            "B" => Piece::WB,
            "R" => Piece::WR,
            "Q" => Piece::WQ,
            "K" => Piece::WK,
            "p" => Piece::BP,
            "n" => Piece::BN,
            "b" => Piece::BB,
            "r" => Piece::BR,
            "q" => Piece::BQ,
            "k" => Piece::BK,
            _ => unimplemented!()
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
}