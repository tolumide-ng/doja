use std::fmt::Display;

use arrayvec::ArrayVec;

use crate::{board::piece::Piece, color::Color, squares::Square};

pub mod feature;

/// The size of the input layer of the network.
pub const INPUT: usize = 768;
/// The amount to scale the output of the network by.
/// This is to allow for the sigmoid activation to diffrentiate positions 
/// with a small difference in evaluation.
const SCALE: i32 = 400;
/// The size of one-half of the hidden layer of the network.
pub const L1_SIZE: usize = 2048;



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FeatureUpdate {
    pub sq: Square,
    pub piece: Piece
}

impl Display for FeatureUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{piece} on {sq}", piece = self.piece, sq = self.sq)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct UpdatedBuffer {
    add: ArrayVec<FeatureUpdate, 2>,
    sub: ArrayVec<FeatureUpdate, 2>,
}

impl UpdatedBuffer {
    pub fn move_piece(&mut self, from: Square, to: Square, piece: Piece) {
        self.add.push(FeatureUpdate { sq: to, piece: piece });
        self.sub.push(FeatureUpdate {sq: from, piece })
    }

    pub fn clear_piece(&mut self, sq: Square, piece: Piece) {
        self.sub.push(FeatureUpdate{sq, piece})
    }

    pub fn add_piece(&mut self, sq: Square, piece: Piece) {
        self.add.push(FeatureUpdate { sq, piece })
    }

    pub fn adds(&self) -> &[FeatureUpdate] {
        &self.add[..]
    }

    pub fn subs(&self) -> &[FeatureUpdate] {
        &self.sub[..]
    }
}


#[derive(Debug, Copy, Clone)]
pub struct PovUpdate {
    pub white: bool, pub black: bool,
}

impl PovUpdate {
    pub const BOTH: Self = Self { white: true, black: true };

    pub const fn color(color: Color) -> Self {
        match color {
            Color::White => Self { white: true, black: flse },
            Color::Black => Self { white: false, black: true },
             _ => {}
        }
    }
}