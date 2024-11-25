use crate::{constants::params::MAX_DEPTH, move_logic::bitmove::Move};

#[derive(Debug, Clone)]
pub(crate) struct PVTable {
    pub(crate) length: usize,
    /// The moves here are arranged in the opposite order.
    pub(crate) mvs: [u16; MAX_DEPTH],
    // /// used internally only to know where we are when using the iterator
    // at: usize
}

impl Default for PVTable {
    fn default() -> Self {
        Self { length: 0, mvs: [0; MAX_DEPTH] }
    }
}

impl PVTable {
    pub(crate) fn update(&mut self, mv: Move, old: &Self) {
        self.length = old.length + 1;
        self.mvs[0] = *mv;
        self.mvs[1..=old.length].copy_from_slice(&old.mvs[..old.length]);
    }

    pub(crate) fn mvs(&self) -> &[u16; MAX_DEPTH] {
        &self.mvs
    }
}

impl std::fmt::Display for PVTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut pv = String::from("");

        for m in &self.mvs[0..self.length] {
            pv = format!("{} {}", pv, Move::from(*m));
        }
        write!(f, "{}", pv)
    }
}