// p_idx = piece_type * 2 + piece_color
// halfkp_idx = piece_square + (p_idx + king_square * 10) * 64

pub(crate) mod align64;
pub(crate) mod accumulator;
pub(crate) mod commons;