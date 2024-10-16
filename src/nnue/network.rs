use std::alloc::{self, alloc_zeroed, dealloc, Layout};
use std::arch::x86_64::*;
use std::{ptr, usize};


use crate::board::{piece::Piece, state::board::Board};
use crate::color::{Color, Color::*};
use crate::nnue::PARAMS;
use crate::squares::Square;

use super::accumulator::{QA, QAB};
use super::accumulator_ptr::AccumulatorPtr;
use super::halfka_idx;
use super::{accumulator::Accumulator, accumulator::Feature, align64::Align64};

pub(crate) const MAX_DEPTH: usize = 127;
pub(crate) const SCALE: i32 = 400;


/// M: is input_size(768 for halfKPA) * l1_size(1024 for HalfKPA in this case)
/// N: is L1_SIZE
/// P: L1_SIZE * 2
/// T: Expected type of the weight/bias
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct NNUEParams<const M: usize, const N: usize, const P: usize, T: Copy> {
    pub(crate) input_weight: Align64<[T; M]>,
    pub(crate) input_bias: Align64<[T; N]>,

    pub(crate) output_weights: [i16; P],
    pub(crate) output_bias: i16,
}


/// U is the size of L1 in this case (i.e. (768*2) -> 1024 -> 1 model), that would be 1024
#[derive(Debug)]
pub(crate) struct NNUEState<T, const U: usize> {
    accumulators: AccumulatorPtr<T, U>,
    current_acc: usize,
}

impl<T, const U: usize>  NNUEState<T, U> {
    fn new() -> Self {
        let layout = Layout::array::<Accumulator<T, U>>(MAX_DEPTH + 1).unwrap();
        let ptr = unsafe {alloc_zeroed(layout) as *mut Accumulator<T, U>};
        if ptr.is_null() {
            alloc::handle_alloc_error(layout);
        }

        NNUEState {
            accumulators: AccumulatorPtr(ptr), // Initializer with raw pointer
            current_acc: 0
        }
    }
}

impl<T, const U: usize> Clone for NNUEState<T, U> {
    fn clone(&self) -> Self {
        unsafe {
            let layout = Layout::array::<Accumulator<T, U>>(MAX_DEPTH + 1).unwrap();
            let ptr = alloc_zeroed(layout) as *mut Accumulator<T, U>;
            if ptr.is_null() {
                alloc::handle_alloc_error(layout);
            }

            std::ptr::copy_nonoverlapping(*self.accumulators, ptr, MAX_DEPTH + 1);

            Self { accumulators: AccumulatorPtr(ptr), current_acc: self.current_acc }
        }
    }
}

impl<T, const U: usize> Drop for NNUEState<T, U> {
    fn drop(&mut self) {
        unsafe {
            let layout = Layout::array::<Accumulator<T, U>>(MAX_DEPTH + 1).unwrap();
            let ptr = self.accumulators.0;
            if !ptr.is_null() {
                dealloc(ptr as *mut u8, layout);
            }
        }
    }
}

impl<const U: usize> From<&Board> for NNUEState<Feature, U> {
    fn from(board: &Board) -> Self {
        let mut state = NNUEState::<Feature, U>::new();

        unsafe {
            let acc = Accumulator::refresh(&board);
            let target = state.accumulators.add(0);

            ptr::write(target, acc);
        }
        state.current_acc = 0;

        
        state
    }
}


impl<const U: usize> NNUEState<Feature, U> {
    pub(crate) fn pop(&mut self) {
        self.current_acc -= 1;
    }

    /// Increases the curr_acc index, and copies the previous accumualator to the new_idx
    pub(crate) fn push(&mut self) {
        unsafe {
            *self.accumulators.add(self.current_acc + 1) = *self.accumulators.add(self.current_acc);
            self.current_acc += 1;
        }
    }

    pub(crate) fn update(&mut self, removed: Vec<(Piece, Square)>, added: Vec<(Piece, Square)>) {
        unsafe {
            let acc = *(self.accumulators.add(self.current_acc));
            let added = added.into_iter().map(|(p, sq)| halfka_idx(p, sq)).collect::<Vec<_>>();
            let removed = removed.into_iter().map(|(p, sq)| halfka_idx(p, sq)).collect::<Vec<_>>();
            
            let new_acc = acc.update(removed, added);
            self.current_acc += 1;
            *self.accumulators.add(self.current_acc) = new_acc;
        }
    }

    pub(crate) fn refresh(&mut self, board: &Board) {
        unsafe {
            let acc = Accumulator::refresh(board);
            self.current_acc = 0;
            *self.accumulators.add(self.current_acc) = acc;
        }
    }

    /// Register size for AVX2
    pub(crate) const REGISTER_WIDTH: usize = 256/16; 
    
    /// The input here are 16 *i16s per m156i 
    pub(crate) unsafe fn propagate(inputs: [Align64<[Feature; U]>; 2], stm: &Color) -> i32 {
        assert!(U%16 == 0, "We're ecpecting i16 values");        
        let mut output: i32 = 0;

        let colors = match stm {
            White => [White, Black],
            Black => [Black, White],
            _ => unreachable!("Unrecognized player")
        };


        for color in colors {
            for i in 0..U {
                let w_idx = ((color as usize) * (PARAMS.output_weights.len()/2)) + (i * Self::REGISTER_WIDTH);

                let data = _mm256_load_si256(inputs[color].as_ptr().add(i) as *const __m256i);
                let weights = _mm256_load_si256(PARAMS.output_weights.as_ptr().add(w_idx) as *const __m256i);

                let datalo = _mm256_cvtepi16_epi32(_mm256_castsi256_si128(data));
                let multiplier_lo = _mm256_cvtepi16_epi32(_mm256_castsi256_si128(weights));

                let datahi = _mm256_cvtepi16_epi32(_mm256_extracti128_si256(data, 1));
                let multiplier_hi = _mm256_cvtepi16_epi32(_mm256_extracti128_si256(weights, 1));
             
                let result_lo = _mm256_mullo_epi32(datalo, multiplier_lo);
                let result_hi = _mm256_mullo_epi32(datahi, multiplier_hi);
                
                let r_lo: [i32; 8] = std::mem::transmute(result_lo);
                let r_hi: [i32; 8] = std::mem::transmute(result_hi);

                output += r_hi.iter().sum::<i32>();
                output += r_lo.iter().sum::<i32>();
            }
        }
        output
    }

    pub(crate) fn evaluate(&self, stm: Color) -> i32 {
        unsafe {
            let acc = self.accumulators.add(self.current_acc);
            
            let clipped_acc = (*acc).sq_crelu16(stm); // [i16; 16]
            let output = Self::propagate(clipped_acc, &stm);

            let abc = (output / (QA as i32) + PARAMS.output_bias as i32) * SCALE / QAB;
            return abc;
        }
    }
}