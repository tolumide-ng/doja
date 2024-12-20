use std::{fmt::Display, ops::{Deref, DerefMut}};

use crate::board::piece::Piece;
use crate::bitboard::Bitboard;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PieceMap([Bitboard; 12]);

impl Deref for PieceMap {
    type Target = [Bitboard; 12];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PieceMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0   
    }
}


impl PieceMap {
    pub fn new() -> Self {
        Self([Bitboard::new(); 12])
    }

    pub(crate) fn get(&self, index: usize) -> &Bitboard {
        &self.0[index]
    }
}



impl Display for PieceMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // let pieces = Piece::unicode_pieces();
        let pieces = Piece::ascii_pieces();

        writeln!(f, "")?;
        for rank in (0..8).rev() {
            for file in 0..8 {
                if file == 0 {
                    write!(f, "  {} ", rank+1)?;
                }
                let square = rank * 8 + file;
                let mut piece = '\u{002E}';
                
                // loop over all piece bitboards
                for bitboard_piece in pieces {
                    // bitboard belonging to the current piece
                    let bitboard =  self[bitboard_piece as usize];
                    if bitboard.get_bit(square) != 0 {
                        piece = bitboard_piece.into();
                    }
                }
                
                write!(f, " {piece} ")?;
            }
            writeln!(f, "")?;
        }
        writeln!(f, "    \n     a  b  c  d  e  f  g  h\n")?;
        
        Ok(())
    }
}

