// Not implementing this for now: owing to comments from here:
// https://talkchess.com/viewtopic.php?t=76255
// I should probably read: https://www.researchgate.net/publication/306413279_The_Countermove_Heuristic

use crate::{board::position::Position, move_logic::bitmove::Move, squares::Square};

/// CounterMove Heuristic
/// https://www.chessprogramming.org/Countermove_Heuristic
pub(crate) struct CounterMove([u16; Square::TOTAL * Square::TOTAL]);

impl CounterMove {
    pub(crate) fn new() -> Self {
        CounterMove([0; Square::TOTAL * Square::TOTAL])
    }

    pub(crate) fn add(&mut self, pos: &Position, mv: Move) {
        if let Some(history) = pos.last_history() {
            let src = history.mv().src(); let tgt = history.mv().tgt();
            self.0[Self::index(src, tgt)] = *mv;
        } 
    }

    pub(crate) fn add_many(&mut self, pos: &Position, mvs: &Vec<Move>) {
        for mv in mvs {
            self.add(pos, *mv);
        }
    }

    const fn index(src: Square, tgt: Square) -> usize {
        ((src as usize) * Square::TOTAL) + tgt as usize
    }

    pub(crate) fn get(&self, src: Square, tgt: Square) -> Move {
        Move::from(self.0[Self::index(src, tgt)])
    }

    // pub(crate) fn get_counter_mv(&self, pos: &Position) -> Option<Move> {
    //     // let position. 
    // }
}