use std::ops::{Deref, DerefMut};

use super::move_action::MoveAction;


pub(crate) const STACK_SIZE: usize = 218; // should be changed to 218

#[derive(Debug, Clone, Copy)]
pub struct MoveStack<T: MoveAction> {
    list: [T; STACK_SIZE], // maximum possible legal MoveStack
    count: usize,
    /// Only used internally for the implementation of the iterator
    at: usize
}

impl<T: MoveAction> Default for MoveStack<T> {
    fn default() -> Self {
        Self { list: [T::default(); STACK_SIZE], count: 0, at: 0 }
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
            self.at += 1;
            let current = Some(self.list[self.at]);
            return current;
        }
        return None
    }
}


impl<T: MoveAction> Deref for MoveStack<T> {
    type Target = [T; STACK_SIZE];

    fn deref(&self) -> &Self::Target {
        &self.list    
    }
}


impl<T: MoveAction> DerefMut for MoveStack<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.list
    }
}