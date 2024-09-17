// when the king moves, the accumulator is refreshed

use crate::color::Color;

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
    pub(crate) fn refresh<const U: usize, const M: usize>(&self, color: Color, layer: LinearLayer<i16, U, M>, active_features: Vec<FeatureIdx>) -> Accumulator {
        let mut new_acc = Self::default();

        // First we copy the layer bias, that's out starting point
        for i in 0..U {
            [&mut new_acc.white, &mut new_acc.black][color][i] = layer.bias[i];
        }

        // Then we just accumulate all the columns for the active features. That's what accumulator's do
        for a in active_features {
            for i in 0..U {
                [&mut new_acc.white, &mut new_acc.black][color][i] = layer.weight[*a][i];
            }
        }

        new_acc
    }


    pub(crate) fn update<const U: usize, const M: usize>(&self, color: Color, layer: LinearLayer<i16, U, M>, removed_features: Vec<FeatureIdx>, added_features: Vec<FeatureIdx>) -> Self {
        let mut new_acc = Self::default();

        // First we copy the previous values, that's our starting point
        for i in 0..M {
            [&mut new_acc.white, &mut new_acc.black][color][i] = [&self.white, &self.white][color][i];
        }

        // Then we subtract the weights of the removed features
        for r in removed_features {
            for i in 0..M {
                [&mut new_acc.white, &mut new_acc.black][color][i] -= layer.weight[*r][i];
            }
        }

        for a in added_features {
            for i in 0..M {
                [&mut new_acc.white, &mut new_acc.black][color][i] += layer.weight[*a][i];
            }
        }

        new_acc
    }
}