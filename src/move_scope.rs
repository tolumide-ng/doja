#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MoveScope {
    QuietOnly = 0,
    CapturesOnly,
    AllMoves,
}

impl From<MoveScope> for u8 {
    fn from(value: MoveScope) -> Self {
        match value {
            MoveScope::AllMoves => 0,
            MoveScope::CapturesOnly => 1,
            MoveScope::QuietOnly => 2,
        }
    }
}