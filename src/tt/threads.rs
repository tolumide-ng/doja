use std::thread;

use crate::{bit_move::Move, board::{position::Position, state::board::Board}};

use super::{signal::Signal, table::TTable};

pub(crate) const MAX_THREADS: usize = 32;


#[repr(align(64))]
pub(crate) struct ThreadData {
    tt: TTable,
    signal: Signal,
    id: u8,
    // depth: u8,
    // info: Info
    // best_move: Move,
    // pos: u64,
    // info: u64,

    // id: u8,
    // depth: u8,
    // best: Move
}

impl ThreadData {
    pub(crate) fn new(board: &Position, signal: Signal, tt: TTable, id: u8) -> Self {
        Self {tt, id, signal}
    }
}




fn itertive() {}


fn xx() {
    // let tt = TTable::default();
    let bb = Position::new();
    // let signal = Signal {depth: 2};

    for xx in 0..3 {
        // thread::spawn(move || {
        //     ThreadData::new(&bb, signal, tt, 0);
        // });
    }
}