use crate::{bit_move::Move, constants::params::MAX_DEPTH};

pub(crate) struct PVTable {
    /// Triangular PV-Table
    pv: [u16; MAX_DEPTH * MAX_DEPTH],
    /// Lengths of each PV lines (based on depth search)
    lengths: [usize; MAX_DEPTH],
}


impl PVTable {
    pub(crate) fn new() -> Self {
        Self { pv: [0; MAX_DEPTH * MAX_DEPTH], lengths: [0; MAX_DEPTH] }
    }

    #[inline(always)]
    const fn index(depth: usize) -> usize {
        MAX_DEPTH * depth
    }

    /// Store a move at a specific depth on the PV-Table
    pub(crate) fn store_pv(&mut self, depth: usize, mv: &Move) {
        // Prepends the new move to the PVs at this depth("d")
        let index = Self::index(depth);
        self.pv[index] = **mv;

        let prev_depth_index = Self::index(depth + 1);
        let len = self.lengths[depth + 1];

        // Copy the PV from depth + 1 into this depth
        for i in 0..len {
            self.pv[index] = self.pv[prev_depth_index + i];
        }

        self.lengths[depth] = len + 1;
    }

    pub(crate) fn get_pv(&self, depth: usize) -> &[u16] {
        let index = Self::index(depth);
        &self.pv[index.. index + self.lengths[depth]]
    }

    /// Clear PV lines (when starting a new search)
    #[inline(always)]
    pub(crate) fn clear(&mut self) {
        self.lengths.fill(0);
    }
}