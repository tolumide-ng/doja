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



/// todo! change this to a macro, include validation of the move in this.
/// e.g. a white_pawn_cannot_be_promoted_to_a_black_queen e.t.c
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
 pub struct Move(u32);

 impl Move {


    /// enpassant? to know whether this is a move trying to take advantage of an existing enpassant (that resulted form a doublepush) on the board
    pub(crate) fn new(source: u32, target: u32, piece: Piece, promotion: Option<Piece>, capture: bool, double_push: bool, enpassant: bool, castling: bool) -> Self {
        // println!("source==={source} target=={target}");
        let promotion_piece = if let Some(p) = promotion {p as u32} else {0};

        let bmove = source | target << 6 | (piece as u32) << 12 | promotion_piece << 16 | 
            (capture as u32) << 20 |(double_push as u32) << 21 | (enpassant as u32) << 22 | (castling as u32) << 23;
        Move(bmove)
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
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
 }


 impl From<u32> for Move {
    fn from(value: u32) -> Self {
        Self(value)
    }
 }

 impl From<Move> for u32 {
    fn from(value: Move) -> Self {
        *value
    }
 }


 #[cfg(test)]
 mod move_tests {
    use crate::{board::piece::Piece, squares::Square};

    use super::Move;

    #[test]
    fn should_return_a_valid_u32_after_creaton() {
        let bmove = Move::new(0, 9, Piece::WP, None, true, false, false, false);
        assert_eq!(1049152, *bmove);
    }

    #[test]
    fn should_return_data_stored_in_the_move_for_a_queen_capture() {
        let queen_capture =  Move::new(12, 28, Piece::BQ, None, true, false, false, false);

        assert_eq!(queen_capture.get_capture(), true);
        assert_eq!(queen_capture.get_castling(), false);
        assert_eq!(queen_capture.get_double_push(), false);
        assert_eq!(queen_capture.get_enpassant(), false);
        assert_eq!(queen_capture.get_promotion(), None);
        assert_eq!(queen_capture.get_target(), Square::from(28));
        assert_eq!(queen_capture.get_src(), Square::from(12));
    }


    #[test]
    fn should_return_stored_data_for_a_promoted_pawn() {
        let pawn_to_bishop = Move::new(12, 5, Piece::BP, Some(Piece::BB), true, false, false, false);

        assert_eq!(pawn_to_bishop.get_src(), Square::from(12));
        assert_eq!(pawn_to_bishop.get_target(), Square::from(5));
        assert_eq!(pawn_to_bishop.get_piece(), Piece::BP);
        assert_eq!(pawn_to_bishop.get_promotion(), Some(Piece::BB));
        assert_eq!(pawn_to_bishop.get_capture(), true);
        assert_eq!(pawn_to_bishop.get_double_push(), false);
        assert_eq!(pawn_to_bishop.get_enpassant(), false);
        assert_eq!(pawn_to_bishop.get_castling(), false);
    }


    #[test]
    fn should_return_stored_data_for_a_castling_move() {
        let castling_move = Move::new(4, 2, Piece::WK, None,  false, false, false, true);

        assert_eq!(castling_move.get_src(), Square::from(4));
        assert_eq!(castling_move.get_target(), Square::from(2));
        assert_eq!(castling_move.get_piece(), Piece::WK);
        assert_eq!(castling_move.get_promotion(), None);
        assert_eq!(castling_move.get_capture(), false);
        assert_eq!(castling_move.get_double_push(), false);
        assert_eq!(castling_move.get_enpassant(), false);
        assert_eq!(castling_move.get_castling(), true);
    }

    #[test]
    fn should_return_stored_data_for_a_double_push() {
        let double_push = Move::new(12, 26, Piece::WP, None,  false, true, false, false);

        assert_eq!(double_push.get_src(), Square::from(12));
        assert_eq!(double_push.get_target(), Square::from(26));
        assert_eq!(double_push.get_piece(), Piece::WP);
        assert_eq!(double_push.get_promotion(), None);
        assert_eq!(double_push.get_capture(), false);
        assert_eq!(double_push.get_double_push(), true);
        assert_eq!(double_push.get_enpassant(), false);
        assert_eq!(double_push.get_castling(), false);
    }

    #[test]
    fn should_return_stored_data_for_an_enpassant() {
        let enpassant = Move::new(20, 12, Piece::BP, None,  true, false, true, false);

        assert_eq!(enpassant.get_src(), Square::from(20));
        assert_eq!(enpassant.get_target(), Square::from(12));
        assert_eq!(enpassant.get_piece(), Piece::BP);
        assert_eq!(enpassant.get_promotion(), None);
        assert_eq!(enpassant.get_capture(), true);
        assert_eq!(enpassant.get_double_push(), false);
        assert_eq!(enpassant.get_enpassant(), true);
        assert_eq!(enpassant.get_castling(), false);
    }
 }