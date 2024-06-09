use crate::{bitboard::Bitboard, board::{board_state::BoardState, occupancies, piece::{self, Piece}}, color::Color, constants::{BISHOP_MOBILITY_ENDGAME, BISHOP_MOBILITY_OPENING, BISHOP_UNIT, CMK_BISHOP_SCORE, CMK_KING_SCORE, CMK_KNIGHT_SCORE, CMK_PAWN_SCORE, CMK_ROOK_SCORE, DOUBLE_PAWN_PENALTY_ENDGAME, DOUBLE_PAWN_PENALTY_OPENING, EVAL_MASKS, ISOLATED_PAWN_PENALTY_ENDGAME, ISOLATED_PAWN_PENALTY_OPENING, KING_SHIELD_BONUS, MATERIAL_SCORE, MIRROR_SCORE, OPENING_PHASE_SCORE, OPEN_FILE_SCORE, PASSED_PAWN_BONUS, PIECE_ATTACKS, PLAYER_PIECES, POSITIONAL_SCORES, QUEEN_MOBILITY_ENDGAME, QUEEN_MOBILITY_OPENING, QUEEN_UNIT, SEMI_OPEN_FILE_SCORE}, game_phase::GamePhase, kogge_stone::KoggeStone, masks::EvaluationMasks, squares::Square};

pub(crate) struct Evaluation;

impl Evaluation {
    pub(crate) fn evaluate(board: &BoardState) -> i32 {
        let (mut score_opening, mut score_endgame): (i32, i32) = (0, 0);

        let game_phase_score = Self::get_game_phase_score(board);
        let game_phase = GamePhase::from(game_phase_score);

        for piece in Piece::ascii_pieces() {
            let mut bitboard = board[piece];

            
            while bitboard.not_zero() {
                let square = Square::from(bitboard.get_lsb1().unwrap());
                let mirror_index = MIRROR_SCORE[square];

                // interpolate scores in middle game
                // (score_opening * game_phase_score + score_endgame * (opening_phase_score - game_phase_score))/opening_phase_score
                // e.g. the score for pawn on d4 at phase say 5000 would be:
                // interpolated_score = (12 * 5000 + (-7) * (6192 - 5000)/6192 = 8,342377261
                let piece_index = (piece as usize) % PLAYER_PIECES;

                score_opening += MATERIAL_SCORE[GamePhase::Opening][piece];
                score_endgame += MATERIAL_SCORE[GamePhase::EndGame][piece];


                match piece {
                    Piece::WP | Piece::WN | Piece::WR | Piece::WB | Piece::WQ | Piece::WK => {
                        score_opening += POSITIONAL_SCORES[GamePhase::Opening][piece_index][mirror_index];
                        score_endgame  += POSITIONAL_SCORES[GamePhase::EndGame][piece_index][mirror_index];
                    }
                    Piece::BP | Piece::BN | Piece::BR | Piece::BB | Piece::BQ | Piece::BK  => {
                        score_opening -= POSITIONAL_SCORES[GamePhase::Opening][piece_index][square];
                        score_endgame  -= POSITIONAL_SCORES[GamePhase::EndGame][piece_index][square];
                    }
                }

                match piece {
                    // we're using this order because our board is inverted i.e. starts from the left --> to right, and then bottom --> up
                    Piece::WP => {
                        let double_pawns = (*board[piece] & EVAL_MASKS.file_masks[mirror_index]).count_ones() as i32;
                        // double pawn penalty
                        if double_pawns > 1 {
                            score_opening += (double_pawns - 1) * DOUBLE_PAWN_PENALTY_OPENING;
                            score_endgame += (double_pawns - 1) * DOUBLE_PAWN_PENALTY_ENDGAME;
                        }

                        // isolated pawn
                        if *board[piece] & EVAL_MASKS.isolated_masks[mirror_index] == 0 {
                            score_opening += ISOLATED_PAWN_PENALTY_OPENING;
                            score_endgame += ISOLATED_PAWN_PENALTY_ENDGAME;
                        }

                        // passed pawn bonus
                        // if EVAL_MASKS.white_passed_masks[square] & *board[piece] == 0 {
                        if EVAL_MASKS.white_passed_masks[square] & *board[Piece::BP] == 0 {
                            // give passed pawn bonus
                            score_opening += PASSED_PAWN_BONUS[Square::from(mirror_index).rank()] as i32;
                            score_endgame += PASSED_PAWN_BONUS[Square::from(mirror_index).rank()] as i32;
                        }
                    },
                    Piece::WN => {},
                    Piece::WB => {
                        let bishop = PIECE_ATTACKS.get_bishop_attacks(square.into(), board.get_occupancy(Color::Both)).count_ones() as i32 - BISHOP_UNIT;
                        // mobility
                        score_opening += bishop * BISHOP_MOBILITY_OPENING;
                        score_endgame += bishop * BISHOP_MOBILITY_ENDGAME;
                    },
                    Piece::WR => {
                        // semi open file bonus
                        if *board[Piece::WP] & EVAL_MASKS.file_masks[mirror_index] == 0 {
                            score_opening += SEMI_OPEN_FILE_SCORE;
                            score_endgame += SEMI_OPEN_FILE_SCORE;
                        }
                        
                        // open file bonus
                        if (*board[Piece::WP] | *board[Piece::BP]) & EVAL_MASKS.file_masks[mirror_index] == 0 {
                            score_opening += OPEN_FILE_SCORE;
                            score_endgame += OPEN_FILE_SCORE;
                        }
                    },
                    Piece::WQ => {
                        let queen = PIECE_ATTACKS.get_queen_attacks(square as u64, board.get_occupancy(Color::Both)).count_ones() as i32 - QUEEN_UNIT;
                        score_opening += queen * QUEEN_MOBILITY_OPENING;
                        score_endgame += queen * QUEEN_MOBILITY_ENDGAME;
                    }
                    Piece::WK => {
                        // semi open file penalty (discourage semi-open files on the king)
                        if *board[Piece::WP] & EVAL_MASKS.file_masks[mirror_index] == 0 {
                            score_opening -= SEMI_OPEN_FILE_SCORE;
                            score_endgame -= SEMI_OPEN_FILE_SCORE;
                        }
                        
                        // open file penalty (discourage open files on the king)
                        if (*board[Piece::WP] | *board[Piece::BP]) & EVAL_MASKS.file_masks[mirror_index] == 0 {
                            score_opening -= OPEN_FILE_SCORE;
                            score_endgame -= OPEN_FILE_SCORE;
                        }

                        let king_shields = ((PIECE_ATTACKS.king_attacks[square] & board.get_occupancy(Color::White)).count_ones() as i32 * KING_SHIELD_BONUS);
                        score_opening += king_shields;
                        score_endgame += king_shields;
                    },

                    Piece::BP => {
                        // let pawn_piece = Piece::WP;
                        // println!("score before {}", score);
                        let double_pawns = (*board[Piece::BP] & EVAL_MASKS.file_masks[square]).count_ones() as i32;
                        if double_pawns > 1 {
                            score_opening -= double_pawns * DOUBLE_PAWN_PENALTY_OPENING;
                            score_endgame -= double_pawns * DOUBLE_PAWN_PENALTY_ENDGAME;
                        }
                        // isolated pawn
                        if *board[Piece::BP] & EVAL_MASKS.isolated_masks[square] == 0 {
                            score_opening -= ISOLATED_PAWN_PENALTY_OPENING;
                            score_endgame -= ISOLATED_PAWN_PENALTY_OPENING;
                        }
                        // passed pawn bonus 
                        if EVAL_MASKS.black_passed_masks[square] & *board[Piece::WP] == 0 {
                            score_opening -= PASSED_PAWN_BONUS[Square::from(square).rank()] as i32;
                            score_endgame -= PASSED_PAWN_BONUS[Square::from(square).rank()] as i32;
                        }
                    },
                    Piece::BN => {},
                    Piece::BB => {
                        let bishop_mobility_units = PIECE_ATTACKS.get_bishop_attacks(square as u64, board.get_occupancy(Color::Both)).count_ones() as i32 - BISHOP_UNIT;
                        score_opening -= bishop_mobility_units * BISHOP_MOBILITY_OPENING;
                        score_endgame -= bishop_mobility_units * BISHOP_MOBILITY_OPENING;
                    },
                    Piece::BR => {
                        // semi open file bonus
                        if *board[Piece::BP] & EVAL_MASKS.file_masks[mirror_index] == 0 {
                            score_opening -= SEMI_OPEN_FILE_SCORE;
                            score_endgame -= SEMI_OPEN_FILE_SCORE;
                        }
                        
                        // open file bonus
                        if (*board[Piece::WP] | *board[Piece::BP]) & EVAL_MASKS.file_masks[mirror_index] == 0 {
                            score_opening -= OPEN_FILE_SCORE;
                            score_endgame -= OPEN_FILE_SCORE;
                        }
                    },
                    Piece::BQ => {
                        let queen_mobility = PIECE_ATTACKS.get_queen_attacks(square as u64, board.get_occupancy(Color::Both)).count_ones() as i32 - QUEEN_UNIT;
                        score_opening -= queen_mobility *  QUEEN_MOBILITY_OPENING;
                        score_endgame -= queen_mobility * QUEEN_MOBILITY_ENDGAME;
                    }
                    Piece::BK => {
                         // semi open file penalty (discourage semi-open files on the king)
                        if *board[Piece::WP] & EVAL_MASKS.file_masks[mirror_index] == 0 {
                            score_opening += SEMI_OPEN_FILE_SCORE;
                            score_endgame += SEMI_OPEN_FILE_SCORE;
                        }
                        
                        // open file penalty (discourage open files on the king)
                        if (*board[Piece::WP] | *board[Piece::BP]) & EVAL_MASKS.file_masks[mirror_index] == 0 {
                            score_opening += OPEN_FILE_SCORE;
                            score_endgame += OPEN_FILE_SCORE;
                        }

                        score_opening -= (PIECE_ATTACKS.king_attacks[square] & board.get_occupancy(Color::Black)).count_ones() as i32 * KING_SHIELD_BONUS;
                        score_endgame -= (PIECE_ATTACKS.king_attacks[square] & board.get_occupancy(Color::Black)).count_ones() as i32 * KING_SHIELD_BONUS;
                    },

                    _ => {}
                }
                
                bitboard.pop_bit(square.into());
            }

        }

        println!("game phase>>>>>> {:?}", game_phase);
        println!("::::opening >>> {score_opening} :::::endgame >>> {score_endgame}");

        let score = match game_phase {
            GamePhase::MiddleGame => {
                (score_opening * game_phase_score + 
                    score_endgame * (OPENING_PHASE_SCORE - game_phase_score)
                ) / OPENING_PHASE_SCORE
            }
            GamePhase::Opening => score_opening,
            GamePhase::EndGame => score_endgame
        };

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
    pub(crate) fn get_game_phase_score(board: &BoardState) -> i32 {
        let white_rooks =   (*board[Piece::WR]).count_ones() as i32 * MATERIAL_SCORE[GamePhase::Opening][Piece::WR];
        let white_bishops = (*board[Piece::WB]).count_ones() as i32 * MATERIAL_SCORE[GamePhase::Opening][Piece::WB];
        let white_knights = (*board[Piece::WN]).count_ones() as i32 * MATERIAL_SCORE[GamePhase::Opening][Piece::WN];
        let white_queens =  (*board[Piece::WQ]).count_ones() as i32 * MATERIAL_SCORE[GamePhase::Opening][Piece::WQ];


        let black_rooks =   (*board[Piece::BR]).count_ones() as i32 * -MATERIAL_SCORE[GamePhase::Opening][Piece::BR];
        let black_bishops = (*board[Piece::BB]).count_ones() as i32 * -MATERIAL_SCORE[GamePhase::Opening][Piece::BB];
        let black_knights = (*board[Piece::BN]).count_ones() as i32 * -MATERIAL_SCORE[GamePhase::Opening][Piece::BN];
        let black_queens =  (*board[Piece::BQ]).count_ones() as i32 * -MATERIAL_SCORE[GamePhase::Opening][Piece::BQ];


        let white = white_rooks + white_bishops + white_knights + white_queens;
        let black = black_rooks + black_bishops + black_knights + black_queens;

 
        return white + black;
    }
}