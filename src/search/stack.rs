// use crate::{move_logic::bitmove::Move, board::piece::Piece};

// #[derive(Debug, Clone, Copy, Default)]
// pub(crate) struct SearchE {
//     pub(crate) ply: usize,
//     /// The piece that was moved
//     pub(crate) piece: Option<Piece>,
//     /// What move was made
//     pub(crate) mv: Option<Move>,
//     pub(crate) score: i32,
//     pub(crate) skipped: Option<Move>,
//     pub(crate) eval: i32,
// }

use std::{array, ops::{Deref, DerefMut}};

use crate::{constants::MAX_PLY, move_logic::bitmove::Move};



#[derive(Debug, Default)]
pub(crate) struct StackItem {
    pub(crate) eval: i32,
    pub(crate) best_move: Option<Move>,
    pub(crate) excluded: Option<Move>,
}


#[derive(Debug)]
pub(crate) struct Stack([StackItem; MAX_PLY + 10]);

impl Deref for Stack {
    type Target = [StackItem; MAX_PLY + 10];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


impl DerefMut for Stack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self(array::from_fn(|_| StackItem::default()))
    }
}