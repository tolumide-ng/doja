use crate::{
    board::{piece::{Piece, PieceType}, position::Position}, move_scope::MoveScope, search::heuristics::{capture_history::CaptureHistory, continuation_history::ContinuationHistory, history::HistoryHeuristic}
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
    stage: Stage,
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
                _ => (Piece::MVV[victim as usize % 6] + caphist.get(attacker, mv.get_target(), PieceType::from(victim))) as i32,
            };

            // if ["d2c3x", "e2a6x"].contains(&(mv.to_string()).as_str()) {
            //     let xx = caphist.get(attacker, mv.get_target(), PieceType::from(victim));
            //     println!("{} the score here is {} ----->>>>>>> {xx}", mv.to_string(), pre_score);
            //     // println!(":::: ---- {}", Piece::MVV[victim as usize % 6])
            // }

            let good_capture = position.see(&mv, self.see_threshold);
            if ["d2c3x", "e2a6xz"].contains(&(mv.to_string()).as_str()) {
                // println!("victim>>>>> {}", victim);
                // let xx = caphist.get(attacker, mv.get_target(), PieceType::from(victim));
                // let abc = Piece::MVV[victim as usize % 6];
                // println!("{} the score here is {} ----->>>>>>> {xx} {}", mv.to_string(), good_capture, abc);
                // println!(":::: ---- {}", Piece::MVV[victim as usize % 6])
            }
            let score = if good_capture {
                self.total_good_captures += 1;
                Self::GOOD_CAPTURE + pre_score
            } else {
                Self::BAD_CAPTURE
            };

            capture.update_score(score);
        }
    }

    fn score_quiets(&mut self, position: &Position, history_table: &HistoryHeuristic, conthist: &ContinuationHistory) {
        let start = self.index; let end = self.moves.count_mvs();
        let prev_mv_idx = position.history_len().checked_sub(1);
        let parent = prev_mv_idx.and_then(|idx| position.history_at(idx)).map(|history| (history.mvd_piece(), history.mv()));

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
                let value = conthist.get(prev_piece, prev_mv.get_tgt(), p, quiet_mv.mv().get_tgt());
                // println!("the value of mv--> {} is {}", mv, value);
                quiet_mv.update_score(value as i32);
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

    pub(crate) fn next(&mut self, position: &Position, history_table: &HistoryHeuristic, caphist: &CaptureHistory, conthist: &ContinuationHistory) -> Option<Move> {
        let move_scope = MoveScope::from(T);
        if self.stage == Stage::Done {
            return None;
        }

        if self.stage == Stage::TTMove {
            if move_scope == MoveScope::QuietOnly { self.stage = Stage::Quiets; } else { 
                self.stage = Stage::InitCaptures;
                if let Some(mv) = self.tt_move {
                    return Some(mv);
                }
            }
        }

        if self.stage == Stage::InitCaptures {
            // println!("should only be here once");
            self.stage = Stage::GoodCaptures;
            position.gen_movement::<{ MoveScope::CAPTURES }, ScoredMove>(&mut self.moves);

            // for i in 0..self.moves.count() {
            //     let m = self.moves[i];
            //     print!("==={}--{}===>>", m.mv(), m.score());
            // }

            // println!("\n\n");
            self.score_captures(position, caphist);
            // for i in 0..self.moves.count() {
            //     let m = self.moves[i];
            //     print!("==={}--{}===>>", m.mv(), m.score());
            // }
            // println!("\n total={}, good={} \n\n", self.total_captures, self.total_good_captures);
        }
        
        if self.stage == Stage::GoodCaptures {
            if self.index < self.total_good_captures {
                // need to confirm that this is not the TT_MOVE or PV_MOVE that was returned earlier
                return self.partial_insertion_sort::<true>(self.index);
            }
            
            // we wouldn't be here in the first place if the move is not CapturesOnly or AllMoves, so those are the only two we need to check
            if move_scope == MoveScope::AllMoves {
                self.stage = Stage::Quiets;
                self.index = self.total_captures;
                
                position.gen_movement::<{MoveScope::QUIETS}, ScoredMove>(&mut self.moves);
                self.score_quiets(position, history_table, conthist);
            } else  { // movescope == MoveScope::CapturesOnly
                self.stage = Stage::BadCapture;
                self.index = self.total_good_captures;
            }
        }

        

        if self.stage == Stage::Quiets {
            if self.index < self.moves.count_mvs() {
                // need to confirm that this is not the TT_MOVE or PV_MOVE that was returned earlier
                return self.partial_insertion_sort::<true>(self.index);
            }
            if move_scope == MoveScope::QuietOnly { return None }
            self.index = self.total_good_captures;
            self.stage = Stage::BadCapture;
        }
        
        // println!("[[[[[[[[[********************]]]]]]]]]");
        // BAD CAPTURES
        if self.stage == Stage::BadCapture {
            if self.total_captures != self.total_good_captures && self.index < self.total_captures {
                return self.partial_insertion_sort::<true>(self.index);
            }
        }
        
        None
    }


}
