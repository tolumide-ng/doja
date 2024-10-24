use std::{fmt::Display, ops::Deref};

use crate::{board::piece::PieceType, squares::Square};


const SOURCE_SQUARE: u16 = 0b0000_0000_0011_1111;
const TARGET_SQUARE: u16 = 0b0000_1111_1100_0000;
const MOVE_TYPE: u16 = 0b1111_0000_0000_0000;

/**
 * binary representation
 * 0000 0000 0011 1111      source square 0x3f
 * 0000 1111 11xx xxxx      target square 0xfc0
 * 0001 xxxx xxxx xxxx      quiet  move
 * 0001 xxxx xxxx xxxx      capture
 * 0010 xxxx xxxx xxxx      enpassant
 * 0011 xxxx xxxx xxxx      double push
 * 0100 xxxx xxxx xxxx      castling
 * 0101 xxxx xxxx xxxx      Promotion to Knight
 * 0110 xxxx xxxx xxxx      Promotion to Bishop
 * 0111 xxxx xxxx xxxx      Promotion to Rook
 * 1000 xxxx xxxx xxxx      Promotion to Queen
 * 1001 xxxx xxxx xxxx      Captures and Promotes to Knight
 * 1010 xxxx xxxx xxxx      Captures and Promotes to Bishop
 * 1011 xxxx xxxx xxxx      Captures and Promotes to Rook
 * 1100 xxxx xxxx xxxx      Captures and Promotes to Queen
 */


 const SQUARE_OFFSET: u64 = 12;
 const MOVE_TYPE_MASK: u16 = 0b1111_0000_0000_0000;


#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum MoveType {
    Quiet = 0b0000,
    Capture = 0b0001,
    Enpassant = 0b0010,
    DoublePush = 0b0011,
    Castling = 0b0100,
    PromotedToKnight = 0b0101,
    PromotedToBishop = 0b0110,
    PromotedToRook = 0b0111,
    PromotedToQueen = 0b1000,
    CaptureAndPromoteToKnight = 0b1001,
    CaptureAndPromoteToBishop = 0b1010,
    CaptureAndPromoteToRook = 0b1011,
    CaptureAndPromoteToQueen = 0b1100,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Move(u16);

impl Move {
     pub(crate) fn new(src: u8, tgt: u8, variant: MoveType) -> Self {
         let bmove = src as u16 | (tgt as u16) << 6 | (variant as u16) << 12;
         Self(bmove)
     }
     
    pub(crate) fn get_src(&self) -> Square {
        let sq = (**self & SOURCE_SQUARE) as u64;
        Square::from(sq)
    }

    pub(crate) fn get_target(&self) -> Square {
        let sq = ((**self & TARGET_SQUARE) >> 6) as u64;
        Square::from(sq)
    }

    pub(crate) fn get_promotion(&self) -> Option<PieceType> {
        let value = ((**self & MOVE_TYPE) >> 12) as u8;

        match value {
            0b0101 | 0b1001 => Some(PieceType::N),
            0b0110 | 0b1010 => Some(PieceType::B),
            0b0111 | 0b1011 => Some(PieceType::R),
            0b1000 | 0b1100 => Some(PieceType::Q),
            _ => None
        }
    }

    pub(crate) fn get_capture(&self) -> bool {
        [MoveType::Capture, MoveType::CaptureAndPromoteToBishop, MoveType::CaptureAndPromoteToKnight, MoveType::CaptureAndPromoteToQueen, MoveType::CaptureAndPromoteToRook, 
            MoveType::Enpassant, // only updated recently (confirm tests) 12/10/24
        ].contains(&self.move_type())
    }

    pub(crate) fn get_double_push(&self) -> bool {
        let value = (**self >> SQUARE_OFFSET) & (MoveType::DoublePush as u16);
        value == MoveType::DoublePush as u16
    }

    pub(crate) fn get_enpassant(&self) -> bool {
        self.move_type() == MoveType::Enpassant
        // let value = (**self  >> SQUARE_OFFSET) & (MoveType::Enpassant as u16);
        // value != 0
    }

    pub(crate) fn get_castling(&self) -> bool {
        // let value = (**self  >> SQUARE_OFFSET) & (MoveType::Castling as u16);
        // value != 0
        self.move_type() == MoveType::Castling
    }

    pub(crate) fn move_type(&self) -> MoveType {
        let value = (**self & MOVE_TYPE_MASK) >> SQUARE_OFFSET;

        match value {
            0b0000 => MoveType::Quiet,
            0b0001 => MoveType::Capture,
            0b0010 => MoveType::Enpassant,
            0b0011 => MoveType::DoublePush,
            0b0100 => MoveType::Castling,
            0b0101 => MoveType::PromotedToKnight,
            0b0110 => MoveType::PromotedToBishop,
            0b0111 => MoveType::PromotedToRook,
            0b1000 => MoveType::PromotedToQueen,
            0b1001 => MoveType::CaptureAndPromoteToKnight,
            0b1010 => MoveType::CaptureAndPromoteToBishop,
            0b1011 => MoveType::CaptureAndPromoteToRook,
            0b1100 => MoveType::CaptureAndPromoteToQueen,
            _ => panic!("Unrecognized MoveType {value}")
        }
    }
 }



/// for UCI purpose 
 impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let src = self.get_src().to_string();
        let target = self.get_target().to_string();
        let promotion = self.get_promotion().map(|x| x.to_string().to_lowercase());
        // let promotion = self.get_promotion().map(|x| x.to_string().to_lowercase()).or(Some(String::from(" ")));

        let mut str = format!("{src}{target}");
        
        if let Some(promoted_to) = promotion {
            str.push_str(&promoted_to);
        }

        if self.get_capture() {
            str.push_str("x");
        }

        return write!(f, "{str}");

    }
 }

 impl Deref for Move {
    type Target = u16;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
 }


 impl From<u16> for Move {
    fn from(value: u16) -> Self {
        Self(value)
    }
 }

 impl From<Move> for u16 {
    fn from(value: Move) -> Self {
        *value
    }
 }


 #[cfg(test)]
 mod move_tests {
    use crate::squares::Square;
    use super::MoveType::*;

    use super::*;

    #[test]
    fn should_return_a_valid_u32_after_creaton() {
        let bmove = Move::new(0, 9, Capture);
        assert_eq!(0x1240, *bmove);
    }

    #[test]
    fn should_return_data_stored_in_the_move_for_a_queen_capture() {
        let queen_capture =  Move::new(12, 28, Capture);
        assert_eq!(*queen_capture, 0x170C);

        assert_eq!(queen_capture.get_capture(), true);
        assert_eq!(queen_capture.get_castling(), false);
        assert_eq!(queen_capture.get_enpassant(), false);
        assert_eq!(queen_capture.get_promotion(), None);
        assert_eq!(queen_capture.get_target(), Square::from(28u8));
        assert_eq!(queen_capture.get_src(), Square::from(12u8));
        assert_eq!(queen_capture.move_type(), Capture);
    }


    #[test]
    fn should_return_stored_data_for_a_promoted_pawn() {
        let pawn_to_bishop = Move::new(12, 5, CaptureAndPromoteToBishop);

        assert_eq!(pawn_to_bishop.get_src(), Square::from(12u8));
        assert_eq!(pawn_to_bishop.get_target(), Square::from(5u8));
        assert_eq!(pawn_to_bishop.get_promotion(), Some(PieceType::B));
        assert_eq!(pawn_to_bishop.get_capture(), true);
        assert_eq!(pawn_to_bishop.get_enpassant(), false);
        assert_eq!(pawn_to_bishop.get_castling(), false);
        assert_eq!(pawn_to_bishop.move_type(), CaptureAndPromoteToBishop);
    }


    #[test]
    fn should_return_stored_data_for_a_castling_move() {
        let castling_move = Move::new(4, 2, Castling);

        assert_eq!(castling_move.get_src(), Square::from(4u8));
        assert_eq!(castling_move.get_target(), Square::from(2u8));
        assert_eq!(castling_move.get_promotion(), None);
        assert_eq!(castling_move.get_capture(), false);
        assert_eq!(castling_move.get_enpassant(), false);
        assert_eq!(castling_move.get_castling(), true);
        assert_eq!(castling_move.move_type(), Castling);
    }

    #[test]
    fn should_return_stored_data_for_a_double_push() {
        let double_push = Move::new(12, 26, DoublePush);

        assert_eq!(double_push.get_src(), Square::from(12u8));
        assert_eq!(double_push.get_target(), Square::from(26u8));
        assert_eq!(double_push.get_promotion(), None);
        assert_eq!(double_push.get_capture(), false);
        assert_eq!(double_push.get_enpassant(), false);
        assert_eq!(double_push.get_castling(), false);
        assert_eq!(double_push.move_type(), DoublePush);
    }

    #[test]
    fn should_return_stored_data_for_an_enpassant() {
        let enpassant = Move::new(20, 12, Enpassant);

        assert_eq!(enpassant.get_src(), Square::from(20u8));
        assert_eq!(enpassant.get_target(), Square::from(12u8));
        assert_eq!(enpassant.get_promotion(), None);
        assert_eq!(enpassant.get_capture(), true);
        assert_eq!(enpassant.get_enpassant(), true);
        assert_eq!(enpassant.get_castling(), false);
        assert_eq!(enpassant.move_type(), MoveType::Enpassant);
    }
 }