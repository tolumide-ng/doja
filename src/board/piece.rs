#[derive(Debug, Clone, Copy, derive_more::Display)]
pub enum Piece {
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
}

impl From<&str> for Piece {
    fn from(value: &str) -> Self {
        match value {
            "p" => Piece::BP,
            "n" => Piece::BN,
            "b" => Piece::BB,
            "r" => Piece::BR,
            "q" => Piece::BQ,
            "k" => Piece::BK,
            "P" => Piece::WP,
            "N" => Piece::WN,
            "B" => Piece::WB,
            "R" => Piece::WR,
            "Q" => Piece::WQ,
            "K" => Piece::WK,
            _ => unimplemented!()
        }
    }
}


impl Piece {
    pub fn ascii_pieces() -> [Piece; 12] {
        [
            Piece::BP, Piece::BN, Piece::BB, Piece::BR, Piece::BQ, Piece::BK, Piece::WP, Piece::WN, Piece::WB, Piece::WR, Piece::WQ, Piece::WK,
        ]
    }

    pub fn unicode_pieces() -> [char; 12] {
        [
            '\u{265A}',
            '\u{265B}',
            '\u{265C}',
            '\u{265D}',
            '\u{265E}',
            '\u{265F}',
            '\u{2654}',
            '\u{2655}',
            '\u{2656}',
            '\u{2657}',
            '\u{2658}',
            '\u{2659}',
        ]
    }
}