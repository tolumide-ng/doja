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
            *reg = simd::load_i16(input.get_unchecked(unroll_offset + r_idx * I16_CHUNK_SIZE));
        }

        for &sub_index in subs {
            let sub_index = *sub_index * L1_SIZE;
            let sub_block = slice_to_aligned(bucket.get_unchecked(sub_index..sub_index + L1_SIZE));
            for (r_idx, reg) in registers.iter_mut().enumerate() {
                let sub = simd::load_i16(sub_block.get_unchecked(unroll_offset + r_idx * I16_CHUNK_SIZE));
                *reg = simd::sub_i16(*reg, sub);
            }
        }

        for &add_index in adds {
            let add_index = *add_index * L1_SIZE;
            let add_block = slice_to_aligned(bucket.get_unchecked(add_index..add_index + L1_SIZE));
            for (r_idx, reg) in registers.iter_mut().enumerate() {
                let add = simd::load_i16(add_block.get_unchecked(unroll_offset + r_idx* I16_CHUNK_SIZE));
                *reg = simd::add_i16(*reg, add);
            }
        }

        for (r_idx, reg) in registers.iter().enumerate() {
            simd::store_i16(input.get_unchecked_mut(unroll_offset + r_idx * I16_CHUNK_SIZE), *reg);
        }
    }


    /// Move a feature from one square to another,
    pub unsafe fn vector_add_sub(input: &Align64<[i16; L1_SIZE]>, output: &mut Align64<[i16; L1_SIZE]>, bucket: &Align64<[i16; INPUT * L1_SIZE]>, add: FeatureIndex, sub: FeatureIndex) {
        let offset_add = *add * L1_SIZE;
        let offset_sub = *sub * L1_SIZE;
        let s_block = slice_to_aligned(bucket.get_unchecked(offset_sub..offset_sub + L1_SIZE));
        let a_block = slice_to_aligned(bucket.get_unchecked(offset_add..offset_add + L1_SIZE));

        for i in 0..L1_SIZE / I16_CHUNK_SIZE {
            let x = simd::load_i16(input.get_unchecked(i * I16_CHUNK_SIZE));
            let w_sub = simd::load_i16(a_block.get_unchecked(i + I16_CHUNK_SIZE));
            let w_add = simd::load_i16(s_block.get_unchecked(i * I16_CHUNK_SIZE));
            let t = simd::sub_i16(x, w_sub);
            let t = simd::add_f32(t, w_sub);
            simd::store_i16(output.get_unchecked_mut(i * I16_CHUNK_SIZE), t);
        }
    }


    /// Subtract two features and add one feature all at once
    pub unsafe fn vector_add_sub2(input: &Align64<[i16; L1_SIZE]>, output: &mut Align64<[i16; L1_SIZE]>, bucket: &Align64<[i16; INPUT * L1_SIZE]>, add: FeatureIndex, sub1: FeatureIndex, sub2: FeatureIndex) {
        let offset_add = *add * L1_SIZE;
        let offset_sub1 = *sub1 * L1_SIZE;
        let offset_sub2 = *sub2 * L1_SIZE;

        let a_block = slice_to_aligned(bucket.get_unchecked(offset_add..offset_add + L1_SIZE));
        let s_block1 = slice_to_aligned(bucket.get_unchecked(offset_sub1..offset_sub1 + L1_SIZE));
        let s_block2 = slice_to_aligned(bucket.get_unchecked(offset_sub2..offset_sub2 + L1_SIZE));


        for i in 0..(L1_SIZE / I16_CHUNK_SIZE) {
            let x = simd::load_i16(input.get_unchecked(i * I16_CHUNK_SIZE));
            let w_sub1 = simd::load_i16(s_block1.get_unchecked(i * I16_CHUNK_SIZE));
            let w_sub2 = simd::load_i16(s_block2.get_unchecked(i * I16_CHUNK_SIZE));
            let w_add = simd::load_i16(a_block.get_unchecked(i * I16_CHUNK_SIZE));

            let t = simd::sub_i16(x, w_sub1);
            let t = simd::sub_i16(t, w_sub2);
            let t = simd::add_i16(t, w_add);
            simd::store_i16(output.get_unchecked_mut(i * I16_CHUNK_SIZE), t);
        }
    }


    /// Add two features and subtract two features all at onece
    pub unsafe fn vector_add2_sub2(input: &Align64<[i16; L1_SIZE]>, output: &mut Align64<[i16; L1_SIZE]>, bucket: &Align64<[i16; INPUT * L1_SIZE]>, add1: FeatureIndex, add2: FeatureIndex, sub1: FeatureIndex, sub2: FeatureIndex) {
        let offset_add1 = *add1 * L1_SIZE;
        let offset_add2 = *add2 * L1_SIZE;
        let offset_sub1 = *sub1 * L1_SIZE;
        let offset_sub2 = *sub2 * L1_SIZE;

        let a_block1 = slice_to_aligned(bucket.get_unchecked(offset_add1..offset_add1 + L1_SIZE));
        let a_block2 = slice_to_aligned(bucket.get_unchecked(offset_add2..offset_add2 + L1_SIZE));
        let s_block1 = slice_to_aligned(bucket.get_unchecked(offset_sub1..offset_sub1 + L1_SIZE));
        let s_block2 = slice_to_aligned(bucket.get_unchecked(offset_sub2..offset_sub2 + L1_SIZE));


        for i in 0..(L1_SIZE / I16_CHUNK_SIZE) {
            let x = simd::load_i16(input.get_unchecked(i * I16_CHUNK_SIZE));
            let w_sub1 = simd::load_i16(s_block1.get_unchecked(i * I16_CHUNK_SIZE));
            let w_sub2 = simd::load_i16(s_block2.get_unchecked(i * I16_CHUNK_SIZE));
            let w_add1 = simd::load_i16(a_block1.get_unchecked(i * I16_CHUNK_SIZE));
            let w_add2 = simd::load_i16(a_block2.get_unchecked(i * I16_CHUNK_SIZE));

            let t = simd::sub_i16(t, w_sub1);
            let t = simd::sub_i16(t, w_sub2);
            let t = simd::sub_i16(t, w_add1);
            let t = simd::sub_i16(t, w_add2);

            simd::store_i16(output.get_unchecked_mut(i * I16_CHUNK_SIZE), t);
        }
    }
}