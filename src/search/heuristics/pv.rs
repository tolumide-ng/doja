use crate::{constants::params::MAX_DEPTH, move_logic::bitmove::Move};

#[derive(Debug, Clone)]
pub(crate) struct PVTable {
    pub(crate) length: usize,
    pub(crate) mvs: [u16; MAX_DEPTH]
}

impl Default for PVTable {
    fn default() -> Self {
        Self { length: 0, mvs: [0; MAX_DEPTH] }
    }
}

impl PVTable {
    pub(crate) fn update(&mut self, mv: &Move, old: &Self) {
        self.length = old.length + 1;
        self.mvs[0] = **mv;
        self.mvs[1..=old.length].copy_from_slice(&old.mvs[..old.length]);
    }

    pub(crate) fn mvs(&self) -> &[u16; MAX_DEPTH] {
        &self.mvs
    }
}