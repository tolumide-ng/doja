use crate::{bit_move::Move, constants::MATE_SCORE};

use super::{entry::TTEntry, flag::HashFlag};


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
#[derive(Debug, Clone)]
pub(crate) struct TTable {
   table: Box<[TTEntry; BYTES_PER_MB]>, // we need to be able to dynamically allocate this in the future, see CMK's method on Video 88
   entries: usize,
}

impl Default for TTable {
   fn default() -> Self {
       Self {
           table: Box::new([TTEntry::default(); BYTES_PER_MB]),
           entries: 0
       }
   }
}


impl TTable {
   pub(crate) fn probe(&self, zobrist_key: u64, depth: u8, alpha: i32, beta: i32, ply: usize, mv: Option<Move>) -> Option<i32> {
       let index = zobrist_key as usize % BYTES_PER_MB;
       let ptr = self.table.as_ptr();
       unsafe {
           let phahse = *ptr.add(index);
           // we can turst the #[default] implementation to work without any issue because the default key is 0,
           // and that would likely not match any zobtist key

           let test_key = zobrist_key ^ phahse.smp_data;
           if test_key == phahse.smp_key { 

                let mv = mv;
               if depth == phahse.smp_data.depth {
                   let score  = phahse.smp_data.score;
                   let value = if score < -MATE_SCORE {score + (ply as i32)} else if score > MATE_SCORE {score - (ply as i32)} else {score};
                   match phahse.smp_data.flag {
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
                   }
               }
           }
       }
       None
   }

   pub(crate) fn record(&mut self, zobrist_key: u64, depth: u8, score: i32, ply: usize, flag: HashFlag, age: u8) {
       let index = zobrist_key as usize % BYTES_PER_MB;
       let ptr = self.table.as_mut_ptr();

       let mut replace = false;

       if self.table[index] == TTEntry::default() {
            replace = true;    
        } else if self.table[index].age < age || self.table[index].smp_data.depth <= depth {
            replace = true;
        }

        if replace == false { return }
    
    let value = if score < -MATE_SCORE { score - (ply as i32)} else if score > MATE_SCORE  { score + (ply as i32) } else { score };
       unsafe {
           // println!("the index is {index}");
        //    (*ptr.add(index)).key = zobrist_key;
        //    (*ptr.add(index)).best = best;
        //    (*ptr.add(index)).score = value;
        //    (*ptr.add(index)).flag = flag;
        //    (*ptr.add(index)).depth = depth;
       }
       self.entries += 1;
   }
}

