use std::alloc::{self, alloc_zeroed, Layout};

use crate::board::{piece::Piece, state::board::Board};
use crate::color::Color::*;

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
    fn from(board: Board) -> Self {
        let mut boxed: Box<Self> = unsafe {
            let layout = Layout::new::<Accumualator<Feature, U>>();
            let ptr = alloc_zeroed(layout);

            if ptr.is_null() {
                alloc::handle_alloc_error(layout);
            }

            Box::from_raw(ptr.cast())
        };

        boxed.accumulator_stack[0] = Accumualator::default();

        let mut board_sqs = board.get_occupancy(Both);

        while board_sqs != 0 {
            let sq = board_sqs.trailing_ones() as u64;
        }

        let bitmaps = board.board;

        for (p, board) in (*bitmaps).into_iter().enumerate() {
            let mut sqs = *board;

            while sqs != 0 {
                let sq = sqs.trailing_zeros() as u8;

                let piece = Piece::from(p as u8);
            }
        }

        // Self { white: [0; U], black: [0; U] }
        0
    }
}


impl<T, const U: usize> NNUEState<T, U> {
    pub(crate) fn update() -> Self {
        //  use the update_accumualtor method below to generate this
    }
}




// impl Accumualator<Feature, L1_SIZE> {
// 
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
//     pub(crate) fn refresh_accumulator<const U: usize, const V: usize, W: Copy>(
//         layer: LinearLayer<U, V, W>, 
//         new_acc: &mut Self,
//         active_features: &Vec<FeatureIdx>,
//         color: Color
//     ) {
//         const REGISTER_WIDTH: usize = 256/16;
//         const NUM_CHUNKS: usize = L1_SIZE / REGISTER_WIDTH;
//         let mut regs: [__m256i; NUM_CHUNKS] = unsafe { [_mm256_setzero_si256(); NUM_CHUNKS] };

//         // Load bias to registers and operate on registers only
//         for i in 0..NUM_CHUNKS {
//             unsafe {
//                 let bias = layer.bias.as_ptr().add(i * REGISTER_WIDTH) as *const __m256i;
//                 *regs.as_mut_ptr().add(i) = _mm256_loadu_si256(bias);
//             };
//         }

//         for a in active_features {
//             for i in 0..NUM_CHUNKS {
//                 unsafe {
//                     // let xx = (*(layer.weight.as_ptr().add(**a))).as_ptr().add(i * REGISTER_WIDTH);
//                     let weights = layer.weight.as_ptr().add(i * REGISTER_WIDTH) as *const __m256i;
//                     *regs.as_mut_ptr().add(i) = _mm256_add_epi16(regs[i], _mm256_load_si256(weights));
//                 };
//             }
//         }

//         // Only after all the accumulation is done do the write.
//         for i in 0..NUM_CHUNKS {
//             unsafe { _mm256_store_si256(&mut new_acc[color][i], regs[i]) }
//         }
//     }
// }
