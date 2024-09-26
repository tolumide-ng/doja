use std::alloc::{self, alloc_zeroed, dealloc, Layout};
use std::arch::x86_64::{__m256i, _mm256_add_epi16, _mm256_castsi256_si128, _mm256_cvtepi16_epi32, _mm256_extractf128_si256, _mm256_extracti128_si256, _mm256_load_si256, _mm256_mullo_epi16, _mm256_mullo_epi32, _mm256_setzero_si256, _mm256_store_si256};
use std::ptr;

use crate::board::{piece::Piece, state::board::Board};
use crate::color::Color::{self, *};
use crate::nnue::net::MODEL;

use super::accumulator::{QA, QAB};
use super::L1_SIZE;
use super::{accumulator::Accumualator, accumulator::Feature, align64::Align64};

pub(crate) const MAX_DEPTH: usize = 127;
pub(crate) const SCALE: i32 = 400;

#[repr(C)]
#[derive(Debug)]
pub(crate) struct NNUEParams<const M: usize, const N: usize, const P: usize, T: Copy> {
    // pub(crate) weight: [[T; M]; 2], // where U = 2(colors) * layer's size
    pub(crate) input_weight: Align64<[T; M]>,
    pub(crate) input_bias: Align64<[T; N]>,

    pub(crate) output_weights: [i16; P],
    pub(crate) output_bias: i16,
}


#[derive(Debug)]
pub(crate) struct NNUEState<T, const U: usize> {
    // accumulators: [Accumualator<T, U>; MAX_DEPTH + 1],
    accumulators: *mut Accumualator<T, U>,
    current_acc: usize,
}

impl<T, const U: usize>  NNUEState<T, U> {
    fn new() -> Self {
        let layout = Layout::array::<Accumualator<T, U>>(MAX_DEPTH + 1).unwrap();
        let ptr = unsafe {alloc_zeroed(layout) as *mut Accumualator<T, U>};
        if ptr.is_null() {
            alloc::handle_alloc_error(layout);
        }

        NNUEState {
            accumulators: ptr, current_acc: 0
        }
    }
}

impl<T, const U: usize> Drop for NNUEState<T, U> {
    fn drop(&mut self) {
        unsafe {
            let layout = Layout::array::<Accumualator<T, U>>(MAX_DEPTH + 1).unwrap();
            dealloc(self.accumulators as *mut u8, layout);
        }
    }
}

impl<const U: usize> From<Board> for NNUEState<Feature, U> {
    fn from(board: Board) -> Self {
        let mut state = NNUEState::<Feature, U>::new();

        
        unsafe {
            println!("XXXX");
            let acc = Accumualator::refresh(&board);
            let target = state.accumulators.add(0);
            ptr::write(target, acc);


            println!(":::::::::::::::::::::::::::::::: {:?}", (*state.accumulators).white[0]);
            println!(":::::::::::::::::::::::::::::::: {:?}", (*state.accumulators).white[1]);
            println!("\n\n");
            println!(":::::::::::::::::::::::::::::::: {:?}", (*state.accumulators).black[0]);
            println!(":::::::::::::::::::::::::::::::: {:?}", (*state.accumulators).black[1]);
        }
        state.current_acc = 0;

        
        state
    }
}



impl<const U: usize> NNUEState<Feature, U> {
    pub(crate) fn update(&mut self) {}
    
    /// The input here are 16 *i16s per m156i 
    pub(crate) unsafe fn propagate(inputs: [Align64<[Feature; U]>; 2]) -> i32 {
        assert!(U%16 == 0, "We're ecpecting i16 values");
        // where U is 1024, this would be 64, the assumption here is that we are using i16
        // 16 * i16 values would be in a single _m256i, so we can only load 16 values at once
        // hence, the result is the number of times we have to loop
        let num_chunks: usize = U/16;
        const INPUT_REGISTER_WIDTH: usize = 256/16; // 16
        
        let mut output: i32 = 0;

        for color in 0..2 {
            for i in 0..num_chunks {
                let data = _mm256_load_si256(inputs[color].as_ptr().add(i * INPUT_REGISTER_WIDTH) as *const __m256i);
                let w_idx = (color * U) + (i * INPUT_REGISTER_WIDTH);
                let weights = _mm256_load_si256(MODEL.output_weights.as_ptr().add(w_idx) as *const __m256i);

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

    fn material_scale() -> i32 {
        0
    }

    pub(crate) unsafe fn evaluate(&self, stm: Color) -> i32 {
        // let acc = &self.accumulators[self.current_acc];
        let acc = self.accumulators.add(self.current_acc);
        // let (us, them) = if stm == Color::White {(&acc.white, &acc.black)} else {(&acc.black, &acc.white)};
        // let curr_input = if stm == Color::White {[&acc.white, &acc.black]} else {[&acc.black, &acc.white]};

        // let clipped_acc = acc.crelu16(stm); // [i8s; 32]
        let clipped_acc = (*acc).sq_crelu16(stm); // [i16; 16]
        let output = Self::propagate(clipped_acc);

        // let value = (output/QA as i32);
        // let layer_output = output + MODEL.output_bias as i32;

        // let value = layer_output * SCALE;

        // let v = Self::material_scale() / 1024;

        // // let v = v * (200 - )

        return (output/QA as i32 + MODEL.output_bias as i32) * SCALE / QAB;
    }
}