use crate::{bitboard::Bitboard, board::{board_state::BoardState, piece::Piece}, color::Color, constants::{CMK_BISHOP_SCORE, CMK_KING_SCORE, CMK_KNIGHT_SCORE, CMK_PAWN_SCORE, CMK_ROOK_SCORE, DOUBLE_PAWN_PENALTY, EVAL_MASKS, ISOLATED_PAWN_PENALTY, MIRROR_SCORE, PASSED_PAWN_BONUS}, masks::EvaluationMasks, squares::Square};

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
                match piece {
                    // we're using this order because our board is inverted i.e. starts from the left --> to right, and then bottom --> up
                    Piece::WP => {
                        score += CMK_PAWN_SCORE[mirror_index];
                        let double_pawns = (*board[piece] & EVAL_MASKS.file_masks[mirror_index]).count_ones() as i16;
                        // double pawn penalty
                        if double_pawns > 1 {
                            score += double_pawns * DOUBLE_PAWN_PENALTY as i16;
                        }

                        // isolated pawn
                        if *board[piece] & EVAL_MASKS.isolated_masks[mirror_index] == 0 {
                            score += ISOLATED_PAWN_PENALTY as i16;
                        }

                        // passed pawn bonus
                        if EVAL_MASKS.white_passed_masks[square] & *board[piece] == 0 {
                            // give passed pawn bonus
                            score += PASSED_PAWN_BONUS[Square::from(mirror_index).rank()] as i16; 
                        }
                    },
                    Piece::WN => score += CMK_KNIGHT_SCORE[mirror_index],
                    Piece::WB => score += CMK_BISHOP_SCORE[mirror_index],
                    Piece::WR => score += CMK_ROOK_SCORE[mirror_index],
                    Piece::WK => score += CMK_KING_SCORE[mirror_index],
                    

                    Piece::BP => {
                        score -= CMK_PAWN_SCORE[square];

                        // println!("score before {}", score);
                        let double_pawns = (*board[Piece::BP] & EVAL_MASKS.file_masks[square]).count_ones() as i16;
                        if double_pawns > 1 {
                            score -= double_pawns * DOUBLE_PAWN_PENALTY as i16;
                        }
                        if *board[Piece::BP] & EVAL_MASKS.isolated_masks[square] == 0 {
                            score -= ISOLATED_PAWN_PENALTY as i16;
                        }
                        // passed pawn bonus 
                        if EVAL_MASKS.black_passed_masks[square] & *board[piece] == 0 {
                            score -= PASSED_PAWN_BONUS[Square::from(square).rank()] as i16;
                        }
                    },
                    Piece::BN => score -= CMK_KNIGHT_SCORE[square],
                    Piece::BB => score -= CMK_BISHOP_SCORE[square],
                    Piece::BR => score -= CMK_ROOK_SCORE[square],
                    Piece::BK => score -= CMK_KING_SCORE[square],

                    _ => {}
                }
                
                bitboard.pop_bit(square.into());
            }

        }

        // println!("::::scoring >>> {score}");

        match board.turn {
            Color::White => score,
            _ => -score
        }
    }
}