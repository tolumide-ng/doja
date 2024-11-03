use crate::{board::state::board::Board, move_logic::bitmove::Move};


#[derive(Debug, Clone)]
pub(crate) struct History {
    pub(super) board: Board,
    pub(super) mv: Move
}

impl History {
    pub(crate) fn new(board: Board, mv: Move) -> Self {
        Self { board, mv }
    }

    pub(crate) fn mv(&self) -> Move {
        self.mv
    }

    pub(crate) fn board(&self) -> Board {
        self.board
    }
}