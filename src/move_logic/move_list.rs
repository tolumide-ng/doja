use std::{fmt::Display, ops::Deref};

use crate::{move_logic::bitmove::Move, board::piece::Piece};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct ScoredMove {mv: u16, score: u32}

#[derive(Debug, Clone, Copy)]
pub struct Moves {
    pub(crate) list: [Move; Self::SIZE],
    count: usize,
    /// Only used internally for the implementation of the iterator
    at: usize
}

impl Default for Moves {
    fn default() -> Self {
        Self { list: [Move::default(); Self::SIZE], count: 0, at: 0 }
    }
}

// pub(crate) const MOVE_SIZE: usize = 256;

impl Moves {
    pub(crate) const SIZE: usize = 256;

    /// Creates a new move list with 256 items all intiialized as 0(zero)
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Addsa  Move to the move list
    pub(crate) fn push(&mut self, m: Move) {
        self.list[self.count] = m;
        self.count+=1;
    }

    pub(crate) fn count_mvs(&self) -> usize {self.count}

    pub(crate) fn to_vec(self) -> Vec<Move> {
        self.list.into_iter().collect::<Vec<_>>()[..self.count].to_vec()
    }

    pub(crate) fn at(&self, index: usize) -> Option<&Move> {
        self.list.get(index)
    }
}


impl Iterator for Moves {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        if self.at < self.count {
            let current = Some(&self.list[self.at]);
            self.at += 1;
            return current.copied();
        }
        return None
    }
}


impl Display for Moves {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pieces = Piece::unicode_pieces();
        writeln!(f, " I/O |    move      | piece     | promotion   | capture   | double  | enpassant  | castling   |")?;
        writeln!(f, "------------------------------------------------------------------------------------------------")?;
        for count in 0..self.count {
            print!("   {count:2}");
            let l = self.list[count];
            print!("| ");
            print!("{}        ", l.to_string());
            let promotion = if let Some(p) = l.get_promotion() {pieces[p as usize]} else {' '};
            // |   {}       
            println!("
            |   {}         |  {:5}    | {:5}   | {:5}      | {:5}      |",
            //  pieces[l.get_piece()], 
             promotion, l.get_capture(), l.get_double_push(), l.get_enpassant(), l.get_castling());
        }
 
        writeln!(f, "\n\nTotal number of moves {} \n\n", self.count)?;

        Ok(())
    }
}
