use crate::bit_move::Move;


const NUM_KILLER_MOVES: usize = 2;

#[derive(Debug)]
pub(crate) struct KillerMoves([[u16; 2]; NUM_KILLER_MOVES]);

impl KillerMoves {
    pub(crate) fn new() -> Self {
        Self([[0; 2]; NUM_KILLER_MOVES])
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
}