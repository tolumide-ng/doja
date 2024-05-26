use lazy_static::lazy_static;

use crate::{piece_attacks::PieceAttacks, shift::ShiftData};

///  ----NOT_A_FILE----
/// 8   0  1  1  1  1  1  1  1 \
/// 7   0  1  1  1  1  1  1  1 \
/// 6   0  1  1  1  1  1  1  1 \
/// 5   0  1  1  1  1  1  1  1 \
/// 4   0  1  1  1  1  1  1  1 \
/// 3   0  1  1  1  1  1  1  1 \
/// 2   0  1  1  1  1  1  1  1 \
/// 1   0  1  1  1  1  1  1  1 \
///     a  b  c  d  e  f  g  h
pub const NOT_A_FILE: u64 = 18374403900871474942; // 0xfefefefefefefefe


/// ----NOT_H_FILE----
/// 8   1  1  1  1  1  1  1  0 \
/// 7   1  1  1  1  1  1  1  0 \
/// 6   1  1  1  1  1  1  1  0 \
/// 5   1  1  1  1  1  1  1  0 \
/// 4   1  1  1  1  1  1  1  0 \
/// 3   1  1  1  1  1  1  1  0 \
/// 2   1  1  1  1  1  1  1  0 \
/// 1   1  1  1  1  1  1  1  0 \
///     a  b  c  d  e  f  g  h
pub const NOT_H_FILE: u64 = 9187201950435737471; // 0x7f7f7f7f7f7f7f7f


/// ----NOT_GH_FILE----
/// 8   1  1  1  1  1  1  0  0 \
/// 7   1  1  1  1  1  1  0  0 \
/// 6   1  1  1  1  1  1  0  0 \
/// 5   1  1  1  1  1  1  0  0 \
/// 4   1  1  1  1  1  1  0  0 \
/// 3   1  1  1  1  1  1  0  0 \
/// 2   1  1  1  1  1  1  0  0 \
/// 1   1  1  1  1  1  1  0  0 \
///     a  b  c  d  e  f  g  h
pub const NOT_GH_FILE: u64 = 4557430888798830399;



/// ----NOT_AB_FILE---- \
/// 8 &nbsp;  0  0  1  1  1  1  1  1 \
/// 7 &nbsp;  0  0  1  1  1  1  1  1 \
/// 6 &nbsp;  0  0  1  1  1  1  1  1 \
/// 5 &nbsp;  0  0  1  1  1  1  1  1 \
/// 4 &nbsp;  0  0  1  1  1  1  1  1 \
/// 3 &nbsp;  0  0  1  1  1  1  1  1 \
/// 2 &nbsp;  0  0  1  1  1  1  1  1 \
/// 1 &nbsp;  0  0  1  1  1  1  1  1 \
///   &nbsp;  a  b  c  d  e  f  g  h 
pub const NOT_AB_FILE: u64 = 18229723555195321596;


/// 8   &nbsp;   0  1  1  1  1  1  1  1 \
/// 7   &nbsp;   0  1  1  1  1  1  1  1 \
/// 6   &nbsp;   0  1  1  1  1  1  1  1 \
/// 5   &nbsp;   0  1  1  1  1  1  1  1 \
/// 4   &nbsp;   0  1  1  1  1  1  1  1 \
/// 3   &nbsp;   0  1  1  1  1  1  1  1 \
/// 2   &nbsp;   0  1  1  1  1  1  1  1 \
/// 1   &nbsp;   0  0  0  0  0  0  0  0 \
///     &nbsp;   a  b  c  d  e  f  g  h
pub(crate) const NOT_A_FILE_NOT_1_RANK: u64 = 0xfefefefefefefe00; // NOT(A) FILE AND NOT(1) RANK
pub(crate) const NOT_A_FILE_NOT_8_RANK: u64 = 0x00fefefefefefefe; // NOT(A) FILE and NOT(8) RANK
pub(crate) const NOT_8_RANK: u64 = 0x00ffffffffffffff; // NOT(8) RANK
pub(crate) const NOT_H_FILE_NOT_8_RANK: u64 = 0x007f7f7f7f7f7f7f; // NOT(H) FILE AND NOT(8) RANK
pub(crate) const NOT_H_FILE_NOT_1_RANK: u64 = 0x7f7f7f7f7f7f7f00; // NOT(H) FILE AND NOT(1) RANK
pub(crate) const NOT_1_RANK: u64 = 0xffffffffffffff00; // NOT(1) RANK

pub(crate) const AVOID_WRAP: [u64; 8] = [
    NOT_A_FILE_NOT_1_RANK, NOT_A_FILE,
    NOT_A_FILE_NOT_8_RANK, NOT_8_RANK,
    NOT_H_FILE_NOT_8_RANK, NOT_H_FILE,
    NOT_H_FILE_NOT_1_RANK, NOT_1_RANK,
];

pub(crate) const SHIFT_DATA: [ShiftData; 8] = [
    ShiftData::new(NOT_A_FILE_NOT_1_RANK, 9),
    ShiftData::new(NOT_A_FILE, 1),
    ShiftData::new(NOT_A_FILE_NOT_8_RANK, -7),
    ShiftData::new(NOT_8_RANK, -8),
    ShiftData::new(NOT_H_FILE_NOT_8_RANK, -9),
    ShiftData::new(NOT_H_FILE, -1), //southwest
    ShiftData::new(NOT_H_FILE_NOT_1_RANK, 7), //south
    ShiftData::new(NOT_1_RANK, 8), // north
];

/// Postive left, negative right shifts
pub(crate) const SHIFT: [i8; 8] = [9, 1, -7, -8, -9, -1, 7, 8];

// FEN debug positions
pub const EMPTY_BOARD: &str = "8/8/8/8/8/8/8/8 w - - ";
pub const START_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 ";
pub const TRICKY_POSITION: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1 ";
pub const KILLER_POSITION: &str = "rnbqkb1r/pp1p1pPp/8/2p1pP2/1P1P4/3P3P/P1P1P3/RNBQKBNR w KQkq e6 0 1 ";
pub const CMK_POSITION: &str = "r2q1rk1/ppp2ppp/2n1bn2/2b1p3/3pP3/3P1NPP/PPP1NPB1/R1BQ1RK1 b - - 0 9 ";
pub const POS_6: &str = "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10";
pub const POSITION_4: &str = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";



pub(crate) const PLAYERS_COUNT: usize = 2;
pub(crate) const PLAYER_PIECES: usize = 6;
pub(crate) const TOTAL_PIECES: usize = 12;
pub(crate) const OCCUPANCIES: usize = 3; // white, black, and both colors 
pub(crate) const RANK: usize = 8;
pub(crate) const FILE: usize = 8;
pub(crate) const SQUARES: usize = 64;

pub(crate) const RANK_4: u64 = 0x0000_0000_FF00_0000;
pub(crate) const RANK_5: u64 = 0x0000_00FF_0000_0000;
pub(crate) const RANK_8: u64 = 0xff00_0000_0000_0000; // RANK 8 IS FILLED
pub(crate) const RANK_1: u64 = 0xff; // RANK 1 IS FILLED
pub(crate) const WHITE_KING_CASTLING_CELLS: u64 = 0xf0;
pub(crate) const E1_F1_FILLED: u64 = 0x90; // out of the white king castling cells only E1 and F1 cells bits are set
pub(crate) const BLACK_KING_CASTLING_CELLS: u64 = 0xf000000000000000;
pub(crate) const E8_F8_IS_FILLED: u64 = 0x9000000000000000; // out of the black kings castling cells only E8 and F8 cell bits are set
pub(crate) const WHITE_QUEEN_CASTLING_CELLS: u64 = 0x1f;
pub(crate) const A1_E1_IS_FILLED: u64 = 0x11; // out of the white queen castling cells, only A1 and E1 cell bits are set
pub(crate) const BLACK_QUEEN_CASTLING_CELLS: u64 = 0x1f00000000000000;
pub(crate) const A8_E8_IS_FILLED: u64  = 0x1100000000000000;
// pub const 

// 0xFEFE_FEFE_FEFE_FEFE


/// King-castling_rights mask for white player
pub(crate)const WHITE_KING_CASTLING_MASK: u8 = 0b0001; 
/// Queen-castling_rights mask for white player
pub(crate)const WHITE_QUEEN_CASTLING_MASK: u8 = 0b0010;
/// King-castling_rights mask for black player
pub(crate)const BLACK_KING_CASTLING_MASK: u8 = 0b0100;
/// Queen-castling_rights mask for black player
pub(crate)const BLACK_QUEEN_CASTLING_MASK: u8 = 0b1000;


lazy_static! {
    pub static ref PIECE_ATTACKS: PieceAttacks = PieceAttacks::new();
}


// pub(crate) const DIAGONAL_MASK_EX: [u64; 15] = [0x8040201008040200];



///                             castling    move     in      in
///                              right     update  binary  decimal
/// king & rooks didn't move:    1111   &   1111 =  1111    15
/// 
///         white king moved:    1111   &   1100 =  1100    12
///  white king's rook moved:    1111   &   1110 =  1100    14
/// white queen's rook moved:    1111   &   1101 =  1101    13
/// 
///         black king moved:    1111   &   0011 =  0011    3
///  black king's rook moved:    1111   &   1011 =  1011    11
/// black queen's rook moved:    1111   &   0111 =  0111    7
pub(crate) const CASTLING_TABLE: [u8; 64] = [
    13, 15, 15, 15, 12, 15, 15, 14,
    15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15,
    7, 15, 15, 15,  3, 15, 15, 11,
];
