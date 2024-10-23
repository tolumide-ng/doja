use std::sync::atomic::{AtomicU64, AtomicU8, Ordering};

use crate::{move_logic::bitmove::Move, constants::MATE_SCORE, tt::flag::HashFlag};

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
pub(crate) struct SMPData {
    pub(crate) flag: HashFlag,
    pub(crate) depth: u8,
    eval: i16,
    score: i16,
    pub(crate) mv: Option<Move>,
    pub(super) key: u64,
 }

impl From<SMPData> for u64 {
    fn from(value: SMPData) -> Self {
        let SMPData { depth, flag, score, mv, eval, .. } = value;
        let mv = mv.unwrap_or(Move::from(0));
        // should everything here be in i64, since the mvs are already in i64
        let result = (flag as u64) << FLAG_OFFSET | (depth as u64) << DEPTH_OFFSET | (*mv as u64) << MV_OFFSET | (score as u64) << SCORE_OFFSET
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



impl From<u64> for SMPData {
    fn from(value: u64) -> Self {
        let mv = Move::from(((value & MV_MASK) >> MV_OFFSET) as u16);
        let mv: Option<Move> = Option::from((*mv > 0).then(|| mv));

        SMPData { 
            depth: ((value & DEPTH_MASK) >> DEPTH_OFFSET) as u8, 
            flag: HashFlag::from(((value & FLAG_MASK) >> FLAG_OFFSET) as u8) , 
            score: ((value & SCORE_MASK) >> SCORE_OFFSET) as i16, 
            eval: ((value & EVAL_MASK) >> EVAL_OFFSET) as i16,
            mv, 
            key: 0, 
         }
    }
}


impl TTEntry {
    pub(crate) fn new(key: u64, age: u8, depth: u8, score: i16, eval: i16, mv: Option<Move>, flag: HashFlag) -> Self {
        let smp_data = AtomicU64::new(SMPData::new(key, depth, score, eval, mv, flag).into());

        let smp_key = key ^ smp_data.load(Ordering::Relaxed);
        Self { age: AtomicU8::new(age), smp_key: AtomicU64::new(smp_key), smp_data }
    }

    pub(crate) fn write(&self, key: u64, age: u8, depth: u8, score: i16, eval: i16, mv: Option<Move>, flag: HashFlag) {
        let smp_data = SMPData::new(key, depth, score, eval, mv, flag);

        let smp_key = key ^ u64::from(smp_data);

        self.smp_data.store(smp_data.into(), Ordering::SeqCst);
        self.smp_key.store(smp_key, Ordering::SeqCst);
        self.age.store(age, Ordering::SeqCst);
    }
}


impl SMPData {
    fn new(key: u64, depth: u8, score: i16, eval: i16, mv: Option<Move>, flag: HashFlag) -> Self {
        Self {key, depth, score, eval, mv, flag}
    }

    pub fn score(&self, ply: usize) -> i32 {
        let score = self.score as i32;
        let result =  if score < -MATE_SCORE {score + (ply as i32)} else if score > MATE_SCORE {score - (ply as i32)} else {score};

        return result
    }

    pub(crate) fn eval(&self) -> i32 { self.eval as i32 }
}