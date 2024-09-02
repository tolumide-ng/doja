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


    #[cfg(test)]
    mod  single_push_targets {
        use crate::{bitboard::Bitboard, board::{piece::Piece, state::board_state::BoardState}, color::Color, squares::Square};

        #[test]
        fn single_push_target_for_white_pawn() {
            let mut board = BoardState::new();
            let wp = 0x200260000u64; // white pawns
            let enemy_pawns = 0x2002000u64; // black pawns
            board.board[Piece::WP] = Bitboard::from(wp);
            board.occupancies[Color::White] = wp;
            board.board[Piece::BP] = Bitboard::from(enemy_pawns);
            board.occupancies[Color::Black] = enemy_pawns;
            let result = board.single_push_targets(Color::White);

            let targets = [Square::B6, Square::B4, Square::C4, Square::F4];

            assert_eq!(result.count_ones() as usize,targets.len());
            for sq in targets {
                assert!((result & 1 << (sq as u64)) != 0);
            }
        }


        #[test]
        fn single_push_target_for_black_pawns() {
            let mut board = BoardState::new();
            let bp = 0x200260000u64; // black pawns
            let enemy_pawns = 0x2002000u64; // white pawns
            board.board[Piece::BP] = Bitboard::from(bp);
            board.occupancies[Color::Black] = bp;
            board.board[Piece::WP] = Bitboard::from(enemy_pawns);
            board.occupancies[Color::White] = enemy_pawns;
            let result = board.single_push_targets(Color::Black);

            let targets = [Square::B4, Square::B2, Square::C2, Square::F2];
            assert_eq!(result.count_ones() as usize, targets.len());
            for sq in targets {
                assert!(result & (1 << sq as u64) != 0);
            }
        }
    }

    #[cfg(test)]
    mod double_push_targets {
        use crate::{bitboard::Bitboard, board::{piece::Piece, state::board_state::BoardState}, color::Color, squares::Square};

        #[test]
        fn double_push_for_white_pawns() {
            let mut board = BoardState::new();
            let wp = 0x9a00u64;
            let enemy = 0x200002040000u64;
            board.board[Piece::WP] = Bitboard::from(wp);
            board.occupancies[Color::White] = wp;
            board.board[Piece::BP] = Bitboard::from(enemy);
            board.occupancies[Color::Black] = enemy;

            let result = board.double_push_targets(Color::White);
            
            let targets = [Square::B4, Square::D4, Square::E4, Square::H4];
            assert_eq!(targets.len(), result.count_ones() as usize);
            for sq in targets {
                assert!(result & (1 << sq as u64) != 0);
            }
        }

        #[test]
        fn double_push_for_black_pawns() {
            let mut board = BoardState::new();
            let bp = 0x35000000000000u64;
            let enemy =  0x500000042000u64;
            board.board[Piece::BP] = Bitboard::from(bp);
            board.occupancies[Color::Black] = bp;
            board.board[Piece::WP] = Bitboard::from(enemy);
            board.occupancies[Color::White] = enemy;

            let result = board.double_push_targets(Color::Black);
            let targets = [Square::A5, Square::C5, Square::E5, Square::F5];

            assert_eq!(result.count_ones() as usize, targets.len());
            for sq in targets {
                println!("on the square sq {:#?}", sq);
                assert!(result & (1<<sq as u64) != 0);
            }
        }
    }


    #[cfg(test)]
    mod pawns_able_to_double_push {
        use crate::board::piece::Piece;

        use super::*;

        #[test]
        fn white_pawns_eligible_to_double_push() {
            let mut board = BoardState::new();
            let wp = 0x500000042000u64;
            let enemy = 0x200002040000u64;
            board.board[Piece::WP] = Bitboard::from(wp);
            board.occupancies[Color::White] = wp;
            board.board[Piece::BP] = Bitboard::from(enemy);
            board.occupancies[Color::Black] = enemy;

            let result = board.pawns_able_to_double_push(Color::White);
            let targets = [Square::F2];
            assert_eq!(targets.len(), result.count_ones() as usize);
            for sq in targets {
                assert!(result & (1 << sq as u64) != 0);
            }
        }

        #[test]
        fn black_pawns_eligible_to_double_push() {
            let mut board = BoardState::new();
            let bp = 0x30000402000000u64;
            let enemy =  0x2000040000u64;
            board.board[Piece::BP] = Bitboard::from(bp);
            board.occupancies[Color::Black] = bp;
            board.board[Piece::WP] = Bitboard::from(enemy);
            board.occupancies[Color::White] = enemy;

            let result = board.pawns_able_to_double_push(Color::Black);
            let targets = [Square::E7, Square::F7];

            assert_eq!(result.count_ones() as usize, targets.len());
            for sq in targets {
                assert!(result & (1<<sq as u64) != 0);
            }
        }
    }

}