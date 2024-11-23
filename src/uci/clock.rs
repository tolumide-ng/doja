use std::{str::SplitWhitespace, time::{Duration, Instant}};

use super::UciError;


#[derive(Debug, Default, Clone, PartialEq)]
pub(crate) enum Counter {
    /// Only search up to depth x
    Depth(u8),
    /// Search for a move in x msec
    Time(u64),
    /// Search x nodes
    Nodes(u64),
    #[default]
    Infinite,
    /// Search for a mate in x plys
    Mate(u64),
    Dynamic {
        wtime: u64,
        btime: u64,
        winc: Option<u64>,
        binc: Option<u64>,
        movestogo: Option<u64>
    }
    // Search
}

impl<'a> TryFrom<SplitWhitespace<'a>> for Counter {
    type Error = UciError;
    fn try_from(mut input: SplitWhitespace) -> Result<Self, Self::Error> {
        // let depth
        match input.next() {
            Some("infinite") => { return Ok(Counter::Infinite) }
            // Some("searchmoves") => {}
            Some("depth") => {
                let Some(d) = input.next() else {return Err(UciError::NoValue("depth"))};
                let Ok(depth) = u8::from_str_radix(d, 10) else {return Err(UciError::InvalidIntegerArgument(d.to_string()))};
                return Ok(Counter::Depth(depth))
            }
            Some("binc") => {
                let Some(binc_value) = input.next() else { return Err(UciError::NoValue("binc")) };
                let Ok(bincv) = u64::from_str_radix(binc_value, 10) else {return Err(UciError::InvalidIntegerArgument(binc_value.to_string()))};

                let counter = match Self::try_from(input) {
                    Ok(mut mgr) if matches!(mgr, Counter::Dynamic { .. })=> {
                        if let Counter::Dynamic { binc, .. } = &mut mgr {
                            *binc = Some(bincv)
                        }
                        mgr
                    },
                    Err(e) if e == UciError::EmptyArgument => { Self::Dynamic { wtime: 0, btime: 0, winc: None, binc: None, movestogo: None } },
                    Err(e) => return Err(e),
                    _ => return Err(UciError::NoValue(""))
                };
                Ok(counter)
            }
            Some("winc") => {
                let Some(winc) = input.next() else { return Err(UciError::NoValue("winc")) };
                let Ok(wincv) = u64::from_str_radix(winc, 10) else {return Err(UciError::InvalidIntegerArgument(winc.to_string()))};

                let counter = match Self::try_from(input) {
                    Ok(mut mgr) if matches!(mgr, Counter::Dynamic { .. }) => {
                        if let Counter::Dynamic { binc, .. } = &mut mgr {
                            *binc = Some(wincv)
                        }
                        mgr
                    }
                    Err(e) if e == UciError::EmptyArgument => { Self::Dynamic { wtime: 0, btime: 0, winc: None, binc: None, movestogo: None } },
                    Err(e) => return Err(e),
                    _ => return Err(UciError::NoValue(""))
                };
                
                Err("()")
            }
            _ => Err(UciError::EmptyArgument)
        }
        // Err("")
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
}