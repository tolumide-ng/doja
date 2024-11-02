use std::sync::atomic::{AtomicU64, AtomicU8, Ordering};

use crate::{constants::{MATED_IN_MAX_PLY, MATE_IN_MAX_PLY, NONE}, move_logic::bitmove::Move, tt::flag::HashFlag};

const FLAG_MASK: u64 = 0x3;
const DEPTH_MASK: u64 = 0x1FC;
const MV_MASK: u64 = 0x1FFFE00;
const EVAL_MASK: u64 = 0x1FFFE000000;
const SCORE_MASK: u64 = 0x1FFFE0000000000;

  
const FLAG_OFFSET: u64 = 0;
const DEPTH_OFFSET: u64 = 2;
const MV_OFFSET: u64 = 9;
const EVAL_OFFSET: u64 = 25;
const SCORE_OFFSET: u64 = 41;

// 3FFFC0000000 -> score offset

/// 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000011 - (2b)  - 0x3                   - FLAG  
/// 00000000 00000000 00000000 00000000 00000000 00000000 00000001 11111100 - (7b)  - 0x1FC                 - DEPTH 
/// 00000000 00000000 00000000 00000000 00000001 11111111 11111110 00000000 - (16b) - 0x1FFFE00             - MOVE
/// 00000000 00000000 00000001 11111111 11111110 00000000 00000000 00000000 - (16b) - 0x1FFFE000000         - STATIC EVAL
/// 00000001 11111111 11111110 00000000 00000000 00000000 00000000 00000000 - (16b) - 0x1FFFE0000000000     - SCORE
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
// #[repr(align(64))]
pub(crate) struct TTData {
    pub(crate) flag: HashFlag,
    pub(crate) depth: u8,
    pub(crate) eval: i16,
    pub(crate) score: i32,
    pub(crate) mv: Option<Move>,
    pub(super) key: u64,
 }

impl From<TTData> for u64 {
    fn from(value: TTData) -> Self {
        let TTData { depth: ply, flag, score, mv, eval, .. } = value;
        let score = to_tt(score, ply);
        let mv = mv.unwrap_or(Move::from(0));
        // should everything here be in i64, since the mvs are already in i64
        let result = (flag as u64) << FLAG_OFFSET | (ply as u64) << DEPTH_OFFSET | (*mv as u64) << MV_OFFSET | (score as u64) << SCORE_OFFSET
         | (score as u64) <<  SCORE_OFFSET | (eval as u64) << EVAL_OFFSET;
        result
    }
 }


 /// Transposition table Entry
#[derive(Debug, Default)]
pub(crate) struct TTEntry {
    pub(super) age: AtomicU8, // todo! readup papers on transposition table repalcement schemes
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
            flag: HashFlag::from(((value & FLAG_MASK) >> FLAG_OFFSET) as u8) , 
            score: from_tt(((value & SCORE_MASK) >> SCORE_OFFSET) as i16, depth), 
            eval: ((value & EVAL_MASK) >> EVAL_OFFSET) as i16,
            mv, 
            key: 0, 
         }
    }
}


impl TTEntry {
    pub(crate) fn new(key: u64, age: u8, depth: u8, score: i16, eval: i16, mv: Option<Move>, flag: HashFlag) -> Self {
        let smp_data = AtomicU64::new(TTData::new(key, depth, from_tt(score, depth), eval, mv, flag).into());

        let smp_key = key ^ smp_data.load(Ordering::Relaxed);
        Self { age: AtomicU8::new(age), smp_key: AtomicU64::new(smp_key), smp_data }
    }

    pub(crate) fn write(&self, key: u64, age: u8, depth: u8, score: i32, eval: i16, mv: Option<Move>, flag: HashFlag) {
        let smp_data = TTData::new(key, depth, score, eval, mv, flag);

        let smp_key = key ^ u64::from(smp_data);

        self.smp_data.store(smp_data.into(), Ordering::SeqCst);
        self.smp_key.store(smp_key, Ordering::SeqCst);
        self.age.store(age, Ordering::SeqCst);
    }
}


impl TTData {
    fn new(key: u64, depth: u8, score: i32, eval: i16, mv: Option<Move>, flag: HashFlag) -> Self {
        Self {key, depth, score, eval, mv, flag}
    }

    pub(crate) fn eval(&self) -> i32 { self.eval as i32 }
}


pub(crate) fn to_tt(value: i32, ply: u8) -> i16 {
    let ply = ply as i32;
    if value >= MATE_IN_MAX_PLY {
        (value + ply) as i16
    } else if value <= MATED_IN_MAX_PLY {
        (value - ply) as i16
    } else {
        value as i16
    }
}

pub(crate) fn from_tt(value: i16, ply: u8) -> i32 {
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