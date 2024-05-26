use std::{cell::RefCell, io::{self, stdout, Stdout, Write}, rc::Rc, str::SplitWhitespace};

use thiserror::Error;

use crate::{bit_move::BitMove, board::{board_state::BoardState, fen::FEN}, constants::START_POSITION, move_type::MoveType};


// #[derive(Debug, Error)]
// pub enum UCIError {
//     InvalidFen
// }


#[derive(Error, Debug)]
pub enum UciError {
    #[error("FenError: {0} is invalid")]
    FenError(String),
    #[error("MoveError: {0}")]
    InvalidMoveError(String)
}

pub(crate) struct UCI;

impl UCI {
    pub(crate) fn reader() -> std::io::Result<()> {
        // global board
        let global_board: Rc<RefCell<Option<BoardState>>> = Rc::new(RefCell::new(None));


        loop {
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer).expect("Failed to read line");

            let mut input = buffer.trim().split_whitespace();

            match input.next() {
                Some("position") => {
                    match Self::parse_position(input) {
                        Ok(result) => {
                            if let Some(board) = result {
                                write!(stdout(), "{}", board.to_string())?;
                                global_board.as_ref().replace(Some(board));

                            }
                        }
                        Err(e) => {
                            write!(stdout(), "{}", e)?;
                        }
                    }
                }
                Some("ucinewgame") => {
                    global_board.as_ref().replace(Some(BoardState::parse_fen(START_POSITION).unwrap()));
                    write!(stdout(), "{}", global_board.as_ref().borrow().as_ref().unwrap().to_string())?;
                }
                Some("go") => {
                    if let Some(c_board) = global_board.borrow_mut().as_mut() {
                        match Self::parse_go(c_board, input) {
                            Ok(result) => {
                                if let Some(board) = result {
                                    write!(stdout(), "{}", board.to_string())?;
                                    *c_board = board;
                                }
                            }
                            Err(e) => {
                                write!(stdout(), "{}", e)?;
                            }
                        }
                    }
                }
                Some("quit") => { break; }
                Some("isready") => {stdout().write(b"uci ok")?; continue}
                Some("uci") => { 
                    for data in Self::identify() {
                        writeln!(stdout(), "{}", data)?;
                    }
                }
                _ => {}
            };
        }

        Ok(())
    }


    fn identify() -> [&'static str; 4] {
        ["id name: papa", "id author: Tolumide", "id email: tolumideshopein@gmail.com", ""]
    }


    fn apply_moves_to_board(board: BoardState, mut moves: SplitWhitespace) -> BoardState {
        let mut b = board;
        while let Some(mv) = moves.next()  {
            if let Some(b_move) = Self::parse_move(&b, mv) {
                b = b.make_move(b_move, MoveType::AllMoves).unwrap();
            }
        }

        b
    }

    pub(crate) fn parse_move(board: &BoardState, mv: &str) -> Option<BitMove> {
        let board_moves = board.gen_movement();

        for bmove in board_moves {
            if bmove.to_string().trim() == mv.trim() {
                return Some(bmove)
            }
        }

        
        None
    }

    fn parse_position(mut input: SplitWhitespace) -> Result<Option<BoardState>, UciError> {
        match input.next() {
            Some("startpos") => {
                // create a startpos
                let mut board = BoardState::parse_fen(START_POSITION).unwrap();
                match input.next() {
                    Some("moves") => {
                        // loop through and apply the moves
                        board = Self::apply_moves_to_board(board, input);
                        return Ok(Some(board))
                    }
                    _ => {}
                }
                // returns the created startpos
                return Ok(Some(board))
            }
            Some("fen") => {
                // read the provided fen (all the remaining string after the text 'fen')
                let remaning_input = input.into_iter().map(|s| format!("{s} ")).collect::<String>();

                match BoardState::parse_fen(&remaning_input) {
                    Ok(mut board) => {
                        println!("should be fine");
                        // split remaining string at 'moves' and apply the moves to the boardState derived from the parsed fen string
                        if let Some((_, moves)) = remaning_input.split_once("moves") {
                            // loop through and apply the moves
                            board = Self::apply_moves_to_board(board, moves.split_whitespace());
                        }
                        return Ok(Some(board))
                    }
                    Err(e) => { return Err(UciError::FenError(e.to_string())) }
                }
            }
            _ => {}
        }

        Ok(None)
    }


    fn parse_go(board: &BoardState, mut input: SplitWhitespace) -> Result<Option<BoardState>, UciError> {
        match input.next() {
            Some("searchmoves") => {},
            Some("perft") | Some("depth") => {
                return Ok(Some(board.clone())) // to be changed
            },
            _ => {}
        }
        Ok(None)
    }
}