#[cfg(test)]
mod board_tests {
    use crate::board::piece::Piece;
    use crate::board::piece::Piece::*;
    use crate::color::Color::*;
    use crate::bit_move::Move;
    use crate::board::fen::FEN;
    use crate::move_type::MoveType::*;

    use crate::squares::Square::*;
    use crate::{bitboard::Bitboard, board::{castling::Castling, state::board::Board}, color::Color, squares::Square};

    #[test]
    fn should_create_a_new_board() {
        let board = Board::new();
        assert_eq!(board.turn, Color::White);
        assert_eq!(board.castling_rights, Castling::all());
        assert_eq!(board.enpassant, None);
        assert_eq!(board.hash_key, 12825486226133058263);
        assert_eq!(board.fifty, [0, 0]);
    }


    #[test]
    fn basic_set_and_get_methods() {
        let mut board = Board::new();

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
        use crate::board::fen::FEN;

        use super::*;

        #[test]
        fn should_test_if_the_square_is_under_attack_by_pawns() {
            let board = Board::parse_fen("r3k3/8/2p5/8/2pP4/2P5/1p1P4/3Q1p1K w KQkq - 0 1").unwrap();
            
            let result = board.is_square_attacked(Square::D3 as u64, Color::Black);
            assert!(result);
            
            assert!(!board.is_square_attacked(Square::C3 as u64, Color::Black));
        }

        #[test]
        fn should_test_if_the_square_is_under_attack_by_knights() {
            let board = Board::parse_fen("3qk3/8/2N5/8/3p4/8/1p2NPPP/2BQK2R b KQkq - 1 1").unwrap();

            println!("{:#?}", board.to_string());
            assert!(board.is_square_attacked(Square::D4 as u64, Color::White));
            assert!(!board.is_square_attacked(Square::E2 as u64, Color::Black));
        }

        #[test]
        fn should_test_if_the_square_is_under_attack_by_a_king() {
            let mut board = Board::new();

            let bk = 0x1000u64;  // e2
            board.board[Piece::BK] = Bitboard::from(bk);
            board.set_occupancy(Color::Black, bk);

            assert!(!board.is_square_attacked(Square::G4 as u64, Color::Black));
            assert!(board.is_square_attacked(Square::F1 as u64, Color::Black));
        }

        #[test]
        fn should_test_if_the_square_is_under_attack_by_a_bishop_or_queen() {
            let board = Board::parse_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1").unwrap();

            assert!(board.is_square_attacked(Square::G4 as u64, White));
            assert!(board.is_square_attacked(Square::F1 as u64, White));
            assert!(!board.is_square_attacked(Square::H8 as u64,White));
            assert!(board.is_square_attacked(Square::E1 as u64,White));


            let board = Board::parse_fen("rnkb2nr/p1pp2pp/8/7q/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1").unwrap();

            assert!(board.is_square_attacked(Square::G4 as u64, Color::Black));
            assert!(board.is_square_attacked(Square::F3 as u64, Color::Black));
            assert!(board.is_square_attacked(Square::E2 as u64, Color::Black));
            assert!(!board.is_square_attacked(Square::H8 as u64, Color::Black));
            assert!(!board.is_square_attacked(Square::A3 as u64, Color::Black));
        }

        #[test]
        fn should_test_if_the_square_is_under_attack_by_a_rook_or_queen() {
            let mut board = Board::new();

            let br = 0x400001000u64;
            board.board[Piece::BR] = Bitboard::from(br);
            board.set_occupancy(Color::Black, br);

            assert!(board.is_square_attacked(Square::E5 as u64, Color::Black));
            assert!(!board.is_square_attacked(Square::D3 as u64, Color::Black));
            assert!(!board.is_square_attacked(Square::G7 as u64, Color::Black));
            assert!(board.is_square_attacked(Square::C1 as u64, Color::Black));
            assert!(board.is_square_attacked(Square::A2 as u64, Color::Black));
            assert!(board.is_square_attacked(Square::G5 as u64, Color::Black));


            let mut board = Board::new();

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
        use crate::{bitboard::Bitboard, board::{piece::Piece, state::board::Board}, color::Color, squares::Square};

        #[test]
        fn single_push_target_for_white_pawn() {
            let mut board = Board::new();
            let wp = 0x200260000u64; // white pawns
            let enemy_pawns = 0x2002000u64; // black pawns
            board.board[Piece::WP] = Bitboard::from(wp);
            board.set_occupancy(Color::White, wp);
            board.board[Piece::BP] = Bitboard::from(enemy_pawns);
            board.set_occupancy(Color::Black, enemy_pawns);
            let result = board.single_push_targets(Color::White);

            let targets = [Square::B6, Square::C4, Square::F4];

            assert_eq!(result.count_ones() as usize,targets.len());
            for sq in targets {
                assert!((result & 1 << (sq as u64)) != 0);
            }
        }


        #[test]
        fn single_push_target_for_black_pawns() {
            let mut board = Board::new();
            let bp = 0x200260000u64; // black pawns
            let enemy_pawns = 0x2002000u64; // white pawns
            board.board[Piece::BP] = Bitboard::from(bp);
            board.set_occupancy(Color::Black, bp);
            board.board[Piece::WP] = Bitboard::from(enemy_pawns);
            board.set_occupancy(Color::White, enemy_pawns);
            let result = board.single_push_targets(Color::Black);

            let targets = [Square::B2, Square::C2];
            assert_eq!(result.count_ones() as usize, targets.len());
            for sq in targets {
                assert!(result & (1 << sq as u64) != 0);
            }
        }
    }

    #[cfg(test)]
    mod double_push_targets {
        use crate::{bitboard::Bitboard, board::{piece::Piece, state::board::Board}, color::Color, squares::Square};

        #[test]
        fn double_push_for_white_pawns() {
            let mut board = Board::new();
            let wp = 0x9a00u64;
            let enemy = 0x200002040000u64;
            board.board[Piece::WP] = Bitboard::from(wp);
            board.set_occupancy(Color::White, wp);
            board.board[Piece::BP] = Bitboard::from(enemy);
            board.set_occupancy(Color::Black, enemy);

            let result = board.double_push_targets(Color::White);
            
            let targets = [Square::D4, Square::E4, Square::H4];
            assert_eq!(targets.len(), result.count_ones() as usize);
            for sq in targets {
                assert!(result & (1 << sq as u64) != 0);
            }
        }

        #[test]
        fn double_push_for_black_pawns() {
            let mut board = Board::new();
            let bp = 0x35000000000000u64;
            let enemy =  0x500000042000u64;
            board.board[Piece::BP] = Bitboard::from(bp);
            board.set_occupancy(Color::Black, bp);
            board.board[Piece::WP] = Bitboard::from(enemy);
            board.set_occupancy(Color::White, enemy);

            let result = board.double_push_targets(Color::Black);
            let targets = [Square::A5, Square::C5, Square::F5];

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
            let mut board = Board::new();
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
            let mut board = Board::new();
            let bp = 0x30000402000000u64;
            let enemy =  0x2000040000u64;
            board.board[Piece::BP] = Bitboard::from(bp);
            board.set_occupancy(Color::Black, bp);
            board.board[Piece::WP] = Bitboard::from(enemy);
            board.set_occupancy(Color::White, enemy);

            let result = board.pawns_able_to_double_push(Color::Black);
            let targets = [Square::E7];

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
            let mut board = Board::new();
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
                let expected = Move::new(src as u32, target  as u32, Piece::WP, None, false, true, false, false);
                assert!(result.contains(&expected));
            }
        }

        #[test]
        fn get_single_pawn_moves_for_black_pawns() {
            let mut board = Board::new();
            let bp = 0x30000402000000u64;
            let enemy =  0x2000060000u64;
            board.board[Piece::BP] = Bitboard::from(bp);
            board.set_occupancy(Color::Black, bp);
            board.board[Piece::WP] = Bitboard::from(enemy);
            board.set_occupancy(Color::White, enemy);

            let result = board.get_pawn_movement(Color::Black, false);
            let targets = [(Square::E7, Square::E6), (Square::F7, Square::F6), (Square::C5, Square::C4)];

            assert_eq!(result.len(), targets.len());
            for (src, target) in targets {
                let expected = Move::new(src as u32, target  as u32, Piece::BP, None, false, false, false, false);
                assert!(result.contains(&expected));
            }
        }

        #[test]
        fn should_get_the_promotions() {
            let mut board = Board::new();
            let bp = 0x82400u64;
            board.board[Piece::BP] = Bitboard::from(bp);
            board.set_occupancy(Color::Black, bp);

            let result = board.get_pawn_movement(Color::Black, false);
            let targets = [(Square::C2, Square::C1, Some(Piece::BQ)), (Square::C2, Square::C1, Some(Piece::BB)), (Square::C2, Square::C1, Some(Piece::BR)), (Square::C2, Square::C1, Some(Piece::BN)), (Square::F2, Square::F1, Some(Piece::BQ)), (Square::F2, Square::F1, Some(Piece::BB)), (Square::F2, Square::F1, Some(Piece::BR)), (Square::F2, Square::F1, Some(Piece::BN)), (Square::D3, Square::D2, None)];

            assert_eq!(result.len(), targets.len());

            for (src, target, promoted_to) in targets {
                let expected = Move::new(src as u32, target as u32, Piece::BP, promoted_to, false, false, false, false);
                assert!(result.contains(&expected));
            }
        }
    }

    #[cfg(test)]
    mod pawn_attacks {
        use super::*;


        #[test]
        fn should_return_pawn_attacks() {
            let mut board = Board::new();
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
                let expected = Move::new(src as u32, target as u32, Piece::BP, promoted_to, true, false, false, false);
                assert!(result.contains(&expected));
            }
        }


        #[test]
        fn should_return_enpassant_pawn_attacks() {
            let mut board = Board::new();
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
                let expected = Move::new(src as u32, target as u32, Piece::BP, None, true, false, true, false);
                assert!(result.contains(&expected));
            }
        }
    }

    #[cfg(test)]
    mod castling_rights {
        use super::*;

        #[test]
        fn black_king_should_queen_castle() {
            let mut board = Board::new();
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
                let Move = Move::new(src as u32, target as u32, piece, None, false, false, false, true);
                assert!(received.contains(&Move));
            }
        }
    
        #[test]
        fn black_king_can_castle_kingside() {
            let mut board = Board::new();
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
                let Move = Move::new(src as u32, target as u32, piece, None, false, false, false, true);
                assert!(received.contains(&Move));
            } 
        }

        #[test]
        /// Can kingside castle as a white king, but cannot queenside castle if the king would be under attack if it tries to move the queen direction
        fn white_king_can_castle_kingside() {
            let mut board = Board::new();
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
                let Move = Move::new(src as u32, target as u32, piece, None, false, false, false, true);
                assert!(received.contains(&Move));
            } 
        }


        #[test]
        fn white_king_can_castle_queenside() {
            let mut board = Board::new();
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
                let Move = Move::new(src as u32, target as u32, piece, None, false, false, false, true);
                assert!(received.contains(&Move));
            } 
        }

        #[test]
        fn king_cannot_castle_in_any_direction_if_it_is_under_attack() {
            let mut board = Board::new();
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


    #[cfg(test)]
    mod sliding_moves {
        use super::*;
        use crate::squares::Square::*;
        use crate::board::piece::Piece::*;


        #[test]
        fn should_return_all_possible_destinations_including_capturs_for_knight() {
            let mut board = Board::new();

            let black_pawns = 0x82000004200u64;
            let black_queen = 1 << B4 as u64;
            let white_knight = 1 << D3 as u64; // attacker
            let white_rook = 1 << F4 as u64;

            board.set_occupancy(Color::Black, black_pawns | black_queen);
            board.set_occupancy(Color::White, white_knight | white_rook);
            board.board[BP] = Bitboard::from(black_pawns);
            board.board[BQ] = Bitboard::from(black_queen);
            board.board[WN] = Bitboard::from(white_knight);
            board.board[WR] = Bitboard::from(white_rook);

            let received = board.get_sliding_and_leaper_moves(WN);
            let targets = [(B2, Some(BP)), (B4, Some(BQ)), (C1, None), (C5, None), (F2, None), (E1, None), (E5, None)];

            assert_eq!(received.len(), targets.len());

            for (target, captured) in targets {
                let expected = Move::new(D3 as u32, target as u32, WN, None, captured.is_some(), false, false, false);
                assert!(received.contains(&expected));
            }
        }

        #[test]
        fn should_return_all_possible_destinations_including_capturs_for_bishop() {
            let mut board = Board::new();

            let black_pawns = 0x82000004200u64;
            let black_queen = 1 << H8 as u64;
            let white_bishop = 1 << E5 as u64; // attacker
            let white_rook = 1 << F4 as u64;

            board.set_occupancy(Color::Black, black_pawns | black_queen);
            board.set_occupancy(Color::White, white_bishop | white_rook);
            board.board[BP] = Bitboard::from(black_pawns);
            board.board[BQ] = Bitboard::from(black_queen);
            board.board[WB] = Bitboard::from(white_bishop);
            board.board[WR] = Bitboard::from(white_rook);

            let received = board.get_sliding_and_leaper_moves(WB);
            let targets = [(D4, false), (C3, false), (B2, true), (D6, true), (F6, false), (G7, false), (H8, true), ];

            assert_eq!(received.len(), targets.len());
            for (target, captured) in targets {
                let expected = Move::new(E5 as u32 as u32, target as u32, WB, None, captured, false, false, false);
                assert!(received.contains(&expected));
            }
        }

        #[test]
        fn should_return_all_possible_destinations_including_captures_for_rook() {
            let mut board = Board::new();

            let white_pawns = 0x082000004200u64;
            let white_knights = 0x20020u64;
            let black_rooks = 1 << D2 as u64 | 1 << (C1 as u64);

            board.set_occupancy(Color::Black, black_rooks);
            board.set_occupancy(Color::White, white_pawns | white_knights);
            board.board[WP] = Bitboard::from(white_pawns);
            board.board[WN] = Bitboard::from(white_knights);
            board.board[BR] = Bitboard::from(black_rooks);

            let received = board.get_sliding_and_leaper_moves(BR);
            let targets = [(D2, D1, false), (D2, D3, false), (D2, D4, false), (D2, D5, false), (D2, D6, true), (D2, C2, false), (D2, B2, true), (D2, E2, false), (D2, F2, false), (D2, G2, true),
            (C1, A1, false), (C1, B1, false), (C1, D1, false), (C1, E1, false), (C1, F1, true), (C1, C2, false), (C1, C3, false), (C1, C4, false), (C1, C5, false), (C1, C6, false), (C1, C7, false), (C1, C8, false)];

            assert_eq!(received.len(), targets.len());

            for (src, target, is_capture) in targets {
                let expected = Move::new(src as u32, target as u32, BR, None, is_capture, false, false, false);
                assert!(received.contains(&expected));
            }
        }


        #[test]
        fn should_return_all_possible_destinations_including_captures_for_queen() {
            let mut board = Board::new();

            let white_pawns = 0x082000004200u64;
            let white_knights = 0x20020u64;
            let black_queen = 1 << D2 as u64;

            board.set_occupancy(Color::Black, black_queen);
            board.set_occupancy(Color::White, white_pawns | white_knights);
            board.board[WP] = Bitboard::from(white_pawns);
            board.board[WN] = Bitboard::from(white_knights);
            board.board[BQ] = Bitboard::from(black_queen);

            let received = board.get_sliding_and_leaper_moves(BQ);
            let targets = [(D2, D1, false), (D2, D3, false), (D2, D4, false), (D2, D5, false), (D2, D6, true), (D2, C2, false), (D2, B2, true), (D2, E2, false), (D2, F2, false), (D2, G2, true),
            (D2, E1, false), (D2, C1, false), (D2, C3, false), (D2, B4, false), (D2, A5, false), (D2, E3, false), (D2, F4, false), (D2, G5, false), (D2, H6, false)];

            assert_eq!(received.len(), targets.len());

            for (src, target, is_capture) in targets {
                let expected = Move::new(src as u32, target as u32, BQ, None, is_capture, false, false, false);
                assert!(received.contains(&expected));
            }
        }
    }


    #[cfg(test)]
    mod make_move {
        use super::*;
        
        use crate::move_type::MoveType;
        use crate::move_type::MoveType::*;
        #[cfg(test)]
        mod capturing_moves_and_regular_moves {

        use super::*;

        #[test]
        fn bishop_can_move_or_capture() {

            let mut board = Board::new();

            let white_pawns = 0x082000004200u64;
            let white_knights = 0x20020u64;
            let black_king = 1 << E8 as u64;
            let white_king = 1 << E1 as u64;
            let subject = 1 << D2 as u64; // black bishop
            let white_bishop = 1 << (A6 as u64);
            board.set_occupancy(Color::Black, subject | black_king);
            board.set_occupancy(Color::White, white_pawns | white_knights | white_king | white_bishop);
            board.board[WP] = Bitboard::from(white_pawns);
            board.board[WN] = Bitboard::from(white_knights);
            board.board[WB] = Bitboard::from(white_bishop);
            board.board[BB] = Bitboard::from(subject);
            board.board[BK] = Bitboard::from(black_king);
            board.board[WK] = Bitboard::from(white_king);
            board.turn = Black;

            let Move = Move::new(D2 as u32, D3 as u32, BB, None, false, false, false, false);
            assert_eq!(Square::from((*board.board[BB]).trailing_zeros() as u64), D2);

            let zobrist_key = board.hash_key;
            let result = board.make_move(Move, MoveType::AllMoves).unwrap();

            assert_ne!(result.hash_key, zobrist_key);
            assert_eq!(Square::from((*result.board[BB]).trailing_zeros() as u64), D3);

            let Move = Move::new(A6 as u32, D3 as u32, WB, None, true, false, false, false);
            let capture_result = result.make_move(Move, MoveType::CapturesOnly).unwrap();

            assert_ne!(capture_result.hash_key, zobrist_key);
            assert_ne!(capture_result.hash_key, result.hash_key);
            assert_eq!(Square::from((*capture_result.board[WB]).trailing_zeros() as u64), D3);
            assert_eq!((*capture_result.board[BB]).trailing_zeros(), 64);
            assert_eq!((*capture_result.board[BB]).count_ones(), 0);
        }

        #[test]
        fn queen_move_and_capture() {
            let mut board = Board::new();

            let black_pawns = 0x2000008800u64;
            let black_king = 1 << H8 as u64;
            let white_king = 1 << A3 as u64;
            let white_pawns = 0x1000020000400u64;
            let subject = 1 << E4 as u64; // white queen
            let black_queen = 1<< C8 as u64;
            board.set_occupancy(Color::White, subject | white_king | white_pawns);
            board.set_occupancy(Black, black_king | black_pawns | black_queen);
            board.board[BP] = Bitboard::from(black_pawns);
            board.board[BK] = Bitboard::from(black_king);
            board.board[WK] = Bitboard::from(white_king);
            board.board[WP] = Bitboard::from(white_pawns);
            board.board[WQ] = Bitboard::from(subject);
            board.board[BQ] = Bitboard::from(black_queen);

            let zobrist = board.hash_key;
            let Move = Move::new(E4 as u32, F5 as u32, WQ, None, true, false, false, false);

            assert_eq!((*board.board[BP]).count_ones(), 3);
            let result = board.make_move(Move, MoveType::AllMoves).unwrap();
            
            assert_ne!(result.hash_key, zobrist);
            assert_eq!(Square::from((*result.board[WQ]).trailing_zeros() as u64), F5);
            assert_eq!((*result.board[BP]).count_ones(), 2);
            
            let Move = Move::new(C8 as u32, F5 as u32, BQ, None, true, false, false, false);
            let capture_result = result.make_move(Move, MoveType::CapturesOnly).unwrap();

            assert_ne!(capture_result.hash_key, zobrist);
            assert_ne!(capture_result.hash_key, result.hash_key);
            assert_eq!(Square::from((*capture_result.board[BQ]).trailing_zeros() as u64), F5);
            assert_eq!((*capture_result.board[WQ]).trailing_zeros(), 64);
            assert_eq!((*capture_result.board[WQ]).count_ones(), 0);
        }

        #[test]
        fn king_move_and_capture() {
            let board = Board::parse_fen("rnb1kbnr/p1p1p1pp/8/3p1N2/3K4/7q/PP1PPPPP/R2Q1BNR w KQkq - 0 1").unwrap();

            let zobrist = board.hash_key;
            let Move = Move::new(D4 as u32, D5 as u32, WK, None, true, false, false, false);

            assert_eq!((*board.board[WK]).count_ones(), 1);
            assert_eq!(Square::from((*board.board[WK]).trailing_zeros() as u64), D4);

            let result = board.make_move(Move, MoveType::AllMoves).unwrap();
            assert_ne!(result.hash_key, board.hash_key);
            assert_eq!(Square::from((*result.board[WK]).trailing_zeros()  as u64), D5);
            
            let Move = Move::new(E8 as u32, D7 as u32, BK, None, false, false, false, false);
            let black_king_move = result.make_move(Move, MoveType::AllMoves).unwrap();

            assert_ne!(black_king_move.hash_key, result.hash_key);
            assert_ne!(black_king_move.hash_key, zobrist);
            assert_eq!(Square::from((*black_king_move.board[BK]).trailing_zeros() as u64), D7);
        }

        #[test]
        fn rook_move_and_capture() {
            let board = Board::parse_fen("r2qkbnr/4pppp/8/4P3/2R5/PpP1P3/1P1P2PP/1NBQKBNR b KQk - 3 3").unwrap();

            let Move = Move::new(A8 as u32, A4 as u32, BR, None, false, false, false, false);

            assert_eq!((*board.board[BR]).count_ones(), 2);
            assert_eq!((*board.board[WR]).count_ones(), 2);

            let black_move = board.make_move(Move, AllMoves).unwrap();

            
            assert_eq!((*black_move.board[BR]).count_ones(), 2);
            assert_eq!((*black_move.board[WR]).count_ones(), 2);

            let Move = Move::new(C4 as u32, A4 as u32, WR, None, true, false, false, false);;
            let white_captures = black_move.make_move(Move, AllMoves).unwrap();
            
            println!("{:#?}", white_captures.to_string());

            assert_ne!(white_captures.hash_key, black_move.hash_key);
            assert_ne!(white_captures.hash_key, board.hash_key);
            assert_eq!((*white_captures.board[BR]).count_ones(), 1);
            assert_eq!((*white_captures.board[WR]).count_ones(), 2);
        }

        #[test]
        fn knight_move_and_capture() {
            let board = Board::parse_fen("r2qkbnr/4pppp/8/4P3/2R5/PpP1P3/1P1P2PP/N1BQKBNR w KQk - 3 3").unwrap();

            let Move = Move::new(A1 as u32, B3 as u32, WN, None, true, false, false, false);

            assert_eq!((*board.board[BN]).count_ones(), 1);
            assert_eq!((*board.board[WN]).count_ones(), 2);
            assert_eq!((*board.board[BP]).count_ones(), 5);
            assert_eq!((*board.board[WP]).count_ones(), 8);

            let white_captures = board.make_move(Move, AllMoves).unwrap();


            assert_eq!((*white_captures.board[BN]).count_ones(), 1);
            assert_eq!((*white_captures.board[WN]).count_ones(), 2);
            assert_eq!((*white_captures.board[BP]).count_ones(), 4);
            assert_eq!((*white_captures.board[WP]).count_ones(), 8);
            

            println!("{:#?}", white_captures.to_string());
            let Move = Move::new(G8 as u32, F6 as u32, BN, None, false, false, false, false);
            let black_move = white_captures.make_move(Move, AllMoves).unwrap();
            
            
            assert_ne!(black_move.hash_key, white_captures.hash_key);
            assert_ne!(black_move.hash_key, board.hash_key);
            assert_eq!((*black_move.board[BN]).count_ones(), 1);
            assert_eq!((*white_captures.board[WN]).count_ones(), 2);
            assert_eq!((*white_captures.board[BP]).count_ones(), 4);
            assert_eq!((*white_captures.board[WP]).count_ones(), 8);
            assert_eq!(Square::from((*black_move.board[BN]).trailing_zeros() as u64), F6)
        }

        #[test]
        fn pawn_move_and_capture() {
            let board = Board::parse_fen("r2qkbnr/4pppp/2R5/1p2P3/8/PP2P3/1P1P2PP/N1BQKBNR w KQk - 4 3").unwrap();
            let Move = Move::new(A3 as u32, A4 as u32, WP, None, false, false, false, false);

            assert_eq!((*board.board[WP]).count_ones(), 8);
            assert_eq!((*board.board[BP]).count_ones(), 5);

            let mut white_moves = board.make_move(Move, AllMoves).unwrap();

            assert_eq!((*white_moves.board[BP]).count_ones(), 5);
            assert_eq!((*white_moves.board[WP]).count_ones(), 8);

            let Move = Move::new(B5 as u32, A4 as u32, BP, None, true, false, false, false);
            white_moves.set_turn(Black);
            println!("{:#?}", white_moves.to_string());
            // println!()
            let black_captures = white_moves.make_move(Move, AllMoves).unwrap();

            assert_eq!((*black_captures.board[BP]).count_ones(), 5);
            assert_eq!((*black_captures.board[WP]).count_ones(), 7);
        }

        #[test]
        fn enpassant_move_and_capture() { // double moves must always result in an enpassant
            let board = Board::parse_fen("r2qkbnr/4pppp/2R5/4P3/1p6/1P2P3/PP1P2PP/N1BQKBNR w KQk - 0 4").unwrap();
            let Move = Move::new(A2 as u32, A4 as u32, WP, None, false, true, false, false);

            assert_eq!((*board.board[WP]).count_ones(), 8);
            assert_eq!((*board.board[BP]).count_ones(), 5);
            assert!((1 <<  A2 as u64) & board.occupancies[Both] != 0);
            assert!((1 <<  A4 as u64) & board.occupancies[Both] == 0);
            assert!(board.enpassant.is_none());

            let white_moves = board.make_move(Move, AllMoves).unwrap();

            assert_eq!((*white_moves.board[WP]).count_ones(), 8);
            assert_eq!((*white_moves.board[BP]).count_ones(), 5);
            assert!((1 <<  A2 as u64) & white_moves.occupancies[Both] == 0);
            assert!((1 <<  A4 as u64) & white_moves.occupancies[Both] != 0);
            assert!(white_moves.enpassant == Some(A3));

            let Move = Move::new(B4 as u32, A3 as u32, BP, None, true, false, true, false);
            let black_captures = white_moves.make_move(Move, AllMoves).unwrap();
            
            assert_eq!((*black_captures.board[WP]).count_ones(), 7);
            assert_eq!((*black_captures.board[BP]).count_ones(), 5);
            assert!((1 <<  B4 as u64) & black_captures.occupancies[Both] == 0);
            assert!((1 <<  A3 as u64) & black_captures.occupancies[Both] != 0);
            assert!(black_captures.enpassant.is_none());

        }


        #[test]
        fn should_not_move_if_the_source_position_is_not_correct() {
            let board = Board::parse_fen("3qk3/r3p3/3p4/1p3P2/1b1P3n/8/6PP/P3KBNR w KQkq - 0 1").unwrap();
            let src = C6;
            let Move = Move::new(src as u32, C5 as u32, BP, None, false, false, false, false);

            assert!(board.occupancies[Both] & (1 << C6 as u64) == 0);
            assert!(board.occupancies[Both] & (1 << C5 as u64) == 0);

            let result = board.make_move(Move, AllMoves);
            assert!(result.is_none());
        }

        #[test]
        fn should_not_change_position_if_the_target_move_is_invalid() {
            let board = Board::parse_fen("3qk3/r3p3/3p4/1p3P2/1b1P3n/8/6PP/P3KBNR w KQkq - 0 1").unwrap();
            let Move = Move::new(H4 as u32, H6 as u32, BN, None, false, false, false, false);

            assert!(board.occupancies[Both] & (1 << H4 as u64) != 0);
            assert!(board.occupancies[Both] & (1 << H6 as u64) == 0);
            
            let result = board.make_move(Move, AllMoves);
         
            assert!(result.is_none());
            assert!(board.occupancies[Both] & (1 << H4 as u64) != 0);
            assert!(board.occupancies[Both] & (1 << H6 as u64) == 0);
         
        }

        #[test]
        fn black_pawns_should_only_move_southwards() {
            let board = Board::parse_fen("3qk3/r3p3/3p4/1p3P2/1b1P3n/8/6PP/P3KBNR w KQkq - 0 1").unwrap();
            let black_moving_north = Move::new(D6 as u32, D7 as u32, BP, None, false, false, false, false);

            assert!(board.occupancies[Both] & (1 << D6 as u64) != 0);
            assert!(board.occupancies[Both] & (1 << D7 as u64) == 0);


            let result = board.make_move(black_moving_north, AllMoves);
            assert!(result.is_none());
            assert!(board.occupancies[Both] & (1 << D7 as u64) == 0);
            assert!(board.occupancies[Both] & (1 << D7 as u64) == 0);
        }

        #[test]
        fn white_pawns_should_only_move_northwards() {
            let board = Board::parse_fen("3qk3/r3p3/3p4/1p3P2/1b1P3n/8/6PP/P3KBNR w KQkq - 0 1").unwrap();
            let white_pawn_moving_south = Move::new(D4 as u32, D3 as u32, WP, None, false, false, false, false);

            assert!(board.occupancies[Both] & (1 << D4 as u64) != 0);
            assert!(board.occupancies[Both] & (1 << D3 as u64) == 0);

            let result = board.make_move(white_pawn_moving_south, AllMoves);
            
            assert!(result.is_none());
            assert!(board.occupancies[Both] & (1 << D4 as u64) != 0);
            assert!(board.occupancies[Both] & (1 << D3 as u64) == 0);
        }

        #[test]
        fn pawns_should_not_be_able_to_move_north_north_or_south_south_if_it_is_occupied() {
            let board = Board::parse_fen("3qk3/r3p3/4P3/1p3P2/1b5n/8/6PP/P3KBNR w KQkq - 0 1").unwrap();
            let white_pawn_moving_south = Move::new(E6 as u32, E7 as u32, WP, None, false, false, false, false);

            assert!(board.occupancies[Both] & (1 << E6 as u64) != 0);
            assert!(board.occupancies[Both] & (1 << E7 as u64) != 0);

            let result = board.make_move(white_pawn_moving_south, AllMoves);
            
            assert!(result.is_none());

            let black_pawn_moving_north = Move::new(E7 as u32, E6 as u32, WP, None, false, false, false, false);

            assert!(board.occupancies[Both] & (1 << E6 as u64) != 0);
            assert!(board.occupancies[Both] & (1 << E7 as u64) != 0);

            let result = board.make_move(black_pawn_moving_north, AllMoves);
            
            assert!(result.is_none());
        }


        #[test]
        fn  should_return_none_if_it_is_not_the_players_turn() {
            let board = Board::parse_fen("3qk3/r3p3/4P3/1p3P2/1b5n/8/6PP/P3KBNR w KQkq - 0 1").unwrap();
            let black_bishop_move = Move::new(B4 as u32, E1 as u32, BB, None, true, false, false, false);

            assert!(board.occupancies[Both] & (1 << B4 as u32) != 0);
            assert!(board.occupancies[Both] & (1 << E1 as u32) != 0);

            let result = board.make_move(black_bishop_move, AllMoves);
            assert!(result.is_none());
        }
    }
    }

    #[cfg(test)] 
    mod pawn_promotion {
        use super::*;
        #[test]
        fn should_promote_a_pawn_move_to_queen() {
            let board = Board::parse_fen("2b5/k4P2/p7/7p/8/8/1P1P4/3QK3 w k - 2 2").unwrap();
    
            let mv = Move::new(F7 as u32, F8 as u32, WP, Some(WQ), false, false, false, false);
            assert_eq!(board[WQ].count_ones(), 1);
            assert_eq!(board[WP].count_ones(), 3);
            assert_eq!(board[BQ].count_ones(), 0);
            assert_eq!(board[BP].count_ones(), 2);
    
            let result = board.make_move(mv, AllMoves).unwrap();
    
            assert_eq!(result[WQ].count_ones(), 2);
            assert_eq!(result[WP].count_ones(), 2);
            assert_eq!(result[BQ].count_ones(), 0);
            assert_eq!(result[BP].count_ones(), 2);
        }
    
        #[test]
        fn should_return_none_if_a_promotion_is_invalid() {
            let board = Board::parse_fen("2b5/k4P2/p7/7p/8/8/1P1P4/3QK3 w k - 2 2").unwrap();
            let mv = Move::new(H5 as u32, H1 as u32, BP, Some(BQ), false, false, false, false);
    
            let result = board.make_move(mv, AllMoves);
            assert!(result.is_none());
        }
    
        #[test]
        fn should_make_a_capture_and_promotion_at_the_same_time() {
            let board = Board::parse_fen("2b5/k4P2/p7/8/8/1K6/1P1Pp3/3Q4 b k - 2 2").unwrap();
            let mv = Move::new(E2 as u32, D1 as u32, BP, Some(BR), true, false, false, false);
    
            assert_eq!(board[WQ].count_ones(), 1);
            assert_eq!(board[WP].count_ones(), 3);
            assert_eq!(board[BQ].count_ones(), 0);
            assert_eq!(board[BP].count_ones(), 2);
            assert_eq!(board[BR].count_ones(), 0);
            assert_eq!(board.occupancies[Black].count_ones(), 4);
            assert_eq!(board.occupancies[White].count_ones(), 5);
            
            let result = board.make_move(mv, AllMoves).unwrap();
            
            assert_eq!(result[WQ].count_ones(), 0);
            assert_eq!(result[WP].count_ones(), 3);
            assert_eq!(result[BQ].count_ones(), 0);
            assert_eq!(result[BP].count_ones(), 1);
            assert_eq!(result[BR].count_ones(), 1);
            assert_eq!(result.occupancies[Black].count_ones(), 4);
            assert_eq!(result.occupancies[White].count_ones(), 4);
        }
    }

    #[cfg(test)]
    mod castling_moves {
        use super::*;

        #[test]
        fn black_king_can_castle_queenside() {
            let board = Board::parse_fen("r3k2r/pb2p2p/1pq2p1n/3p3P/2PN2N1/3B4/3P1PPP/R2QK2R b KQkq - 1 2").unwrap();
            let mv = Move::new(E8 as u32, C8 as u32, BK, None, false, false, false, true);

            let result = board.make_move(mv, AllMoves).unwrap();
            assert_eq!(Square::from(result[BK].trailing_zeros() as u64), C8);
            assert_eq!(Square::from(result[BR].trailing_zeros() as u64), D8);
            assert_eq!(result[BR].count_ones(), 2);
        }

        #[test]
        fn black_king_can_castle_kingside() {
            let board = Board::parse_fen("r3k2r/pb2p2p/1pq2p1n/3p3P/2PN2N1/3B4/3P1PPP/R2QK2R b KQkq - 1 2").unwrap();
            let mv = Move::new(E8 as u32, G8 as u32, BK, None, false, false, false, true);

            let result = board.make_move(mv, AllMoves).unwrap();
            println!("{:#?}", result.to_string());
            assert_eq!(Square::from(result[BK].trailing_zeros() as u64), G8);
            assert_eq!(Square::from(result[BR].trailing_zeros() as u64), A8);
            assert_eq!(Square::from(((*result[BR] ^ (1 << A8 as u64))).trailing_zeros() as u64), F8);
            assert_eq!(result[BR].count_ones(), 2);
            assert_eq!(result[BK].count_ones(), 1);
        }

        #[test]
        fn white_king_can_castle_kingside() {
            let board = Board::parse_fen("r3k2r/pb2p2p/1pq2p1n/3p3P/2PN2N1/3B4/3P1PPP/R2QK2R w KQkq - 1 2").unwrap();

            let mv = Move::new(E1 as u32, G1 as u32, WK, None, false, false, false, true);
            let result = board.make_move(mv, AllMoves).unwrap();

            assert_eq!(result[WR].count_ones(), 2);

            assert_eq!(Square::from(result[WK].trailing_zeros() as u64), G1);

            println!("{:#?}", result.to_string());
            assert_eq!(Square::from(result[WK].trailing_zeros() as u64), G1);
            assert_eq!(Square::from(result[WR].trailing_zeros() as u64), A1);
            assert_eq!(Square::from(((*result[WR] ^ (1<< A1 as u64)).trailing_zeros()) as u64), F1);
            assert_eq!(result[BR].count_ones(), 2);
            assert_eq!(result[BK].count_ones(), 1);
        }

        #[test]
        fn should_fail_to_castle_whiteking_queenside_if_there_are_pieces_between_the_castling_sides() {
            let board = Board::parse_fen("r3k2r/pb2p2p/1pq2p1n/3p3P/2PN2N1/3B4/3P1PPP/R2QK2R w KQkq - 1 2").unwrap();

            let mv = Move::new(E1 as u32, C1 as u32, WK, None, false, false, false, true);
            let result = board.make_move(mv, AllMoves);
            assert!(result.is_none());
        }

        #[test]
        fn should_fail_to_castle_blacking_kingside_if_there_are_pieces_between_the_king_and_the_rook() {
            let board = Board::parse_fen("r3kq1r/pb2p2p/1p3p1n/3p3P/2PN2N1/3B4/3P1PPP/R2QK2R b KQkq - 1 2").unwrap();
            let mv = Move::new(E8 as u32, G8 as u32, BK, None, false, false, false, true);
            println!("{:#?}", board.to_string());

            assert!(board.make_move(mv, AllMoves).is_none());
        }

        #[test]
        fn should_fail_to_castle_whiteking_kingside_if_there_are_pieces_between_the_king_and_the_rook() {
            let board = Board::parse_fen("r3k2r/pb2p2p/1pq2p1n/3p3P/2PN2N1/3B4/3P1PPP/4KQ2 w KQkq - 1 2").unwrap();
            let mv = Move::new(E1 as u32, G1 as u32, WK, None, false, false, false, true);

            let result = board.make_move(mv, AllMoves);

            assert!(result.is_none());
        }

        #[test]
        fn should_fail_to_castle_blackking_queenside_if_there_are_pieces_between_the_king_and_the_rook() {
            let board = Board::parse_fen("r1q1k2r/pb2p2p/1p3p1n/3p3P/2PN2N1/3B4/3P1PPP/R3KQ1R b KQkq - 1 2").unwrap();
            let mv = Move::new(E8 as u32, C8 as u32, BK, None, false, false, false, true);
            println!("{:#?}", board.to_string());
            assert!(board.make_move(mv, AllMoves).is_none());
        }

        #[test]
        fn should_fail_to_castle_if_side_does_not_have_castling_rights_anymore() {
            let board = Board::parse_fen("r3k2r/pb2p2p/1pq2p1n/3p3P/2PN2N1/3B4/3P1PPP/R2QK2R w ---- - 1 2").unwrap();

            let mv = Move::new(E1 as u32, G1 as u32, WK, None, false, false, false, true);
            assert!(board.make_move(mv, AllMoves).is_none());
        }
    }




    #[test]
    fn returns_the_piece_at_a_particular_position() {
        let board = Board::parse_fen("4rkn1/P5pp/8/7q/4P3/8/5PPP/2R1K1NR w KQkq e3 0 1").unwrap();
        
        assert_eq!(board.get_piece_at(C1, White).unwrap(), WR);
    }

}