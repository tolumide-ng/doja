use crate::{board::{board_state::BoardState, piece::Piece}, squares::Square};

pub(crate) struct Evaluation;

impl Evaluation {
    pub(crate) fn evaluate(board: &BoardState) -> i16 {
        let mut score = 0;

        for piece in Piece::ascii_pieces() {
            let mut bitboard = board[piece];
            
            while bitboard.not_zero() {
                let square = Square::from(bitboard.get_lsb1().unwrap());
                score += piece.material_score();
                bitboard.pop_bit(square.into());
            }
        }

        return score;
    }
}