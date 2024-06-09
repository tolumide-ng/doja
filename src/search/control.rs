use std::{io::{self, stdout, BufRead, Read, Write}, os::fd::AsRawFd, sync::mpsc, thread, time::Instant};

use nix::fcntl::{self, fcntl, FcntlArg, OFlag};

use super::{negamax::NegaMax, time_control::TimeControl};


/// Use this for commands that can be executed while a search is ongoing
#[derive(Debug, Clone, Copy)]
pub struct Control {
    // UCI "movestogo" command moves counter
    movestogo: u32,
    // search exactly x mseconds: amount of time allowed to spend making a move
    movetime: u128,
    // UCI "time" command holder (ms)
    time: u128,
    // UCI "inc" command's time increment holder
    inc: u32,
    // UCI "starttime" command time holder
    starttime: Instant,
    // UCI "stoptime" command time holder
    stoptime: u128,
    // variable to flag time control availiability
    timeset: bool,
    // negamax
    stopped: bool,
    depth: u8
}


impl Default for Control {
    fn default() -> Self {
        Self { movestogo: 30, movetime: 0, time: 0, inc: 0, starttime: Instant::now(), stoptime: 0, timeset: false, stopped: false, depth: 0 }
    }
}


impl Control {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn set_time(&mut self, time: u128) {
        self.time = time;
    }

    pub(crate) fn set_inc(&mut self, inc: u32) {
        self.inc = inc;
    }

    pub(crate) fn set_movestogo(&mut self, movestogo: u32) {
        self.movestogo = movestogo;
    }

    pub(crate) fn set_movetime(&mut self, movetime: u128) {
        self.movetime = movetime;
    }

    pub(crate) fn set_depth(&mut self, depth: u8) {
        self.depth = depth;
    }

    pub(crate) fn setup_timerange(&mut self) {
        let start = Instant::now();
        let start_ms = start.duration_since(start).as_millis();

        self.starttime = start;
        if self.time > 0 {
            let mut time = self.time / self.movestogo as u128;
            if time > 1500 { time -= 50 }
            self.time = time;
            self.stoptime = start_ms + time + (self.inc as u128);
            self.timeset = true;
            // if time < 1500  && self.inc > 0 && depth == 64 { self.starttime += self.inc - 50; }
        }
        
    }

    pub(crate) fn stop(&mut self) {
        self.stopped = true;
    }

    pub(crate) fn time(&self) -> u128 {self.time}
    pub(crate) fn inc(&self) -> u32 {self.inc}
    pub(crate) fn movetime(&self) -> u128 {self.time}
    pub(crate) fn movestogo(&self) -> u32 {self.movestogo}

    pub(crate) fn depth(&self) -> u8 {self.depth}


}


impl TimeControl for Control {
    fn communicate(&mut self) {
        let elapsed_since_start = self.starttime.elapsed().as_millis();

        if self.timeset && elapsed_since_start > self.stoptime {
            self.stopped = true;
        }
    }

    fn stopped(&self) -> bool {
        self.stopped
    }
}
