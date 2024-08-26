// p_idx = piece_type * 2 + piece_color
// halfkp_idx = piece_square + (p_idx + king_square * 10) * 64

/// this is basically a copycat of the CARP library, I intend to build/learn from here
pub(crate) mod align64;
pub(crate) mod accumulator;
pub(crate) mod commons;
pub(crate) mod net;