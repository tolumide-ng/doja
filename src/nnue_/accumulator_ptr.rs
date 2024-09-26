use std::ops::{Deref, DerefMut};

use super::accumulator::Accumulator;

#[derive(Debug, Clone)]
pub(crate) struct AccumulatorPtr<T, const U: usize>(pub(crate) *mut Accumulator<T, U>);
unsafe impl<T: Send, const U: usize> Send for AccumulatorPtr<T, U> {}
unsafe impl<T: Sync, const U: usize> Sync for AccumulatorPtr<T, U> {}


impl<T, const U: usize> Deref for AccumulatorPtr<T, U> {
    type Target = *mut Accumulator<T, U>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const U: usize> DerefMut for AccumulatorPtr<T, U> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
