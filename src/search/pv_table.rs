use crate::{bit_move::Move, constants::params::MAX_DEPTH};

pub(crate) struct PVTable {
    /// Triangular PV-Table
    pv: [[u16; MAX_DEPTH]; MAX_DEPTH],
    /// Lengths of each PV lines (based on depth search)
    lengths: [usize; MAX_DEPTH],
}


impl PVTable {
    pub(crate) fn new() -> Self {
        Self { pv: [[0; MAX_DEPTH]; MAX_DEPTH], lengths: [0; MAX_DEPTH] }
    }

    /// Store a move at a specific depth on the PV-Table
    pub(crate) fn store_pv(&mut self, depth: usize, mv: &Move) {
        // Prepends the new move to the PVs at this depth("d")
        self.pv[depth][0] = **mv;

        // Copy the PV from depth + 1 into this depth
        for i in 0..self.lengths[depth + 1] {
            self.pv[depth][i+1] = self.pv[depth + 1][i];
        }

        self.lengths[depth] = self.lengths[depth + 1] + 1;
    }

    pub(crate) fn get_pv(&self, depth: usize) -> &[u16] {
        &self.pv[depth][..self.lengths[depth]]
    }

    /// Clear PV lines (when starting a new search)
    pub(crate) fn clear(&mut self) {
        self.lengths.fill(0);
    }
}