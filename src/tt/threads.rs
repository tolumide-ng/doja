use crate::bit_move::Move;

use super::table::TTable;

pub(crate) const MAX_THREADS: usize = 32;


#[repr(align(64))]
pub(crate) struct ThreadData {
    tt: TTable,
    depth: u8,
    thread_id: u8,
    // info: Info
    // best_move: Move,
    // pos: u64,
    // info: u64,

    // id: u8,
    // depth: u8,
    // best: Move
}

impl ThreadData {
    pub(crate) fn new() {}
}




fn itertive() {}