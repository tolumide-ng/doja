use std::ops::Deref;

use history::History;

use crate::move_logic::bitmove::MoveType;
use crate::constants::PIECE_ATTACKS;
use crate::nnue::accumulator::Feature;
use crate::{move_logic::bitmove::Move, move_scope::MoveScope, squares::Square};
use crate::nnue::network::NNUEState;
use crate::color::Color::{self, *};
use crate::nnue::constants::custom_kp::*;
use super::{piece::{Piece, Piece::*}, state::board::Board};

#[cfg(test)]
#[path ="./position.tests.rs"]
mod tests;
mod history;


/// If Feature is m256i, then the size = 32, and then that would be (1024/32) * 2 = 64 values
/// If Feature is i16, then the size = 2, and then that would be (1024/2) * 2 = 1024 values
pub(crate) const ACCUMULATOR_SIZE: usize = (L1_SIZE / (std::mem::size_of::<Feature>())) * 2;


#[derive(Debug, Clone)]
pub(crate) struct Position {
    pub(crate) board: Board,
    nnue_state: NNUEState<Feature, ACCUMULATOR_SIZE>,
    history: Vec<Option<History>>,
}


impl Position {
    pub(crate) fn new() -> Self {
        let board = Board::new();
        let nnue_state = NNUEState::from(&board);
        Self { history: Vec::new(), board, nnue_state }
    }

    /// NOT YET IMPLEMENTED, PLEASE IMPLEMENT ME!!!
    /// Read: https://www.chessprogramming.org/Material#InsufficientMaterial
    /// https://www.chessprogramming.org/Delta_Pruning
    /// https://www.chessprogramming.org/Zugzwang
    pub(crate) fn is_engame() {
        todo!()
    }

    pub(crate) fn history_at(&self, index: usize) -> Option<&History>  {
        self.history.get(index).unwrap().as_ref()
    }

    pub(crate) fn last_history(&self) -> Option<&History> {
        self.history.last().unwrap_or(&None).as_ref()
    }

    pub(crate) fn history_len(&self) -> usize {
        self.history.len()
    }

    /// For null moves only
    pub(crate) fn nnue_push(&mut self) {
        self.history.push(None); 
        self.nnue_state.push();
    }

    /// For null moves only
    pub(crate) fn nnue_pop(&mut self) {
        self.history.pop(); 
        self.nnue_state.pop(); }

    pub(crate) fn with(board: Board) -> Self {
        let nnue_state = NNUEState::from(&board);
        Self { board, nnue_state, history: Vec::new() }
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

    /// https://www.chessprogramming.net/static-exchange-evaluation-in-chess/
    pub(crate) fn see(&self, mv: &Move, threshold: i32) -> bool {
        let src = mv.get_src();
        let tgt = mv.get_target();
        let mt = mv.move_type();

        // Castling cannot have a bad SEE, since all squares the king passes through are not under attack
        if mt == MoveType::Castling { return true }

        // Ony captures are evaluated with SEE
        let piece_at_tgt = self.piece_at(tgt).unwrap();
        let piece_at_src = self.piece_at(src).unwrap();

        let mut move_value = if mv.get_capture() { piece_at_tgt.piece_value() } else { 0};

        // Piece being removed later on is the promoted piece
        let next_victim = if let Some(piece) = mv.get_promotion() {Piece::from((piece, self.turn))} else { self.piece_at(src).unwrap()};
        if mv.get_promotion().is_some() { move_value += next_victim.piece_value() - Piece::pawn(White).piece_value() }

        // Lose if the balance is already in our opponent's favour, and it's their turn
        let mut balance = move_value - threshold;
        if balance < 0 { return false }

        // Assuming we lose the piece that made this capture, if balance is still positive (in our favour), then we can return true immediately
        balance -= next_victim.piece_value();
        if balance >= 0 { return true }
        
        let mut see_board = self.board.clone();
        // Update the positions on the board: 1. Remove the moved piece, and place it at the target, 2. Remove the captured piece
        see_board.remove_piece(piece_at_src, src);
        see_board.remove_piece(piece_at_tgt, if mv.get_enpassant() {Board::enpass_tgt(tgt, see_board.turn).into()} else {tgt});
        // Add the moved piece to the new position
        see_board.add_piece(next_victim, tgt);
        
        let diaginal_sliders = *see_board[WB] | *see_board[BB] | *see_board[WQ] | *see_board[BQ];
        let orthogonal_sliders = *see_board[WR] | *see_board[BR] | *see_board[WQ] | *see_board[BQ];
        
        // Get all possible pieces(regardless of the color) that can attack the `tgt` square
        let mut attackers = see_board.get_all_attacks(tgt);

        let mut stm = !see_board.turn;
        let tgt_mask = 1u64 << u64::from(tgt);

        loop {
            // SEE terminates when no recapture is possible
            // Pieces of stm that can attack the target square
            let stm_attack_pieces = attackers & see_board.occupancies[stm];
            if stm_attack_pieces == 0 { break }

            // Get the least valuable attacker and simulate the recapture
            let (attacker, sq_of_the_attacker) = see_board.get_lva(stm_attack_pieces, stm).unwrap();
            see_board.remove_piece(attacker, sq_of_the_attacker);

            // Diagonal recaptures uncover bishops/queens
            if [Piece::pawn(stm), Piece::bishop(stm), Piece::queen(stm)].contains(&attacker) {
                attackers |= PIECE_ATTACKS.nnbishop_attacks(tgt_mask, see_board.occupancies[Both]) & diaginal_sliders;
            }
            
            // Orthognal recpatures uncover rooks/queens
            if [Piece::rook(stm), Piece::queen(stm)].contains(&attacker) {
                attackers |= PIECE_ATTACKS.nnrook_attacks(tgt_mask, see_board.occupancies[Both]) & orthogonal_sliders;
            }

            // Negamax the balance, cutoff if losing out attacker would still win the exchange
            stm = !stm;
            balance = -balance - 1 - attacker.piece_value();

            if balance >= 0 {
                // If the recapturing piece is a king, and the opponent has another attacker,
                // a positrive balance should not translate to an exchange win.
                if attacker == Piece::king(!stm) && ((attackers & *see_board[stm]) != 0) {
                    return see_board.turn == stm
                }
                break;
            }
        }
        // We win the exchange if we are not the one who should recapture
        see_board.turn != stm
    }


    pub(crate) fn make_move(&mut self, mv: Move, scope: MoveScope) -> bool {
        let (src, tgt) = (mv.get_src(), mv.get_target());
        let tgt_sq = Square::from(tgt);
        let turn = self.board.turn;
        let victim = self.board.get_piece_at(tgt, !turn);
        let mut rook_mvs = None;
        if mv.get_castling() { 
            rook_mvs = self.board.validate_castling_move(&mv); // rook movements
        };
        
        let Some(piece) = self.board.piece_at(src) else {
            return false
        };

        if let Some(new_board) = self.board.make_move(mv, scope) {
            let old_board = std::mem::replace(&mut self.board, new_board);
            self.history.push(Some(History::new(old_board, mv)));

            let mut remove = vec![]; let mut add = vec![];
            
            if mv.get_enpassant() {
                // let enpass_target = match board.turn {Color::Black => to as u64 + 8, _ => to as u64 -  8};
                let enpass_tgt = Square::from(match turn {White => tgt as u64 -  8, _ => tgt as u64 + 8});
                // self.nnue_state.manual_update::<OFF>(Piece::pawn(!turn), enpass_tgt);
                remove.push((Piece::pawn(!turn), enpass_tgt));
            } else if mv.get_capture() {
                // self.nnue_state.manual_update::<OFF>(victim.unwrap(), tgt_sq);
                remove.push((victim.unwrap(), tgt_sq));
            } else if mv.get_castling() {
                let (rook_src, rook_tgt) = rook_mvs.unwrap();
                let rook = Piece::rook(turn);
                // self.nnue_state.move_update(Piece::rook(turn), rook_src, rook_tgt);
                remove.push((rook, rook_src));
                add.push((rook, rook_tgt));
            }
            
            if let Some(promoted) =  mv.get_promotion() {
                remove.push((piece, src));
                add.push((Piece::from((promoted, turn)), tgt));
            } else {
                remove.push((piece, src));
                add.push((piece, tgt));
            }
            
            
            self.nnue_state.update(remove, add);
            
            return true;
        }
        
        false
    }

    // pub(crate) fn make_null_move(&mut self, beta: i32, depth: u8, pv)


    pub(crate) fn undo_move(&mut self, with_nnue: bool) {
        let last = self.history.pop();
        if let Some(History { board, mv }) = last.unwrap() { // because we know this function would never be called by a null move
            let _ = std::mem::replace(&mut self.board, board);
            if with_nnue {
                self.nnue_state.pop();
            }
        }
    }

    pub(crate) fn evaluate(&self) -> i32 {
        let eval = self.nnue_state.evaluate(self.board.turn);
  
        let total_material = 
            (self.board[WN].count_ones() + self.board[BN].count_ones()) as i32 * Piece::PIECE_VALUES[WN] +
            (self.board[WB].count_ones() + self.board[BB].count_ones()) as i32 * Piece::PIECE_VALUES[WB] + 
            (self.board[WR].count_ones() + self.board[BR].count_ones()) as i32 * Piece::PIECE_VALUES[WR] + 
            (self.board[WQ].count_ones() + self.board[BQ].count_ones()) as i32 * Piece::PIECE_VALUES[WQ];

        (eval * (700 + total_material / 32)) / 1024
    }
}

impl Deref for Position {
    type Target = Board;

    fn deref(&self) -> &Self::Target {
        &self.board
    }
}


impl From<Board> for Position {
    fn from(board: Board) -> Self {
        let nnue_state = NNUEState::from(&board);
        Self { board, nnue_state, history: Vec::new() }
    }
}