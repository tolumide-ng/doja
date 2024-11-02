pub(crate) mod pv_table;
pub(crate) mod killer_moves;
pub(crate) mod history;
pub(crate) mod countermove;
// https://www.chessprogramming.org/Repetitions
pub(crate) mod repetitions;
pub(crate) mod capture_history;
pub(crate) mod continuation_history;


// from Carp
pub(crate) const HISTORY_MAX: i32 = 16384;
pub(crate) const HISTORY_FACTOR: i16 = 350;
pub(crate) const HISTORY_OFFSET: i16 = 350;

pub(super) fn calc_history_bonus(depth: usize) -> i16 {
    (HISTORY_MAX as i16).min(HISTORY_FACTOR * depth as i16 - HISTORY_OFFSET)
}


pub(crate) fn r_history(depth: u8) -> i32 {
    if depth > 17 { return 0 } else { 
        (depth * depth + 2 * depth -2) as i32
    }
}

/// From 'Stockfish'
const MAX_HISTORY: i32 = i32::MAX/2;
pub(super) const fn malus(depth: u8) -> i32 {
    if depth < 4 { return 736 * (depth+1) as i32} else {2044}
}
pub(crate) const fn taper_bonus(prev: i32, value: i32) -> i16 {
    (prev + value - (prev * value.abs()) / HISTORY_MAX) as i16
}
pub(crate) const fn history_bonus(depth: u8) -> i32 {
    let value = 190 * (depth as i16) - 298;
    if value < 20 { return 20; }
    if value > 1596 { return 1596 }
    return value as i32;
}