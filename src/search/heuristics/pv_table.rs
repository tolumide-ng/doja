use crate::{bit_move::Move, constants::params::MAX_DEPTH};

pub(crate) struct PVTable {
    /// Triangular PV-Table
    pub(crate) pv: [u16; MAX_DEPTH * MAX_DEPTH],
    /// Lengths of each PV lines (based on depth search)
    lengths: [usize; MAX_DEPTH],
}


impl PVTable {
    pub(crate) fn new() -> Self {
        Self { pv: [0; MAX_DEPTH * MAX_DEPTH], lengths: [0; MAX_DEPTH] }
    }

    #[inline(always)]
    const fn index(depth: usize) -> usize {
        MAX_DEPTH * depth
    }

    /// Store a move at a specific depth on the PV-Table
    pub(crate) fn store_pv(&mut self, depth: usize, mv: &Move) {
        // println!("called with {} at depth {depth}", mv.to_string());
        // println!("called****** with {} at depth {}", mv.to_string(), depth);
        // Prepends the new move to the PVs at this depth("d")
        let index = Self::index(depth);
        let prev_depth_index = Self::index(depth + 1);


        // println!("(((((((((((((({index})))))))))))))) {} {}", self.pv[index..index+MAX_DEPTH].iter().filter(|x| **x != 0 ).collect::<Vec<_>>().len(), Move::from(self.pv[index]).to_string());
        // println!(">>>>>>>>>>>>>>>>{prev_depth_index}<<<<<<<<<<<<< {}-->>{}", self.pv[prev_depth_index..prev_depth_index+MAX_DEPTH].iter().filter(|x| **x != 0 ).collect::<Vec<_>>().len(), Move::from(self.pv[prev_depth_index]).to_string());
        self.pv[index] = **mv;

        // println!("index is {index}, and the previous is {prev_depth_index}");
        let len = self.lengths[depth + 1];
        // println!("len at that position is {}", len);

        // if depth == 0 {
        //     println!("got zero ---")
        // }

        // let mut children = 

        // Copy the PV from depth + 1 into this depth
        for i in 0..len {
            self.pv[index + i + 1] = self.pv[prev_depth_index + i];
        }

        // let curr = self.pv[index..index+MAX_DEPTH].iter().filter(|x| **x != 0 ).collect::<Vec<_>>().len();
        // let prev = self.pv[prev_depth_index..prev_depth_index+MAX_DEPTH].iter().filter(|x| **x != 0 ).collect::<Vec<_>>().len();

        // for i in 0..curr {
        //     print!("curr:: {}-->>", Move::from(self.pv[index + i]).to_string())
        // }
        // println!("total current is {curr}");
        // println!("\n");
        

        // for i in 0..prev {
        //     println!("prev** {}->", Move::from(self.pv[prev_depth_index + i]).to_string());
        // }
        // println!("the previous count was {prev}");
        // println!("\n\n\n");



        // println!("::::::::::::::::::::{index} <<<< {} {}", self.pv[index..index+MAX_DEPTH].iter().filter(|x| **x != 0 ).collect::<Vec<_>>().len(), Move::from(self.pv[index]).to_string());
        // println!("[[[[[[[[[[[[[[++++++++{prev_depth_index}+++++++]]]]]]]]]]]]]] {}-->>{}", self.pv[prev_depth_index..prev_depth_index+MAX_DEPTH].iter().filter(|x| **x != 0 ).collect::<Vec<_>>().len(), Move::from(self.pv[prev_depth_index]).to_string());

        self.lengths[depth] = len + 1;
    }

    pub(crate) fn get_pv(&self, depth: usize) -> &[u16] {
        let index = Self::index(depth);
        &self.pv[index.. index + self.lengths[depth]]
    }

    pub(crate) fn len(&self, depth: usize) -> usize {
        self.lengths[depth]
    }

    /// Clear PV lines (when starting a new search)
    #[inline(always)]
    pub(crate) fn clear(&mut self) {
        self.lengths.fill(0);
    }
}