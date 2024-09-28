use std::sync::atomic::Ordering;

use crate::{bit_move::Move, constants::MATE_SCORE};

use super::{entry::{SMPData, TTEntry}, flag::HashFlag};


/**
 * Transposition Table
 * https://web.archive.org/web/20071031100051/http://www.brucemo.com/compchess/programming/hashing.htm
 * 
 * keys:
 * PV -- means Principal Variation
 */


 /// 4MegaByte
//  pub(crate) const BYTES_PER_MB: usize = 0x400000;
//  pub(crate) const BYTES_PER_MB: usize = 0x10000; // 1MB
pub(crate) const BYTES_PER_MB: usize = 0x10000; // 1MB





/// Transposition Table
#[derive(Debug)]
pub(crate) struct TTable {
   table: Box<[Option<TTEntry>; BYTES_PER_MB]>, // we need to be able to dynamically allocate this in the future, see CMK's method on Video 88
   entries: usize,
}

const TT_ENTRY: Option<TTEntry> = None;
impl Default for TTable {
   fn default() -> Self {
       Self {
           table: Box::new([TT_ENTRY; BYTES_PER_MB]),
           entries: 0
       }
   }
}


impl TTable {
   pub(crate) fn probe(&self, zobrist_key: u64, depth: u8, alpha: i32, beta: i32, ply: usize) -> Option<i32> {
       let index = zobrist_key as usize % BYTES_PER_MB;
        let phahse = &self.table[index];
        // we can turst the #[default] implementation to work without any issue because the default key is 0,
        // and that would likely not match any zobtist key

        let Some(entry) = phahse else {return None};
        let data = SMPData::from(entry.smp_data.load(Ordering::Relaxed));

        let test_key = zobrist_key ^ u64::from(data);
        if test_key == entry.smp_key {
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

   pub(crate) fn record(&mut self, zobrist_key: u64, depth: u8, score: i32, ply: usize, flag: HashFlag, age: u8, mv: Option<Move>) {
       let index = zobrist_key as usize % BYTES_PER_MB;
       
       let mut replace = false;

       if let Some(entry) = &self.table[index] {
           let data = SMPData::from(entry.smp_data.load(Ordering::Relaxed));
            if entry.age < age || data.depth <= depth { replace = true;}
        } else {
            replace = true;
        }
        
        if replace == false { return }
    
    let value = if score < -MATE_SCORE { score - (ply as i32)} else if score > MATE_SCORE  { score + (ply as i32) } else { score };
    let ptr = self.table.as_mut_ptr();
       unsafe {
            *ptr.add(index) = Some(TTEntry::new(zobrist_key, age, depth, value, mv, flag))
       }
       self.entries += 1;
   }
}

