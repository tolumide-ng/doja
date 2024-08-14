pub(crate) const RANK_1: u64 = 0x00000000000000FF;
pub(crate) const RANK_2: u64 = 0x000000000000FF00;
pub(crate) const RANK_3: u64 = 0x0000000000FF0000;
pub(crate) const RANK_4: u64 = 0x00000000FF000000;
pub(crate) const RANK_5: u64 = 0x000000FF00000000;
pub(crate) const RANK_6: u64 = 0x0000FF0000000000;
pub(crate) const RANK_7: u64 = 0x00FF000000000000;
pub(crate) const RANK_8: u64 = 0xFF00000000000000;
pub(crate) const FILE_A: u64 = 0x0101010101010101;
pub(crate) const FILE_B: u64 = 0x0202020202020202;
pub(crate) const FILE_C: u64 = 0x0404040404040404;
pub(crate) const FILE_D: u64 = 0x0808080808080808;
pub(crate) const FILE_E: u64 = 0x1010101010101010;
pub(crate) const FILE_F: u64 = 0x2020202020202020;
pub(crate) const FILE_G: u64 = 0x4040404040404040;
pub(crate) const FILE_H: u64 = 0x8080808080808080;

pub(crate) const WHITE_SQUARES: u64 = 0x55AA55AA55AA55AA;
pub(crate) const BLACK_SQUARES: u64 = 0xAA55AA55AA55AA55;

pub(crate) const LONG_DIAGONALS: u64 = 0x8142241818244281;
pub(crate) const CENTER_SQUARES: u64 = 0x0000001818000000;
pub(crate) const CENTER_BIG: u64     = 0x00003C3C3C3C0000;

pub(crate) const LEFT_FLANK: u64  = FILE_A | FILE_B | FILE_C | FILE_D;
pub(crate) const RIGHT_FLANK: u64 = FILE_E | FILE_F | FILE_G | FILE_H;

pub(crate) const FILES: [u64; 8] = [FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H];
pub(crate) const RANKS: [u64; 8] = [RANK_1, RANK_2, RANK_3, RANK_4, RANK_5, RANK_6, RANK_7, RANK_8];

pub(crate) const PROMOTION_RANKS: u64 = RANK_1 | RANK_8;



/// Input size
pub const IN_SIZE: usize = 20480;
pub const KP_SIZE: usize = 768;
/// layer 1 size
pub const L1_SIZE: usize = 1536;
/// layer 2 size
pub const L2_SIZE: usize = 8;
/// layer 3 size
pub const L3_SIZE: usize = 32; 
/// Output size
pub const OUT_SIZE: usize = 1;
// Number of registers
pub const NUM_REGS: usize = 16;

pub const COLOR_NB: usize = 2;
pub const PIECE_NB: usize = 6;
pub const MAX_PLY: usize = 64; // can/should be changed
pub const SQUARE_NB: usize = 64;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NNUEDelta {
    pub piece: i32, pub from: i32, pub to: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[repr(align(64))]
pub struct NNUEAccumulator {
    pub changes: i32, 
    pub accurate: [i32; COLOR_NB],
    pub deltas: [NNUEDelta; 3],
    pub values: [[i16; KP_SIZE]; COLOR_NB]
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NNUEAccumulatorTableEntry {
    pub accumulator: NNUEAccumulator,
    pub occupancy: [[[u64; PIECE_NB -1]; COLOR_NB]; COLOR_NB]
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NNUEEvaluator {
    pub stack: [NNUEAccumulator; MAX_PLY + 4],
    pub current: *mut NNUEAccumulator,
    pub table: [NNUEAccumulatorTableEntry; SQUARE_NB]
}

#[repr(align(64))]
pub struct Align64<T>(pub T);