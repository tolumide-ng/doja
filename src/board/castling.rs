// castling bits binary representation

use std::fmt::Display;

use bitflags::bitflags;

use crate::constants::{BLACK_KING_CASTLING_MASK, BLACK_QUEEN_CASTLING_MASK, WHITE_KING_CASTLING_MASK, WHITE_QUEEN_CASTLING_MASK};

bitflags! {
///  \
/// Casting bits binary representation \
///  \
/// binary      description \
/// 0001         1      white king can castle to the king side \
/// 0010         2      white king can castle to the queen side \
/// 0100         4      black king can castle to the king side \
/// 1000         8      black king can castle to the queen side \
///  \
/// examples \
/// 1111         both sides can castle both directions \
/// 1001         black king => queen side \
///              whitre king => king side \

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Castling: u8 {
        const WHITE_KING = WHITE_KING_CASTLING_MASK; // white can castle king side
        const WHITE_QUEEN = WHITE_QUEEN_CASTLING_MASK; // white can castle queen side
        const BLACK_KING = BLACK_KING_CASTLING_MASK; // black can castle king side
        const BLACK_QUEEN = BLACK_QUEEN_CASTLING_MASK; // black can castle queen side
        const ALL_WHITE = WHITE_KING_CASTLING_MASK | WHITE_QUEEN_CASTLING_MASK;
        const ALL_BLACK = BLACK_KING_CASTLING_MASK | BLACK_QUEEN_CASTLING_MASK;
        const NONE = 0b0000;
    }
}

impl Display for Castling {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut r = String::new();

        if self.contains(Castling::WHITE_KING) {
            r.push_str("K");
        } else {
            r.push_str("-")
        }

        if self.contains(Castling::WHITE_QUEEN) {
            r.push_str("Q");
        } else {
            r.push_str("-")
        }
        if self.contains(Castling::BLACK_KING) {
            r.push_str("k");
        } else {
            r.push_str("-")
        }
        if self.contains(Castling::BLACK_QUEEN) {
            r.push_str("q");
        } else {
            r.push_str("-")
        }

        write!(f, "{r}")
    }
}



impl From<u8> for Castling {
    fn from(value: u8) -> Self {
        Castling::from_bits_retain(value)
    }
}

impl From<&str> for Castling {
    fn from(value: &str) -> Self {
        let mut result = Self::NONE;


        let chars = value.chars().into_iter();

        chars.for_each(|c| {
            if c != '-' {
                match c {
                    'K' => result |= Castling::WHITE_KING,
                    'k' => result |= Castling::BLACK_KING,
                    'Q' => result |= Castling::WHITE_QUEEN,
                    'q' => result |= Castling::BLACK_QUEEN,
                    _ => panic!("Unrecognized castling character {c}")
                }
            }
        });
        result
    }
}


#[cfg(test)]
mod castling_tests {
    use crate::board::castling::Castling;

    #[test]
    fn should_convert_str_to_castling() {
        let black_king = "k";
        let white_king = "K";
        let black_queen = "q";
        let white_queen = "Q";

        assert_eq!(Castling::from(black_king), Castling::BLACK_KING);
        assert_eq!(Castling::from(white_king), Castling::WHITE_KING);
        assert_eq!(Castling::from(white_queen), Castling::WHITE_QUEEN);
        assert_eq!(Castling::from(black_queen), Castling::BLACK_QUEEN);

        let both_kings = format!("{}{}", black_king, white_king);
        assert_eq!(Castling::from(both_kings.as_str()), Castling::WHITE_KING | Castling::BLACK_KING);
        let white_queen_black_king = format!("{}{}", white_queen, black_king);
        let all_castlers = format!("{}{}{}{}", black_king, white_queen, white_king, black_queen);

        assert_eq!(Castling::from(white_queen_black_king.as_str()), Castling::WHITE_QUEEN | Castling::BLACK_KING);
        assert_eq!(Castling::from(all_castlers.as_str()), Castling::WHITE_KING | Castling::BLACK_KING | Castling::WHITE_QUEEN | Castling::BLACK_QUEEN);
    }

    #[test]
    #[should_panic(expected = "Unrecognized castling character p")]
    fn should_fail_if_the_castling_str_contains_invalid_letter() {
        let _ = Castling::from("p");
    }

    #[test]
    fn should_convert_u8_to_castling() {
        // black king can castle on the queen side
        assert_eq!(Castling::from(8), Castling::BLACK_QUEEN);
        // Black king can castle from the king side
        assert_eq!(Castling::from(4), Castling::BLACK_KING);
        // White king can castle from the king side
        assert_eq!(Castling::from(1), Castling::WHITE_KING);
        // White king can castle from the queen side
        assert_eq!(Castling::from(2), Castling::WHITE_QUEEN);

        // White king can castle on both sides
        assert_eq!(Castling::from(3), Castling::WHITE_KING | Castling::WHITE_QUEEN);

        // Black king can castle on both sides
        assert_eq!(Castling::from(12), Castling::BLACK_KING | Castling::BLACK_QUEEN);
    }

    #[test]
    fn should_convert_castling_to_string() {
        assert_eq!((Castling::BLACK_KING | Castling::WHITE_KING).to_string(), String::from("K-k-"));
        assert_eq!((Castling::WHITE_KING | Castling::BLACK_KING | Castling::WHITE_QUEEN | Castling::BLACK_QUEEN).to_string(), String::from("KQkq"));
        assert_eq!((Castling::BLACK_QUEEN | Castling::WHITE_KING | Castling::WHITE_QUEEN).to_string(), String::from("KQ-q"));

    }
}