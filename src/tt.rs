use std::ops::{Deref, DerefMut};

use crate::{bit_move::BitMove, constants::MATE_SCORE};

/**
 * Transposition Table
 * https://web.archive.org/web/20071031100051/http://www.brucemo.com/compchess/programming/hashing.htm
 * 
 * keys:
 * PV -- means Principal Variation
 */


 /// 4MegaByte
//  pub(crate) const HASH_SIZE: usize = 0x400000;
//  pub(crate) const HASH_SIZE: usize = 0x10000; // 1MB
 pub(crate) const HASH_SIZE: usize = 0x10000; // 4MB


 #[derive(Debug, Default, Clone, Copy)]
 #[repr(u8)]
pub(crate) enum HashFlag {
    /// PV-nodes: have scores inside the window i.e. alpha < score < beta
    #[default]
    Exact = 0,
    /// Beta-cutoff nodes (FailHigh) score >= beta
    LowerBound = 1,
    /// Alpha nodes (FailLow) score < alpha
    UpperBound = 2,
}
 


/// Transposition table
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct TT {
    /// "almost" unique chess position identifier
    key: u64,
    /// current search depth
    depth: u8,
    /// type of the node (e.g. Failed-low/Failed-high/PV)
    flag: HashFlag,
    /// Score (alpha/beta/PV)
    score: i32,
    // best: BitMove,
    // age: u16 // todo! readup papers on transposition table repalcement schemes
 }

//  impl 



 /// Transposition Table
 #[derive(Debug, Clone)]
pub(crate) struct TTable(Box<[TT; HASH_SIZE]>);

impl Default for TTable {
    fn default() -> Self {
        Self(Box::new([TT::default(); HASH_SIZE]))
    }
}

impl Deref for TTable {
    type Target = Box<[TT; HASH_SIZE]>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TTable {
    pub(crate) fn probe(&self, zobrist_key: u64, depth: u8, alpha: i32, beta: i32, ply: usize) -> Option<i32> {
        let index = zobrist_key as usize % HASH_SIZE;
        let ptr = self.as_ptr();
        // println!("retrieving from {}", index);

        
        unsafe {
            let phahse = *ptr.add(index);
            // we can turst the #[default] implementation to work without any issue because the default key is 0,
            // and that would likely not match any zobtist key
            if phahse.key == zobrist_key { 
                if phahse.depth == depth {
                    let score  = phahse.score;
                    let value = if score < -MATE_SCORE {score + (ply as i32)} else if score > MATE_SCORE {score - (ply as i32)} else {score};
                    match phahse.flag {
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

    pub(crate) fn record(&mut self, zobrist_key: u64, depth: u8, score: i32, ply: usize, flag: HashFlag) {
        let index = zobrist_key as usize % HASH_SIZE;
        let ptr = self.as_mut_ptr();

        let value = if score < -MATE_SCORE { score - (ply as i32)} 
        else if score > MATE_SCORE  { score + (ply as i32) }
        else { score };



        unsafe {
            // println!("the index is {index}");
            // let mut x = ptr.add(index);
            (*ptr.add(index)).key = zobrist_key;
            // (*ptr.add(index)).best = best;
            (*ptr.add(index)).score = value;
            (*ptr.add(index)).flag = flag;
            (*ptr.add(index)).depth = depth;
        }
    }
}

