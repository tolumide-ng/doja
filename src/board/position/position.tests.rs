#[cfg(test)]
mod do_and_undo_a_move {
    use crate::board::position::Position;
    // use crate::color::Color::*;
    // use crate::squares::Square::*;
    // use crate::bit_move::BitMove;
    // use crate::board::piece::Piece::*;
    
    use crate::board::state::board_state::BoardState;
    use crate::board::piece::Piece::*;
    use crate::color::Color::*;
    use crate::bit_move::BitMove;
    use crate::board::fen::FEN;
    use crate::move_type::MoveType::*;
    use crate::squares::Square::*;


    #[test]
    fn should_update_self_after_move() {
        let board = BoardState::parse_fen("4kb2/3p1n2/1r2p3/3b1p1p/PpN5/8/BP1P3P/R2QK3 b -Q-- - 0 2").unwrap();
        let mv = BitMove::new(D5 as u32, C4 as u32, BB, None, true, false, false, false);
        let mut position = Position::with(board);

        let black_bishop_d5 = 1u64 << D5 as u64;
        assert!(position.board.occupancies[Both] & black_bishop_d5 != 0);
        assert!(position.board.occupancies[Black] & black_bishop_d5 != 0);
        assert!(position.board.occupancies[White] & black_bishop_d5 == 0);
        let white_knight_c4 = 1u64 << C4 as u64;
        assert!(position.board.occupancies[Both] & white_knight_c4 != 0);
        assert!(position.board.occupancies[Black] & white_knight_c4 == 0);
        assert!(position.board.occupancies[White] & white_knight_c4 != 0);

        println!("{:#?}", position.board.to_string());

        position.make_move(mv, CapturesOnly);

        println!("{:#?}", position.board.to_string());

        assert!(position.board.occupancies[Both] & black_bishop_d5 == 0);
        assert!(position.board.occupancies[Black] & black_bishop_d5 == 0);
        assert!(position.board.occupancies[White] & black_bishop_d5 == 0);

        assert!(position.board.occupancies[White] & white_knight_c4 == 0);
        let black_knight_c4 = 1u64 << C4 as u64;

        assert!(position.board.occupancies[White] & black_knight_c4 == 0);
        assert!(position.board.occupancies[Both] & black_knight_c4 != 0);
    }


    #[test]
    fn should_be_able_to_undo_a_regular_capture() {
        let board = BoardState::parse_fen("4kb2/3p1n2/1r2p3/3b1p1p/PpN5/8/BP1P3P/R2QK3 b -Q-- - 0 2").unwrap();
        let mv = BitMove::new(D5 as u32, C4 as u32, BB, None, true, false, false, false);

        let mut position = Position::with(board);

        let black_bishop_d5 = 1u64 << D5 as u64;
        assert!(position.board.occupancies[Both] & black_bishop_d5 != 0);
        assert!(position.board.occupancies[Black] & black_bishop_d5 != 0);
        assert!(position.board.occupancies[White] & black_bishop_d5 == 0);
        let white_knight_c4 = 1u64 << C4 as u64;
        assert!(position.board.occupancies[Both] & white_knight_c4 != 0);
        assert!(position.board.occupancies[Black] & white_knight_c4 == 0);
        assert!(position.board.occupancies[White] & white_knight_c4 != 0);

        position.board.make_move(mv, CapturesOnly);
        
        assert!(position.board.occupancies[Both] & black_bishop_d5 == 0);
        assert!(position.board.occupancies[Black] & black_bishop_d5 == 0);
        assert!(position.board.occupancies[White] & black_bishop_d5 == 0);
        
        assert!(position.board.occupancies[White] & white_knight_c4 == 0);
        let black_knight_c4 = 1u64 << C4 as u64;
        
        assert!(position.board.occupancies[White] & black_knight_c4 == 0);
        assert!(position.board.occupancies[Both] & black_knight_c4 != 0);
        
        position.undo_move();
        println!("{:#?}", position.board.to_string());
        assert!(position.board.occupancies[Both] & black_bishop_d5 != 0);
        assert!(position.board.occupancies[Black] & black_bishop_d5 != 0);
        assert!(position.board.occupancies[White] & black_bishop_d5 == 0);


        assert!(position.board.occupancies[Both] & white_knight_c4 != 0);
        assert!(position.board.occupancies[Black] & white_knight_c4 == 0);
        assert!(position.board.occupancies[White] & white_knight_c4 != 0);

    }

    #[test]
    fn should_undo_an_enpassant_move() {
        let mut board = BoardState::parse_fen("rnbqk1nr/p2p3p/4p3/8/Pp1P1B2/6Pp/1PP2P1P/R2QKB1R b KQkq a3 0 1").unwrap();
        let zobrist_before_move = board.hash_key;

        let mv = BitMove::new(B4 as u32, A3 as u32, BP, None, true, false, true, false);

        let mut position = Position::with(board);


        let black_pawn_b4 = 1u64 << B4 as u64;
        let white_pawn_a4 = 1u64 << A4 as u64;

        assert!(position.board.occupancies[Black] & white_pawn_a4 == 0);
        assert!(position.board.occupancies[White] & black_pawn_b4 == 0);
        assert!(position.board.occupancies[Black] & black_pawn_b4 != 0);
        assert!(position.board.occupancies[White] & white_pawn_a4 != 0);
        assert_eq!(position.board.enpassant, Some(A3));
        assert_eq!(position.board.turn, Black);

        position.make_move(mv, CapturesOnly);
        let zobrist_after_move = position.board.hash_key;

        let black_pawn_on_a3 = 1u64 << A3 as u64;
        assert!(position.board.occupancies[White] & white_pawn_a4 == 0);
        assert!(position.board.occupancies[Black] & black_pawn_b4 == 0);
        assert!(position.board.occupancies[Black] & black_pawn_on_a3 != 0);
        assert!(position.board.occupancies[White] & black_pawn_on_a3 == 0);
        assert_eq!(position.board.enpassant, None);
        assert_eq!(position.board.turn, White);
        assert_ne!(zobrist_before_move, zobrist_after_move);
        
        position.undo_move();
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
}

