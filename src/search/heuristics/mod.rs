pub(crate) mod pv_table;
pub(crate) mod killer_moves;
pub(crate) mod history;
pub(crate) mod countermove;
// https://www.chessprogramming.org/Repetitions
pub(crate) mod repetitions;
pub(crate) mod capture_history;
pub(crate) mod continuation_history;


// from Carp
pub(crate) const HISTORY_MAX_BONUS: i16 = 1600;
pub(crate) const HISTORY_FACTOR: i16 = 350;
pub(crate) const HISTORY_OFFSET: i16 = 350;

pub(super) fn calc_history_bonus(depth: usize) -> i16 {
    HISTORY_MAX_BONUS.min(HISTORY_FACTOR * depth as i16 - HISTORY_OFFSET)
}


/// From 'Stockfish'
const MAX_HISTORY: i32 = i32::MAX/2;
pub(super) const fn malus(depth: u8) -> i32 {
    if depth < 4 { return 736 * (depth+1) as i32} else {2044}
}
pub(crate) const fn taper_bonus(prev: i32, value: i32) -> i16 {
    (prev + value - (prev * value.abs()) / MAX_HISTORY) as i16
}
pub(crate) const fn history_bonus(depth: u8) -> i32 {
    let value = 190 * (depth as i16) - 298;
    if value < 20 { return 20; }
    if value > 1596 { return 1596 }
    return value as i32;
}