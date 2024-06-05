use std::ops::{Deref, DerefMut};

use crate::bit_move::BitMove;

/**
 * Transposition Table
 * https://web.archive.org/web/20071031100051/http://www.brucemo.com/compchess/programming/hashing.htm
 * 
 * keys:
 * PV -- means Principal Variation
 */


 /// 4MegaByte
//  pub(crate) const HASH_SIZE: usize = 0x400000;
//  pub(crate) const HASH_SIZE: usize = 0x410000; // 1MB
 pub(crate) const HASH_SIZE: usize = 0x40000; // 4MB


 #[derive(Debug, Default, Clone, Copy)]
//  #[repr(u8)]
pub(crate) enum NodeType {
    /// PV-nodes: have scores inside the window i.e. alpha < score < beta
    #[default]
    Exact = 0,
    /// Beta-cutoff nodes (FailHigh) score >= beta
    LowerBound = 1,
    /// Alpha nodes (FailLow) score < alpha
    UpperBound = 2,
}
 

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct TT {
    /// "almost" unique chess position identifier
    key: u64,
    /// current search depth
    depth: u16,
    /// type of the node (e.g. Failed-low/Failed-high/PV)
    flag: NodeType,
    /// Score (alpha/beta/PV)
    score: i32,
    best: BitMove,
    times: u16
 }



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
    pub(crate) fn get(&self, zobrist_key: u64, depth: u16, alpha: i32, beta: i32) -> Option<i32> {
        let index = zobrist_key as usize % HASH_SIZE;
        let ptr = self.as_ptr();
        println!("retrieving from {}", index);

        
        unsafe {
            let phahse = *ptr.add(index);
            // we can turst the #[default] implementation to work without any issue because the default key is 0,
            // and that would likely not match any zobtist key
            if phahse.key == zobrist_key && phahse.times > 0 { 
                if phahse.depth == depth {
                    match phahse.flag {
                        NodeType::Exact => {
                            // matches exact (PVNode)
                            return Some(phahse.score)
                        }
                        NodeType::UpperBound => {
                            if phahse.score <= alpha {
                                // matches (Fail-low) node
                                return Some(alpha);
                            }
                        }
                        NodeType::LowerBound => {
                            if  phahse.score >= beta {
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

    pub(crate) fn set(&mut self, zobrist_key: u64, best: BitMove, depth: u16, score: i32, flag: NodeType) {
        let index = zobrist_key as usize % HASH_SIZE;
        let ptr = self.as_mut_ptr();

        unsafe {
            println!("the index is {index}");
            // let mut x = ptr.add(index);
            (*ptr.add(index)).key = zobrist_key;
            (*ptr.add(index)).best = best;
            (*ptr.add(index)).score = score;
            (*ptr.add(index)).flag = flag;
            (*ptr.add(index)).depth = depth;
        }
    }
}

