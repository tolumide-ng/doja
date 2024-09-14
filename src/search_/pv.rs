use arrayvec::ArrayVec;

use crate::{bit_move::BitMove, constants::MAX_PLY};

#[derive(Debug, Clone)]
pub(crate) struct PVariation {
    pub(crate) score: i32, pub(crate) moves: ArrayVec<BitMove, MAX_PLY>,
}

impl Default for PVariation {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl PVariation {
    const EMPTY: Self = Self {score: 0, moves: ArrayVec::new_const() };

    // pub(crate) fn load_from(&mut self, m: BitMove, rest)
}