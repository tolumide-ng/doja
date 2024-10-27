use crate::move_logic::{bitmove::Move, move_action::MoveAction};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct ScoredMove(Move, i32);

impl ScoredMove {
    pub(crate) fn mv(&self) -> Move {
        self.0
    }

    pub(crate) fn score(&self) -> i32 { self.1 }

    pub(crate) fn update_score(&mut self, score: i32) { self.1 = score }
}


impl MoveAction for ScoredMove {
    fn create(input: Move) -> Self {
        ScoredMove(input, 0)
    }
}