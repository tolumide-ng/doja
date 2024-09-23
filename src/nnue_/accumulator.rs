// when the king moves, the accumulator is refreshed

use std::arch::x86_64::{__m256i, _mm256_add_epi16, _mm256_load_si256, _mm256_loadu_si256, _mm256_setzero_si256, _mm256_store_si256, _mm256_sub_epi16};
use std::ops::{Index, IndexMut};


use crate::color::Color;
use crate::nnue_::constants::halfKA::*;

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



impl Accumualator<Feature, L1_SIZE> {
    pub(crate) fn refresh_accumulator<const U: usize, const V: usize, W: Copy>(
        layer: LinearLayer<U, V, W>, 
        new_acc: &mut Self,
        active_features: &Vec<FeatureIdx>,
        color: Color
    ) {
        const REGISTER_WIDTH: usize = 256/16;
        const NUM_CHUNKS: usize = L1_SIZE / REGISTER_WIDTH;
        let mut regs: [__m256i; NUM_CHUNKS] = unsafe { [_mm256_setzero_si256(); NUM_CHUNKS] };

        // Load bias to registers and operate on registers only
        for i in 0..NUM_CHUNKS {
            unsafe {
                let bias = layer.bias.as_ptr().add(i * REGISTER_WIDTH) as *const __m256i;
                *regs.as_mut_ptr().add(i) = _mm256_loadu_si256(bias);
            };
        }

        for a in active_features {
            for i in 0..NUM_CHUNKS {
                unsafe {
                    // let xx = (*(layer.weight.as_ptr().add(**a))).as_ptr().add(i * REGISTER_WIDTH);
                    let weights = layer.weight.as_ptr().add(i * REGISTER_WIDTH) as *const __m256i;
                    *regs.as_mut_ptr().add(i) = _mm256_add_epi16(regs[i], _mm256_load_si256(weights));
                };
            }
        }

        // Only after all the accumulation is done do the write.
        for i in 0..NUM_CHUNKS {
            unsafe { _mm256_store_si256(&mut new_acc[color][i], regs[i]) }
        }
    }

    pub(crate) fn update_accumulator<const U: usize, const V: usize, W: Copy>(
        &self,
        layer: LinearLayer<U, V, W>,
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