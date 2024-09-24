use std::alloc::{self, alloc_zeroed, Layout};
use std::arch::x86_64::{__m256i, _mm256_add_epi16, _mm256_load_si256, _mm256_setzero_si256, _mm256_store_si256};

use crate::board::{piece::Piece, state::board::Board};
use crate::color::Color::{self, *};
use crate::nnue::net::{halfka_idx, nnue_index, MODEL};
use crate::nnue_::L1_SIZE;
use crate::squares::Square;

use super::{accumulator::Accumualator, accumulator::Feature, align64::Align64};

pub(crate) const MAX_DEPTH: usize = 127;

#[derive(Debug)] 
#[repr(C)]
pub(crate) struct NNUEParams<const M: usize, const N: usize, const P: usize, T: Copy> {
    // pub(crate) weight: [[T; M]; 2], // where U = 2(colors) * layer's size
    pub(crate) input_weight: Align64<[T; M]>,
    pub(crate) input_bias: Align64<[T; N]>,

    pub(crate) output_weights: [i16; P],
    pub(crate) output_bias: i16,
}

pub(crate) struct NNUEState<T, const U: usize> {
    accumulators: [Accumualator<T, U>; MAX_DEPTH + 1],
    current_acc: usize,
}

impl<const U: usize> From<Board> for NNUEState<Feature, U> {
    fn from(board: Board) -> NNUEState<Feature, U> {
        let mut boxed: Box<Self> = unsafe {
            let layout = Layout::new::<Accumualator<Feature, U>>();
            let ptr = alloc_zeroed(layout);

            if ptr.is_null() {
                alloc::handle_alloc_error(layout);
            }

            Box::from_raw(ptr.cast())
        };

        let acc = unsafe { Accumualator::refresh(&board) };
        boxed.accumulators[0] = acc;
        boxed.current_acc = 0;

        *boxed
    }
}


impl<const U: usize> NNUEState<Feature, U> {
    pub(crate) fn update(&mut self) {}

    pub(crate) unsafe fn evaluate(&self, stm: Color) -> i32 {
        let acc = &self.accumulators[self.current_acc];
        // let (us, them) = if stm == Color::White {(&acc.white, &acc.black)} else {(&acc.black, &acc.white)};
        // let curr_input = if stm == Color::White {[&acc.white, &acc.black]} else {[&acc.black, &acc.white]};

        let clipped_acc = acc.crelu16(stm); // [i8s; 32]

        // loop through each of them `clipped_acc` and multiply with the output_weight, add all of them together;


        
        // let l1_outputs = Align

        0 
    }
}