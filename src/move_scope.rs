#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MoveScope {
    QuietOnly = 0,
    CapturesOnly,
    AllMoves,
}


impl MoveScope {
    pub(crate) const QUIETS: u8 = 0;
    pub(crate) const CAPTURES: u8 = 1;
    pub(crate) const ALL: u8 = 2;
}

impl From<MoveScope> for u8 {
    fn from(value: MoveScope) -> Self {
        match value {
            MoveScope::QuietOnly => 0,
            MoveScope::CapturesOnly => 1,
            MoveScope::AllMoves => 2,
        }
    }
}

impl From<u8> for MoveScope {
    fn from(value: u8) -> Self {
        match value {
            0 => MoveScope::QuietOnly,
            1 => MoveScope::CapturesOnly,
            2 => MoveScope::AllMoves,
            _ => unimplemented!("Unrecognized value of move-scope={value}")
        }
    }
}