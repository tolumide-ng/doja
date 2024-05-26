use crate::{board::{board_state::BoardState, piece::Piece}, constants::{CMK_BISHOP_SCORE, CMK_KING_SCORE, CMK_KNIGHT_SCORE, CMK_PAWN_SCORE, CMK_ROOK_SCORE, MIRROR_SCORE}, squares::Square};

pub(crate) struct Evaluation;

impl Evaluation {
    pub(crate) fn evaluate(board: &BoardState) -> i16 {
        let mut score = 0;

        for piece in Piece::ascii_pieces() {
            let mut bitboard = board[piece];
            
            while bitboard.not_zero() {
                let square = Square::from(bitboard.get_lsb1().unwrap());
                score += piece.material_score();
                
                let mirror_index = MIRROR_SCORE[square];
                println!("{square} {} ====>>>>>>> {mirror_index}", square as u8);
                
                match piece {
                    Piece::WP => score += CMK_PAWN_SCORE[square],
                    Piece::WN => score += CMK_KNIGHT_SCORE[square],
                    Piece::WB => score += CMK_BISHOP_SCORE[square],
                    Piece::WR => score += CMK_ROOK_SCORE[square],
                    Piece::WK => score += CMK_KING_SCORE[square],
                    

                    Piece::BP => score -= CMK_PAWN_SCORE[mirror_index],
                    Piece::BN => score -= CMK_KNIGHT_SCORE[mirror_index],
                    Piece::BB => score -= CMK_BISHOP_SCORE[mirror_index],
                    Piece::BR => score -= CMK_ROOK_SCORE[mirror_index],
                    Piece::BK => score -= CMK_KING_SCORE[mirror_index],

                    _ => {}
                }
                
                bitboard.pop_bit(square.into());
            }

        }

        return score;
    }
}