// castling bits binary representation

use std::fmt::Display;

use bitflags::{bitflags, Flags};

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
        const WHITE_KING = 0b0001;
        const WHITE_QUEEN = 0b0010;
        const BLACK_KING = 0b0100;
        const BLACK_QUEEN = 0b1000;
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