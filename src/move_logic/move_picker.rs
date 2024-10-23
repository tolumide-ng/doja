#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum Stage {
    PV=0,
    TTMove,
    GoodCapture,
    KillerZero,
    KillerOne,
    BadCapture,
}