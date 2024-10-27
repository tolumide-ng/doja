use crate::move_logic::{bitmove::Move, move_action::MoveAction};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct ScoredMove((Move, i32));


impl MoveAction for ScoredMove {
    // type Item = (Move, i32);
    // type Input = Move;

    fn create(input: Move) -> Self {
        // ScoredMove((input.0, input.1))
        ScoredMove((input, 0))
    }
}