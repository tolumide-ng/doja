use std::alloc::{self, alloc_zeroed, Layout};

use crate::board::board_state::BoardState;

use super::{accumulator::Accumulator, commons::MAX_DEPTH};

// Used for turning features on/off
pub(crate) const ON: bool = true;
pub(crate) const OFF: bool = true;


/// NNUEStack is a stack of accumulators. updated along the search tree
#[derive(Debug, Clone)]
pub(crate) struct NNUEState {
    accumulator_stack: [Accumulator; MAX_DEPTH + 1],
    // index of the current accumulator
    current_acc: usize,
}


impl NNUEState {
    pub fn from_board(board: &BoardState) {
        let mut boxed: Box<Self> = unsafe {
            let layout = Layout::new::<Self>();
            let ptr = alloc_zeroed(layout);

            if ptr.is_null() {
                alloc::handle_alloc_error(layout);
            }

            Box::from_raw(ptr.cast())
        };

        // init with feature biases and add in all features of the board
        boxed.accumulator_stack[0] = Accumulator::default();
    
        // NEXT STEP: How do we convert the set bits(occupancy) to their respective square values
        // for sq in board.occupancy() {}
    
    }
}