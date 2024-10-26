use crate::{board::position::Position, move_scope::MoveScope};

use super::{bitmove::Move, move_list::Moves};

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum Stage {
    PV=0,
    TTMove,
    InitCaptures,
    GoodCapture,
    KillerZero,
    KillerOne,
    BadCapture,
    Done,
}



/// Maximum legal moves is 218
/// At any point in time, there can always only be 16 captures
#[derive(Debug)]
pub(crate) struct MovePicker<'a, const QUIET: bool> {
    moves: Moves,
    scores: [i32; Moves::SIZE],
    // used only by the iterator (to indicate where we currently are in the loop)
    index: usize,
    stage: Stage,
    tt_move: Option<Move>,
    see_threshold: i32,
    killers: [Option<Move>; 2],
    position: &'a Position,
    // the index where bad captures start and end (start, end)
    bad_captures: (usize, usize),
}

impl<'a, const QUIET: bool> MovePicker<'a, QUIET> {
    pub(crate) fn new(see_threshold: i32, tt_move: Option<Move>, killers: [Option<Move>; 2], position: &'a Position) -> Self {
        Self { moves: Moves::new(), scores: [0; Moves::SIZE], index: 0, stage: Stage::TTMove, tt_move, see_threshold, killers, position, bad_captures: (0, 0) }
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

        if self.stage == Stage::TTMove {
            self.stage = Stage::InitCaptures;
            if let Some(mv) = self.tt_move {
                return Some(mv)
            }
        }

        if self.stage == Stage::InitCaptures {
            self.stage = Stage::GoodCapture;
            self.position.gen_movement::<{MoveScope::CAPTURES}>(&mut self.moves);
            
        }

        if self.stage == Stage::GoodCapture {}




        unimplemented!()
    }
}