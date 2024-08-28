use super::{align64::Align64, commons::HIDDEN, net::MODEL};

pub(crate) type SubAccumulator = Align64<[i16; HIDDEN]>;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Accumulator {
    pub(crate) white: SubAccumulator,
    pub(crate) black: SubAccumulator,
}


impl Default for Accumulator {
    fn default() -> Self {
        Self { white: MODEL.features_bias, black: MODEL.features_bias }
    }
}

impl Accumulator {
    /// Updates weights for a single feature, either turning them on or off
    pub(crate) fn update_weights<const ON: bool>(&mut self, idx: (usize, usize)) {
        fn update<const ON: bool>(acc: &mut SubAccumulator, idx: usize) {
            let zip = acc.iter_mut().zip(&MODEL.feature_weights[idx..idx + HIDDEN]);

            for (acc_val, &weight) in zip {
                if ON {
                    *acc_val += weight;
                } else {
                    *acc_val -= weight;
                }
            }
        }

        update::<ON>(&mut self.white, idx.0);
        update::<ON>(&mut self.black, idx.1);
    }

    /// Update the accumulator for quite move.
    /// Adds in features for the destination and removes the features of the source
    pub(crate) fn add_sub_weights(&mut self, from: (usize, usize), to: (usize, usize)) {
        fn add_sub(acc: &mut SubAccumulator, from: usize, to: usize) {
            let zip = acc.iter_mut().zip(MODEL.feature_weights[from..from+HIDDEN].iter().zip(&MODEL.feature_weights[to..to+HIDDEN]));

            for (acc_val, (&remove_weight, &add_weight)) in zip {
                *acc_val += add_weight - remove_weight;
            }
        }

        add_sub(&mut self.white, from.0, to.0);
        add_sub(&mut self.black, from.1, to.1);
    }
}