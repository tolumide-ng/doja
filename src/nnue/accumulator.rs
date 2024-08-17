use crate::color::Color;

use super::{commons::{Align64, MovedPiece}, network::{PovUpdate, UpdatedBuffer, L1_SIZE}};

pub mod avx2;
pub mod others;


/// Activations of the hidden layer
#[derive(Debug, Clone)]
pub struct Accumulator {
    pub white: Align64<[i16; L1_SIZE]>,
    pub black: Align64<[i16; L1_SIZE]>,

    pub mv: MovedPiece,   
    pub update_buffer: UpdatedBuffer,
    pub correct: [bool; 2]
}

impl Accumulator {
    /// Initializes the accumulator with the given bias
    pub fn init(&mut self, bias: Align64<[i16; L1_SIZE]>, update: PovUpdate) {
        if update.white {
            self.white = bias.clone();
        }
        if update.black {
            self.black = bias.clone();
        }
    }

    /// Select the buffer by color
    pub fn select_mut(&mut self, color: Color) -> &mut Align64<[i16; L1_SIZE]> {
        match color {
            Color::White => &mut self.white,
            Color::Black => &mut self.black,
            _ => {}
        }
    }
}


unsafe fn slice_to_aligned<'a>(slice: &'a [i16]) -> &'a Align64<[i16; L1_SIZE]> {
    // don't immediately cast to Align64, as we want to check the alignment first
    let ptr = slice.as_ptr();
    debug_assert_eq!(ptr.align_offset(64), 0);
    // alignments are sensible, so we can safely cast
    &*ptr.cast()
}

