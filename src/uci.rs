use std::str::SplitWhitespace;

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
    pub(crate) fn reader() {
        // global board
        let mut g_board: Option<BoardState> = None;

        loop {
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer).expect("Failed to read line");

            let mut input = buffer.trim().split_whitespace();

            match (input.next(), input.next()) {
                (Some("position"), Some("startpos")) => {
                    g_board = None;
                    let mut board = BoardState::parse_fen(START_POSITION).unwrap();

                    match input.next() {
                        Some("moves") => {
                            // loop through and apply the moves
                            board = Self::apply_moves_to_board(board, input);
                            println!("{}", board.to_string());
                        }
                        _ => {
                            // create a startpos
                            println!("{}", board.to_string());
                        }
                    }

                    g_board = Some(board);
                }
                (Some("position"), Some("fen")) => {
                    // read the provided fen
                    let remaning_input = input.into_iter().map(|s| format!("{s} ")).collect::<String>();

                    g_board = None;
                    match BoardState::parse_fen(&remaning_input) {
                        Ok(board) => {
                            // while remaning_input.split_whitespace().next() != Some("move") {}
                            if let Some((_, moves)) = remaning_input.split_once("moves") {
                                let board = Self::apply_moves_to_board(board, moves.split_whitespace());
                                println!("{}", board.to_string());
                                g_board = Some(board);
                            }
                        }
                        Err(e) => {println!("{}", UciError::FenError(e.to_string()));}
                    }
                }
                (Some("go"), Some("perft")) => {
                    if let Some(ref board) = g_board {
                        println!("{}", board.to_string());
                    }
                    // match input.next() {}
                }
                _ => {}
            };
        }
    }


    pub(crate) fn apply_moves_to_board(board: BoardState, mut moves: SplitWhitespace) -> BoardState {
        let mut b = board;
        while let Some(mv) = moves.next()  {
            if let Some(b_move) = Self::parse(&b, mv) {
                b = b.make_move(b_move, MoveType::AllMoves).unwrap();
            }
        }

        b
    }

    pub(crate) fn parse(board: &BoardState, mv: &str) -> Option<BitMove> {
        let board_moves = board.gen_movement();

        for bmove in board_moves {
            if bmove.to_string().trim() == mv.trim() {
                return Some(bmove)
            }
        }

        
        None
    }

    pub(crate) fn parse_position() {}
}