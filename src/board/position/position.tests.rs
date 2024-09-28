#[cfg(test)]
mod do_and_undo_a_move {
    use crate::board::position::Position;
    // use crate::color::Color::*;
    // use crate::squares::Square::*;
    // use crate::bit_move::Move;
    // use crate::board::piece::Piece::*;
    
    use crate::board::state::board::Board;
    use crate::color::Color::*;
    use crate::bit_move::{Move, MoveType::*};
    use crate::board::fen::FEN;
    use crate::move_scope::MoveScope::*;
    use crate::squares::Square::*;


    #[test]
    fn should_update_self_after_move() {
        let board = Board::parse_fen("4kb2/3p1n2/1r2p3/3b1p1p/PpN5/8/BP1P3P/R2QK3 b -Q-- - 0 2").unwrap();
        let mv = Move::new(D5 as u8, C4 as u8, Capture);
        let mut position = Position::with(board);

        let black_bishop_d5 = 1u64 << D5 as u8;
        assert!(position.board.occupancies[Both] & black_bishop_d5 != 0);
        assert!(position.board.occupancies[Black] & black_bishop_d5 != 0);
        assert!(position.board.occupancies[White] & black_bishop_d5 == 0);
        let white_knight_c4 = 1u64 << C4 as u8;
        assert!(position.board.occupancies[Both] & white_knight_c4 != 0);
        assert!(position.board.occupancies[Black] & white_knight_c4 == 0);
        assert!(position.board.occupancies[White] & white_knight_c4 != 0);

        position.make_move(mv, CapturesOnly);

        assert!(position.board.occupancies[Both] & black_bishop_d5 == 0);
        assert!(position.board.occupancies[Black] & black_bishop_d5 == 0);
        assert!(position.board.occupancies[White] & black_bishop_d5 == 0);

        assert!(position.board.occupancies[White] & white_knight_c4 == 0);
        let black_knight_c4 = 1u64 << C4 as u8;

        assert!(position.board.occupancies[White] & black_knight_c4 == 0);
        assert!(position.board.occupancies[Both] & black_knight_c4 != 0);
    }


    #[test]
    fn should_be_able_to_undo_a_regular_capture() {
        let board = Board::parse_fen("4kb2/3p1n2/1r2p3/3b1p1p/PpN5/8/BP1P3P/R2QK3 b -Q-- - 0 2").unwrap();
        let mv = Move::new(D5 as u8, C4 as u8, Capture);

        let mut position = Position::with(board);

        let black_bishop_d5 = 1u64 << D5 as u8;
        assert!(position.board.occupancies[Both] & black_bishop_d5 != 0);
        assert!(position.board.occupancies[Black] & black_bishop_d5 != 0);
        assert!(position.board.occupancies[White] & black_bishop_d5 == 0);
        let white_knight_c4 = 1u64 << C4 as u8;
        assert!(position.board.occupancies[Both] & white_knight_c4 != 0);
        assert!(position.board.occupancies[Black] & white_knight_c4 == 0);
        assert!(position.board.occupancies[White] & white_knight_c4 != 0);

        position.make_move(mv, CapturesOnly);
        
        assert!(position.board.occupancies[Both] & black_bishop_d5 == 0);
        assert!(position.board.occupancies[Black] & black_bishop_d5 == 0);
        assert!(position.board.occupancies[White] & black_bishop_d5 == 0);
        
        assert!(position.board.occupancies[White] & white_knight_c4 == 0);
        let black_knight_c4 = 1u64 << C4 as u8;
        
        assert!(position.board.occupancies[White] & black_knight_c4 == 0);
        assert!(position.board.occupancies[Both] & black_knight_c4 != 0);
        
        position.undo_move(false);

        assert!(position.board.occupancies[Both] & black_bishop_d5 != 0);
        assert!(position.board.occupancies[Black] & black_bishop_d5 != 0);
        assert!(position.board.occupancies[White] & black_bishop_d5 == 0);


        assert!(position.board.occupancies[Both] & white_knight_c4 != 0);
        assert!(position.board.occupancies[Black] & white_knight_c4 == 0);
        assert!(position.board.occupancies[White] & white_knight_c4 != 0);

    }

    #[test]
    fn should_undo_an_enpassant_move() {
        let board = Board::parse_fen("rnbqk1nr/p2p3p/4p3/8/Pp1P1B2/6Pp/1PP2P1P/R2QKB1R b KQkq a3 0 1").unwrap();
        let zobrist_before_move = board.hash_key;

        let mv = Move::new(B4 as u8, A3 as u8, Enpassant);

        let mut position = Position::with(board);


        let black_pawn_b4 = 1u64 << B4 as u8;
        let white_pawn_a4 = 1u64 << A4 as u8;

        assert!(position.board.occupancies[Black] & white_pawn_a4 == 0);
        assert!(position.board.occupancies[White] & black_pawn_b4 == 0);
        assert!(position.board.occupancies[Black] & black_pawn_b4 != 0);
        assert!(position.board.occupancies[White] & white_pawn_a4 != 0);
        assert_eq!(position.board.enpassant, Some(A3));
        assert_eq!(position.board.turn, Black);

        position.make_move(mv, CapturesOnly);
        let zobrist_after_move = position.board.hash_key;

        let black_pawn_on_a3 = 1u64 << A3 as u8;
        assert!(position.board.occupancies[White] & white_pawn_a4 == 0);
        assert!(position.board.occupancies[Black] & black_pawn_b4 == 0);
        assert!(position.board.occupancies[Black] & black_pawn_on_a3 != 0);
        assert!(position.board.occupancies[White] & black_pawn_on_a3 == 0);
        assert_eq!(position.board.enpassant, None);
        assert_eq!(position.board.turn, White);
        assert_ne!(zobrist_before_move, zobrist_after_move);
        
        position.undo_move(false);
        let zobrist_after_undo = position.board.hash_key;
        assert!(position.board.occupancies[Black] & white_pawn_a4 == 0);
        assert!(position.board.occupancies[White] & black_pawn_b4 == 0);
        assert!(position.board.occupancies[Black] & black_pawn_b4 != 0);
        assert!(position.board.occupancies[White] & white_pawn_a4 != 0);
        assert_eq!(position.board.enpassant, Some(A3));
        assert_eq!(position.board.turn, Black);
        assert_ne!(zobrist_after_move, zobrist_after_undo);
        assert_eq!(zobrist_before_move, zobrist_after_undo);


        
    }

    #[test]
    fn should_be_able_to_undo_multiple_captures() {
        let board = Board::parse_fen("r3k1n1/4p1pp/8/1p1p1Q2/P2p1N2/4P3/5P1P/R3KBNR w KQkq - 0 1").unwrap();
        let mut position = Position::with(board);
        let white_knight_captures_pawn = Move::new(F4 as u8, D5 as u8, Capture);

        assert!(position.board.occupancies[Both] & (1u64 << F4 as u8) != 0);
        assert!(position.board.occupancies[Both] & (1u64 << D5 as u8) != 0);
        assert!(position.board.occupancies[White] & (1u64 << F4 as u8) != 0);
        assert!(position.board.occupancies[Black] & (1u64 << D5 as u8) != 0);

        position.make_move(white_knight_captures_pawn, CapturesOnly);

        assert!(position.board.occupancies[Both] & (1u64 << F4 as u8) == 0);
        assert!(position.board.occupancies[Black] & (1u64 << D5 as u8) == 0);
        assert!(position.board.occupancies[White] & (1u64 << D5 as u8) != 0);
        assert!(position.board.occupancies[White] & (1u64 << F4 as u8) == 0);

        let black_pawn_captures_pawn = Move::new(B5 as u8, A4 as u8, Capture);

        assert!((position.board.occupancies[Black] & 1u64 << B5 as u8) != 0);
        assert!((position.board.occupancies[White] & 1u64 << A4 as u8) != 0);

        position.make_move(black_pawn_captures_pawn, CapturesOnly);

        assert!((position.board.occupancies[Black] & 1u64 << B5 as u8) == 0);
        assert!((position.board.occupancies[White] & 1u64 << A4 as u8) == 0);
        assert!((position.board.occupancies[Black] & 1u64 << A4 as u8) != 0);

        assert!((position.board.occupancies[Black] & 1u64 << A4 as u8) != 0);
        assert!((position.board.occupancies[White] & 1u64 << A1 as u8) != 0);

        let white_rook_captures = Move::new(A1 as u8, A4 as u8, Capture);

        position.make_move(white_rook_captures, CapturesOnly);

        assert!((position.board.occupancies[Black] & 1u64 << A4 as u8) == 0);
        assert!((position.board.occupancies[White] & 1u64 << A1 as u8) == 0);
        assert!((position.board.occupancies[White] & 1u64 << A4 as u8) != 0);

        position.undo_move(false);
        position.undo_move(false);

        assert!((position.board.occupancies[Black] & 1u64 << B5 as u8) != 0);
        assert!((position.board.occupancies[White] & 1u64 << A4 as u8) != 0);
        
        position.undo_move(false);

        assert!(position.board.occupancies[Both] & (1u64 << F4 as u8) != 0);
        assert!(position.board.occupancies[Both] & (1u64 << D5 as u8) != 0);
        assert!(position.board.occupancies[White] & (1u64 << F4 as u8) != 0);
        assert!(position.board.occupancies[Black] & (1u64 << D5 as u8) != 0);
    }
}

