use std::sync::atomic::{AtomicU64, Ordering};

use crate::{bit_move::Move, tt::flag::HashFlag};



// const AGE_MASK: u64 = 0x7F;
const FLAG_MASK: u64 = 0x3;
const DEPTH_MASK: u64 = 0x1FC;
const MV_MASK: u64 = 0x1FFFE00;
const SCORE_MASK: u64 = 0x1FFFFFFFE000000;
// const KEY_MASK: u64 = 0x1;

  
const FLAG_OFFSET: u64 = 0;
const DEPTH_OFFSET: u64 = 2;
const MV_OFFSET: u64 = 9;
const SCORE_OFFSET: u64 = 25;
// const KEY_OFFSET: u64 = 62;

// 3FFFC0000000 -> score offset


#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
// #[repr(align(64))]
pub(crate) struct SMPData {
    pub(super) key: u64,
    pub(super) depth: u8,
    pub(super) flag: HashFlag,
    pub(super) score: i32,
    pub(super) mv: Option<Move>,
 }

impl From<SMPData> for u64 {
    fn from(value: SMPData) -> Self {
        let SMPData { depth, flag, score, mv, .. } = value;
        let mv = mv.unwrap_or(Move::from(0));
        // should everything here be in i64, since the mvs are already in i64
        let result = (flag as u64) << FLAG_OFFSET | (depth as u64) << DEPTH_OFFSET | (*mv as u64) << MV_OFFSET | (score as u64) << SCORE_OFFSET;
        //  | (key <<  KEY_OFFSET);
        result
    }
 }


 /// Transposition table Entry
#[derive(Debug, Default)]
#[repr(C)]
pub(crate) struct TTEntry {
    pub(super) age: u8, // todo! readup papers on transposition table repalcement schemes
    pub(super) smp_key: u64,
    pub(super) smp_data: AtomicU64,
}





impl From<u64> for SMPData {
    fn from(value: u64) -> Self {
        let mv = Move::from(((value & MV_MASK) >> MV_OFFSET) as u16);
        let mv: Option<Move> = Option::from((*mv > 0).then(|| mv));

        SMPData { 
            depth: ((value & DEPTH_MASK) >> DEPTH_OFFSET) as u8, 
            flag: HashFlag::from(((value & FLAG_MASK) >> FLAG_OFFSET) as u8) , 
            score: ((value & SCORE_MASK) >> SCORE_OFFSET) as i32, 
            mv, 
            key: 0, 
            // age: (value & AGE_MASK) as u8, 
            // value: ((value & VALUE_MASK) >> VALUE_OFFSET) as u16,
            // smp_data: 0, 
            // smp_key: 0
         }
    }
}


impl TTEntry {
    pub(crate) fn new(key: u64, age: u8, depth: u8, score: i32, mv: Option<Move>, flag: HashFlag) -> Self {
        let smp_data = AtomicU64::new(SMPData::new(key, depth, score, mv, flag).into());

        let smp_key = key ^ smp_data.load(Ordering::Relaxed);
        Self { age, smp_key, smp_data }
    }
}


impl SMPData {
    fn new(key: u64, depth: u8, score: i32, mv: Option<Move>, flag: HashFlag) -> Self {
        Self {key, depth, score, mv, flag}
    }

    // fn verify_smp(&self) {
    //     let data = self.key();
    //     let key = self.key ^ data;

    //     if data != self.smp_data {
    //         // data error
    //     }
    //     if key != self.smp_key {
    //         // smp_key error
    //     }
    // }


    // pub(crate) fn record(&mut self, zobrist_key: u64, depth: u8, score: i32, ply: usize, flag: HashFlag) {
    //     let index = zobrist_key as usize % BYTES_PER_MB;
    //     let ptr = self.table.as_mut_ptr();

    //     let value = if score < -MATE_SCORE { score - (ply as i32)} else if score > MATE_SCORE  { score + (ply as i32) } else { score };

    //     unsafe {
    //         // println!("the index is {index}");
    //         (*ptr.add(index)).key = zobrist_key;
    //         // (*ptr.add(index)).best = best;
    //         // (*ptr.add(index)).score = value;
    //         (*ptr.add(index)).flag = flag;
    //         (*ptr.add(index)).depth = depth;
    //     }
    //     self.entries += 1;
    // }
}


impl PartialEq for TTEntry {
    fn eq(&self, other: &Self) -> bool {
        (self.age == other.age) && (self.smp_key == other.smp_key) && (self.smp_data.load(Ordering::Relaxed) == other.smp_data.load(Ordering::Relaxed))
    }
}