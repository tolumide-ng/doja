// castling bits binary representation

use std::fmt::Display;

use bitflags::{bitflags, Flags};

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