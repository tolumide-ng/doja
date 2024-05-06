// castling bits binary representation

use bitflags::bitflags;

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
    struct Castling: u8 {
        const WHITE_KING = 0b0001;
        const WHITE_QUEEN = 0b0010;
        const BLACK_KING = 0b0100;
        const BLACK_QUEEN = 0b1000;
    }
}