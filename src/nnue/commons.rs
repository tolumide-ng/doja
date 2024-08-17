use std::ops::{Deref, DerefMut};

use crate::{board::piece::Piece, squares::Square};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[repr(C, align(64))]
pub struct Align64<T>(pub T);

impl<T, const SIZE: usize> Deref for Align64<[T; SIZE]> {
    type Target = [T; SIZE];
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


impl<T, const SIZE: usize> DerefMut for Align64<[T; SIZE]> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}



#[derive(Debug, Clone, Copy)]
pub struct MovedPiece {
    pub from: Square,
    pub to: Square,
    pub piece: Piece
}