// Not implementing this for now: owing to comments from here:
// https://talkchess.com/viewtopic.php?t=76255
// I should probably read: https://www.researchgate.net/publication/306413279_The_Countermove_Heuristic

// use crate::{move_logic::bitmove::Move, constants::TOTAL_SQUARES, squares::Square};

// /// CounterMove Heuristic
// /// https://www.chessprogramming.org/Countermove_Heuristic
// pub(crate) struct CounterMove([u16; TOTAL_SQUARES * TOTAL_SQUARES]);

// impl CounterMove {
//     pub(crate) fn new() -> Self {
//         CounterMove([0; TOTAL_SQUARES * TOTAL_SQUARES])
//     }

//     pub(crate) fn store(&mut self, src: Square, tgt: Square, mv: &Move) {
//         self.0[(src as usize * TOTAL_SQUARES) + tgt as usize] = **mv;
//     }
// }