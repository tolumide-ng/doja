use std::ffi::CString;

use crate::board::position::Position;

use super::{bindings::{tb_init, TB_BLESSED_LOSS, TB_CURSED_WIN, TB_DRAW, TB_LOSS, TB_WIN}, SyBoard};

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum WDL {
    Win, Loss, Draw
}

impl TryFrom<u32> for WDL {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            TB_WIN => Ok(WDL::Win),
            TB_LOSS => Ok(WDL::Loss),
            TB_DRAW | TB_CURSED_WIN | TB_BLESSED_LOSS => Ok(WDL::Draw),
            _ => Err("Unrecognized value for win_draw_loss"),
        }
    }
}



/// SyZyGy Tablase
pub(crate) struct TableBase {
    /// distance to zero
    dst: usize,
}

impl TableBase {
    pub(crate) fn init(path: &str) {
        unsafe {
            let p = CString::new(path).unwrap();
            let res = tb_init(p.as_ptr());
            assert!(res, "Could not initialize Syzygy tablebase at: {path}");
        }
    }

    pub(crate) fn probe_wdl(board: Position) -> Option<WDL> {
        let Some(board) =  SyBoard::try_from(board).ok() else {return None};

        unsafe {
            // let wdl = tb_probe
        }

        None
    }
}