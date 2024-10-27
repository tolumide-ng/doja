use super::move_action::MoveAction;


pub(crate) const STACK_SIZE: usize = 256; // should be changed to 218

#[derive(Debug, Clone, Copy)]
pub struct MoveStack<T: MoveAction> {
    pub(crate) list: [T; 256], // maximum possible legal MoveStack
    count: usize,
    /// Only used internally for the implementation of the iterator
    at: usize
}

impl<T: MoveAction> Default for MoveStack<T> {
    fn default() -> Self {
        Self { list: [T::default(); 256], count: 0, at: 0 }
    }
}


impl<T: MoveAction> MoveStack<T> {
    /// Creates a new move list with 256 items all intiialized as 0(zero)
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Adds a Move to the move list
    pub(crate) fn push(&mut self, m: T) {
        self.list[self.count] = m;
        self.count+=1;
    }

    pub(crate) fn count_mvs(&self) -> usize {self.count}

    pub(crate) fn to_vec(self) -> Vec<T> {
        self.list.into_iter().collect::<Vec<_>>()[..self.count].to_vec()
    }

    pub(crate) fn at(&self, index: usize) -> Option<&T> {
        self.list.get(index)
    }

    pub(crate) fn at_mut(&mut self, index: usize) -> Option<&mut T> {
        self.list.get_mut(index)
    }
}


impl<T: MoveAction> Iterator for MoveStack<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.at < self.count {
            let value = self.list[self.at];
            // println!(":::::calling at {}, and count{}", self.at, self.count);
            // for x in self.list {
            //     print!("====>>>{:?}", x);
            // }
            self.at += 1;
            let current = Some(self.list[self.at]);
            return current;
        }
        return None
    }
}