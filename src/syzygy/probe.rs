use std::ffi::CString;
use std::ptr;

use crate::squares::Square;
use crate::{bit_move::Move, board::position::Position};
use crate::board::piece::{Piece::*, PieceType};
use crate::color::Color::{self, *};

use super::bindings::{tb_probe_root, TB_PROMOTES_BISHOP, TB_PROMOTES_KNIGHT, TB_PROMOTES_QUEEN, TB_PROMOTES_ROOK, TB_RESULT_DTZ_MASK, TB_RESULT_FAILED, TB_RESULT_FROM_MASK, TB_RESULT_FROM_SHIFT, TB_RESULT_PROMOTES_MASK, TB_RESULT_PROMOTES_SHIFT, TB_RESULT_TO_MASK, TB_RESULT_TO_SHIFT, TB_RESULT_WDL_MASK, TB_RESULT_WDL_SHIFT};
use super::{bindings::{tb_init, tb_probe_wdl, TB_BLESSED_LOSS, TB_CURSED_WIN, TB_DRAW, TB_LOSS, TB_WIN}, SyZyGyBoard};

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
pub(crate) struct TableBase;

impl TableBase {
    pub(crate) fn init(path: &str) {
        unsafe {
            let p = CString::new(path).unwrap();
            let res = tb_init(p.as_ptr());
            assert!(res, "Could not initialize Syzygy tablebase at: {path}");
        }
    }

    /// tb_probe_wdl probes the Win-Draw-Loss (WDL) table for a given position.
    pub(crate) fn probe_wdl(board: Position) -> Option<WDL> {
        let Some(board) =  SyZyGyBoard::try_from(board).ok() else {return None};

        // enpassant square
        let enp_sq = if let Some(sq) = board.enpassant {sq.flipv() as u32} else {0};

        let wdl = 
        unsafe { tb_probe_wdl(
        board.get_occupancy(White).count_ones() as u64,
        board.get_occupancy(Black).count_ones() as u64,
        (board[WK].count_ones() + board[BK].count_ones()) as u64,
        (board[WQ].count_ones() + board[BQ].count_ones()) as u64,
        (board[WR].count_ones() + board[BR].count_ones()) as u64,
        (board[WB].count_ones() + board[BB].count_ones()) as u64,
        (board[WN].count_ones() + board[BN].count_ones()) as u64,
        (board[WP].count_ones() + board[BP].count_ones()) as u64,
        0, 0, enp_sq, board.turn == White)};

        WDL::try_from(wdl).ok()
    }

    /// Given a position, it probes the DTZ(Distance to zero) table, and returns the bestMove, and possible consequence
    pub(crate) fn proble_root(board: Position ) -> Option<TBResult> {
        let Some(board) = SyZyGyBoard::try_from(board).ok() else {return None};

        let enp_sq = if let Some(sq) = board.enpassant {sq.flipv() as u32} else {0};

        let value = unsafe {tb_probe_root(
            board.get_occupancy(White).count_ones() as u64, 
            board.get_occupancy(Black).count_ones() as u64,
            (board[WK].count_ones() + board[BK].count_ones()) as u64,
            (board[WQ].count_ones() + board[BQ].count_ones()) as u64,
            (board[WR].count_ones() + board[BR].count_ones()) as u64,
            (board[WB].count_ones() + board[BB].count_ones()) as u64,
            (board[WN].count_ones() + board[BN].count_ones()) as u64,
            (board[WP].count_ones() + board[BP].count_ones()) as u64,
            0, 0, enp_sq, board.turn == White, ptr::null_mut())
        };

        TBResult::try_from((board, value)).ok()
    }

    pub(crate) fn wdl_white(board: Position) -> Option<WDL> {
        let stm = board.turn == Color::White;
        let r = Self::proble_root(board)?;

        let rr = match r.wdl {
            WDL::Draw => WDL::Draw,
            WDL::Win => if stm {WDL::Win} else {WDL::Loss},
            WDL::Loss => if stm {WDL::Loss} else {WDL::Win}
        };

        Some(rr)
    }
}

pub(crate) struct TBResult {
    pub(crate) mv: Move, pub(crate) wdl: WDL, pub(crate) dtz: u32
}



impl TryFrom<(SyZyGyBoard, u32)> for TBResult {
    type Error = &'static str;

    fn try_from((board, result): (SyZyGyBoard, u32)) -> Result<Self, Self::Error> {
        if result == TB_RESULT_FAILED {
            return Err("Probe failed")
        }

        let pre_wdl = (result & TB_RESULT_WDL_MASK) >> TB_RESULT_WDL_SHIFT;
        let wdl = WDL::try_from(pre_wdl).unwrap_or(WDL::Draw);

        let from = Square::from(((result & TB_RESULT_FROM_MASK) >> TB_RESULT_FROM_SHIFT) as u64);
        let to = Square::from(((result & TB_RESULT_TO_MASK) >> TB_RESULT_TO_SHIFT) as u64);
        let promotion = (result & TB_RESULT_PROMOTES_MASK) >> TB_RESULT_PROMOTES_SHIFT;

        let promoted_to = match promotion {
            TB_PROMOTES_QUEEN => Some(PieceType::Q),
            TB_PROMOTES_ROOK => Some(PieceType::R),
            TB_PROMOTES_BISHOP => Some(PieceType::B),
            TB_PROMOTES_KNIGHT => Some(PieceType::N),
            _ => None
        };

        let moves = board.gen_movement();
        let dtz = (result & TB_RESULT_DTZ_MASK) >> TB_RESULT_DTZ_MASK;

        moves.into_iter().find(|m| m.get_src() == from && m.get_target() == to && 
            m.get_promotion() == promoted_to
        ).map(|mv| Self {mv, wdl, dtz}).ok_or("")
    }
}