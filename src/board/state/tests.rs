#[cfg(test)]
mod board_state_tests {
    use super::*;
    use crate::board::piece::Piece;
    use crate::bit_move::BitMove;
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
            board.set_occupancy(Color::White, wp);
            board.board[Piece::BP] = Bitboard::from(enemy_pawns);
            board.set_occupancy(Color::Black, enemy_pawns);
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
            board.set_occupancy(Color::Black, bp);
            board.board[Piece::WP] = Bitboard::from(enemy_pawns);
            board.set_occupancy(Color::White, enemy_pawns);
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
            board.set_occupancy(Color::White, wp);
            board.board[Piece::BP] = Bitboard::from(enemy);
            board.set_occupancy(Color::Black, enemy);

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
            board.set_occupancy(Color::Black, bp);
            board.board[Piece::WP] = Bitboard::from(enemy);
            board.set_occupancy(Color::White, enemy);

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
        use super::*;

        #[test]
        fn white_pawns_eligible_to_double_push() {
            let mut board = BoardState::new();
            let wp = 0x500000042000u64;
            let enemy = 0x200002040000u64;
            board.board[Piece::WP] = Bitboard::from(wp);
            board.set_occupancy(Color::White, wp);
            board.board[Piece::BP] = Bitboard::from(enemy);
            board.set_occupancy(Color::Black, enemy);

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
            board.set_occupancy(Color::Black, bp);
            board.board[Piece::WP] = Bitboard::from(enemy);
            board.set_occupancy(Color::White, enemy);

            let result = board.pawns_able_to_double_push(Color::Black);
            let targets = [Square::E7, Square::F7];

            assert_eq!(result.count_ones() as usize, targets.len());
            for sq in targets {
                assert!(result & (1<<sq as u64) != 0);
            }
        }
    }

    #[cfg(test)]
    mod pawn_movement {
        use super::*;

        #[test]
        fn get_double_moves_for_white_pawns() {
            let mut board = BoardState::new();
            let wp = 0x500000042000u64;
            let enemy = 0x200002040000u64;
            board.board[Piece::WP] = Bitboard::from(wp);
            board.set_occupancy(Color::White, wp);
            board.board[Piece::BP] = Bitboard::from(enemy);
            board.set_occupancy(Color::Black, enemy);

            let result = board.get_pawn_movement(Color::White, true);
            let targets = [(Square::F2, Square::F4)];
            assert_eq!(result.len(), targets.len());

            for (src, target) in targets {
                let expected = BitMove::new(src as u32, target  as u32, Piece::WP, None, false, true, false, false);
                assert!(result.contains(&expected));
            }
        }

        #[test]
        fn get_single_pawn_moves_for_black_pawns() {
            let mut board = BoardState::new();
            let bp = 0x30000402000000u64;
            let enemy =  0x2000060000u64;
            board.board[Piece::BP] = Bitboard::from(bp);
            board.set_occupancy(Color::Black, bp);
            board.board[Piece::WP] = Bitboard::from(enemy);
            board.set_occupancy(Color::White, enemy);

            let result = board.get_pawn_movement(Color::Black, false);
            let targets = [(Square::E7, Square::E6), (Square::F7, Square::F6), (Square::B4, Square::B3), (Square::C5, Square::C4)];

        
            assert_eq!(result.len(), targets.len());
            for (src, target) in targets {
                let expected = BitMove::new(src as u32, target  as u32, Piece::BP, None, false, false, false, false);
                assert!(result.contains(&expected));
            }
        }

        #[test]
        fn should_get_the_promotions() {
            let mut board = BoardState::new();
            let bp = 0x82400u64;
            board.board[Piece::BP] = Bitboard::from(bp);
            board.set_occupancy(Color::Black, bp);

            let result = board.get_pawn_movement(Color::Black, false);
            let targets = [(Square::C2, Square::C1, Some(Piece::BQ)), (Square::C2, Square::C1, Some(Piece::BB)), (Square::C2, Square::C1, Some(Piece::BR)), (Square::C2, Square::C1, Some(Piece::BN)), (Square::F2, Square::F1, Some(Piece::BQ)), (Square::F2, Square::F1, Some(Piece::BB)), (Square::F2, Square::F1, Some(Piece::BR)), (Square::F2, Square::F1, Some(Piece::BN)), (Square::D3, Square::D2, None)];

            assert_eq!(result.len(), targets.len());

            for (src, target, promoted_to) in targets {
                let expected = BitMove::new(src as u32, target as u32, Piece::BP, promoted_to, false, false, false, false);
                assert!(result.contains(&expected));
            }
        }
    }

    #[cfg(test)]
    mod pawn_attacks {
        use super::*;


        #[test]
        fn should_return_pawn_attacks() {
            let mut board = BoardState::new();
            let black_pawns = 0x22002400u64; // attacker
            let white_pawns = 0x8000060000u64; //  victim B3, C3, H5
            let white_bishop = 0x40u64; // G1
            board.board[Piece::WP] = Bitboard::from(white_pawns);
            board.board[Piece::WB] = Bitboard::from(white_bishop);
            board.board[Piece::BP] = Bitboard::from(black_pawns);
            board.set_occupancy(Color::White, white_pawns | white_bishop);
            board.set_occupancy(Color::Black, black_pawns);

            println!("{:#?}", Bitboard::from(black_pawns).to_string());
            println!("{:#?}", Bitboard::from(white_pawns).to_string());

            let result = board.get_pawn_attacks(Color::Black);
            let targets = [(Square::B4, Square::C3, None), (Square::F2, Square::G1, Some(Piece::BB)), (Square::F2, Square::G1, Some(Piece::BR)), (Square::F2, Square::G1, Some(Piece::BN)), (Square::F2, Square::G1, Some(Piece::BQ))];
        
            assert_eq!(result.len(), targets.len());
            for (src, target, promoted_to) in targets {
                let expected = BitMove::new(src as u32, target as u32, Piece::BP, promoted_to, true, false, false, false);
                assert!(result.contains(&expected));
            }
        }


        #[test]
        fn should_return_enpassant_pawn_attacks() {
            let mut board = BoardState::new();
            let black_pawns = 0x24000000u64; // attacker
            let white_pawns = 0x8002000400u64; //  victim B3, C3, H5
            let white_bishop = 0x40u64; // G1
            board.board[Piece::WP] = Bitboard::from(white_pawns);
            board.board[Piece::WB] = Bitboard::from(white_bishop);
            board.board[Piece::BP] = Bitboard::from(black_pawns);
            board.set_occupancy(Color::White, white_pawns | white_bishop);
            board.set_occupancy(Color::Black, black_pawns);
            board.enpassant = Some(Square::B3);

            println!("{:#?}", Bitboard::from(black_pawns).to_string());
            println!("{:#?}", Bitboard::from(white_pawns).to_string());

            let result = board.get_pawn_attacks(Color::Black);
            let targets = [(Square::C4, Square::B3)];
        
            assert_eq!(result.len(), targets.len());
            for (src, target) in targets {
                let expected = BitMove::new(src as u32, target as u32, Piece::BP, None, true, false, true, false);
                assert!(result.contains(&expected));
            }
        }
    }

    #[cfg(test)]
    mod castling_rights {
        use super::*;

        #[test]
        fn black_king_should_queen_castle() {
            let mut board = BoardState::new();
            let white_king = 0x10u64;
            let black_king = 1 << (Square::E8 as u64);
            let white_rooks = 0x81u64;
            let black_rooks = 0x8100000000000000u64;
            let black_knight = 1 << (Square::G8 as u64) | 1 << (Square::C3 as u64);
            
            board.set_occupancy(Color::White, white_king | white_rooks);
            board.set_occupancy(Color::Black, black_king | black_knight | black_rooks);
            
            board.board[Piece::WK] = Bitboard::from(white_king);
            board.board[Piece::BK] = Bitboard::from(black_king);
            board.board[Piece::WR] = Bitboard::from(white_rooks);
            board.board[Piece::BR] = Bitboard::from(black_rooks);
            board.board[Piece::BN] = Bitboard::from(black_knight);
    
            let expected = [(Square::E8, Square::C8, Piece::BK)];
            let received = board.get_castling(Color::Black);
    
            assert_eq!(expected.len(), received.len());
            for (src, target, piece) in expected {
                let bitmove = BitMove::new(src as u32, target as u32, piece, None, false, false, false, true);
                assert!(received.contains(&bitmove));
            } 
        }
    
        #[test]
        fn black_king_can_castle_kingside() {
            let mut board = BoardState::new();
            let white_king = 0x10u64;
            let black_king = 1 << (Square::E8 as u64);
            let white_rooks = 0x81u64;
            let black_rooks = 0x8100000000000000u64;
            let black_knight = 1 << (Square::C8 as u64) | 1 << (Square::C3 as u64);
            
            board.set_occupancy(Color::White, white_king | white_rooks);
            board.set_occupancy(Color::Black, black_king | black_knight | black_rooks);
            
            board.board[Piece::WK] = Bitboard::from(white_king);
            board.board[Piece::BK] = Bitboard::from(black_king);
            board.board[Piece::WR] = Bitboard::from(white_rooks);
            board.board[Piece::BR] = Bitboard::from(black_rooks);
            board.board[Piece::BN] = Bitboard::from(black_knight);
    
            let expected = [(Square::E8, Square::G8, Piece::BK)];
            let received = board.get_castling(Color::Black);
    
            assert_eq!(expected.len(), received.len());
            for (src, target, piece) in expected {
                let bitmove = BitMove::new(src as u32, target as u32, piece, None, false, false, false, true);
                assert!(received.contains(&bitmove));
            } 
        }

        #[test]
        /// Can kingside castle as a white king, but cannot queenside castle if the king would be under attack if it tries to move the queen direction
        fn white_king_can_castle_kingside() {
            let mut board = BoardState::new();
            let white_king = 0x10u64;
            let black_king = 1 << (Square::E8 as u64);
            let white_rooks = 0x81u64;
            let black_rooks = 0x8100000000000000u64;
            let black_pawn = 1 << (Square::B2 as u64);
            
            board.set_occupancy(Color::White, white_king | white_rooks | black_pawn);
            board.set_occupancy(Color::Black, black_king | black_rooks);
            
            board.board[Piece::WK] = Bitboard::from(white_king);
            board.board[Piece::BK] = Bitboard::from(black_king);
            board.board[Piece::WR] = Bitboard::from(white_rooks);
            board.board[Piece::BP] = Bitboard::from(black_pawn);
            board.board[Piece::BR] = Bitboard::from(black_rooks);
    
            let expected = [(Square::E1, Square::G1, Piece::WK)];
            let received = board.get_castling(Color::White);
    
            assert_eq!(expected.len(), received.len());
            for (src, target, piece) in expected {
                let bitmove = BitMove::new(src as u32, target as u32, piece, None, false, false, false, true);
                assert!(received.contains(&bitmove));
            } 
        }


        #[test]
        fn white_king_can_castle_queenside() {
            let mut board = BoardState::new();
            let white_king = 0x10u64;
            let black_king = 1 << (Square::E8 as u64);
            let white_rooks = 0x81u64;
            let black_rooks = 0x8100000000000000u64;
            let white_knight = 1 << (Square::F1 as u64);
            
            board.set_occupancy(Color::White, white_king | white_rooks);
            board.set_occupancy(Color::Black, black_king | black_rooks | white_knight);
            
            board.board[Piece::WK] = Bitboard::from(white_king);
            board.board[Piece::BK] = Bitboard::from(black_king);
            board.board[Piece::WR] = Bitboard::from(white_rooks);
            board.board[Piece::WN] = Bitboard::from(white_knight);
            board.board[Piece::BR] = Bitboard::from(black_rooks);
    
            let expected = [(Square::E1, Square::C1, Piece::WK)];
            let received = board.get_castling(Color::White);
 
            assert_eq!(expected.len(), received.len());
            for (src, target, piece) in expected {
                let bitmove = BitMove::new(src as u32, target as u32, piece, None, false, false, false, true);
                assert!(received.contains(&bitmove));
            } 
        }

        #[test]
        fn king_cannot_castle_in_any_direction_if_it_is_under_attack() {
            let mut board = BoardState::new();
            let white_king = 0x10u64;
            let black_king = 1 << (Square::E8 as u64);
            let white_rooks = 0x81u64;
            let black_rooks = 0x8100000000000000u64;
            let black_pawn = 1 << (Square::D2 as u64);
            
            board.set_occupancy(Color::White, white_king | white_rooks | black_pawn);
            board.set_occupancy(Color::Black, black_king | black_rooks);
            
            board.board[Piece::WK] = Bitboard::from(white_king);
            board.board[Piece::BK] = Bitboard::from(black_king);
            board.board[Piece::WR] = Bitboard::from(white_rooks);
            board.board[Piece::BP] = Bitboard::from(black_pawn);
            board.board[Piece::BR] = Bitboard::from(black_rooks);
    
            let received = board.get_castling(Color::White);
    
            assert_eq!(received.len(), 0);
        }
    }


}