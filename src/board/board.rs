use std::{fmt::Display, ops::{Deref, DerefMut}};

use crate::{board::piece::Piece, Bitboard};

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
        let pieces = Piece::unicode_pieces();
        
        for rank in 0..8 {
            for file in 0..8 {
                if file == 0 {
                    print!("{}  ", 8-rank)
                }
                // square
                let square = (rank * 8 + file) as usize;
                let piece = -1;

                let char = if piece == -1 {'\u{002E}'} else {pieces[square]};

                print!(" {} ", char)
            }
            println!("");
        }
        println!("    \n    a  b  c  d  e  f  g  h\n");
        write!(f, "")
    }
}

