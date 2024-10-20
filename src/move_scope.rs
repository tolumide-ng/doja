pub enum MoveScope {
    AllMoves,
    CapturesOnly,
}

impl From<MoveScope> for bool {
    fn from(value: MoveScope) -> Self {
        match value {
            MoveScope::AllMoves => true,
            MoveScope::CapturesOnly => false,
        }
    }
}