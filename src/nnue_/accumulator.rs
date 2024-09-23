// when the king moves, the accumulator is refreshed

use std::arch::x86_64::{__m256i, _mm256_add_epi16, _mm256_load_si256, _mm256_loadu_si256, _mm256_setzero_si256, _mm256_store_si256, _mm256_sub_epi16};
use std::ops::{Index, IndexMut};


use crate::board::piece::Piece;
use crate::board::state::board::Board;
use crate::color::Color::*;
use crate::color::Color;
use crate::constants::PLAYERS_COUNT;
use crate::nnue::net::{halfka_idx, MODEL};
use crate::nnue_::constants::halfKA::*;
use crate::squares::Square;

use super::{feature_idx::FeatureIdx, linear_layer::LinearLayer};


// A single AVX2 register can fit 16 i16 values, and there are 16AVX2 registers (32 since AVX512) (stockfish docs)

pub(crate) type Feature = __m256i;

#[derive(Debug, Clone, Copy)]
#[repr(align(64))]
pub(crate) struct Accumualator<T, const U: usize> {
    white: [T; U],
    black: [T; U],
}

impl<const U: usize> Default for Accumualator<Feature, U> {
    fn default() -> Self {
        unsafe { Self { white: [_mm256_setzero_si256(); U], black: [_mm256_setzero_si256(); U] } }
    }
}



impl<const U: usize> Accumualator<Feature, U> {
    pub(crate) unsafe fn refresh(board: &Board) -> Self {
        let mut acc = Accumualator::default();

        const REGISTER_WIDTH: usize = 256/16; // 16
        let num_chunks: usize = U / REGISTER_WIDTH; // 1024/16 = 64 (U would usually be the L1SIZE)

        let mut active_features = Vec::with_capacity(board.get_occupancy(Both).count_ones() as usize);
        
        let bitmaps = &board.board;
        for (p, board) in (*bitmaps).into_iter().enumerate() {
            let sqs = *board;

            while sqs != 0 {
                let sq = Square::from(sqs.trailing_zeros() as u8);
                let piece = Piece::from(p as u8);

                let f_idx = halfka_idx(piece, sq);

                active_features.push((f_idx, piece.color()));
            }
        }

        // let mut regs: [Feature; num_chunks] = unsafe { [_mm256_setzero_si256(); num_chunks] };
        let mut regs: Vec<__m256i> = Vec::with_capacity(num_chunks * PLAYERS_COUNT);

        // Load bias into registers
        for color in [White, Black] {
            for i in 0..num_chunks {
                // If there is a problem here, please confirm that halfa_idx works as it should, compare directly with the one from pytorch-NNUE
                let color_idx = (color as usize * num_chunks) + i;
                *regs.as_mut_ptr().add(color_idx) = _mm256_load_si256(MODEL.features_bias.as_ptr().add(i * REGISTER_WIDTH) as *const __m256i);
                // _mm256_store_si256(acc[color].as_mut_ptr().add(i) as *mut __m256i, *(MODEL.features_bias.as_ptr().add(i * REGISTER_WIDTH) as *const __m256i));
            }
        }


        for (a, color) in active_features {
            for i in 0..num_chunks {
                // the number of weights per feature, is equivalent to the L1_SIZE, in this case 1024figures represent the weight of a feature "a"
                // for any value of "a", by the end of this loop, we'd have loaded all weights that belong to "a"
                // this specifically updates all the values representing the feature "a" on the board
                // we can only load 16 i16s at a time, weil 16*16=256 (size of an AVX2 register)
                let idx = (*a * U) + (i*REGISTER_WIDTH);
                let color_idx = (color as usize * num_chunks) + i;

                let weights = *(MODEL.feature_weights.as_ptr().add(idx) as *const __m256i);
                // y = Ax + b (where A is the feature, x is the weight, and b is bias)
                *regs.as_mut_ptr().add(color_idx) = _mm256_add_epi16(weights, *(regs.as_ptr().add(color as usize + i)));
            }
        }

        for i in 0..num_chunks {
                let black_idx = num_chunks + i;
            _mm256_store_si256(acc[Color::White].as_mut_ptr().add(i*REGISTER_WIDTH) as *mut __m256i, *regs.as_ptr().add(i));
            _mm256_store_si256(acc[Color::Black].as_mut_ptr().add(i*REGISTER_WIDTH) as *mut __m256i, *regs.as_ptr().add(black_idx));
        }
            acc
    }


    // pub(crate) fn refresh_accumulator<const U: usize, const V: usize, W: Copy>(
    //     layer: LinearLayer<U, V, W>, 
    //     new_acc: &mut Self,
    //     active_features: &Vec<FeatureIdx>,
    //     color: Color
    // ) {
    //     const REGISTER_WIDTH: usize = 256/16;
    //     const NUM_CHUNKS: usize = L1_SIZE / REGISTER_WIDTH;
    //     let mut regs: [__m256i; NUM_CHUNKS] = unsafe { [_mm256_setzero_si256(); NUM_CHUNKS] };

    //     // Load bias to registers and operate on registers only
    //     for i in 0..NUM_CHUNKS {
    //         unsafe {
    //             let bias = layer.bias.as_ptr().add(i * REGISTER_WIDTH) as *const __m256i;
    //             *regs.as_mut_ptr().add(i) = _mm256_loadu_si256(bias);
    //         };
    //     }

    //     for a in active_features {
    //         for i in 0..NUM_CHUNKS {
    //             unsafe {
    //                 // let xx = (*(layer.weight.as_ptr().add(**a))).as_ptr().add(i * REGISTER_WIDTH);
    //                 let weights = layer.weight.as_ptr().add(i * REGISTER_WIDTH) as *const __m256i;
    //                 *regs.as_mut_ptr().add(i) = _mm256_add_epi16(regs[i], _mm256_load_si256(weights));
    //             };
    //         }
    //     }

    //     // Only after all the accumulation is done do the write.
    //     for i in 0..NUM_CHUNKS {
    //         unsafe { _mm256_store_si256(&mut new_acc[color][i], regs[i]) }
    //     }
    // }

    pub(crate) fn update_accumulator<const L: usize, const M: usize, N: Copy>(
        &self,
        layer: LinearLayer<L, M, N>,
        removed_features: &Vec<FeatureIdx>,
        added_features: &Vec<FeatureIdx>,
        color: Color 
    ) -> Self {
        const REGISTER_WIDTH: usize = 256/16;
        const NUM_CHUNKS: usize = L1_SIZE /REGISTER_WIDTH;
        
        let mut regs: [__m256i; NUM_CHUNKS] = unsafe {[_mm256_setzero_si256(); NUM_CHUNKS]};
        let mut new_acc = self.clone();

        for i in 0..NUM_CHUNKS {
            unsafe { 
                let bias = new_acc[color].as_ptr().add(i * REGISTER_WIDTH) as *const __m256i;
                *regs.as_mut_ptr().add(i) = _mm256_load_si256(bias) 
            };
        }

        for r in removed_features {
            for i in 0..NUM_CHUNKS {
                unsafe {
                    let weights = layer.weight.as_ptr().add(i * REGISTER_WIDTH) as *const __m256i;
                    *regs.as_mut_ptr().add(i) = _mm256_sub_epi16(*regs.as_ptr().add(i), _mm256_load_si256(weights));
                }
            }
        }

        for a in added_features {
            for i in 0..NUM_CHUNKS {
                unsafe {
                    let weights = layer.weight.as_ptr().add(i * REGISTER_WIDTH) as *const __m256i;
                    *regs.as_mut_ptr().add(i) = _mm256_sub_epi16(*regs.as_ptr().add(i), _mm256_load_si256(weights));
                }
            }
        }

         // Only after all the accumulation is done, do the write
         for i in 0..NUM_CHUNKS {
             unsafe {
                let src = *(regs.as_ptr().add(i));
                let dst = &mut new_acc[color][i * REGISTER_WIDTH];
                _mm256_store_si256(dst, src);
            }
         }
        
        new_acc
    }
}


impl<T, const U: usize> Index<Color> for Accumualator<T, U> {
    type Output = [T; U];

    fn index(&self, index: Color) -> &Self::Output {
        match index {
            Color::White => &self.white,
            Color::Black => &self.black,
            _ => panic!("unrecognized color")
        }
    }
}



impl<T, const U: usize> IndexMut<Color> for Accumualator<T, U> {
    fn index_mut(&mut self, index: Color) -> &mut Self::Output {
        match index {
            Color::White => &mut self.white,
            Color::Black => &mut self.black,
            _ => panic!("unrecognized color")
        }
    }
} 