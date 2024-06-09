use std::ops::Index;

use crate::constants::{END_PHASE_SCORE, OPENING_PHASE_SCORE};



#[derive(Debug, Clone, Copy)]
pub enum GamePhase {
    Opening = 0,
    EndGame = 1,
    MiddleGame = 2,
}

impl<T> Index<GamePhase> for [T] {
    type Output = T;

    fn index(&self, index: GamePhase) -> &Self::Output {
        match index {
            GamePhase::Opening => &self[0],
            GamePhase::EndGame => &self[1],
            GamePhase::MiddleGame => &self[2],
        }
    }
}

impl From<i32> for GamePhase {
    fn from(score: i32) -> Self {
         if score > OPENING_PHASE_SCORE {
            return GamePhase::Opening
        } else if score < OPENING_PHASE_SCORE && score < END_PHASE_SCORE {
            return GamePhase::EndGame
        }

        return GamePhase::MiddleGame
    }
}
