use crate::{bitboard::Bitboard, board::{board_state::BoardState, occupancies, piece::{self, Piece}}, color::Color, constants::{CMK_BISHOP_SCORE, CMK_KING_SCORE, CMK_KNIGHT_SCORE, CMK_PAWN_SCORE, CMK_ROOK_SCORE, DOUBLE_PAWN_PENALTY, EVAL_MASKS, ISOLATED_PAWN_PENALTY, KING_SHIELD_BONUS, MATERIAL_SCORE, MIRROR_SCORE, OPEN_FILE_SCORE, PASSED_PAWN_BONUS, PIECE_ATTACKS, SEMI_OPEN_FILE_SCORE}, kogge_stone::KoggeStone, masks::EvaluationMasks, pesto::GamePhase, squares::Square};

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
                // match piece {
                //     // we're using this order because our board is inverted i.e. starts from the left --> to right, and then bottom --> up
                //     Piece::WP => {
                //         score += CMK_PAWN_SCORE[mirror_index];
                //         let double_pawns = (*board[piece] & EVAL_MASKS.file_masks[mirror_index]).count_ones() as i16;
                //         // double pawn penalty
                //         if double_pawns > 1 {
                //             score += double_pawns * DOUBLE_PAWN_PENALTY as i16;
                //         }

                //         // isolated pawn
                //         if *board[piece] & EVAL_MASKS.isolated_masks[mirror_index] == 0 {
                //             score += ISOLATED_PAWN_PENALTY as i16;
                //         }

                //         // passed pawn bonus
                //         if EVAL_MASKS.white_passed_masks[square] & *board[piece] == 0 {
                //             // give passed pawn bonus
                //             score += PASSED_PAWN_BONUS[Square::from(mirror_index).rank()] as i16; 
                //         }
                //     },
                //     Piece::WN => score += CMK_KNIGHT_SCORE[mirror_index],
                //     Piece::WB => {
                //         score += CMK_BISHOP_SCORE[mirror_index];
                //         score += PIECE_ATTACKS.get_bishop_attacks(square as u64, board.get_occupancy(Color::Both)).count_ones() as i16;
                //     },
                //     Piece::WR => {
                //         // positional score
                //         score += CMK_ROOK_SCORE[mirror_index];

                //         // semi open file bonus
                //         if *board[Piece::WP] & EVAL_MASKS.file_masks[mirror_index] == 0 {
                //             score += SEMI_OPEN_FILE_SCORE
                //         }
                        
                //         // open file bonus
                //         if (*board[Piece::WP] | *board[Piece::BP]) & EVAL_MASKS.file_masks[mirror_index] == 0 {
                //             score += OPEN_FILE_SCORE;
                //         }
                //     },
                //     Piece::WQ => {
                //         score += PIECE_ATTACKS.get_queen_attacks(square as u64, board.get_occupancy(Color::Both)).count_ones() as i16;
                //     }
                //     Piece::WK => {
                //         score += CMK_KING_SCORE[mirror_index];
                //         // semi open file penalty (discourage semi-open files on the king)
                //         if *board[Piece::WP] & EVAL_MASKS.file_masks[mirror_index] == 0 {
                //             score -= SEMI_OPEN_FILE_SCORE
                //         }
                        
                //         // open file penalty (discourage open files on the king)
                //         if (*board[Piece::WP] | *board[Piece::BP]) & EVAL_MASKS.file_masks[mirror_index] == 0 {
                //             score -= OPEN_FILE_SCORE;
                //         }

                //         score += (PIECE_ATTACKS.king_attacks[square] & board.get_occupancy(Color::White)).count_ones() as i16 * KING_SHIELD_BONUS;
                //     },

                //     Piece::BP => {
                //         score -= CMK_PAWN_SCORE[square];

                //         // println!("score before {}", score);
                //         let double_pawns = (*board[Piece::BP] & EVAL_MASKS.file_masks[square]).count_ones() as i16;
                //         if double_pawns > 1 {
                //             score -= double_pawns * DOUBLE_PAWN_PENALTY as i16;
                //         }
                //         // isolated pawn
                //         if *board[Piece::BP] & EVAL_MASKS.isolated_masks[square] == 0 {
                //             score -= ISOLATED_PAWN_PENALTY as i16;
                //         }
                //         // passed pawn bonus 
                //         if EVAL_MASKS.black_passed_masks[square] & *board[piece] == 0 {
                //             score -= PASSED_PAWN_BONUS[Square::from(square).rank()] as i16;
                //         }
                //     },
                //     Piece::BN => score -= CMK_KNIGHT_SCORE[square],
                //     Piece::BB => {
                //         score -= CMK_BISHOP_SCORE[square];
                //         score -= PIECE_ATTACKS.get_bishop_attacks(square as u64, board.get_occupancy(Color::Both)).count_ones() as i16;
                //     },
                //     Piece::BR => {
                //         score -= CMK_ROOK_SCORE[square];
                //         // semi open file bonus
                //         if *board[Piece::BP] & EVAL_MASKS.file_masks[mirror_index] == 0 {
                //             score -= SEMI_OPEN_FILE_SCORE;
                //         }
                        
                //         // open file bonus
                //         if (*board[Piece::WP] | *board[Piece::BP]) & EVAL_MASKS.file_masks[mirror_index] == 0 {
                //             score -= OPEN_FILE_SCORE;
                //         }
                //     },
                //     Piece::BQ => {
                //         score -= PIECE_ATTACKS.get_queen_attacks(square as u64, board.get_occupancy(Color::Both)).count_ones() as i16;
                //     }
                //     Piece::BK => {
                //         score -= CMK_KING_SCORE[square];

                //          // semi open file penalty (discourage semi-open files on the king)
                //         if *board[Piece::WP] & EVAL_MASKS.file_masks[mirror_index] == 0 {
                //             score += SEMI_OPEN_FILE_SCORE
                //         }
                        
                //         // open file penalty (discourage open files on the king)
                //         if (*board[Piece::WP] | *board[Piece::BP]) & EVAL_MASKS.file_masks[mirror_index] == 0 {
                //             score += OPEN_FILE_SCORE;
                //         }

                //         score -= (PIECE_ATTACKS.king_attacks[square] & board.get_occupancy(Color::Black)).count_ones() as i16 * KING_SHIELD_BONUS;
                //     },

                //     _ => {}
                // }
                
                bitboard.pop_bit(square.into());
            }

        }

        // println!("::::scoring >>> {score}");

        match board.turn {
            Color::White => score,
            _ => -score
        }
    }

    /// The game phase score of the game is derived from the pieces (not counting pawns and kings)
    /// that are still on the board.
    /// The full material starting position game phase is:
    /// (4 * knight count * material score in the opening) + 
    /// (4 * bishop count *  material score in the opening) +
    /// (4 * rook count * material score in the opening) +
    /// (2 * queen count * material score in the opening)
    pub(crate) fn get_game_phase_score(board: &BoardState) -> i16 {
        let white_rooks =   (*board[Piece::WR]).count_ones() as i16 * MATERIAL_SCORE[GamePhase::Opening][Piece::WR];
        let white_bishops = (*board[Piece::WB]).count_ones() as i16 * MATERIAL_SCORE[GamePhase::Opening][Piece::WB];
        let white_knights = (*board[Piece::WN]).count_ones() as i16 * MATERIAL_SCORE[GamePhase::Opening][Piece::WN];
        let white_queens =  (*board[Piece::WQ]).count_ones() as i16 * MATERIAL_SCORE[GamePhase::Opening][Piece::WQ];


        let black_rooks =   (*board[Piece::BR]).count_ones() as i16 * -MATERIAL_SCORE[GamePhase::Opening][Piece::BR];
        let black_bishops = (*board[Piece::BB]).count_ones() as i16 * -MATERIAL_SCORE[GamePhase::Opening][Piece::BB];
        let black_knights = (*board[Piece::BN]).count_ones() as i16 * -MATERIAL_SCORE[GamePhase::Opening][Piece::BN];
        let black_queens =  (*board[Piece::BQ]).count_ones() as i16 * -MATERIAL_SCORE[GamePhase::Opening][Piece::BQ];


        let white = white_rooks + white_bishops + white_knights + white_queens;
        let black = black_rooks + black_bishops + black_knights + black_queens;

 
        return white + black;
    }
}