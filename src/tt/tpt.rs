use std::sync::atomic::Ordering;

use crate::{bit_move::Move, constants::MATE_SCORE};

use super::{entry::{SMPData, TTEntry}, flag::HashFlag, table::TOTAL_SIZE};

/// TPT => Transposition Table (derived)
/// This is the exposed derivation of the original Transposition Table (TT)
#[derive(Debug)]
pub(crate) struct TPT<'a> {
    pub(crate) table : &'a [TTEntry]
}


impl<'a> TPT<'a> {
    pub(crate) fn record(&self, zobrist_key: u64, depth: u8, score: i32, ply: usize, flag: HashFlag, age: u8, mv: Option<Move>) {
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
            (*ptr.add(index)).write(zobrist_key, age, depth, value, mv, flag);
       }
    }

   pub(crate) fn probe(&self, zobrist_key: u64, depth: u8, alpha: i32, beta: i32, ply: usize) -> Option<i32> { 
        // let index = zobrist_key & (TOTAL_SIZE as u64 -1);
        let index = zobrist_key as usize % TOTAL_SIZE;
        let phahse = &self.table[index];
        // we can turst the #[default] implementation to work without any issue because the default key is 0,
        // and that would likely not match any zobtist key

        // let Some(entry) = phahse else {return None};
        if phahse.smp_data.load(Ordering::Relaxed) == 0 {return None};
        let entry = phahse;

        let data = SMPData::from(entry.smp_data.load(Ordering::Relaxed));

        let test_key = zobrist_key ^ u64::from(data);
        if test_key == entry.smp_key.load(Ordering::Relaxed) {
            if depth == data.depth {
                let score  = data.score;
                let value = if score < -MATE_SCORE {score + (ply as i32)} else if score > MATE_SCORE {score - (ply as i32)} else {score};
                match data.flag {
                    HashFlag::Exact => {
                        // matches exact (PVNode)
                        return Some(value)
                    }
                    HashFlag::UpperBound => {
                        if value <= alpha {
                            // matches (Fail-low) node
                            return Some(alpha);
                        }
                    }
                    HashFlag::LowerBound => {
                        if  value >= beta {
                            // matches (Fail-high) node
                            return Some(beta);
                        }
                    }
                    _ => return None,
                }
            }
        }

        None
    }
}