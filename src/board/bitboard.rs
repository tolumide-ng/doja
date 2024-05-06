use std::ops::{Deref, DerefMut};

use crate::Mask;

pub struct Bitboard([Mask; 12]);

impl Deref for Bitboard {
    type Target = [Mask; 12];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Bitboard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0   
    }
}


impl Bitboard {
    pub fn new() -> Self {
        Self([Mask::new(); 12])
    }

    pub(crate) fn get(&self, index: usize) -> &Mask {
        &self.0[index]
    }
}

