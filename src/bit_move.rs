use std::{fmt::Display, ops::Deref};

use crate::{board::piece::Piece, squares::Square};


/**
 * binary represenation
 * 0000 0000 0000 0000 0011 1111    source square       0x3f
 * 0000 0000 0000 1111 1100 0000    target square       0xfc0
 * 0000 0000 1111 0000 0000 0000    piece               0xf000
 * 0000 1111 0000 0000 0000 0000    promoted piece      0xf0000
 * 0001 0000 0000 0000 0000 0000    capture flag        0x100000
 * 0010 0000 0000 0000 0000 0000    double push flag    0x200000
 * 0100 0000 0000 0000 0000 0000    enpassant           0x400000
 * 1000 0000 0000 0000 0000 0000    castling flag       0x800000
 * 
 */

const SOURCE_SQUARE: u32 = 0b0000_0000_0000_0011_1111;
const TARGET_SQUARE: u32 = 0b0000_0000_1111_1100_0000;
const PIECE: u32 = 0b0000_0000_1111_0000_0000_0000;
const PROMOTED_PIECE: u32 = 0b0000_1111_0000_0000_0000_0000;
const CAPTURED_PIECE: u32 = 0b0001_0000_0000_0000_0000_0000;
const DOUBLE_PUSH_FLAG: u32 = 0b0010_0000_0000_0000_0000_0000;
const ENPASSANT: u32 = 0b0100_0000_0000_0000_0000_0000;
const CATSLING: u32 = 0b1000_0000_0000_0000_0000_0000;



#[derive(Debug, Default, Clone, Copy)]
 pub struct NBitMove(u32);

 impl NBitMove {


    pub(crate) fn new(source: u32, target: u32, piece: Piece, promotion: Option<Piece>, capture: bool, double_push: bool, enpassant: bool, castling: bool) -> Self {
        let promotion_piece = if let Some(p) = promotion {p as u32} else {0};

        let bmove = source | target << 6 | (piece as u32) << 12 | promotion_piece << 16 | 
            (capture as u32) << 20 |(double_push as u32) << 21 | (enpassant as u32) << 22 | (castling as u32) << 23;
        NBitMove(bmove)
    }

    pub(crate) fn get_src(&self) -> Square {
        let sq = (**self & SOURCE_SQUARE) as u64;
        Square::from(sq)
    }

    pub(crate) fn get_target(&self) -> Square {
        let sq = ((**self & TARGET_SQUARE) >> 6) as u64;
        Square::from(sq)
    }

    pub(crate) fn get_piece(&self) -> Piece {
        let value = ((**self & PIECE) >> 12) as u8;
        Piece::from(value)
    }

    pub(crate) fn get_promotion(&self) -> Piece {
        let value = ((**self & PROMOTED_PIECE) >> 16) as u8;
        Piece::from(value)
    }

    pub(crate) fn get_capture(&self) -> bool {
        let capture = (**self & CAPTURED_PIECE) != 0;
        capture
    }

    pub(crate) fn get_double_push(&self) -> bool {
        let capture = (**self & DOUBLE_PUSH_FLAG) != 0;
        capture
    }

    pub(crate) fn get_enpassant(&self) -> bool {
        let enpass = (**self & ENPASSANT) != 0;
        enpass
    }

    pub(crate) fn get_castling(&self) -> bool {
        **self & CATSLING != 0
    }
 }



/// for UCI purpose 
 impl Display for NBitMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let src = self.get_src().to_string();
        let target = self.get_target().to_string();
        let promotion = match self.get_promotion() {
            Piece::WP | Piece::BP => String::new(),
            x => x.to_string().to_lowercase()
        };

        print!("{src}{target}{promotion}");
        
        Ok(())
    }
 }

 impl Deref for NBitMove {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
 }


 impl From<u32> for NBitMove {
    fn from(value: u32) -> Self {
        Self(value)
    }
 }

 impl From<NBitMove> for u32 {
    fn from(value: NBitMove) -> Self {
        *value
    }
 }


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