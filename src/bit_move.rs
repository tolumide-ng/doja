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
 * 
 * TODO! CHANGE THIS TO 64bits, SO THAT WE CAN STORE THE CAPTURED PIECE TOO
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
 pub struct BitMove(u32);

 impl BitMove {


    pub(crate) fn new(source: u32, target: u32, piece: Piece, promotion: Option<Piece>, capture: bool, double_push: bool, enpassant: bool, castling: bool) -> Self {
        // println!("source==={source} target=={target}");
        let promotion_piece = if let Some(p) = promotion {p as u32} else {0};

        let bmove = source | target << 6 | (piece as u32) << 12 | promotion_piece << 16 | 
            (capture as u32) << 20 |(double_push as u32) << 21 | (enpassant as u32) << 22 | (castling as u32) << 23;
        BitMove(bmove)
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

    pub(crate) fn get_promotion(&self) -> Option<Piece> {
        let value = ((**self & PROMOTED_PIECE) >> 16) as u8;

        match Piece::from(value) {
            Piece::WP | Piece::BP => None,
            x => Some(x)
        }
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
 impl Display for BitMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let src = self.get_src().to_string();
        let target = self.get_target().to_string();
        let promotion = self.get_promotion().map(|x| x.to_string().to_lowercase());
        // let promotion = self.get_promotion().map(|x| x.to_string().to_lowercase()).or(Some(String::from(" ")));

        if let Some(promoted_to) = promotion {
            return write!(f, "{src}{target}{}", promoted_to);
        }

        return write!(f, "{src}{target}");

    }
 }

 impl Deref for BitMove {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
 }


 impl From<u32> for BitMove {
    fn from(value: u32) -> Self {
        Self(value)
    }
 }

 impl From<BitMove> for u32 {
    fn from(value: BitMove) -> Self {
        *value
    }
 }