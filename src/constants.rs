use lazy_static::lazy_static;

use crate::{masks::EvaluationMasks, piece_attacks::PieceAttacks, shift::ShiftData, squares::Square, zobrist::Zobrist};

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
    ShiftData::new(NOT_A_FILE_NOT_1_RANK, 9), // northeast
    ShiftData::new(NOT_A_FILE, 1), //east
    ShiftData::new(NOT_A_FILE_NOT_8_RANK, -7), // southeast
    ShiftData::new(NOT_8_RANK, -8), // south
    ShiftData::new(NOT_H_FILE_NOT_8_RANK, -9), // southwest
    ShiftData::new(NOT_H_FILE, -1), // west
    ShiftData::new(NOT_H_FILE_NOT_1_RANK, 7), //northwest
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
pub const REPETITIONS: &str = "2r3k1/R7/8/1R6/8/8/P4KPP/8 w - - 0 40 ";


// A contempt factor is just another name for a a draw score, with the implication that the draw score is adjsted in order to reflect hte desirability
// or undesirability of a draw -- high_value (don't draw), low value(draw is understandable)
// I think a good opening-phase contempt value is -0.50 pawns.  A good general-purpose contempt factor is -0.25.  In endgames, 0.00 is suitable, 
// or a value that is not very negative at all.  It is a bad idea to play into a pawn ending with a negative evaluation.


// Score bounds for the range of mating scores
/// [-infinity, -mate_value...-mate_score, ... score ... mate_score ... mate_value, infinity]
/// -- "MATE" is in this case a constant with a large positive value, larger than any score created by summing material and positional factors could be.
/// https://web.archive.org/web/20071031100110/http://www.brucemo.com/compchess/programming/matescore.htm
pub(crate) const MATE_VALUE: i32 = 49_000;
pub(crate) const MATE_SCORE: i32 = 48_000; // i.e. MATE_VALUE - 1000
pub(crate) const INFINITY: i32 = 50_000;

pub(crate) const NO_HASH_ENTRY: i32 = 100000;

pub(crate) const ALPHA: i32 = -INFINITY;
pub(crate) const BETA: i32 = INFINITY;
pub(crate) const FULL_DEPTH_MOVE: u8 = 4;   // // https://web.archive.org/web/20150212051846/http://www.glaurungchess.com/lmr.html
pub(crate) const REDUCTION_LIMIT: u8 = 3;   // // https://web.archive.org/web/20150212051846/http://www.glaurungchess.com/lmr.html
pub(crate) const DEPTH_REDUCTION_FACTOR: u8 = 2; // (suggested deduction factor) https://web.archive.org/web/20071031095933/http://www.brucemo.com/compchess/programming/nullmove.htm
pub(crate) const VAL_WINDOW: i32 = 50; // https://web.archive.org/web/20071031095918/http://www.brucemo.com/compchess/programming/aspiration.htm
pub(crate) const NODES_2047: u64 = 2047;


pub(crate) const PLAYERS_COUNT: usize = 2;
pub(crate) const PLAYER_PIECES: usize = 6;
pub(crate) const TOTAL_PIECES: usize = 12;
pub(crate) const OCCUPANCIES: usize = 3; // white, black, and both colors 
pub(crate) const RANK: usize = 8;
pub(crate) const FILE: usize = 8;
pub(crate) const TOTAL_SQUARES: usize = 64;
/// Castling rights:
/// e.g for player black:
/// 1. Player black has no castling right
/// 2. Player black can only castle king
/// 3. Player black can only castle queen
/// 4. Player black can castle both king and queen
/// (for white and black player) = 4 * 4 = 16
pub(crate) const TOTAL_CASTLING_RIGHTS: usize = 16;
pub(crate) const MAX_PLY: usize = 64;


pub(crate) const RANDOM_STATE_SEED: u32 = 1804289383;

pub(crate) const RANK_4: u64 = 0x0000_0000_FF00_0000;
pub(crate) const RANK_5: u64 = 0x0000_00FF_0000_0000;
pub(crate) const RANK_8: u64 = 0xff00_0000_0000_0000; // RANK 8 IS FILLED
pub(crate) const RANK_1: u64 = 0xff; // RANK 1 IS FILLED
pub(crate) const RANK_2: u64 = 0xff00; // rank 2 is filled
pub(crate) const RANK_3: u64 = 0xff0000; // rank 3 is filled
pub(crate) const RANK_6: u64 = 0xff0000000000; // rank 6 is filled
pub(crate) const RANK_7: u64 = 0xff000000000000; // rank 7 is filled


pub(crate) const A_FILE: u64 = 0x101010101010101;  // A rank is filled
pub(crate) const B_FILE: u64 = 0x202020202020202;  // B rank is filled
pub(crate) const C_FILE: u64 = 0x404040404040404;  // C rank is filled
pub(crate) const D_FILE: u64 = 0x808080808080808;  // D rank is filled
pub(crate) const E_FILE: u64 = 0x1010101010101010; // E rank is filled
pub(crate) const F_FILE: u64 = 0x2020202020202020; // F rank is filled
pub(crate) const G_FILE: u64 = 0x4040404040404040; // G rank is filled
pub(crate) const H_FILE: u64 = 0x8080808080808080; // H file is filled




pub(crate) const WHITE_KING_CASTLING_CELLS: u64 = 0xf0;
pub(crate) const E1_F1_FILLED: u64 = 0x90; // out of the white king castling cells only E1 and F1 cells bits are set
pub(crate) const BLACK_KING_CASTLING_CELLS: u64 = 0xf000000000000000;
pub(crate) const E8_F8_IS_FILLED: u64 = 0x9000000000000000; // out of the black kings castling cells only E8 and F8 cell bits are set
pub(crate) const WHITE_QUEEN_CASTLING_CELLS: u64 = 0x1f;
pub(crate) const A1_E1_IS_FILLED: u64 = 0x11; // out of the white queen castling cells, only A1 and E1 cell bits are set
pub(crate) const BLACK_QUEEN_CASTLING_CELLS: u64 = 0x1f00000000000000;
pub(crate) const A8_E8_IS_FILLED: u64  = 0x1100000000000000;
// pub const 


/// THIS WAS COPIED WORD-FOR-WORD FROM THE [CARP ENGINE](https://github.com/dede1751/carp/blob/main/chess/src/lib.rs)
/// Contains certain engine parameters necessarily kept in the backend.
pub mod params {
    /// Maximum depth supported by the NNUE implementation within the crate.
    pub const MAX_DEPTH: usize = 127;

    /// Eval type returned by the network.
    pub type Eval = i32;

    /// Piece static values used in SEE.
    pub const PIECE_VALUES: [Eval; 6] = [161, 446, 464, 705, 1322, 0];
}

// COPIED FROM STOCKFISH
pub const PIECE_VALUES: [i32; 6] = [ 208, 781, 825, 1276, 2538, 0];

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
    #[derive(Debug)]
    pub static ref ZOBRIST: Zobrist = Zobrist::init_zobrist();
    // evaluation masks
    pub static ref EVAL_MASKS: EvaluationMasks = EvaluationMasks::init();
}


/// double pawns penalty (from CMK (BBC engine))
pub(crate) const DOUBLE_PAWN_PENALTY_OPENING: i32 = -5;
pub(crate) const DOUBLE_PAWN_PENALTY_ENDGAME: i32 = -10;
/// isolated pawns penalty
pub(crate) const ISOLATED_PAWN_PENALTY_OPENING: i32 = -5;
pub(crate) const ISOLATED_PAWN_PENALTY_ENDGAME: i32 = -10;
/// passed pawn bonus 
pub(crate) const PASSED_PAWN_BONUS: [u8; 8] = [0, 5, 10, 20, 35, 60, 100, 200];
/// Semi-open-file: a file on which we do not have a pawn, but the opponent has at least one. It can be used to increase the vertical mobility of the major pieces, to attack weak pawns and to pressure enemy position or to create an outpost.
pub(crate) const SEMI_OPEN_FILE_SCORE: i32 = 10;
/// An Open File is a vertical column with no pawns of either color on it
pub(crate) const OPEN_FILE_SCORE: i32 = 15;
pub(crate) const KING_SHIELD_BONUS: i32 = 5;


// mobility units (values from fruit reloaded)
pub(crate) const BISHOP_UNIT: i32 = 4;
pub(crate) const QUEEN_UNIT: i32 = 9;

// mobility bonuses (values from engine Fruit reloaded)
pub(crate) const BISHOP_MOBILITY_OPENING: i32 = 5;
pub(crate) const BISHOP_MOBILITY_ENDGAME: i32 = 5;
pub(crate) const QUEEN_MOBILITY_OPENING: i32 = 1;
pub(crate) const QUEEN_MOBILITY_ENDGAME: i32 = 2;


pub(crate) const MATERIAL_SCORE: [[i32; 12]; 2] = [
    [82, 337, 365, 477, 1025,  12000, -82, -337, -365, -477, -1025,  -12000], // opening material game score
    [94, 281, 297, 512,  936,  12000, -94, -281, -297, -512,  -936,  -12000], // end-game material game scoresss
];

/// Scores higher than this means that the such game is in the opening phase
pub(crate) const OPENING_PHASE_SCORE: i32 = 6192; // mg
/// Score lesser than the opening but greater than this score is results in middle game phase
/// Scores lower than this score results in the game's end
pub(crate) const END_PHASE_SCORE: i32 = 518;      // eg

// opening positional pieces scores
pub(crate) const POSITIONAL_SCORES: [[[i32; 64]; 6]; 2] = [
    // [0.OPENING_POSITIONAL_SCORES 1. END_GAME_POSITIONAL_SCORES
    //  [PIECES: PAWN, KNIGHT, BISHOP, ROOK, QUEEN, KING
    //      [SQUARES]
    //  ]
    // ]
    // 
    [
        [   // PAWN TABLE
              0,   0,   0,   0,   0,   0,  0,   0,
             98, 134,  61,  95,  68, 126, 34, -11,
             -6,   7,  26,  31,  65,  56, 25, -20,
            -14,  13,   6,  21,  23,  12, 17, -23,
            -27,  -2,  -5,  12,  17,   6, 10, -25,
            -26,  -4,  -4, -10,   3,   3, 33, -12,
            -35,  -1, -20, -23, -15,  24, 38, -22,
              0,   0,   0,   0,   0,   0,  0,   0,
        ],
        [   // KNIGHT TABLE
           -167, -89, -34, -49,  61, -97, -15, -107,
            -73, -41,  72,  36,  23,  62,   7,  -17,
            -47,  60,  37,  65,  84, 129,  73,   44,
             -9,  17,  19,  53,  37,  69,  18,   22,
            -13,   4,  16,  13,  28,  19,  21,   -8,
            -23,  -9,  12,  10,  19,  17,  25,  -16,
            -29, -53, -12,  -3,  -1,  18, -14,  -19,
           -105, -21, -58, -33, -17, -28, -19,  -23,
        ],
        [   // BISHOP TABLE
            -29,   4, -82, -37, -25, -42,   7,  -8,
            -26,  16, -18, -13,  30,  59,  18, -47,
            -16,  37,  43,  40,  35,  50,  37,  -2,
             -4,   5,  19,  50,  37,  37,   7,  -2,
             -6,  13,  13,  26,  34,  12,  10,   4,
              0,  15,  15,  15,  14,  27,  18,  10,
              4,  15,  16,   0,   7,  21,  33,   1,
            -33,  -3, -14, -21, -13, -12, -39, -21,
        ],
        [   // ROOK TABLE
            32,  42,  32,  51, 63,  9,  31,  43,
            27,  32,  58,  62, 80, 67,  26,  44,
            -5,  19,  26,  36, 17, 45,  61,  16,
            -24, -11,   7,  26, 24, 35,  -8, -20,
            -36, -26, -12,  -1,  9, -7,   6, -23,
            -45, -25, -16, -17,  3,  0,  -5, -33,
            -44, -16, -20,  -9, -1, 11,  -6, -71,
            -19, -13,   1,  17, 16,  7, -37, -26,
        ],
        [   // QUEEN TABLE
            -28,   0,  29,  12,  59,  44,  43,  45,
            -24, -39,  -5,   1, -16,  57,  28,  54,
            -13, -17,   7,   8,  29,  56,  47,  57,
            -27, -27, -16, -16,  -1,  17,  -2,   1,
             -9, -26,  -9, -10,  -2,  -4,   3,  -3,
            -14,   2, -11,  -2,  -5,   2,  14,   5,
            -35,  -8,  11,   2,   8,  15,  -3,   1,
             -1, -18,  -9,  10, -15, -25, -31, -50,
        ],
        [   // KING TABLE
            -65,  23,  16, -15, -56, -34,   2,  13,
             29,  -1, -20,  -7,  -8,  -4, -38, -29,
             -9,  24,   2, -16, -20,   6,  22, -22,
            -17, -20, -12, -27, -30, -25, -14, -36,
            -49,  -1, -27, -39, -46, -44, -33, -51,
            -14, -14, -22, -46, -44, -30, -15, -27,
              1,   7,  -8, -64, -43, -16,   9,   8,
            -15,  36,  12, -54,   8, -28,  24,  14,
        ]
    ],
    [
    [   // PAWN SCORES
        0,   0,   0,   0,   0,   0,   0,   0,
        178, 173, 158, 134, 147, 132, 165, 187,
        94, 100,  85,  67,  56,  53,  82,  84,
        32,  24,  13,   5,  -2,   4,  17,  17,
        13,   9,  -3,  -7,  -7,  -8,   3,  -1,
        4,   7,  -6,   1,   0,  -5,  -1,  -8,
        13,   8,   8,  10,  13,   0,   2,  -7,
        0,   0,   0,   0,   0,   0,   0,   0,
    ],
    [   // KNIGHT TABLE
        -58, -38, -13, -28, -31, -27, -63, -99,
        -25,  -8, -25,  -2,  -9, -25, -24, -52,
        -24, -20,  10,   9,  -1,  -9, -19, -41,
        -17,   3,  22,  22,  22,  11,   8, -18,
        -18,  -6,  16,  25,  16,  17,   4, -18,
        -23,  -3,  -1,  15,  10,  -3, -20, -22,
        -42, -20, -10,  -5,  -2, -20, -23, -44,
        -29, -51, -23, -15, -22, -18, -50, -64,
    ],
    [ // BISHOP TABLE
        -14, -21, -11,  -8, -7,  -9, -17, -24,
         -8,  -4,   7, -12, -3, -13,  -4, -14,
          2,  -8,   0,  -1, -2,   6,   0,   4,
         -3,   9,  12,   9, 14,  10,   3,   2,
         -6,   3,  13,  19,  7,  10,  -3,  -9,
        -12,  -3,   8,  10, 13,   3,  -7, -15,
        -14, -18,  -7,  -1,  4,  -9, -15, -27,
        -23,  -9, -23,  -5, -9, -16,  -5, -17,
    ],
    [   // ROOK TABLE
        13, 10, 18, 15, 12,  12,   8,   5,
        11, 13, 13, 11, -3,   3,   8,   3,
         7,  7,  7,  5,  4,  -3,  -5,  -3,
         4,  3, 13,  1,  2,   1,  -1,   2,
         3,  5,  8,  4, -5,  -6,  -8, -11,
        -4,  0, -5, -1, -7, -12,  -8, -16,
        -6, -6,  0,  2, -9,  -9, -11,  -3,
        -9,  2,  3, -1, -5, -13,   4, -20,
    ],
    [   // QUEEN TABLE
         -9,  22,  22,  27,  27,  19,  10,  20,
        -17,  20,  32,  41,  58,  25,  30,   0,
        -20,   6,   9,  49,  47,  35,  19,   9,
          3,  22,  24,  45,  57,  40,  57,  36,
        -18,  28,  19,  47,  31,  34,  39,  23,
        -16, -27,  15,   6,   9,  17,  10,   5,
        -22, -23, -30, -16, -16, -23, -36, -32,
        -33, -28, -22, -43,  -5, -32, -20, -41,
    ],
    [   // KING TABLE
        -74, -35, -18, -18, -11,  15,   4, -17,
        -12,  17,  14,  17,  17,  38,  23,  11,
         10,  17,  23,  15,  20,  45,  44,  13,
         -8,  22,  24,  27,  26,  33,  26,   3,
        -18,  -4,  21,  24,  27,  23,   9, -11,
        -19,  -3,  11,  21,  23,  16,   7,  -9,
        -27, -11,   4,  13,  14,   4,  -5, -17,
        -53, -34, -21, -11, -28, -14, -24, -43
    ]
]
];



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



pub(crate) const MIRROR_SCORE: [Square; 64] = [
    Square::A8, Square::B8, Square::C8, Square::D8, Square::E8, Square::F8, Square::G8, Square::H8, 
    Square::A7, Square::B7, Square::C7, Square::D7, Square::E7, Square::F7, Square::G7, Square::H7,
    Square::A6, Square::B6, Square::C6, Square::D6, Square::E6, Square::F6, Square::G6, Square::H6,
    Square::A5, Square::B5, Square::C5, Square::D5, Square::E5, Square::F5, Square::G5, Square::H5,
    Square::A4, Square::B4, Square::C4, Square::D4, Square::E4, Square::F4, Square::G4, Square::H4,
    Square::A3, Square::B3, Square::C3, Square::D3, Square::E3, Square::F3, Square::G3, Square::H3,
    Square::A2, Square::B2, Square::C2, Square::D2, Square::E2, Square::F2, Square::G2, Square::H2,
    Square::A1, Square::B1, Square::C1, Square::D1, Square::E1, Square::F1, Square::G1, Square::H1,
];
// pub(crate) const MIRROR_SCORE: [Square; 64] = [
//     Square::H8, Square::G8, Square::F8, Square::E8, Square::D8, Square::C8, Square::B8, Square::A8, 
//     Square::H7, Square::G7, Square::F7, Square::E7, Square::D7, Square::C7, Square::B7, Square::A7, 
//     Square::H6, Square::G6, Square::F6, Square::E6, Square::D6, Square::C6, Square::B6, Square::A6, 
//     Square::H5, Square::G5, Square::F5, Square::E5, Square::D5, Square::C5, Square::B5, Square::A5, 
//     Square::H4, Square::G4, Square::F4, Square::E4, Square::D4, Square::C4, Square::B4, Square::A4, 
//     Square::H3, Square::G3, Square::F3, Square::E3, Square::D3, Square::C3, Square::B3, Square::A3, 
//     Square::H2, Square::G2, Square::F2, Square::E2, Square::D2, Square::C2, Square::B2, Square::A2, 
//     Square::H1, Square::G1, Square::F1, Square::E1, Square::D1, Square::C1, Square::B1, Square::A1, 
// ];



/// Most Valuable victim & Less valuable attacker
/// (Victims ->)    Pawn    Knight  Bishop  Rook  Queen  King
/// (Attackers)  
///     Pawn  -->>  105     205    305     405    505    605
///   Knight  -->>  104     204    304     404    504    604
///   Bishop  -->>  103     203    303     403    503    603
///     Rook  -->>  102     202    302     402    502    602
///    Queen  -->>  101     201    301     401    501    601
///     King  -->>  100     200    300     400    500    600
///
/// In order to get this claculation
/// simply do this:
/// to get the index of the attacker or victim regardless of their color (e.g. black or white)
/// so a modulo 6(number of pieces per color) to get the index (irresepctive of the color)
/// e.g where a white rook is attacking a black bishop, we would have
/// whiteRook=3, blackBishop=8
/// a_val ATTACKER -->> whiteRook(3) % 6 = 3
/// v_val VICTIM --->> blackBishop(8) % 6 = 2
/// To get the value of MVV_LVA, you must:
/// Multiply the a_val by 6, and then add v_val
/// hence, (a_val * 6) + v_val
/// in this example we would have ((3%6)*6) + (8%6) = 20
/// so all we need to do is look at index 20 mvv_lvv[20] = 302
// pub(crate) const MVV_LVA:  [u32; 36] = [
//     105, 205, 305, 405, 505, 605,   // pawn
//     104, 204, 304, 404, 504, 604,   // knight
//     103, 203, 303, 403, 503, 603,   // bishop
//     102, 202, 302, 402, 502, 602,   // rook
//     101, 201, 301, 401, 501, 601,   // queen
//     100, 200, 300, 400, 500, 600,   // king
// ];
pub(crate) const MAX: u32 = u32::MAX;
pub(crate) const MVV_LVA: [[u32; 6]; 6] = [ // [attacker][victim] i.e [src][tgt]
 //            Pawn     Knight  Bishop  Rook    Queen   King
 /*P*/         [105,    205,    305,    405,    505,    MAX],  // Attacker is Pawn
 /*N*/         [104,    204,    304,    404,    504,    MAX],  // Attacker is Knight
 /*B*/         [103,    203,    303,    403,    503,    MAX],  // Attacker is Bishop
 /*R*/         [102,    202,    302,    402,    502,    MAX],  // Attacker is Rook
 /*Q*/         [101,    201,    301,    401,    501,    MAX],  // Attacker is Queen
 /*K*/         [MAX,    MAX,    MAX,    MAX,    MAX,    MAX]   // Attacker is King
];


/// Piece Square Tables
pub(crate) const PAWN_SCORES: [i8; 64] = [
    0,  0,  0,  0,  0,  0,  0,  0,
    50, 50, 50, 50, 50, 50, 50, 50,
    10, 10, 20, 30, 30, 20, 10, 10,
    5,  5, 10, 25, 25, 10,  5,  5,
    0,  0,  0, 20, 20,  0,  0,  0,
    5, -5,-10,  0,  0,-10, -5,  5,
    5, 10, 10,-20,-20, 10, 10,  5,
    0,  0,  0,  0,  0,  0,  0,  0
];

pub(crate) const KNIGHT_SCORES: [i8; 64] = [
    -50,-40,-30,-30,-30,-30,-40,-50,
    -40,-20,  0,  0,  0,  0,-20,-40,
    -30,  0, 10, 15, 15, 10,  0,-30,
    -30,  5, 15, 20, 20, 15,  5,-30,
    -30,  0, 15, 20, 20, 15,  0,-30,
    -30,  5, 10, 15, 15, 10,  5,-30,
    -40,-20,  0,  5,  5,  0,-20,-40,
    -50,-40,-30,-30,-30,-30,-40,-50,
];

pub(crate) const BISHOP_SCORES: [i8; 64] = [
    -20,-10,-10,-10,-10,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5, 10, 10,  5,  0,-10,
    -10,  5,  5, 10, 10,  5,  5,-10,
    -10,  0, 10, 10, 10, 10,  0,-10,
    -10, 10, 10, 10, 10, 10, 10,-10,
    -10,  5,  0,  0,  0,  0,  5,-10,
    -20,-10,-10,-10,-10,-10,-10,-20,
];

pub(crate) const ROOKS_SCORES: [i8; 64] = [
  0,  0,  0,  0,  0,  0,  0,  0,
  5, 10, 10, 10, 10, 10, 10,  5,
 -5,  0,  0,  0,  0,  0,  0, -5,
 -5,  0,  0,  0,  0,  0,  0, -5,
 -5,  0,  0,  0,  0,  0,  0, -5,
 -5,  0,  0,  0,  0,  0,  0, -5,
 -5,  0,  0,  0,  0,  0,  0, -5,
  0,  0,  0,  5,  5,  0,  0,  0
];

pub(crate) const QUEENS_SCORES: [i8; 64] = [
    -20,-10,-10, -5, -5,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5,  5,  5,  5,  0,-10,
     -5,  0,  5,  5,  5,  5,  0, -5,
      0,  0,  5,  5,  5,  5,  0, -5,
    -10,  5,  5,  5,  5,  5,  0,-10,
    -10,  0,  5,  0,  0,  0,  0,-10,
    -20,-10,-10, -5, -5,-10,-10,-20
];

pub(crate) const KING_SCORES_MIDDLE: [i8; 64] = [
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -20,-30,-30,-40,-40,-30,-30,-20,
    -10,-20,-20,-20,-20,-20,-20,-10,
     20, 20,  0,  0,  0,  0, 20, 20,
     20, 30, 10,  0,  0, 10, 30, 20  
];

pub(crate) const KING_SCORES_END: [i8; 64] = [
    -50,-40,-30,-20,-20,-30,-40,-50,
    -30,-20,-10,  0,  0,-10,-20,-30,
    -30,-10, 20, 30, 30, 20,-10,-30,
    -30,-10, 30, 40, 40, 30,-10,-30,
    -30,-10, 30, 40, 40, 30,-10,-30,
    -30,-10, 20, 30, 30, 20,-10,-30,
    -30,-30,  0,  0,  0,  0,-30,-30,
    -50,-30,-30,-30,-30,-30,-30,-50
];





//
//     File mask for         Isolated mask            Passed pawn mask
//     square f2             for square g2            for square c4
//  8  0 0 0 0 0 1 0 0    8  0 0 0 0 0 1 0 1       8  0 1 1 1 0 0 0 0
//  7  0 0 0 0 0 1 0 0    7  0 0 0 0 0 1 0 1       7  0 1 1 1 0 0 0 0
//  6  0 0 0 0 0 1 0 0    6  0 0 0 0 0 1 0 1       6  0 1 1 1 0 0 0 0
//  5  0 0 0 0 0 1 0 0    5  0 0 0 0 0 1 0 1       5  0 1 1 1 0 0 0 0
//  4  0 0 0 0 0 1 0 0    4  0 0 0 0 0 1 0 1       4  0 0 0 0 0 0 0 0
//  3  0 0 0 0 0 1 0 0    3  0 0 0 0 0 1 0 1       3  0 0 0 0 0 0 0 0
//  2  0 0 0 0 0 1 0 0    2  0 0 0 0 0 1 0 1       2  0 0 0 0 0 0 0 0
//  1  0 0 0 0 0 1 0 0    1  0 0 0 0 0 1 0 1       1  0 0 0 0 0 0 0 0
//     a b c d e f g h       a b c d e f g h          a b c d e f g h
//  


