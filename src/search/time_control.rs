
/// COMPLETELY REPLACE THIS WITH ATOMICBOOL + RELAXED ORDERING
pub(crate) trait TimeControl: Default {
    fn communicate(&mut self);
    
    fn stopped(&self) -> bool;
}