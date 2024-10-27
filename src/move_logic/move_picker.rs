use crate::{
    board::{piece::Piece, position::Position},
    move_scope::MoveScope,
};

use super::{
    bitmove::{Move, MoveType},
    move_stack::MoveStack,
    scored_move::ScoredMove,
};

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum Stage {
    PV = 0,
    TTMove,
    InitCaptures,
    GoodCaptures,
    KillerZero,
    KillerOne,
    Quiets,
    BadCapture,
    Done,
}

/// Maximum legal moves is 218
/// At any point in time, there can always only be 16 captures
#[derive(Debug)]
pub(crate) struct MovePicker<'a, const QUIET: bool> {
    moves: MoveStack<ScoredMove>,
    // used only by the iterator (to indicate where we currently are in the loop)
    index: usize,
    stage: Stage,
    tt_move: Option<Move>,
    see_threshold: i32,
    killers: [Option<Move>; 2],
    position: &'a Position,
    // the index where bad captures start and end (start, end)
    total_captures: usize,
    total_good_captures: usize,
}

impl<'a, const QUIET: bool> MovePicker<'a, QUIET> {
    pub(crate) fn new(
        see_threshold: i32,
        tt_move: Option<Move>,
        killers: [Option<Move>; 2],
        position: &'a Position,
    ) -> Self {
        Self {
            moves: MoveStack::new(),
            index: 0,
            stage: Stage::TTMove,
            tt_move,
            see_threshold,
            killers,
            position,
            total_captures: 0,
            total_good_captures: 0,
        }
    }

    const GOOD_CAPTURE: i32 = 100_000;
    const QUEEN_PROMOTION: i32 = 30_500;
    const OTHER_PROMOTIONS: i32 = 12_500;
    const PROMOTES_AND_CAPTURE: i32 = 20_000;
    const BAD_CAPTURE: i32 = 1_000;

    const GOOD_SEE_CAPTURE: bool = true;

    fn score_captures(&mut self) {
        self.total_captures = self.moves.count_mvs();
        for index in 0..self.total_captures {
            let capture = self.moves.at_mut(index).unwrap();
            let mv = capture.mv();
            let pre_score = match mv.move_type() {
                MoveType::CaptureAndPromoteToQueen => {
                    Self::QUEEN_PROMOTION + Self::PROMOTES_AND_CAPTURE
                }
                _ if mv.get_promotion().is_some() => Self::OTHER_PROMOTIONS, // undepromotion (promotion to any type that isn't a queen)
                _ => 0,
            };

            let good_capture = self.position.see(&mv, self.see_threshold);
            let score = if good_capture {
                self.total_good_captures += 1;
                Self::GOOD_CAPTURE + pre_score
            } else {
                Self::BAD_CAPTURE
            };

            capture.update_score(score);
        }
    }

    /// Generic T indicates whether this sort is for captures(true) or quiet moves(false)
    /// If T is true, then we're sorting for captures only, else we're sorting for quiet moves only
    fn partial_insertion_sort<const T: bool>(&mut self, start: usize) -> Option<Move> {
        let end = if T {self.total_good_captures} else {self.moves.count_mvs()};
        let best_index = self.index;
        for i in best_index..end {
            if self.moves[i].score() > self.moves[best_index].score() {
                self.moves.swap(best_index, i);
            }
        }
        self.index += 1;
        return Some(self.moves[best_index].mv())
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
                return Some(mv);
            }
        }

        if self.stage == Stage::InitCaptures {
            self.stage = Stage::GoodCaptures;
            self.position
                .gen_movement::<{ MoveScope::CAPTURES }, ScoredMove>(&mut self.moves);
            self.score_captures();
        }

        if self.stage == Stage::GoodCaptures {
            if self.index < self.total_good_captures {
                return self.partial_insertion_sort::<true>(self.index);
            }
            if self.killers[0].is_some() {
                // self.stage = Stage::KillerZero;
            } else {
                self.stage = Stage::Quiets
            }
        }

        // if self.stage == Stage::KillerOne


        //  QUIET MOVES e..t.c 


        // BAD CAPTURES
        if self.stage == Stage::BadCapture {
            if self.total_captures != self.total_good_captures {
                self.index = self.total_good_captures;
                return self.partial_insertion_sort::<true>(self.index);
            }
        }

        unimplemented!()
    }
}
