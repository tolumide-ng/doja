use crate::board::{self, position::Position};

use super::{bitmove::Move, move_list::Moves};

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum Stage {
    PV=0,
    TTMove,
    GoodCapture,
    KillerZero,
    KillerOne,
    BadCapture,
    Done,
}


#[derive(Debug)]
pub(crate) struct MovePicker<'a, const QUIET: bool> {
    moves: [u16; Moves::SIZE],
    scores: [i32; Moves::SIZE],
    index: usize,
    stage: Stage,
    tt_move: Option<Move>,
    see_threshold: i32,
    killers: [Option<Move>; 2],
    position: &'a Position,
    // the index where bad captures end
    // bad_captures: usize,
}

impl<'a, const QUIET: bool> MovePicker<'a, QUIET> {
    pub(crate) fn new(see_threshold: i32, tt_move: Option<Move>, killers: [Option<Move>; 2], position: &'a Position) -> Self {
        Self { moves: [0; Moves::SIZE], scores: [0; Moves::SIZE], index: 0, stage: Stage::TTMove, tt_move, see_threshold, killers, position }
    }

    fn score_captures(&mut self, mvs: &mut Vec<Move>) {
        
    }


    fn partial_insertion_sort(&self) {
        unimplemented!()
    }
}


impl<'a, const QUIET: bool> Iterator for MovePicker<'a, QUIET> {
    type Item = Move;
    fn next(&mut self) -> Option<Self::Item> {
        if self.stage == Stage::Done {
            return None;
        }

        if self.stage == Stage::TTMove {}


        unimplemented!()
    }
}