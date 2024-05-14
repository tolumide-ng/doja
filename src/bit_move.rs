use crate::{board::piece::Piece, squares::Square};

pub struct BitMove(u16);


impl BitMove {
    /// Piece indicates whatever this piece was promoted to, or what it remains as
    pub(crate) fn new(src: u16, target: u16, piece: Piece) -> Self {
        // print!("the src, and t {} ... {} |||| ", src, target);
        let src_bits = src & 0b0011_1111;
        let target_bits = (target & 0b0011_1111) << 6;
        let piece_bits = ((piece as u16) & 0b0011_1111) << 12;
        // println!("src: {src_bits:0b}, target: {target_bits:0b}, promotion: {piece_bits:0b}");
        
        Self(src_bits | target_bits | piece_bits)
    }

    pub(crate) fn get_src(&self) -> Square  {
        let src = (self.0 & 0b0011_1111) as u64;
        Square::from(src)
    }

    pub(crate) fn get_target(&self) -> Square {
        let target = ((self.0 >> 6) & 0b0011_1111) as u64;
        Square::from(target)
    }

    pub(crate) fn get_piece(&self) -> Piece {
        let value = ((self.0 >> 12) & 0b0011_1111) as u8;
        Piece::from(value)
    }
}