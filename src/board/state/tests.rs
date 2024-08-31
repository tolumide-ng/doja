#[cfg(test)]
mod board_state_tests {
    use crate::{board::{castling::Castling, state::board_state::BoardState}, color::Color, constants::{RANK_7, RANK_8}, squares::Square};

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
}