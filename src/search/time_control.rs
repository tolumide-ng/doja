pub(crate) trait TimeControl: Default {
    fn communicate(&mut self);
    
    fn stopped(&self) -> bool;
}