use std::fmt::Debug;

use super::bitmove::Move;

pub(crate) trait MoveAction: Default + Copy + Debug {
    // type Input = Move;

    fn create(input: Move) -> Self;
}