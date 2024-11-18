use crate::{
    board::{piece::{Piece, PieceType}, position::Position}, move_scope::MoveScope, search::heuristics::{capture_history::CaptureHistory, continuation_history::ContinuationHistory, countermove::{self, CounterMove}, history::HistoryHeuristic}
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
    // KillerZero,
    // KillerOne,
    Quiets,
    BadCapture,
    Done,
}

/// Maximum legal moves is 218
/// At any point in time, there can always only be 16 captures
/// T denotes the MoveScope (Capture, Quiet, or All moves)
#[derive(Debug)]
pub(crate) struct MovePicker<const T: u8> {
    moves: MoveStack<ScoredMove>,
    // used only by the iterator (to indicate where we currently are in the loop)
    index: usize,
    pub(crate) stage: Stage,
    tt_move: Option<Move>,
    see_threshold: i32,
    killers: [Option<Move>; 2],
    // the index where bad captures start and end (start, end)
    total_captures: usize,
    total_good_captures: usize,
}

impl<const T: u8> MovePicker<T> {
    pub(crate) fn new(see_threshold: i32, tt_move: Option<Move>, killers: [Option<Move>; 2]) -> Self {
        Self {
            moves: MoveStack::new(),
            index: 0,
            stage: Stage::TTMove,
            tt_move,
            see_threshold,
            killers,
            total_captures: 0,
            total_good_captures: 0,
        }
    }

    const GOOD_CAPTURE: i32 = 50_000;
    const QUEEN_PROMOTION: i32 = 30_500;
    const OTHER_PROMOTIONS: i32 = 12_500;
    const PROMOTES_AND_CAPTURE: i32 = 20_000;
    const BAD_CAPTURE: i32 = 1_000;
    const KILLER_MV: i32 = 6_000;
    const COUNTER_MV: i32 = 5_000;

    fn score_captures(&mut self, position: &Position, caphist: &CaptureHistory) {
        self.total_captures = self.moves.count();
        // println!("{:?}", self.moves);
        for index in 0..self.total_captures {
            let capture = self.moves.at_mut(index).unwrap();
            // println!("############################################# {:?} {}", capture, capture.mv());
            let mv = capture.mv();
            // let attacker = (position.piece_at(mv.get_src()).unwrap() as usize) % 6;
            // let victim = (position.piece_at(mv.get_target()).unwrap() as usize) % 6;

            let attacker = position.piece_at(mv.get_src()).unwrap();
            let victim = position.piece_at(mv.get_target()).unwrap();
            let pre_score = match mv.move_type() {
                MoveType::CaptureAndPromoteToQueen => {
                    Self::QUEEN_PROMOTION + Self::PROMOTES_AND_CAPTURE
                }
                _ if mv.get_promotion().is_some() => Self::OTHER_PROMOTIONS, // undepromotion (promotion to any type that isn't a queen)
                // _ => Piece::MVV_LVA[victim] - Piece::MVV_LVA[attacker],
                _ => {
                    // let scc = caphist.get(attacker, mv.get_target(), PieceType::from(victim));
                    // println!("the scc here is already  {scc}, the max is {}", i16::MAX);
                    (Piece::MVV[victim as usize % 6] as i32 + caphist.get(attacker, mv.get_target(), PieceType::from(victim)) as i32) as i32},
            };

            let good_capture = position.see(&mv, self.see_threshold);
            let score = if good_capture {
                self.total_good_captures += 1;
                Self::GOOD_CAPTURE + pre_score
            } else {
                Self::BAD_CAPTURE
            };

            capture.update_score(score);
        }
    }

    fn score_quiets(&mut self, position: &Position, history_table: &HistoryHeuristic, conthist: &ContinuationHistory, counter_mvs: &CounterMove) {
        let start = self.index; let end = self.moves.count_mvs();
        let prev_mv_idx = position.history_len().checked_sub(1);
        let parent = prev_mv_idx.and_then(|idx| position.history_at(idx)).map(|history| 
            {
                let prev_mv = history.mv();
                let piece = history.board().piece_at(prev_mv.src()).unwrap();
                // (history.mvd_piece(), history.mv())
                (piece, prev_mv)
            }
        );
        
        for index in start..end {
            let quiet_mv = self.moves.at_mut(index).unwrap();
            let mv = quiet_mv.mv();
            if self.killers[0].is_some_and(|m| m == mv) {
                quiet_mv.update_score(Self::KILLER_MV);
                continue;    
            }

            if self.killers[1].is_some_and(|m| m == mv) {
                quiet_mv.update_score(Self::KILLER_MV - 20_000);
                continue;
            }

            let p = position.piece_at(mv.get_src()).unwrap();
            quiet_mv.update_score(history_table.get(p, mv.get_target()) as i32);
            if let Some((prev_piece, prev_mv)) = parent {
                let prev_tgt = prev_mv.get_tgt(); let prev_src = prev_mv.get_src();
                let conthist_score = conthist.get(prev_piece, prev_tgt, p, quiet_mv.mv().get_tgt());
                let countermv = counter_mvs.get(prev_src, prev_tgt);
                if countermv == quiet_mv.mv() { quiet_mv.update_score(conthist_score as i32)}
                quiet_mv.update_score(conthist_score as i32);
            }
        }
    }

    /// Generic T indicates whether this sort is for captures(true) or quiet moves(false)
    /// If T is true, then we're sorting for captures only, else we're sorting for quiet moves only
    fn partial_insertion_sort<const CAPTURES: bool>(&mut self, start: usize) -> Option<Move> {
        let end = if CAPTURES {self.total_captures} else {self.moves.count_mvs()};
        let best_index = self.index;
        for i in best_index..end {
            // println!("current best is {} comparing against ===>>> {}", self.moves[i].mv(), self.moves[best_index].mv());
            if self.moves[i].score() > self.moves[best_index].score() {
                self.moves.swap(best_index, i);
            }
        }
        self.index += 1;
        return Some(self.moves[best_index].mv())
    }

    pub(crate) fn next(&mut self, position: &Position, history_table: &HistoryHeuristic, caphist: &CaptureHistory, conthist: &ContinuationHistory, counter_mvs: &CounterMove) -> Option<Move> {
        let move_scope = MoveScope::from(T);
        if self.stage == Stage::Done {
            return None;
        }

        
        if self.stage == Stage::TTMove {
            // println!("here with *********---------------------------------------------------------------------*********---------------------------------------------------------------------");
            if move_scope == MoveScope::QuietOnly { self.stage = Stage::Quiets } else { self.stage = Stage::InitCaptures };
            
            if let Some(mv) = self.tt_move {
                if position.is_pseudo_legal(&mv) {
                    let expects_quiet_but_tt_is_capturing = move_scope == MoveScope::QuietOnly && mv.is_capture();
                    let expects_capturing_but_tt_is_quiet = move_scope == MoveScope::CapturesOnly && !mv.is_capture();
                    let does_not_fit_expectations = expects_capturing_but_tt_is_quiet || expects_quiet_but_tt_is_capturing;
                    let fits_expectations = !does_not_fit_expectations;

                    if fits_expectations {
                        return Some(mv);
                    }
                 }
            }
        }

        if self.stage == Stage::InitCaptures {
            // println!("should only be here once");
            self.stage = Stage::GoodCaptures;
            position.gen_movement::<{ MoveScope::CAPTURES }, ScoredMove>(&mut self.moves);
            self.score_captures(position, caphist);
        }
        
        if self.stage == Stage::GoodCaptures {
            if self.index < self.total_good_captures {
                // need to confirm that this is not the TT_MOVE or PV_MOVE that was returned earlier
                let next_mv = self.partial_insertion_sort::<true>(self.index);
                if let Some(mv) = next_mv {
                    if self.tt_move.is_some_and(|tt_mv| tt_mv == mv) {
                        return self.next(position, history_table, caphist, conthist, counter_mvs); }
                    return Some(mv);
                }
        }
            
            // we wouldn't be here in the first place if the move is not CapturesOnly or AllMoves, so those are the only two we need to check
            if move_scope == MoveScope::AllMoves {
                self.stage = Stage::Quiets;
                self.index = self.total_captures;
                
                position.gen_movement::<{MoveScope::QUIETS}, ScoredMove>(&mut self.moves);
                self.score_quiets(position, history_table, conthist, counter_mvs);
            } else  { // movescope == MoveScope::CapturesOnly
                self.stage = Stage::BadCapture;
                self.index = self.total_good_captures;
            }
        }

        

        if self.stage == Stage::Quiets {
            if self.index < self.moves.count_mvs() {
                let next_mv = self.partial_insertion_sort::<true>(self.index);
                if let Some(mv) = next_mv {
                    if self.tt_move.is_some_and(|tt_mv| tt_mv == mv) { return self.next(position, history_table, caphist, conthist, counter_mvs); }
                }
                return next_mv;
            }
            if move_scope == MoveScope::QuietOnly { self.stage == Stage::Done; return None }
            self.index = self.total_good_captures;
            self.stage = Stage::BadCapture;
        }
        
        // println!("[[[[[[[[[********************]]]]]]]]]");
        // BAD CAPTURES
        if self.stage == Stage::BadCapture {
            if self.total_captures != self.total_good_captures && self.index < self.total_captures {
                let next_one = self.partial_insertion_sort::<true>(self.index);
                if  let Some(mv) = next_one {
                    if self.tt_move.is_some_and(|tt_mv| tt_mv == mv) { return self.next(position, history_table, caphist, conthist, counter_mvs); }
                }
                return next_one;
            }
        }

        self.stage == Stage::Done;
        
        None
    }


}
