use std::ops::{Deref, DerefMut};

use crate::constants::params::PIECE_VALUES;
use crate::constants::{BLACK_KING_CASTLING_MASK, BLACK_QUEEN_CASTLING_MASK, WHITE_KING_CASTLING_MASK, WHITE_QUEEN_CASTLING_MASK};
use crate::{bit_move::BitMove, move_type::MoveType, nnue::state::NNUEState, squares::Square};
use crate::color::Color::{self, *};
use crate::nnue::state::{ON, OFF};
use crate::squares::Square::*;
use super::castling::Castling;
use super::{piece::{Piece, Piece::*}, state::board::Board};

#[cfg(test)]
#[path ="./position.tests.rs"]
mod tests;


#[derive(Debug, Clone, Copy)]
pub(crate) struct History {
    mv: BitMove, hash: u64, victim: Option<Piece>
}

impl History {
    pub(crate) fn new(mv: BitMove, hash: u64, victim: Option<Piece>) -> Self {
        Self { mv, hash, victim }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Position {
    board: Board,
    nnue_state: Box<NNUEState>,
    history: Vec<History>,
}


impl Position {
    pub(crate) fn new() -> Self {
        let board = Board::new();
        let nnue_state = NNUEState::from_board(&board);
        Self { history: Vec::new(), board, nnue_state }
    }

    pub(crate) fn with(board: Board) -> Self {
        let nnue_state = NNUEState::from_board(&board);
        Self { board, nnue_state, history: Vec::new() }
    }

    pub(crate) fn make_move(&mut self, mv: BitMove, mv_ty: MoveType) -> bool {
        if let Some(new_board) = self.board.make_move(mv, mv_ty) {
            let mut captured = None;
            let tgt = mv.get_target() as u64;

            if mv.get_enpassant() {
                let enpass_tgt = Square::from(match !self.board.turn {Black => tgt + 8, _ => tgt -  8});
                captured = self.board.get_piece_at(enpass_tgt, !self.board.turn);
            }

            if mv.get_capture() && !mv.get_enpassant() {
                captured = match mv.get_capture() {true => { self.board.get_piece_at(mv.get_target(), !mv.get_piece().color()) }, false => None};
            }

            let mv_history = History::new(mv, self.board.hash_key, captured);

            let _ = std::mem::replace(&mut self.board, new_board);
            self.history.push(mv_history);
            
            return true;
        }

        false
    }

    pub(crate) fn set_enpassant(&mut self, enpass: Option<Square>) {
        self.board.set_enpassant(enpass);
    }

    pub(crate) fn set_turn(&mut self, turn: Color) {
        self.board.set_turn(turn);
    }

    pub(crate) fn set_zobrist(&mut self, key: u64) {
        self.board.set_zobrist(key);
    }

    pub(crate) fn make_move_nnue(&mut self, mv: BitMove, mv_ty: MoveType) -> bool {
        let (src, tgt) = (mv.get_src(), mv.get_target());
        let tgt_sq = Square::from(tgt);
        let turn = self.board.turn;
        let victim = self.board.get_piece_at(tgt, !turn);

        if self.make_move(mv, mv_ty) {
            self.nnue_state.push();
            
            if mv.get_enpassant() {
                let enpass_tgt = Square::from(match !turn {Black => tgt as u64 + 8, _ => tgt as u64 -  8});
                self.nnue_state.manual_update::<OFF>(Piece::pawn(!turn), enpass_tgt);
            } else if mv.get_capture() {
                self.nnue_state.manual_update::<OFF>(victim.unwrap(), tgt_sq);
            } else if mv.get_castling() {
                let rook_mvs = self.board.validate_castling_move(&mv); // rook movements
                let (rook_src, rook_tgt) = rook_mvs.unwrap();
                self.nnue_state.move_update(Piece::rook(turn), rook_src, rook_tgt);
            }
    
            if let Some(promoted) =  mv.get_promotion() {
                self.nnue_state.manual_update::<OFF>(mv.get_piece(), src);
                self.nnue_state.manual_update::<ON>(promoted, tgt);
            } else {
                self.nnue_state.move_update(mv.get_piece(), src, tgt);
            }

            return true;
        }

        false
    }

    pub(crate) fn undo_move(&mut self, with_nnue: bool) {
        if self.history.len() == 0 { return }

        let History { mv, hash, victim } = self.history.pop().unwrap();
        let src = mv.get_src() as u64;
        let tgt = mv.get_target() as u64;
        let piece = mv.get_piece();
        let color = piece.color(); // the side that moved

        {
            // remove the acting(src) piece from wherever it moved to (target)
            let new_piece = match mv.get_promotion() {Some(p) => p, None => piece};
            *self.board[new_piece] ^= 1 << tgt;
            self.board.occupancies[color] ^= 1 << tgt;
            self.board.occupancies[Both] ^= 1 << tgt;
        }

        {
            // return the acting(src) piece to the point where it started (src);
            *self.board[piece] |= 1 << src;
            self.board.occupancies[color] |= 1 << src;
            self.board.occupancies[Both] |= 1 << src;
        }

        if mv.get_double_push() {
            self.board.enpassant = None;
        }


        if mv.get_enpassant() { // victim is a pawn of the opposite color
            let sq = match piece.color() {White => tgt - 8, _ => tgt + 8 }; // victim
            
            *self.board[Piece::pawn(!color)] |= 1 << sq;
            self.board.occupancies[!color] |= 1 << sq;
            self.board.occupancies[Both] |= 1 << sq;
            self.board.set_enpassant(Some(Square::from(tgt)));
        };

        if mv.get_castling() {
            let rook = Piece::rook(color);

            let (moved_to, moved_from)= match color {
                White => {
                    if tgt == Square::G1 as u64 {
                        self.board.set_castling(Castling::from(self.board.castling_rights.bits() | WHITE_KING_CASTLING_MASK));
                        ((1u64 << F1 as u64), (1u64 << H1 as u64))
                    } else { // C1 queen side 
                        self.board.set_castling(Castling::from(self.board.castling_rights.bits() | WHITE_QUEEN_CASTLING_MASK));
                        ((1u64 << D1 as u64),  (1u64 << A1 as u64))
                    }
                }
                _ => {
                    if tgt == Square::G8 as u64 {
                        self.board.set_castling(Castling::from(self.board.castling_rights.bits() | BLACK_KING_CASTLING_MASK));
                        ((1u64 << F8 as u64) as u64, 1u64 << H8 as u64)
                    } else { // if tgt == Square::C8 as u64 {}
                        self.board.set_castling(Castling::from(self.board.castling_rights.bits() | BLACK_QUEEN_CASTLING_MASK));
                        (1u64 << D8 as u64, 1u64 << A8 as u64)
                    }
                    
                }
            };
            
            // remove
            *self.board[rook] ^= moved_to;
            self.board.occupancies[color] ^= moved_to;
            self.board.occupancies[Both] ^= moved_to;
            
            // return to former sq
            *self.board[rook] |= moved_from;
            self.board.occupancies[color] |= moved_from;
            self.board.occupancies[Both] |= moved_from;
        }

        if !mv.get_enpassant() && mv.get_capture() {
            //  get the captured piece back
            if let Some(captured_piece) = victim {
                *self.board.board[captured_piece] |= 1 << tgt;
                self.board.occupancies[!color] |= 1 << tgt;
                self.board.occupancies[Both] |= 1 << tgt; 
            };
        }
        self.board.hash_key = hash;

        self.board.turn = color;

        if with_nnue {
            self.nnue_state.pop();
        }        
    }

    pub(crate) fn evaluate(&self) -> i32 {
        let eval = self.nnue_state.evaluate(self.board.turn);

        let total_material = (self.board.board[WN].count_bits() + self.board.board[BK].count_bits()) as i32 * PIECE_VALUES[WN] +
        (self.board.board[WB].count_bits() + self.board.board[BB].count_bits()) as i32 * PIECE_VALUES[WB] + 
        (self.board.board[WQ].count_bits() + self.board.board[BQ].count_bits()) as i32 * PIECE_VALUES[WQ] + 
        (self.board.board[WK].count_bits() + self.board.board[BK].count_bits()) as i32 * PIECE_VALUES[WK];
        
        (eval * (700 + total_material/32)) /1024
    }
}

impl Deref for Position {
    type Target = Board;

    fn deref(&self) -> &Self::Target {
        &self.board
    }
}


// impl DerefMut for Position {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.board
//     }
// }