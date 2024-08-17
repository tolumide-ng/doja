use crate::nnue::{accumulator::slice_to_aligned, commons::Align64, network::{feature::FeatureIndex, INPUT, L1_SIZE}};

pub unsafe fn vector_update_inplace(
    input: &mut Align64<[i16; L1_SIZE]>, 
    bucket: &Align64<[i64; INPUT * L1_SIZE]>, 
    adds: &[FeatureIndex],
    subs: &[FeatureIndex]) {
        const REGISTERS: usize = 16;
        const UNROLL: usize = REGISTERS;

        let mut registers = [0; 16];
        for i in 0..L1_SIZE/UNROLL {
            let unroll_offset = i * UNROLL;
            for (r_idx, reg) in registers.iter_mut().enumerate() {
                *reg = *input.get_unchecked(unroll_offset + r_idx);
            }

            for &index in subs {
                let index = *index * L1_SIZE;
                let block = slice_to_aligned(bucket.get_unchecked(index..index + L1_SIZE));
                
                for (r_idx, reg) in registers.iter().enumerate() {
                    let sub = *block.get_unchecked(unroll_offset + r_idx);
                    *reg -= sub;
                }
            }

            for &index in adds {
                let index = *index * L1_SIZE;
                let block = slice_to_aligned(bucket.get_unchecked(index..index + L1_SIZE));

                for (r_idx, reg) in registers.iter().enumerate() {
                    let add = *block.get_unchecked(unroll_offset + r_idx);
                    *reg += add;
                }
            }

            for (r_idx, reg) in registers.iter().enumerate() {
                *input.get_unchecked(unroll_offset + r_idx) = *reg;
            }
        }
    }

pub unsafe fn vector_add_sub(input: &Align64<[i16; L1_SIZE]>, output: &mut Align64<[i16; L1_SIZE]>, bucket: &Align64<[i64; INPUT * L1_SIZE]>, add: &FeatureIndex, sub: &FeatureIndex) {
    let offset_add = *add * L1_SIZE;
    let offset_sub = *sub * L1_SIZE;

    let s_block = slice_to_aligned(bucket.get_unchecked(offset_sub..offset_sub + L1_SIZE));
    let a_block = slice_to_aligned(bucket.get_unchecked(offset_add..offset_add + L1_SIZE));

    for i in 0..L1_SIZE {
        let x = *input.get_unchecked(i);
        let w_sub = *s_block.get_unchecked(i);
        let w_add = *a_block.get_unchecked(i);
        let t = x - w_sub;
        let t = t + w_add;
        *output.get_unchecked(i) = t;
    }
}


/// Subtract two features and add one feature all at once
pub unsafe fn vector_add_sub2(input: &Align64<[i16; L1_SIZE]>, output: &mut Align64<[i16; L1_SIZE]>, bucket: &Align64<[i64; INPUT * L1_SIZE]>, add: FeatureIndex, sub0: FeatureIndex, sub1: FeatureIndex) {
    let offset_add = *add * L1_SIZE;
    let offset_sub0 = *sub * L1_SIZE;
    let offset_sub1 = *sub1 * L1_SIZE;

    let a_block = slice_to_aligned(bucket.get_unchecked(offset_add..offset_add + L1_SIZE));
    let s_block0 = slice_to_aligned(bucket.get_unchecked(offset_sub0..offset_sub0 + L1_SIZE));
    let s_block1 = slice_to_aligned(bucket.get_unchecked(offset_sub1..offset_sub1 + L1_SIZE));

    for i in 0..L1_SIZE {
        let x = *input.get_unchecked(i);
        let w_sub0 = *s_block0.get_unchecked(i);
        let w_sub1 = *s_block1.get_unchecked(i);
        let w_add = *a_block.get_unchecked(i);

        let t = x - w_sub0;
        let t = t - w_sub1;
        let t = t + w_add;
        *output.get_unchecked_mut(i) = t;
    }
}


/// Add two features and subtract twoi features all at once.
pub unsafe fn vector_add2_sub2(input: &Align64<[i16; L1_SIZE]>, output: &mut Align64<[i16; L1_SIZE]>, bucket: &Align64<[i64; INPUT * L1_SIZE]>, add0: FeatureIndex, add1: FeatureIndex, sub0: FeatureIndex, sub1: FeatureIndex) {
    let offset_add0 = *add0 * L1_SIZE;
    let offset_add1 = *add1 * L1_SIZE;
    let offset_sub0 = *sub * L1_SIZE;
    let offset_sub1 = *sub * L1_SIZE;

    let a_block0 = slice_to_aligned(bucket.get_unchecked(offset_add0..offset_add0 + L1_SIZE));
    let a_block1 =  slice_to_aligned(bucket.get_unchecked(offset_add1..offset_add1 + L1_SIZE));
    let s_block0 = slice_to_aligned(bucket.get_unchecked(offset_sub0..offset_sub0 + L1_SIZE));
    let s_block1 = slice_to_aligned(bucket.get_unchecked(offset_sub1..offset_sub1 + L1_SIZE));

    for i in 0..L1_SIZE {
        let x = *input.get_unchecked(i);
        let w_sub0 = *s_block0.get_unchecked(i);
        let w_sub1 = *s_block1.get_unchecked(i);
        let w_add0 = *a_block0.get_unchecked(i);
        let w_add1 = *a_block1.get_unchecked(i);

        let t = x - w_sub0;
        let t = t - w_sub1;
        let t = t + w_add0;
        let t = t + w_add1;
        *output.get_unchecked_mut(i) = t;
    }
}