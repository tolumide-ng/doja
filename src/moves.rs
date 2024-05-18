use std::fmt::Display;

use crate::{bit_move::NBitMove, board::piece::Piece};


pub struct Moves {
    list: [NBitMove; 256],
    count: usize
}

impl Default for Moves {
    fn default() -> Self {
        Self { list: [NBitMove::default(); 256], count: 0 }
    }
}

impl Moves {
    /// Creates a new move list with 256 items all intiialized as 0(zero)
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Addsa  bitmove to the move list
    pub(crate) fn add(&mut self, m: NBitMove) {
        self.list[self.count] = m;
        self.count+=1;
    }

    pub(crate) fn count(&self) -> usize {self.count}
}


impl Display for Moves {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pieces = Piece::unicode_pieces();
        println!("I/O | move  | piece     | promotion   | capture   | double | enpassant  | castling  |");
        println!("-------------------------------------------------------------------------------------");
        for count in 0..self.count {
            print!(" {count}  ");
            let l = self.list[count];
            print!("| ");
            print!("{} ", l.to_string());
            let promotion = if let Some(p) = l.get_promotion() {pieces[p]} else {' '};
            println!("|   {}       |   {}         |  {}    | {}   | {}      | {}      |", pieces[l.get_piece()], promotion, l.get_capture(), l.get_double_push(), l.get_enpassant(), l.get_castling());
        }
 
        println!("\n\nTotal number of moves {} \n\n", self.count);

        Ok(())
    }
}
