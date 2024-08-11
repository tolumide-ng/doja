// #[cfg(feature = "avx2")]
// mod archs_avx2;

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