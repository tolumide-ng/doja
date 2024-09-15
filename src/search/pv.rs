use crate::{bit_move::Move, constants::MAX_PLY};

pub(crate) struct PVTable {
    length: usize,
    moves: [Move; MAX_PLY],
}