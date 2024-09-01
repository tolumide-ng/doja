#[cfg(test)]
mod board_state_tests {
    use super::*;
    use crate::{bitboard::Bitboard, board::{castling::Castling, state::board_state::BoardState}, color::Color, constants::{RANK_7, RANK_8}, squares::Square};

    #[test]
    fn should_create_a_new_board_state() {
        let board = BoardState::new();
        assert_eq!(board.turn, Color::White);
        assert_eq!(board.castling_rights, Castling::all());
        assert_eq!(board.enpassant, None);
        assert_eq!(board.hash_key, 12825486226133058263);
        assert_eq!(board.fifty, 0);
        assert_eq!(board.prev, None);
    }


    #[test]
    fn basic_set_and_get_methods() {
        let mut board = BoardState::new();

        board.set_turn(Color::Black);
        board.set_enpassant(Some(Square::B3));
        board.set_castling(Castling::from("K-kq"));
        board.set_occupancy(Color::White, 0x20000000);
        
        assert_eq!(board.get_occupancy(Color::Black), 0);
        assert_eq!(board.enpassant.unwrap(), Square::B3);
        assert_eq!(board.castling_rights, Castling::from("K-kq"));
        assert_eq!(board.turn, Color::Black);
    }

    #[cfg(test)]
    mod is_square_under_attack {
        use crate::board::piece::Piece;

        use super::*;

        #[test]
        fn should_test_if_the_square_is_under_attack_by_pawns() {
            let mut board = BoardState::new();

            let bp = 0x40004000220u64;
            board.board[Piece::BP] = Bitboard::from(bp); // attacker
            board.board[Piece::WP] = Bitboard::from(0x8040800u64); // victims
            board.set_occupancy(Color::Black, bp);
            
            let result = board.is_square_attacked(Square::D3 as u64, Color::Black);
            assert!(result);
            
            assert!(!board.is_square_attacked(Square::C3 as u64, Color::Black));
        }

        #[test]
        fn should_test_if_the_square_is_under_attack_by_knights() {
            let mut board = BoardState::new();
            let white_knights = 0x40000000200u64;
            let black_pawns = 0x8001000u64;
            board.board[Piece::WN] = Bitboard::from(white_knights);
            board.board[Piece::BP] = Bitboard::from(black_pawns);
            board.set_occupancy(Color::White, white_knights);
            board.set_occupancy(Color::Black, black_pawns);

            assert!(board.is_square_attacked(Square::D4 as u64, Color::White));
            assert!(!board.is_square_attacked(Square::E2 as u64, Color::White));
        }

        #[test]
        fn should_test_if_the_square_is_under_attack_by_a_king() {
            let mut board = BoardState::new();

            let bk = 0x1000u64;
            board.board[Piece::BK] = Bitboard::from(bk);
            board.set_occupancy(Color::Black, bk);

            assert!(!board.is_square_attacked(Square::G4 as u64, Color::Black));
            assert!(board.is_square_attacked(Square::F1 as u64, Color::Black));
        }

        #[test]
        fn should_test_if_the_square_is_under_attack_by_a_bishop_or_queen() {
            let mut board = BoardState::new();

            let bb = 0x1000u64;
            board.board[Piece::BB] = Bitboard::from(bb);
            board.set_occupancy(Color::Black, bb);

            assert!(board.is_square_attacked(Square::G4 as u64, Color::Black));
            assert!(board.is_square_attacked(Square::F1 as u64, Color::Black));
            assert!(!board.is_square_attacked(Square::H8 as u64, Color::Black));
            assert!(!board.is_square_attacked(Square::E1 as u64, Color::Black));

            let mut board = BoardState::new();

            let bq = 0x1002u64;
            board.board[Piece::BQ] = Bitboard::from(bq);
            board.set_occupancy(Color::Black, bq);

            assert!(board.is_square_attacked(Square::G4 as u64, Color::Black));
            assert!(board.is_square_attacked(Square::F1 as u64, Color::Black));
            assert!(!board.is_square_attacked(Square::H8 as u64, Color::Black));
            assert!(board.is_square_attacked(Square::E1 as u64, Color::Black));
            assert!(!board.is_square_attacked(Square::A3 as u64, Color::Black));
        }

        #[test]
        fn should_test_if_the_square_is_under_attack_by_a_rook_or_queen() {
            let mut board = BoardState::new();

            let br = 0x400001000u64;
            board.board[Piece::BR] = Bitboard::from(br);
            board.set_occupancy(Color::Black, br);

            assert!(board.is_square_attacked(Square::E5 as u64, Color::Black));
            assert!(!board.is_square_attacked(Square::D3 as u64, Color::Black));
            assert!(!board.is_square_attacked(Square::G7 as u64, Color::Black));
            assert!(board.is_square_attacked(Square::C1 as u64, Color::Black));
            assert!(board.is_square_attacked(Square::A2 as u64, Color::Black));
            assert!(board.is_square_attacked(Square::G5 as u64, Color::Black));


            let mut board = BoardState::new();

            let bq = 0x400001000u64;
            board.board[Piece::BQ] = Bitboard::from(bq);
            board.set_occupancy(Color::Black, bq);

            assert!(board.is_square_attacked(Square::E5 as u64, Color::Black));
            assert!(board.is_square_attacked(Square::D3 as u64, Color::Black));
            assert!(!board.is_square_attacked(Square::G7 as u64, Color::Black));
            assert!(!board.is_square_attacked(Square::A8 as u64, Color::Black));
            assert!(!board.is_square_attacked(Square::G3 as u64, Color::Black));
            assert!(board.is_square_attacked(Square::C1 as u64, Color::Black));
            assert!(board.is_square_attacked(Square::A2 as u64, Color::Black));
            assert!(board.is_square_attacked(Square::G5 as u64, Color::Black));
        }
    }

}