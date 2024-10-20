#[cfg(test)]
mod uci_tests {
    use std::io::Cursor;

    use crate::{board::state::board::Board, constants::TRICKY_POSITION, uci::UCI};

    #[test]
    fn should_return_author_identity() {
        let identity = UCI::identify();
        assert_eq!(identity[0], "id name: papa");
        assert_eq!(identity[1], "id author: Tolumide");
        assert_eq!(identity[2], "id email: tolumideshopein@gmail.com");
        assert_eq!(identity[3], "uciok");
    }

    #[test]
    fn should_respond_to_the_uciok_cmd() {
        let mut cursor = Cursor::new(Vec::new());
        
        let _ = UCI::default().process_input(String::from("uci"), &mut cursor);
        let output = String::from_utf8(cursor.get_ref()[..].to_vec()).unwrap();

        let result = output.split("\n").collect::<Vec<_>>();
        
        assert_eq!(result[0], "id name: papa");
        assert_eq!(result[1], "id author: Tolumide");
        assert_eq!(result[2], "id email: tolumideshopein@gmail.com");
        assert_eq!(result[3], "uciok");
    }

    #[test]
    fn should_reply_to_the_isready_cmd() {
        let mut cursor = Cursor::new(Vec::new());

        let _ = UCI::default().process_input(String::from("isready"), &mut cursor);

        let result = String::from_utf8(cursor.get_ref()[..].to_vec()).unwrap();
        assert_eq!(result, "readyok\n");
    }


    #[test]
    fn should_return_startposition_when_called_with_poosition_startpos() {
        let mut cursor = Cursor::new(Vec::new());

        let _ = UCI::default().process_input(String::from("position startpos"), &mut cursor);

        let result = String::from_utf8(cursor.get_ref()[..].to_vec()).unwrap();
        let expected = r#"
  8  ♜  ♞  ♝  ♛  ♚  ♝  ♞  ♜ 
  7  ♟  ♟  ♟  ♟  ♟  ♟  ♟  ♟ 
  6  .  .  .  .  .  .  .  . 
  5  .  .  .  .  .  .  .  . 
  4  .  .  .  .  .  .  .  . 
  3  .  .  .  .  .  .  .  . 
  2  ♙  ♙  ♙  ♙  ♙  ♙  ♙  ♙ 
  1  ♖  ♘  ♗  ♕  ♔  ♗  ♘  ♖ 
    
     a  b  c  d  e  f  g  h

    Side:       White
    Enpass:     None
    Castling:   KQkq
    Hashkey:    e0ac430339c6fb3e
        "#;
        assert_eq!(expected.trim(), result.trim());
    }

    #[test]
    fn should_return_the_board_of_the_provided_fen_string() {
        let mut cursor = Cursor::new(Vec::new());

        let _ = UCI::default().process_input(format!("position fen {TRICKY_POSITION}"), &mut cursor);

        let result = String::from_utf8(cursor.get_ref()[..].to_vec()).unwrap();

        let expected = Board::new().to_string();
        assert_eq!(expected.trim(), result.trim());
    }
}