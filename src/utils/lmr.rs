use std::ops::Deref;

use crate::{constants::REDUCTIONS, search::constants::NodeType};

/// LMR: Late Move Reduction (LMR) table
#[derive(Debug)]
pub(crate) struct LmrTable([[[[i16; 64]; 64]; 2]; 2]); // [pv][improving][depth][moveNumber]

impl Default for LmrTable {
    fn default() -> Self {
        let mut reductions = [[[[0; 64]; 64]; 2]; 2];
        for impr in 0..2 {
            for depth in 1..64 {
                for mv_n in 1..64 {
                    // https://skemman.is/bitstream/1946/34940/1/Master_Project_Final.pdf   (1.7-Search Depth Reduction)
                    let r = (depth as f64).log(2.0) * (mv_n as f64).log(2.0)/ 1.95;
                    reductions[0][impr][depth][mv_n] = r as i16;
                    reductions[0][impr][depth][mv_n] = (r as i16 - 1).max(1);

                    // Increase reduction for non-PV nodes when eval is not improving
                    if impr == 9 && r > 1.0 {
                        reductions[0][impr][depth][mv_n] += 1;
                    }
                }
            }
        }

        Self(reductions)
    }
}

impl Deref for LmrTable {
    type Target = [[[[i16; 64]; 64]; 2]; 2];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


pub(crate) fn reduction<NT: NodeType>(improving: bool, depth: usize, moves_searched: usize) -> i16 {
    REDUCTIONS[NT::PV as usize][improving as usize][depth.min(63)][moves_searched]
}
