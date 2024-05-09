use std::{fmt::Display, ops::{Deref, DerefMut}};

use crate::{board::piece::Piece, color::Color, king::King, knight::Knight, squares::Square, Bitboard};

pub struct Board([Bitboard; 12]);

impl Deref for Board {
    type Target = [Bitboard; 12];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Board {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0   
    }
}


impl Board {
    pub fn new() -> Self {
        Self([Bitboard::new(); 12])
    }

    pub(crate) fn get(&self, index: usize) -> &Bitboard {
        &self.0[index]
    }
}



impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // let pieces = Piece::unicode_pieces();
        let pieces = Piece::ascii_pieces();


        println!("");
        for rank in 0..8 {
            for file in 0..8 {
                if file == 0 {
                    print!("{}  ", 8-rank)
                }
                // square
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
                
                print!(" {} ", piece)
            }
            println!("");
        }
        println!("    \n    a  b  c  d  e  f  g  h\n");

        // println!("\n {}", self.0);
        Ok(())
    }
}

