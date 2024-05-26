#[derive(Debug, Clone, Copy, derive_more::Display)]
pub(crate) enum Command {
    #[display(fmt = "position")]
    Position,
    #[display(fmt = "fen")]
    Fen,
    #[display(fmt = "startpos")]
    StartPos,
    #[display(fmt = "move")]
    Move
}