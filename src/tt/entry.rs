use std::sync::atomic::{AtomicU64, AtomicU8, Ordering};

use crate::{constants::{MATED_IN_MAX_PLY, MATE_IN_MAX_PLY, NONE}, move_logic::bitmove::Move, tt::flag::HashFlag};

const FLAG_MASK: u64 = 0x3;
const AGE_MASK: u64 = 0x1FC;
const DEPTH_MASK: u64 = 0xFE00;
const MV_MASK: u64 = 0xFFFF0000;
const EVAL_MASK: u64 = 0xFFFF00000000;
const SCORE_MASK: u64 = 0xFFFF000000000000;

  
const FLAG_OFFSET: u64 = FLAG_MASK.trailing_zeros() as u64;
const AGE_OFFSET: u64 = AGE_MASK.trailing_zeros() as u64;
const DEPTH_OFFSET: u64 = DEPTH_MASK.trailing_zeros() as u64;
const MV_OFFSET: u64 = MV_MASK.trailing_zeros() as u64;
const EVAL_OFFSET: u64 = EVAL_MASK.trailing_zeros() as u64;
const SCORE_OFFSET: u64 = SCORE_MASK.trailing_zeros() as u64;

// 3FFFC0000000 -> score offset

/// 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000011 - (2- bits)     - 0x3                   - FLAG
/// 00000000 00000000 00000000 00000000 00000000 00000000 00000001 11111100 - (7- bits)     - 0x1FC                 - AGE
/// 00000000 00000000 00000000 00000000 00000000 00000000 11111110 00000000 - (7- bits)     - 0xFE00                - DEPTH
/// 00000000 00000000 00000000 00000000 11111111 11111111 00000000 00000000 - (16-bits)     - 0xFFFF0000            - MOVE (the best move)
/// 00000000 00000000 11111111 11111111 00000000 00000000 00000000 00000000 - (16-bits)     - 0xFFFF00000000        - STATIC EVAL
/// 11111111 11111111 00000000 00000000 00000000 00000000 00000000 00000000 - (16-bits)     - 0xFFFF000000000000    - SCORE (the score of the best move in this position)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(align(64))]
pub(crate) struct TTData {
    pub(crate) flag: HashFlag,
    /// https://citeseerx.ist.psu.edu/document?repid=rep1&type=pdf&doi=1db6bdd7b588c35d01c7f8a5454a610664789ff4
    pub(crate) age: u8,
    pub(crate) depth: u8,
    pub(crate) mv: Option<Move>,
    pub(crate) eval: i16,
    pub(crate) score: i16,
    pub(super) key: u64,
 }

impl From<TTData> for u64 {
    fn from(value: TTData) -> Self {
        let TTData { depth, flag, score, mv, eval, age, .. } = value;
        let mv = mv.unwrap_or(Move::from(0));
        let result = (flag as u64) << FLAG_OFFSET | (age as u64) << AGE_OFFSET | (depth as u64) << DEPTH_OFFSET | (*mv as u64) << MV_OFFSET | (score as u64) << SCORE_OFFSET
         | (score as u64) <<  SCORE_OFFSET | (eval as u64) << EVAL_OFFSET;
        result
    }
 }


 /// Transposition table Entry
 /// (need to get rid of this in the future)
#[derive(Debug, Default)]
pub(crate) struct TTEntry {
    pub(super) age: AtomicU8,
    pub(super) smp_key: AtomicU64,
    pub(super) smp_data: AtomicU64,
}



impl From<u64> for TTData {
    fn from(value: u64) -> Self {
        let mv = Move::from(((value & MV_MASK) >> MV_OFFSET) as u16);
        let mv: Option<Move> = Option::from((*mv > 0).then(|| mv));

        let depth = ((value & DEPTH_MASK) >> DEPTH_OFFSET) as u8;

        TTData { 
            depth,
            age: ((value & AGE_MASK) >> AGE_OFFSET) as u8,
            flag: HashFlag::from(((value & FLAG_MASK) >> FLAG_OFFSET) as u8) , 
            score: ((value & SCORE_MASK) >> SCORE_OFFSET) as i16, 
            eval: ((value & EVAL_MASK) >> EVAL_OFFSET) as i16,
            mv, 
            key: 0, 
         }
    }
}


impl TTEntry {
    // pub(crate) fn new(key: u64, age: u8, depth: u8, score: i16, eval: i16, mv: Option<Move>, flag: HashFlag) -> Self {
    //     let smp_data = AtomicU64::new(TTData::new(key, depth, from_tt(score, depth), eval, mv, flag).into());

    //     let smp_key = key ^ smp_data.load(Ordering::Relaxed);
    //     Self { age: AtomicU8::new(age), smp_key: AtomicU64::new(smp_key), smp_data }
    // }

    pub(crate) fn write(&self, key: u64, age: u8, depth: u8, score: i16, eval: i16, mv: Option<Move>, flag: HashFlag) {
        let smp_data = TTData::new(key, depth, score, eval, mv, flag, age);

        let smp_key = key ^ u64::from(smp_data);

        self.smp_data.store(smp_data.into(), Ordering::SeqCst);
        self.smp_key.store(smp_key, Ordering::SeqCst);
        self.age.store(age, Ordering::SeqCst);
    }
}


impl TTData {
    fn new(key: u64, depth: u8, score: i16, eval: i16, mv: Option<Move>, flag: HashFlag, age: u8) -> Self {
        Self {key, depth, score, eval, mv, flag, age}
    }

    pub(crate) fn eval(&self) -> i32 { self.eval as i32 }
}


pub(crate) fn to_tt(value: i32, ply: usize) -> i16 {
    let ply = ply as i32;
    if value >= MATE_IN_MAX_PLY {
        (value + ply) as i16
    } else if value <= MATED_IN_MAX_PLY {
        (value - ply) as i16
    } else {
        value as i16
    }
}

pub(crate) fn from_tt(value: i16, ply: usize) -> i32 {
    let value = value as i32;
    let ply = ply as i32;
    if value == NONE {
        NONE
    } else if value >= MATE_IN_MAX_PLY {
        value - ply
    } else if value <= MATED_IN_MAX_PLY {
        value + ply
    } else {
        value
    }
}