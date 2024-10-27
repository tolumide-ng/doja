use crate::{board::position::Position, move_scope::MoveScope};

use super::{bitmove::Move, move_stack::MoveStack};

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
    scores: [i32; MoveStack::SIZE],
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
        Self { moves: MoveStack::new(), scores: [0; MoveStack::SIZE], index: 0, stage: Stage::TTMove, tt_move, see_threshold, killers, position, bad_captures: (0, 0) }
    }

    const GOOD_CAPTURE: usize = 20_000;
    const BAD_CAPTURE: usize = 6_000;

    const GOOD_SEE_CAPTURE: bool = true;

    /// Assuming we have 5 Moves => A(bad), B(bad), C(good), D(good), E(bad), F(good)
    /// where good_captures(start) is initialized as 0, and bad_captures(end) is initialized as 5 (i.e. mvs.len()-1); where  
    /// start = 0, and end=5:: If start is bad, and end is good, -> swap, then increase start (start += 1), and decrease end (end -=1)
    /// (s=1, e=4); start is bad, and end is bad, -> decrease the end only (e-=1)
    /// (s=1, e=3); start is bad, and end is good, -> swap, then start (start += 1), and decrease end (e-=1)
    /// (s=2, e=2); if start == end -> break 
    fn score_tacticals(&mut self) {
        // there's no need to sort, I just simply assign a lower score to all bad captures, and record the max length of the captures provided earlier
        // let 
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
            let len = self.moves.count();
            for mv in self.moves {
                let xx = self.position.see(&mv, self.see_threshold);
            }
        }

        if self.stage == Stage::GoodCapture {}




        unimplemented!()
    }
}