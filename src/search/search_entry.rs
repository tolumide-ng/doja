use crate::{bit_move::Move, board::piece::Piece};

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct SearchE {
    pub(crate) ply: usize,
    /// The piece that was moved
    pub(crate) piece: Option<Piece>,
    /// What move was made
    pub(crate) mv: Option<Move>,
    pub(crate) score: i32,
    pub(crate) skipped: Option<Move>,
    pub(crate) eval: i32,
}