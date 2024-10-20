// when the king moves, the accumulator is refreshed

use std::arch::x86_64::*;
use std::ops::{Index, IndexMut};


use crate::board::piece::Piece;
use crate::board::position::ACCUMULATOR_SIZE;
use crate::board::state::board::Board;
use crate::color::Color::*;
use crate::color::Color;
use crate::squares::Square;

use super::align64::Align64;
use super::feature_idx::FeatureIdx;
use super::{halfka_idx, PARAMS};


//  // the size of bias is always the L1 size
//  // if U is 64(values), and Feature is _m256i(16 * i16), where mem::sizeof<m256i>() = 32, then that would be (32/2) * 64 = 1024 i.e. ((mem_size/2) * U)
//  // U = 1024, because Feature is i16, where std::mem::sizof::<i16>() = 2 then  (2/2) * 1024 = 1024
//  // 
//  let bias_len = (std::mem::size_of::<__m256i>()/2) * U;
//  println!("B len is {}", bias_len);
//  let mut accumulator = Self { white: Align64([_mm256_setzero_si256(); U]), black: Align64([_mm256_setzero_si256(); U]) };     
//  for color in [Color::White, Color::Black] {
//      for b in 0..bias_len {
//          // 
//      }
//  }


// A single AVX2 register can fit 16 i16 values, and there are 16AVX2 registers (32 since AVX512) (stockfish docs)

pub(crate) type Feature = __m256i;
pub(crate) const QA: i16 = 255;
pub(crate) const QAB: i32 = 255*64;

/// we're working with accumulator values as i16(16bits)
/// Where our Accumualtor has a data type of _m256i, each _m256i would contains 16 i16 values (i.e 16 * i16 = 256)
/// for a layer that should have 1024(L1_SIZE or U) neurons(values), we only need 1024/16 = 64;
/// so, the length of each side of our accumulator when using _m256i only needs to be 64
#[derive(Debug, Clone, Copy)]
#[repr(align(64))]
pub(crate) struct Accumulator<T, const U: usize> {
    pub(crate) white: Align64<[T; U]>,
    pub(crate) black: Align64<[T; U]>,
}



// In order to ensure that refresh must always be called first, I should remove this soon
impl<const U: usize> Default for Accumulator<Feature, U> {
    fn default() -> Self {
        unsafe { Self { white: Align64([_mm256_setzero_si256(); U]), black: Align64([_mm256_setzero_si256(); U]) } }
    }
}


/// U is the L1 Size (i.e number of first output)
impl<const U: usize> Accumulator<Feature, U> {
    const REGISTER_WIDTH: usize = 256/16; // 16

    /// REGS_LEN: is the length of two accumualtors used here
    unsafe fn update_weights<const ON: bool, const REGS_LEN: usize>(regs: &mut [__m256i; REGS_LEN], idx: (FeatureIdx, FeatureIdx)) {
        let (white_idx, black_idx) = idx;
        // we can only load 16 i16(16bits) value at a time, so we must perform this operation 1024(size of U)/16(bits) i.e. => 64 times

        // There are U(in this case 1024) weights per idx
        // so, we load the 1024 weights(from feature_weights) related to this idx(white, black)
        // and depending on the value of ON (true or false), we add or remove the weights from the current accumulator
        for (color, color_idx) in [white_idx, black_idx].into_iter().enumerate() {
            for i in 0..U {
                // would load weighs @w_idx..w_idx+16 (up until we've read all 1024 of them, which means this would run 64 times)                
                let weights_idx = *color_idx + (i * Self::REGISTER_WIDTH);
                // because we know that REGS_LEN is basically U * 2, where the first half is white, and the second half is black
                let regs_idx = (color  * (REGS_LEN/2)) + i;

                // the interval for weights needs to be a multiple of 16, since weights is actually a bunch of i16 values, i.e 16 * i16 = 256
                let weights_at_i = *(PARAMS.input_weight.as_ptr().add(weights_idx) as *const __m256i);
                // Get the value on the accumulator at this index
                let values_at_i = *regs.as_ptr().add(regs_idx);
                
                let result = match ON {
                    true => _mm256_add_epi16(weights_at_i, values_at_i),
                    false => _mm256_sub_epi16(values_at_i, weights_at_i),
                };

                _mm256_store_si256(regs.as_mut_ptr().add(regs_idx), result);
            }
        }
    }

    pub(crate) unsafe fn refresh(board: &Board) -> Self {
        let mut acc = Accumulator::default();
        
        // U would normally be equal to ACCUMULATOR_SIZE, since rust doesn't allowing using generics in const operations
        const REGS_LEN: usize = ACCUMULATOR_SIZE * 2;
        // const R_LEN: usize = 2 * U;
        let mut regs: [__m256i; REGS_LEN] = [std::mem::zeroed(); REGS_LEN];

        // Load the bias into the self
        for color in [Color::White, Color::Black] {
            for i in 0..U {
                let regs_idx = (ACCUMULATOR_SIZE * color as usize) + i;
                *regs.as_mut_ptr().add(regs_idx) = _mm256_load_si256(PARAMS.input_bias.as_ptr().add(i * Self::REGISTER_WIDTH) as *const __m256i);
            }
        }

        let bitmaps = &board.board;
        
        // identifies the pieces present on the board, and represents them on the the accumulator
        for (p, board) in (*bitmaps).into_iter().enumerate() {
            let mut sqs: u64 = *board;

            while sqs != 0 {
                let sq = Square::from(sqs.trailing_zeros() as u8);
                let piece = Piece::from(p as u8);
                // we need to update black, and white's perspective
                let (white, black) = halfka_idx(piece, sq);

                sqs &= sqs -1;
                Self::update_weights::<true, REGS_LEN>(&mut regs, (white, black));
            }
        }

        // move the computed data into the accumulator
        for color in [Color::White, Color::Black].into_iter() {
            for index in 0..ACCUMULATOR_SIZE {
                let regs_idx = (color as usize * ACCUMULATOR_SIZE) + index;

                // println!("regs_index is {regs_idx}, and index is {index}");
                *acc[color].as_mut_ptr().add(index) = regs[regs_idx];
            }
        }
        acc
    }

    
    pub(crate) unsafe fn update(&self, removed_features: Vec<(FeatureIdx, FeatureIdx)>, added_features: Vec<(FeatureIdx, FeatureIdx)>) -> Self {
        const REGS_LEN: usize = ACCUMULATOR_SIZE * 2;

        let mut regs: [__m256i; REGS_LEN] = [std::mem::zeroed(); REGS_LEN];

        for idx in removed_features.into_iter() {
            Self::update_weights::<false, REGS_LEN>(&mut regs, idx);
        }

        for idx in added_features {
            Self::update_weights::<true, REGS_LEN>(&mut regs, idx);
        }


        let mut acc = self.clone();
        // move the computed data into the accumulator
        for color in [Color::White, Color::Black].into_iter() {
            for index in 0..ACCUMULATOR_SIZE {
                let regs_idx = (color as usize * ACCUMULATOR_SIZE) + index;

                // println!("regs_index is {regs_idx}, and index is {index}");
                if color == White {
                    *acc.white.as_mut_ptr().add(index) = regs[regs_idx];
                } else {
                    *acc.black.as_mut_ptr().add(index) = regs[regs_idx];
                }
            }
        }
    
        acc
    }

    pub(crate) unsafe fn crelu16(&self, stm: Color) -> [[__m256i; U]; 2] {
        const IN_REGISTER_WIDTH: usize = 256/16;
        const OUT_REGISTER_WIDTH: usize = 256/8;
        assert_eq!(U % OUT_REGISTER_WIDTH, 0, "We're processing 32 elements at a time");
        let num_out_chunks = U/OUT_REGISTER_WIDTH;

        let input = if stm == Color::White {[self.white, self.black]} else {[self.black, self.white]};
        let mut output: [[__m256i; U]; 2] = [[_mm256_setzero_si256(); U]; 2];

        let zero = _mm256_setzero_si256();
        const CONTROL: i32 = 0b11011000; // 3, 1, 2, 0; lane 0 is the rightmost one

        for i in 0..num_out_chunks {
            for color in [Color::White, Color::Black] {
                let curr_input = *(input.as_ptr().add(color as usize));
                let in0 = _mm256_load_si256(curr_input.as_ptr().add((i * 2 + 0) * IN_REGISTER_WIDTH));
                let in1 = _mm256_load_si256(curr_input.as_ptr().add((i * 2 + 1) * IN_REGISTER_WIDTH));
                
                let result = _mm256_permute4x64_epi64(_mm256_max_epi8(_mm256_packs_epi16(in0, in1), zero), CONTROL);

                _mm256_store_si256(output.as_mut_ptr().add(i * OUT_REGISTER_WIDTH) as *mut __m256i, result);
            }
        }

        output
    }

    /// Loads input(16 i16 values), and 
    ///     1. Ensures that the max of the input is i8(127) (saturates the input)
    ///     2. And ensures that the min of the input is 0 (i8)
    /// Then multiplies the input(now 16 i8 values) with the weights(16 i16 values)
    /// to generates 16 i16 values (the output)
    pub(crate) unsafe fn sq_crelu16(&self, stm: Color) -> [Align64<[__m256i; U]>; 2] { // U is 1024
        let input = if stm == Color::White {[self.white, self.black]} else {[self.black, self.white]};
        let mut output: [Align64<[__m256i; U]>; 2] = [Align64([_mm256_setzero_si256(); U]); 2];  // [[_; 1024]; 2];

        let min = _mm256_set1_epi16(0);
        let max = _mm256_set1_epi16(255);

        
        // const CONTROL: i32 = 0b11011000; // 3, 1, 2, 0; lane 0 is the rightmost one
        
        for color in [Color::White, Color::Black] {
            for i in 0..U {
                let in0 = _mm256_load_si256(input[color].as_ptr().add(i)); // loads 16 i16 values from curr_input 

                let clamped_min = _mm256_max_epi16(in0, min);
                // despite being inside (i16 * 16) data structure, these are just (i8 * 16) values
                // the reason for this, is to still be capable of returning (i16 * 16) values after the squaring (i.e multiplication)
                let clamped_max = _mm256_min_epi16(clamped_min, max);
                // // y = Ax + b
                let result = _mm256_mullo_epi16(clamped_max, clamped_max); // square
                _mm256_store_si256(output[color as usize].as_mut_ptr().add(i) as *mut __m256i, result);
            }
        }

        output
    }
}


impl<T, const U: usize> Index<Color> for Accumulator<T, U> {
    type Output = [T; U];

    fn index(&self, index: Color) -> &Self::Output {
        match index {
            Color::White => &self.white,
            Color::Black => &self.black,
            _ => panic!("unrecognized color")
        }
    }
}



impl<T, const U: usize> IndexMut<Color> for Accumulator<T, U> {
    fn index_mut(&mut self, index: Color) -> &mut Self::Output {
        match index {
            Color::White => &mut self.white,
            Color::Black => &mut self.black,
            _ => panic!("unrecognized color")
        }
    }
} 