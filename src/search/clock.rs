use std::{sync::atomic::AtomicBool, time::Instant};

pub(crate) struct Clock {
    movestogo: u32,
    movetime: u128,
    time: u128,
    inc: u32, 
    startime: Instant,
    stoptime: u128,
    timeset: bool,
    stopped: AtomicBool,
    depth: u8

}