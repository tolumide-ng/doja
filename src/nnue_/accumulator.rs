// when the king moves, the accumulator is refreshed

use super::{feature_idx::FeatureIdx, network::LinearLayer};

const L1_SIZE: usize = 2 * 512; // 2 represents (black + white)


#[derive(Debug, Clone, Copy)]
pub(crate) struct Accumulator {
    pub(crate) white: [i16; L1_SIZE], // A_w
    pub(crate) black: [i16; L1_SIZE], // A_b
}


impl Default for Accumulator {
    fn default() -> Self {
        Self { white: [0; L1_SIZE], black: [0; L1_SIZE], }
    }
}

impl Accumulator {
    pub(crate) fn refresh<const U: usize>(&self, layer: LinearLayer<i16, U>, active_features: Vec<FeatureIdx>) -> Accumulator {
        let mut new_acc = Self::default();

        // First we copy the layer bias, that's out starting point
        for i in 0..(U/2) {
            new_acc.white[i] = layer.bias[i];
            new_acc.black[i*2] = layer.bias[i*2];
        }

        // Then we just accumulate all the columns for the active features. That's what accumulator's do
        for a in active_features {
            //  we should only do this for the available features
            // we shouldn't loop through all the features in the accumualtor like stockfish suggested at this stage
            // for i
        }

        new_acc
    }
}