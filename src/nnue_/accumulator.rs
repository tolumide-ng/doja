// when the king moves, the accumulator is refreshed

use std::arch::x86_64::{__m256i, _mm256_add_epi16, _mm256_load_si256, _mm256_max_epi16, _mm256_max_epi8, _mm256_min_epi16, _mm256_mullo_epi16, _mm256_packs_epi16, _mm256_permute4x64_epi64, _mm256_set1_epi16, _mm256_setzero_si256, _mm256_store_si256, _mm256_sub_epi16};
use std::ops::{Index, IndexMut};


use crate::board::piece::Piece;
use crate::board::state::board::Board;
use crate::color::Color::*;
use crate::color::Color;
use crate::constants::PLAYERS_COUNT;
use crate::nnue::net::{halfka_idx, MODEL};
use crate::squares::Square;

use super::align64::Align64;
use super::feature_idx::FeatureIdx;


// A single AVX2 register can fit 16 i16 values, and there are 16AVX2 registers (32 since AVX512) (stockfish docs)

pub(crate) type Feature = __m256i;
pub(crate) const QA: i16 = 255;
pub(crate) const QAB: i32 = 255*64;



#[derive(Debug, Clone, Copy)]
#[repr(align(64))]
pub(crate) struct Accumulator<T, const U: usize> {
    pub(crate) white: Align64<[T; U]>,
    pub(crate) black: Align64<[T; U]>,
}


impl<const U: usize> Default for Accumulator<Feature, U> {
    fn default() -> Self {
        unsafe { Self { white: Align64([_mm256_setzero_si256(); U]), black: Align64([_mm256_setzero_si256(); U]) } }
    }
}


// impl<T, const U: usize> From<*mut Accumulator<T, U>> for AccumulatorPtr<T, U> {
//     fn from(value: *mut Accumulator<T, U>) -> Self {
//         Self(value)
//     }
// }




impl<const U: usize> Accumulator<Feature, U> {
    const REGISTER_WIDTH: usize = 256/16; // 16

    pub(crate) unsafe fn refresh(board: &Board) -> Self {
        let mut acc = Accumulator::default();
        let num_chunks: usize = U / Self::REGISTER_WIDTH; // 1024/16 = 64 (U would usually be the L1SIZE)

        let mut active_features = Vec::with_capacity(board.get_occupancy(Both).count_ones() as usize);
        let bitmaps = &board.board;
        
        for (p, board) in (*bitmaps).into_iter().enumerate() {
            let mut sqs: u64 = *board;

            while sqs != 0 {
                let sq = Square::from(sqs.trailing_zeros() as u8);
                let piece = Piece::from(p as u8);
                
                let f_idx = halfka_idx(piece, sq);
                // println!("in the while loop {}", *f_idx + (1200));

                sqs &= sqs -1;

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
                *regs.as_mut_ptr().add(color_idx) = _mm256_load_si256(MODEL.features_bias.as_ptr().add(i * Self::REGISTER_WIDTH) as *const __m256i);
                // _mm256_store_si256(acc[color].as_mut_ptr().add(i) as *mut __m256i, *(MODEL.features_bias.as_ptr().add(i * REGISTER_WIDTH) as *const __m256i));
            }
        }


        for (a, color) in active_features {
            for i in 0..num_chunks {
                // the number of weights per feature, is equivalent to the L1_SIZE, in this case 1024figures represent the weight of a feature "a"
                // for any value of "a", by the end of this loop, we'd have loaded all weights that belong to "a"
                // this specifically updates all the values representing the feature "a" on the board
                // we can only load 16 i16s at a time, weil 16*16=256 (size of an AVX2 register)
                let idx =  *a + (i*Self::REGISTER_WIDTH);
                let reg_idx = (color as usize * num_chunks) + i;

                let weights = *(MODEL.feature_weights.as_ptr().add(idx) as *const __m256i);
                // y = Ax + b (where A is the feature, x is the weight, and b is bias)
                *regs.as_mut_ptr().add(reg_idx) = _mm256_add_epi16(*(regs.as_ptr().add(reg_idx)), weights);
            }
        }

        for i in 0..num_chunks {
                let black_idx = num_chunks + i;
            _mm256_store_si256(acc[Color::White].as_mut_ptr().add(i*Self::REGISTER_WIDTH) as *mut __m256i, *regs.as_ptr().add(i));
            _mm256_store_si256(acc[Color::Black].as_mut_ptr().add(i*Self::REGISTER_WIDTH) as *mut __m256i, *regs.as_ptr().add(black_idx));
        }
        
        acc
    }

    pub(crate) unsafe fn update(&self, removed_features: &Vec<(Color, FeatureIdx)>, added_features: &Vec<(Color, FeatureIdx)>) -> Self {
        let num_chunks: usize = U / Self::REGISTER_WIDTH; // 1024/16 = 64 (U would usually be the L1SIZE)

        let mut acc = Accumulator::default();
        let mut regs: Vec<__m256i> = Vec::with_capacity(num_chunks * PLAYERS_COUNT);

        
        // Load the previous values to registers and operate on registers only
        for i in 0..num_chunks {
            let black_idx = num_chunks + i;
            *regs.as_mut_ptr().add(i) = _mm256_load_si256(self.white.as_ptr().add(i * Self::REGISTER_WIDTH));
            *regs.as_mut_ptr().add(black_idx) = _mm256_load_si256(self.black.as_ptr().add(i * Self::REGISTER_WIDTH));
        }

        for (color, f_idx) in removed_features.iter() {
            for i in 0..num_chunks {
                // let model_idx = (**f_idx * U) + (i * Self::REGISTER_WIDTH);
                let model_idx = **f_idx + (i * Self::REGISTER_WIDTH);
                let regs_idx = (*color as usize * num_chunks) + i;
                
                let weights = *(MODEL.feature_weights.as_ptr().add(model_idx) as *const __m256i);
                *regs.as_mut_ptr().add(regs_idx) = _mm256_sub_epi16(*(regs.as_ptr().add(regs_idx)), weights);
            }
        }
        
        // println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> model_idx>>>> {model_idx}, and weights is {}", MODEL.feature_weights.len());
        for (color, f_idx) in added_features.into_iter() {
            for i in 0..num_chunks {
                let model_idx = **f_idx+ (i * Self::REGISTER_WIDTH);
                let regs_idx = (*color as usize * num_chunks) + i;

                let weights = *(MODEL.feature_weights.as_ptr().add(model_idx) as *const __m256i);
                *regs.as_mut_ptr().add(regs_idx) = _mm256_add_epi16(*(regs.as_ptr().add(regs_idx)), weights);
            }
        }
        
        for i in 0..num_chunks {
            let black_idx = num_chunks + i;
            _mm256_store_si256(acc[Color::White].as_mut_ptr().add(i*Self::REGISTER_WIDTH) as *mut __m256i, *regs.as_ptr().add(i));
            _mm256_store_si256(acc[Color::Black].as_mut_ptr().add(i*Self::REGISTER_WIDTH) as *mut __m256i, *regs.as_ptr().add(black_idx));
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

    pub(crate) unsafe fn sq_crelu16(&self, stm: Color) -> [Align64<[__m256i; U]>; 2] {
        const IN_REGISTER_WIDTH: usize = 256/16; // 16
        const OUT_REGISTER_WIDTH: usize = 256/16; // 16 (output would be in i16, because we would be squaring the clamped values(i8^2) squaredCReLU)
        let num_out_chunks = U/OUT_REGISTER_WIDTH; // 1024/16 = 6

        
        let input = if stm == Color::White {[self.white, self.black]} else {[self.black, self.white]};
        let output: [Align64<[__m256i; U]>; 2] = [Align64([_mm256_setzero_si256(); U]); 2];  // [[_; 1024]; 2];
        
        let min = _mm256_setzero_si256();
        let max = _mm256_set1_epi16(QA);
        
        // const CONTROL: i32 = 0b11011000; // 3, 1, 2, 0; lane 0 is the rightmost one
        
        for color in [Color::White, Color::Black] {
            for i in 0..num_out_chunks {
                let curr_input = *(input.as_ptr().add(color as usize)); // color
                let in0 = _mm256_load_si256(curr_input.as_ptr().add(i * IN_REGISTER_WIDTH)); // loads 16 i16 values from curr_input 
                let r = _mm256_max_epi16(_mm256_min_epi16(in0, min), max);
                let result = _mm256_mullo_epi16(r, r);

                let mut output = *(output.as_ptr().add(color as usize));

                _mm256_store_si256(output.as_mut_ptr().add(i * OUT_REGISTER_WIDTH) as *mut __m256i, result);
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