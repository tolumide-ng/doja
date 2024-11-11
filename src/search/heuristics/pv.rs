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

    // pub(crate) fn push(&mut self, mv: &Move) {
    //     self.mvs[self.length] = **mv;
    //     self.length += 1;
    // }

    pub(crate) fn mvs(&self) -> &[u16; MAX_DEPTH] {
        &self.mvs
    }
}


// impl Iterator for PVTable {
//     type Item = Move;

//     /// We're looping backwards in this implementation ("The user's don't need to know that")
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.at < (self.length-1) {
//             let index = self.length - self.at - 1;
//             self.at += 1;
//             return Some(Move::from(self.mvs[index]));
//         }
//         None
//     }
// }