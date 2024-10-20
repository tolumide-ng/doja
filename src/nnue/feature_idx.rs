use std::ops::Deref;




/// This should be changed eventually to match an index that addresses white and black's perspcetive
/// something like: FeatureIdx(usize, usize) where 0=> white, and 1=> black
#[derive(Debug)]
pub(crate) struct FeatureIdx(usize);

impl FeatureIdx {
    pub(crate) fn new(value: usize) -> Self {
        Self(value)
    }
}

impl From<usize> for FeatureIdx {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl Deref for FeatureIdx {
    type Target = usize;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}