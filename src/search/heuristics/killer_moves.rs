use crate::{constants::params::MAX_DEPTH, move_logic::bitmove::Move};


const NUM_KILLER_MOVES: usize = 2;

#[derive(Debug, Clone)]
pub(crate) struct KillerMoves([[u16; NUM_KILLER_MOVES]; MAX_DEPTH]); // should be changed back to total_squares(64) after pruning is implemented todo!!

impl KillerMoves {
    pub(crate) fn new() -> Self {
        Self([[0; NUM_KILLER_MOVES]; MAX_DEPTH])
    }

    pub(crate) fn store(&mut self, depth: usize, mv: &Move) {
        
        if self.0[depth][0] != **mv {
            self.0[depth][1] = self.0[depth][0];
            self.0[depth][0] = **mv;
        }
    }

    pub(crate) fn is_killer(&self, depth: usize, mv: &Move) -> bool {
        self.0[depth][0] == **mv || self.0[depth][1] == **mv
    }

    pub(crate) fn get_killers(&self, depth: usize) -> [u16; 2] {
        self.0[depth]
    } 
}