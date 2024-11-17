use std::{io::{stdout, Write}, str::SplitWhitespace, sync::{Arc, Mutex}, thread};

use thiserror::Error;

use crate::{board::{position::Position, state::board::Board}, color::Color, constants::START_POSITION, move_logic::{bitmove::Move, move_stack::MoveStack}, move_scope::MoveScope, search::control::Control, syzygy::probe::TableBase, tt::table::TTable};

#[cfg(test)]
#[path = "./uci.tests.rs"]
mod uci_tests;

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
pub(crate) struct UCI { position: Option<Position>, controller: Arc<Mutex<Control>>, tt: TTable }

impl Default for UCI {
    fn default() -> Self {
        Self { position: None, controller: Arc::new(Mutex::new(Control::default())), tt: TTable::default() }
    }
}

impl UCI {
    pub(crate) fn update_board_to(&mut self, board: Position) {
        self.position = Some(board);
        self.controller = Arc::new(Mutex::new(Control::default()));
    }

    pub(crate) fn update_controller(&mut self, control: Control) {
        self.controller = Arc::new(Mutex::new(control));
    }

    pub(crate) fn process_input<W: Write>(&mut self, input: String, mut writer: W) -> std::io::Result<bool> {
        let mut input = input.trim().split_whitespace();
        let tb = TableBase::default();
        let mut table = TTable::default();

        
        match input.next() {
            Some("position") => {
                match self.parse_position(input) {
                    Ok(Some(board)) => {
                            writeln!(writer, "{}", board.to_string())?;
                            self.update_board_to(board);
                    }
                    Ok(None) => {}
                    Err(e) => {
                        write!(writer, "{}", e)?;
                    }
                }
            }
            Some("ucinewgame") => {
                self.update_board_to(Position::with(Board::try_from(START_POSITION).unwrap()));
                write!(writer, "{}", self.position.as_ref().unwrap().to_string())?;
            }
            Some("go") => {
                match self.parse_go(input) {
                    Ok(control) if self.position.is_some() => {
                        println!("the received contro  ller is -------- {}", control.depth());
                        self.update_controller(control);
                        println!("the newly saved controller has a depth of {}", self.controller.lock().unwrap().depth());
                        let controller = Arc::clone(&self.controller);

                        table.increase_age();
                        let mut board = self.position.clone().unwrap();
                        
                        // thread::scope(|s| {
                            //     let mut bb = board.clone();
                            //     s.spawn(move || {
                                //         negamax[0].iterative_deepening(7, &mut bb);
                                //     });
                                // });
                                
                                let result = thread::spawn(move || {
                            // let mut negamax = (0..1).map(|i| NegaMax::new(controller.clone(), table.get(), i)).collect::<Vec<_>>();
                            // let depth = controller.lock().unwrap().depth();
                            // negamax[0].iterative_deepening(depth, &mut board, &tb);
                            // println!("done done >>>>");
                            // write!(writer, "{}", board.to_string()).unwrap();
                            board
                        }).join().unwrap();

                        write!(writer, "{}", result.to_string()).unwrap();


                     }
                    Err(e) => {write!(writer, "{}", e)?;}
                    _ => {}
                }
            }
            Some("quit") => { return  Ok(false); }
            Some("isready") => {writeln!(writer, "readyok")?;}
            Some("uci") => {
            for data in Self::identify() {
                    writeln!(writer, "{}", data)?;
                }
            }
            Some("d") => {writeln!(writer, "{}", self.position.as_ref().unwrap().to_string())?;},
            Some("stop") => {
                // self.quit(); println!("told to quit")
                self.controller.lock().as_mut().unwrap().stop();
                return Ok(false);
            },
            // Some("stop") => {
            //     return Ok(false);
            // },
            _ => {}
        };

        Ok(true)
    }

    pub(crate) fn reader(&mut self) -> std::io::Result<()> {
        loop {
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer).expect("Failed to read line");



            if !self.process_input(buffer, stdout())? {
                break;
            };
        }

        Ok(())
    }


    pub(crate) fn identify() -> [&'static str; 4] {
        ["id name: papa", "id author: Tolumide", "id email: tolumideshopein@gmail.com", "uciok"]
    }


    fn apply_moves_to_board(state: &mut Position, mut moves: SplitWhitespace) {
        // let mut p = state;
        while let Some(mv) = moves.next()  {
            // if let Some(b_move) = Self::parse_move(&b, mv) {
            //     b = b.make_move(b_move, MoveScope::AllMoves).unwrap();
            // }

            if let Some(b_move) = Self::parse_move(&state, mv) {
                // b = b.make_move(b_move, MoveScope::AllMoves).unwrap();
                state.make_move(b_move, MoveScope::AllMoves);
            }
        }
    }

    fn parse_move(board: &Position, mv: &str) -> Option<Move> {
        let mut board_moves = MoveStack::<Move>::new();
        board.gen_movement::<{ MoveScope::ALL }, Move>(&mut board_moves);

        for bmove in board_moves {
            if bmove.to_string().trim() == mv.trim() {
                return Some(bmove)
            }
        }

        
        None
    }

    fn parse_position(&self, mut input: SplitWhitespace) -> Result<Option<Position>, UciError> {
        match input.next() {
            Some("startpos") => {
                // create a startpos
                // let mut board = Board::parse_fen(START_POSITION).unwrap();
                let mut board_state = Position::with(Board::try_from(START_POSITION).unwrap());
                match input.next() {
                    Some("moves") => {
                        // loop through and apply the moves
                        Self::apply_moves_to_board(&mut board_state, input);
                        return Ok(Some(board_state))
                    }
                    _ => {}
                }
                // returns the created startpos
                return Ok(Some(board_state))
            }
            Some("fen") => {
                // read the provided fen (all the remaining string after the text 'fen')
                let remaning_input = input.into_iter().map(|s| format!("{s} ")).collect::<String>();
                

                // let ax = Board::try_from(&remaning_input).unwrap();
                match Board::try_from(remaning_input.as_str()) {
                    Ok(board) => {
                        let mut board_state = Position::with(board);
                        // split remaining string at 'moves' and apply the moves to the boardState derived from the parsed fen string
                        if let Some((_, moves)) = remaning_input.split_once("moves") {
                            // loop through and apply the moves
                            Self::apply_moves_to_board(&mut board_state, moves.split_whitespace());
                        }
                        return Ok(Some(board_state))
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
        let b = self.position.as_ref().unwrap();

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