use crate::nnue::{commons::Align64, network::{L1_SIZE, L2_SIZE, L3_SIZE}};

fn activate_ft(us: &Align64<[i16; L1_SIZE]>, them: &Align64<[i16; L1_SIZE]>, output: &mut Align64<[u8; L1_SIZE]>) {
    for (a, acc) in [us, them].into_iter().enumerate() {
        for i in 0..(L1_SIZE/2) {
            unsafe {
                let l = *acc.get_unchecked(i);
                let r = *acc.get_unchecked(L1_SIZE/2 + i);
                let cl = i32::clamp(i32::from(l), 0, QA);
                let cr = i32::clamp(i32::from(r), 0, QA);

                let r = (cl * cr) / QA;
                *output.get_unchecked(i + a * L1_SIZE/2) = r as u8;
            }
        }
    }
}


fn propagate_l1(inputs: &Align64<[u8; L1_SIZE]>, weights: &Align64<[i8; L1_SIZE * L2_SIZE]>, biases: &Align64<[f32; L2_SIZE]>, output: &mut Align64<[u8; L1_SIZE]>) {
    const SUM_DIV: f32 = (QA * QB) as f32;
    let mut sums = [0; L2_SIZE];
    for i in 0..L1_SIZE {
        for j in 0..L2_SIZE {
            unsafe {
                *sums.get_unchecked(j) += i32::from(*inputs.get_unchecked(i)) * i32::from(*weights.get_unchecked(j * L1_SIZE + i));
            }
        }
    }

    for i in 0..L2_SIZE {
        unsafe {
            let clipped = f32::clamp((*sums.get_unchecked(i) as f32)/SUM_DIV + *biases.get_unchecked(i), 0.0, 1.0);
            *output.get_unchecked_mut(i) = clipped * clipped;
        }
    }
}


pub fn activate_ft_and_propagate_l1(us: &Align64<[i16; L1_SIZE]>, them: &Align64<[i16; L1_SIZE]>, weights: &Align64<[i8; L1_SIZE * L2_SIZE]>, biases: &Align64<[f32; L2_SIZE]>, output: &mut Align64<[f32; L1_SIZE]>) {
    let mut ft_outputs = Align64([0; L1_SIZE]);
    activate_ft(us, them, &mut ft_outputs);
    propagate_l1(&ft_outputs, weights, biases, output);
}

pub fn propagate_l2(inputs: &Align64<[f32; L2_SIZE]>, weights: &Align64<[f32; L2_SIZE * L3_SIZE]>, biases: &Align64<[f32; L3_SIZE]>, output: &mut Align64<[f32; L3_SIZE]>) {
    let mut sums = biases.clone();

    // affine transform for l2
    for i in 0..L2_SIZE {
        for j in 0..L3_SIZE {
            unsafe {
                *sums.get_unchecked_mut(j) += *inputs.get_unchecked(i) * *weights.get_unchecked(j * L2_SIZE + i);
            }
        }
    }

    // activate l2
    for i in 0..L3_SIZE {
        unsafe {
            let clipped = f32::clamp(*sums.get_unchecked(i), 0.0, 1.0);
            *output.get_unchecked(i) = clipped * clipped;
        }
    }
}


pub fn propagate_l3(inputs: &Align64<[f32; L3_SIZE]>, weights: &Align64<[f32; L3_SIZE]>, bias: f32, output: &mut f32) {
    let mut sum = bias;
    for (i, w) in inputs.iter().zip(weights.iter()) {
        sum += *i * *w;
    }

    *output = sum;
}