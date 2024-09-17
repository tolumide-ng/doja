use std::ops::Deref;

pub(crate) struct FeatureIdx(usize);

impl Deref for FeatureIdx {
    type Target = usize;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}