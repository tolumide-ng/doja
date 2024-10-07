use std::sync::atomic::Ordering;

use crate::{bit_move::Move, constants::{MATE_SCORE, NO_HASH_ENTRY}};

use super::{entry::{SMPData, TTEntry}, flag::HashFlag, table::TOTAL_SIZE};

/// TPT => Transposition Table (derived)
/// This is the exposed derivation of the original Transposition Table (TT)
#[derive(Debug)]
pub(crate) struct TPT<'a> {
    pub(crate) table : &'a [TTEntry]
}


impl<'a> TPT<'a> {
    pub(crate) fn record(&self, zobrist_key: u64, depth: u8, score: i32, eval: i32, ply: usize, flag: HashFlag, age: u8, mv: Option<Move>) {
        // let index = zobrist_key & (TOTAL_SIZE as u64 -1);
       let index = zobrist_key as usize % TOTAL_SIZE;
       
       let mut replace = false;
       let exists =  self.table[index].smp_data.load(Ordering::Relaxed) != 0;

       if exists {
        let entry = &self.table[index];
           let data = SMPData::from(entry.smp_data.load(Ordering::Relaxed));
            if entry.age.load(Ordering::Relaxed) < age || data.depth <= depth { replace = true;}
        } else {
            replace = true;
        }
        
        if replace == false { return }
    
        let value = if score < -MATE_SCORE { score - (ply as i32)} else if score > MATE_SCORE  { score + (ply as i32) } else { score };
        unsafe {
            let ptr = self.table.as_ptr();
            (*ptr.add(index)).write(zobrist_key, age, depth, value as i16, eval as i16, mv, flag);
       }
    }

    pub(crate) fn probe(&self, zobrist_key: u64) -> Option<SMPData> { 
        // let index = zobrist_key & (TOTAL_SIZE as u64 -1);
        let index = zobrist_key as usize % TOTAL_SIZE;
        let entry = &self.table[index];
        // we can turst the #[default] implementation to work without any issue because the default key is 0,
        // and that would likely not match any zobtist key

        let atomic_data = entry.smp_data.load(Ordering::Relaxed);
        if atomic_data == 0 {return None};

        let data = SMPData::from(atomic_data);
        if zobrist_key ^ u64::from(data) == entry.smp_key.load(Ordering::Relaxed) {
            return Some(data)
        }

        None
    }
}