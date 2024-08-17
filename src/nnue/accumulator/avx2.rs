use crate::nnue::{accumulator::slice_to_aligned, commons::Align64, network::{feature::FeatureIndex, INPUT, L1_SIZE}, simd::{self, I16_CHUNK_SIZE}};

/// Apply add/subract update in place.
pub unsafe fn vector_update_inplace(
    input: &mut Align64<[i16; L1_SIZE]>,
    bucket: &Align64<[i16; INPUT * L1_SIZE]>,
    adds: &[FeatureIndex],
    subs: &[FeatureIndex]
) {
    const REGISTERS: usize = 16;
    const UNROLL: usize = I16_CHUNK_SIZE * REGISTERS;

    let mut registers = [simd::zero_i16(); 16];
    for i in 0..(L1_SIZE / UNROLL) {
        let unroll_offset = i * UNROLL;
        for (r_idx, reg) in registers.iter_mut().enumerate() {
            *reg = simd::load_i16(input.get_unchecked(unroll_offset * r_idx * I16_CHUNK_SIZE));
        }

        for &sub_index in subs {
            let sub_index = *sub_index * L1_SIZE;
            let sub_block = slice_to_aligned(bucket.get_unchecked(sub_index..sub_index + L1_SIZE));
            for (r_idx, reg) in registers.iter_mut().enumerate() {
                let sub = simd::load_i16(sub_block.get_unchecked(unroll_offset + r_idx * I16_CHUNK_SIZE));
                *reg = simd::sub_i16(*reg, sub);
            }
        }
    }
}