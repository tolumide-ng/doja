///  ----NOT_A-FILE----
/// 8   0  1  1  1  1  1  1  1 \ 
/// 7   0  1  1  1  1  1  1  1 \ 
/// 6   0  1  1  1  1  1  1  1 \ 
/// 5   0  1  1  1  1  1  1  1 \ 
/// 4   0  1  1  1  1  1  1  1 \ 
/// 3   0  1  1  1  1  1  1  1 \ 
/// 2   0  1  1  1  1  1  1  1 \ 
/// 1   0  1  1  1  1  1  1  1 \ 
///     a  b  c  d  e  f  g  h
pub const NOT_A_FILE: u64 = 18374403900871474942;


/// ----NOT_H_FILE----
/// 8   1  1  1  1  1  1  1  0 \
/// 7   1  1  1  1  1  1  1  0 \
/// 6   1  1  1  1  1  1  1  0 \
/// 5   1  1  1  1  1  1  1  0 \
/// 4   1  1  1  1  1  1  1  0 \
/// 3   1  1  1  1  1  1  1  0 \
/// 2   1  1  1  1  1  1  1  0 \
/// 1   1  1  1  1  1  1  1  0 \
///     a  b  c  d  e  f  g  h
pub const NOT_H_FILE: u64 = 9187201950435737471;


/// ----NOT_GH_FILE----
/// 8   1  1  1  1  1  1  0  0 \
/// 7   1  1  1  1  1  1  0  0 \
/// 6   1  1  1  1  1  1  0  0 \
/// 5   1  1  1  1  1  1  0  0 \
/// 4   1  1  1  1  1  1  0  0 \
/// 3   1  1  1  1  1  1  0  0 \
/// 2   1  1  1  1  1  1  0  0 \
/// 1   1  1  1  1  1  1  0  0 \
///     a  b  c  d  e  f  g  h
pub const NOT_GH_FILE: u64 = 4557430888798830399;



/// ----NOT_AB_FILE---- \
/// 8 &nbsp;  0  0  1  1  1  1  1  1 \
/// 7 &nbsp;  0  0  1  1  1  1  1  1 \
/// 6 &nbsp;  0  0  1  1  1  1  1  1 \
/// 5 &nbsp;  0  0  1  1  1  1  1  1 \
/// 4 &nbsp;  0  0  1  1  1  1  1  1 \
/// 3 &nbsp;  0  0  1  1  1  1  1  1 \
/// 2 &nbsp;  0  0  1  1  1  1  1  1 \
/// 1 &nbsp;  0  0  1  1  1  1  1  1 \
///   &nbsp;  a  b  c  d  e  f  g  h 
pub const NOT_AB_FILE: u64 = 18229723555195321596;



// FEN debug positions
pub const EMPTY_BOARD: &str = "8/8/8/8/8/8/8/8 w - - ";
pub const START_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 ";
pub const TRICKY_POSITION: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1 ";
pub const KILLER_POSITION: &str = "rnbqkb1r/pp1p1pPp/8/2p1pP2/1P1P4/3P3P/P1P1P3/RNBQKBNR w KQkq e6 0 1 ";
pub const CMK_POSITION: &str = "r2q1rk1/ppp2ppp/2n1bn2/2b1p3/3pP3/3P1NPP/PPP1NPB1/R1BQ1RK1 b - - 0 9 ";



pub const PLAYERS_COUNT: usize = 2;
pub const PLAYER_PIECES: usize = 6;
pub const OCCUPANCIES: usize = 3;