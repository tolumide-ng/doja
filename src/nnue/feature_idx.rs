use std::ops::Deref;


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