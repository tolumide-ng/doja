use std::alloc::{self, alloc_zeroed, Layout};
use std::arch::x86_64::{__m256i, _mm256_add_epi16, _mm256_load_si256, _mm256_setzero_si256, _mm256_store_si256};

use crate::board::{piece::Piece, state::board::Board};
use crate::color::Color::*;
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
    accumulator_stack: [Accumualator<T, U>; MAX_DEPTH + 1],
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
        boxed.accumulator_stack[0] = acc;
        boxed.current_acc = 0;

        *boxed
    }
}


impl<T, const U: usize> NNUEState<T, U> {
//     pub(crate) fn update_accumulator<const U: usize, const V: usize, W: Copy>(
//         &self,
//         layer: LinearLayer<U, V, W>,
//         removed_features: &Vec<FeatureIdx>,
//         added_features: &Vec<FeatureIdx>,
//         color: Color 
//     ) -> Self {
//         const REGISTER_WIDTH: usize = 256/16;
//         const NUM_CHUNKS: usize = L1_SIZE /REGISTER_WIDTH;
        
//         let mut regs: [__m256i; NUM_CHUNKS] = unsafe {[_mm256_setzero_si256(); NUM_CHUNKS]};
//         let mut new_acc = self.clone();

//         for i in 0..NUM_CHUNKS {
//             unsafe { 
//                 let bias = new_acc[color].as_ptr().add(i * REGISTER_WIDTH) as *const __m256i;
//                 *regs.as_mut_ptr().add(i) = _mm256_load_si256(bias) 
//             };
//         }

//         for r in removed_features {
//             for i in 0..NUM_CHUNKS {
//                 unsafe {
//                     let weights = layer.weight.as_ptr().add(i * REGISTER_WIDTH) as *const __m256i;
//                     *regs.as_mut_ptr().add(i) = _mm256_sub_epi16(*regs.as_ptr().add(i), _mm256_load_si256(weights));
//                 }
//             }
//         }

//         for a in added_features {
//             for i in 0..NUM_CHUNKS {
//                 unsafe {
//                     let weights = layer.weight.as_ptr().add(i * REGISTER_WIDTH) as *const __m256i;
//                     *regs.as_mut_ptr().add(i) = _mm256_sub_epi16(*regs.as_ptr().add(i), _mm256_load_si256(weights));
//                 }
//             }
//         }

//          // Only after all the accumulation is done, do the write
//          for i in 0..NUM_CHUNKS {
//              unsafe {
//                 let src = *(regs.as_ptr().add(i));
//                 let dst = &mut new_acc[color][i * REGISTER_WIDTH];
//                 _mm256_store_si256(dst, src);
//             }
//          }
        
//         new_acc
//     }
// 
// 
// 
// 
// 

// }

}