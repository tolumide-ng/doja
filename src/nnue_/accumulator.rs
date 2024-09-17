// when the king moves, the accumulator is refreshed

use std::arch::x86_64::__m256i;

use crate::{color::Color, constants::PLAYERS_COUNT};

use super::{feature_idx::FeatureIdx, network::LinearLayer};

const L0_SIZE: usize = 2 * 512; // 2 represents (black + white)


// A single AVX2 register can fit 16 i16 values, and there are 16AVX2 registers (32 since AVX512) (stockfish docs)

#[repr(align(64))]
struct Accumualator<const M: usize, N> {
    v: [[N; M]; PLAYERS_COUNT]
}

impl<N> Accumualator<L0_SIZE, N> {
    pub fn refresh_accumulator<T, const U: usize, const V: usize>(
        layer: LinearLayer<T, U, V>, 
        new_acc: &Self,
        active_features: &Vec<FeatureIdx>,
        color: Color
    ) {
        const REGISTER_WIDTH: usize = 256/16;
        const NUM_CHUNKS: usize = L0_SIZE / REGISTER_WIDTH;
        let mut regs: [__m256i; NUM_CHUNKS] = unsafe { [std::mem::zeroed(); NUM_CHUNKS] };
    }
}