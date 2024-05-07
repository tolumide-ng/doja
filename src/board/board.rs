use std::ops::{Deref, DerefMut};

use crate::Bitboard;

pub struct Board([Bitboard; 12]);

impl Deref for Board {
    type Target = [Bitboard; 12];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Board {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0   
    }
}


impl Board {
    pub fn new() -> Self {
        Self([Bitboard::new(); 12])
    }

    pub(crate) fn get(&self, index: usize) -> &Bitboard {
        &self.0[index]
    }
}

