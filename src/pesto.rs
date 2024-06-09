use std::ops::Index;



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
