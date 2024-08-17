use std::ops::Deref;

#[derive(Debug, Clone, Copy)]
pub struct FeatureIndex(usize);

impl Deref for FeatureIndex {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0    
    }
}