use std::{cmp, sync::atomic::{AtomicBool, Ordering}, time::{Duration, Instant}};

use crate::color::Color;

use super::counter::Counter;



#[derive(Debug, PartialEq)]
enum TimeCalc {
    Ideal, Max
}

impl TimeCalc {
    #[inline(always)]
    pub(crate) fn max_ratio(&self) -> f64 {
        match *self {
            Self::Ideal => 1.0,
            Self::Max => 6.32,
        }
    }

    #[inline(always)]
    pub(crate) fn steal_ratio(&self) -> f64 {
        match *self {
            Self::Ideal => 0.0,
            Self::Max => 0.34,
        }
    } 
}


#[derive(Debug, Clone)]
pub(crate) struct Clock {
    /// UCI "starttime" command time holder
    start_time: Instant,
    /// Search Limit
    limit: Counter,
    max_time: Duration,
    opt_time: Duration,
    stop: *const AtomicBool
}

unsafe impl Send for Clock {}
unsafe impl Sync for Clock {}

impl Clock {
    pub(crate) fn new(stop: *const AtomicBool) -> Self {
        Self {
            start_time: Instant::now(),
            limit: Counter::default(),
            max_time: Duration::from_secs(0),
            opt_time: Duration::from_secs(0),
            stop
        }
    }


    pub(crate) fn start(&mut self) {
        self.start_time = Instant::now();
    }

    pub(crate) fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    // const MIN_THINKING_TIME: u64 = 20;
    const MOVE_OVERHEAD: i64 = 100; // try 10 ?
    const MOVE_HORIZON: u64 = 50;
    // const MAX_RATIO: f64 = 6.32;
    // const STEAL_RATIO: f64 = 0.34;

    pub(crate) fn set_limit(&mut self, limit: Counter, stm: Color) {
        if let Counter::Dynamic { wtime, btime, winc, binc, movestogo } = limit {
            // most part of this is currently derived from obsidian
            let (our_inc, our_time) = if stm == Color::White {(winc, wtime)} else {(binc, btime)};

            // let opt_time = our_time.max(Self::MIN_THINKING_TIME);

            let mtg = match movestogo {
                Some(m) if m != 0 => {m.min(Self::MOVE_HORIZON)}
                _ => Self::MOVE_HORIZON
            };

            
            // let time_left = 1.max(other)
            let hyp_time = our_time as i64 + our_inc as i64 * (mtg as i64 - 1) - Self::MOVE_OVERHEAD * (2 + mtg as i64);
            let time_left = cmp::max(1, hyp_time) as u64;
            
            let opt_scale = if movestogo.unwrap_or(0) == 0 {
                0.025_f64.min(0.214_f64 * our_time as f64 / time_left as f64) as u64
            } else {
                (0.95/mtg as f64).min(0.88_f64 * our_time as f64 / time_left as f64) as u64
            };

            self.opt_time = Duration::from_millis(opt_scale  * time_left);
            let max_time = (our_time as f64 * 0.8 - Self::MOVE_OVERHEAD as f64) as u64;
            self.max_time = Duration::from_millis(max_time);
        }

        self.limit = limit;

    }

    // fn remaining(time: i64, movestogo: i64, num: i64, slow_mover: f64, time_type: TimeCalc) {
    //     let move_importance = 
    // }

    pub(crate) fn limit(&self) -> &Counter {
        &self.limit
    }

    pub(crate) fn stop(&self, nodes: u64, depth: u8) -> bool {
        let global_stop = unsafe{self.stop.read()}.load(Ordering::SeqCst);
        if global_stop { return true };

        match self.limit {
            Counter::Depth(d) => depth > d,
            Counter::Mate(ply) => ply/2 > (depth as u64),
            Counter::Nodes(n) => nodes > n,
            Counter::Time(t) => {
                let elapsed = self.start_time.elapsed().as_millis() as u64;
                elapsed > t
            }
            Counter::Dynamic { .. } => {
                self.elapsed() > self.max_time 
            }
            _ => false
        }
    }
}