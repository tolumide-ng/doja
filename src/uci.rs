use std::{cell::RefCell, io::{self, stdout, Stdout, Write}, rc::Rc, str::SplitWhitespace, sync::{Arc, Mutex}, thread, time::Instant};

use thiserror::Error;

use crate::{bit_move::BitMove, board::{board::Board, board_state::BoardState, fen::FEN}, color::Color, constants::{ALPHA, BETA, START_POSITION}, move_type::MoveType, search::{control::{self, Control}, negamax::NegaMax, time_control::TimeControl}};


// #[derive(Debug, Error)]
// pub enum UCIError {
//     InvalidFen
// }


#[derive(Error, Debug)]
pub enum UciError {
    #[error("FenError: {0} is invalid")]
    FenError(String),
    #[error("MoveError: {0}")]
    InvalidMoveError(String),
    #[error("Expected integer but got: {0}")]
    InvalidIntegerArgument(String)
}

#[derive(Debug)]
pub(crate) struct UCI { board: Option<BoardState>, controller: Arc<Mutex<Control>> }

impl Default for UCI {
    fn default() -> Self {
        Self { board: None, controller: Arc::new(Mutex::new(Control::default())) }
    }
}

impl UCI {
    pub(crate) fn update_board_to(&mut self, board: BoardState) {
        self.board = Some(board);
        self.controller = Arc::new(Mutex::new(Control::default()));
    }

    pub(crate) fn update_controller(&mut self, control: Control) {
        self.controller = Arc::new(Mutex::new(control));
    }

    pub(crate) fn reader(&mut self) -> std::io::Result<()> {
        loop {
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer).expect("Failed to read line");

            let mut input = buffer.trim().split_whitespace();

            match input.next() {
                Some("position") => {
                    match self.parse_position(input) {
                        Ok(Some(board)) => {
                                write!(stdout(), "{}", board.to_string())?;
                                self.update_board_to(board);
                        }
                        Ok(None) => {}
                        Err(e) => {
                            write!(stdout(), "{}", e)?;
                        }
                    }
                }
                Some("ucinewgame") => {
                    self.update_board_to(BoardState::parse_fen(START_POSITION).unwrap());
                    write!(stdout(), "{}", self.board.as_ref().unwrap().to_string())?;
                }
                Some("go") => {
                    match self.parse_go(input) {
                        Ok(control) if self.board.is_some() => {
                            println!("the received controller is -------- {}", control.depth());
                            self.update_controller(control);
                            println!("the newly saved controller has a depth of {}", self.controller.lock().unwrap().depth());
                            let controller = Arc::clone(&self.controller);
                            let board = self.board.clone().unwrap();
                            thread::spawn(move || {
                                let depth = controller.lock().unwrap().depth();
                                NegaMax::run(controller, ALPHA, BETA, depth, &board);
                                println!("done done >>>>");
                                write!(stdout(), "{}", board.to_string()).unwrap();
                            });
                         }
                        Err(e) => {write!(stdout(), "{}", e)?;}
                        _ => {}
                    }
                }
                Some("quit") => { break; }
                Some("isready") => {stdout().write(b"uci ok")?; continue}
                Some("uci") => { 
                for data in Self::identify() {
                        writeln!(stdout(), "{}", data)?;
                    }
                }
                Some("d") => {writeln!(stdout(), "{}", self.board.as_ref().unwrap().to_string())?;},
                Some("stop") => {
                    // self.quit(); println!("told to quit")
                    self.controller.lock().as_mut().unwrap().stop();
                },
                Some("stop") => {
                    break;
                },
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

    fn parse_move(board: &BoardState, mv: &str) -> Option<BitMove> {
        let board_moves = board.gen_movement();

        for bmove in board_moves {
            if bmove.to_string().trim() == mv.trim() {
                return Some(bmove)
            }
        }

        
        None
    }

    fn parse_position(&self, mut input: SplitWhitespace) -> Result<Option<BoardState>, UciError> {
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


    fn parse_go(&self, mut input: SplitWhitespace) -> Result<Control, UciError> {
        let mut controller = Control::default();
        let b = self.board.as_ref().unwrap();

        match input.next() {
            // search until the "stop" command. Do not exit the search without being told so in this mode!
            Some("infinite") => {
                println!("infinite >>>>>>>>>>.");
            },
            Some("searchmoves") => {},
            // black increment per move in mseconds if x > 0
            Some("binc") if b.turn == Color::Black => {
                controller.set_inc(input.next().and_then(|x| u32::from_str_radix(x, 10).ok()).unwrap_or(controller.inc()));
            },
            // white increment per move in mseconds if x > 0
            Some("winc") if b.turn == Color::White => {
                controller.set_inc(input.next().and_then(|x| u32::from_str_radix(x, 10).ok()).unwrap_or(controller.inc()));
            },
            // white has x msec left on the clock
            Some("wtime") if b.turn == Color::White => {
                controller.set_time(input.next().and_then(|x| u128::from_str_radix(x, 10).ok()).unwrap_or(controller.time()));
            },
            Some("btime") if b.turn == Color::White => {
                controller.set_time(input.next().and_then(|x| u128::from_str_radix(x, 10).ok()).unwrap_or(controller.time()));
            },
            Some("movestogo") => {
                controller.set_movestogo(input.next().and_then(|x| u32::from_str_radix(x, 10).ok()).unwrap_or(controller.movestogo()));
            },
            Some("movetime") => {
                // amount of time allowed to spend making a move
                controller.set_movetime(input.next().and_then(|x| u128::from_str_radix(x, 10).ok()).unwrap_or(controller.movetime()));
            },
            Some("depth") => {
                controller.set_depth(input.next().and_then(|x| u8::from_str_radix(x, 10).ok()).unwrap_or(controller.depth()));
            },
            _ => {}
        }



        if controller.movetime() > 0 {
            controller.set_time(controller.movetime());
            controller.set_movestogo(1);
        }

        
        // let mut timeset = false;
        // almost impossible to complete...
        if controller.depth() == 0 {
            // controller.set_depth(64);
        }

        controller.setup_timerange();

        Ok(controller)
    }

}