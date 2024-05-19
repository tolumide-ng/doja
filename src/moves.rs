use std::fmt::Display;

use crate::{bit_move::BitMove, board::piece::Piece};


pub struct Moves {
    pub(crate) list: [BitMove; 256],
    count: usize,
    /// Only used internally for the implementation of the iterator
    at: usize
}

impl Default for Moves {
    fn default() -> Self {
        Self { list: [BitMove::default(); 256], count: 0, at: 0 }
    }
}

impl Moves {
    /// Creates a new move list with 256 items all intiialized as 0(zero)
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Addsa  bitmove to the move list
    pub(crate) fn add(&mut self, m: BitMove) {
        self.list[self.count] = m;
        self.count+=1;
    }

    pub(crate) fn count(&self) -> usize {self.count}

    pub(crate) fn add_many(&mut self, m: &[BitMove]) {
        unsafe {
            let src_ptr = m.as_ptr();
            let dest_ptr = self.list.as_mut_ptr().add(self.count);
            std::ptr::copy_nonoverlapping(src_ptr, dest_ptr, m.len())
        };
        self.count += m.len();
    }
}


// impl Iterator for Moves {
//     type Item = BitMove;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.at < self.count {
//             let result = Some(&self.list[self.at]);
//             self.at += 1;
//             return result;
//         }
//         return None
//     }
// }


impl Display for Moves {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pieces = Piece::unicode_pieces();
        println!(" I/O |    move      | piece     | promotion   | capture   | double  | enpassant  | castling   |");
        println!("------------------------------------------------------------------------------------------------");
        for count in 0..self.count {
            print!("   {count:2}");
            let l = self.list[count];
            print!("| ");
            print!("{}        ", l.to_string());
            let promotion = if let Some(p) = l.get_promotion() {pieces[p]} else {' '};
            println!("|   {}       |   {}         |  {:5}    | {:5}   | {:5}      | {:5}      |", pieces[l.get_piece()], promotion, l.get_capture(), l.get_double_push(), l.get_enpassant(), l.get_castling());
        }
 
        println!("\n\nTotal number of moves {} \n\n", self.count);

        Ok(())
    }
}



// pub(crate) struct MoveIterator {
//     at: usize
// }

