use std::time::{Duration, Instant};

use super::counter::Counter;


#[derive(Debug, Clone)]
pub(crate) struct Clock {
    /// UCI "starttime" command time holder
    start_time: Instant,
    /// Search Limit
    limit: Counter,
    max_time: Duration,
    opt_time: Duration,
}

impl Default for Clock {
    fn default() -> Self {
        Self {
            start_time: Instant::now(),
            limit: Counter::default(),
            max_time: Duration::from_secs(0),
            opt_time: Duration::from_secs(0)
        }
    }
}

impl Clock {
    pub(crate) fn start(&mut self) {
        self.start_time = Instant::now();
    }

    pub(crate) fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    pub(crate) fn set_limit(&mut self, limit: Counter) {
        self.limit = limit;
    }

    pub(crate) fn limit(&self) -> &Counter {
        &self.limit
    }

    pub(crate) fn stop(&self) -> bool {
        match self.limit {
            // Counter::Depth(d) => 
            _ => false
        }
    }
}