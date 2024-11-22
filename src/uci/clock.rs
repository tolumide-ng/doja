pub(crate) enum Clock {
    /// Only search up to depth x
    Depth(u64),
    /// Search for a move in x msec
    Time(u64),
    /// Search x nodes
    Nodes(u64),
    Infinite,
    /// Search for a mate in x plys
    Mate(u64),
    Dynamic {
        wtime: u64,
        btime: u64,
        winc: Option<u64>,
        binc: Option<u64>,
        movestogo: Option<u64>
    }
}