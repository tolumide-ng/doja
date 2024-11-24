use std::{io::{stdout, Write}, str::SplitWhitespace, sync::{atomic::{AtomicBool, Ordering}, Arc, Mutex}, thread};

use clock::Clock;
use counter::Counter;
use thiserror::Error;

pub(crate) mod clock;

use crate::{board::{position::Position, state::board::Board}, constants::START_POSITION, move_logic::{bitmove::Move, move_stack::MoveStack}, move_scope::MoveScope, search::{control::Control, search::Search, threads::Thread}, tt::table::TTable};

#[cfg(test)]
#[path = "./uci.tests.rs"]
mod uci_tests;
mod counter;

#[derive(Error, Debug, PartialEq)]
pub enum UciError {
    #[error("FenError: {0} is invalid")]
    FenError(String),
    #[error("MoveError: {0}")]
    InvalidMoveError(String),
    #[error("Expected integer but got: {0}")]
    InvalidIntegerArgument(String),
    #[error("No value received for key {0}")]
    NoValue(&'static str),
    #[error("Empty Argument")]
    EmptyArgument,
}

#[derive(Debug)]
pub(crate) struct UCI { position: Option<Position>, tt: TTable, options: Vec<(String, String)>, clock: Clock, stop: AtomicBool }

impl Default for UCI {
    fn default() -> Self {
        let stop = AtomicBool::new(false);
        let stop_ptr: *const AtomicBool = &stop;

        let clock = Clock::new(stop_ptr);
        Self { position: None, tt: TTable::default(), options: vec![], clock, stop }
    }
}

impl UCI {
    pub(crate) fn update_board_to(&mut self, board: Position) {
        self.position = Some(board);
    }

    pub(crate) fn process_input<W: Write>(&mut self, input: String, mut writer: W) -> std::io::Result<bool> {
        let mut input = input.trim().split_whitespace();
        // let tb = TableBase::default();
        let mut table = TTable::default();

        
        match input.next() {
            Some("position") => {
                match self.parse_position(input) {
                    Ok(Some(board)) => {
                            self.tt = TTable::default(); // we need to reset the Transposition table when we're handling a different position's data
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
                self.tt = TTable::default();
                self.options = vec![];
                write!(writer, "{}", self.position.as_ref().unwrap().to_string())?;
            }
            Some("go") => {
                match Counter::try_from(input) {
                    Ok(counter) if self.position.is_some() => {
                        let board = self.position.clone().unwrap(); // this would be fixed later
                        self.clock.set_limit(counter, board.turn);
                        self.tt.increase_age();
                        self.clock.start();
                        let thread = Thread::new(30, table.get(), 0); // SHOULD BE REMOVED LATER (THIS SERVES NO SERIOUS PURPOSE YET -0->> MORE LIKE A DUPLICATION)

                        let mut negamax = (0..2).map(|_i| Search::new(self.tt.get(), self.clock.clone())).collect::<Vec<_>>();
                        thread::scope(|s| {

                            for negamax in negamax.iter_mut() {
                                let mut bb = board.clone();
                                let mut th = thread.clone();
                                s.spawn(move || {
                                    negamax.iterative_deepening(10, &mut bb, &mut th);
                                });
                            }
                        });
                    }
                    Err(e) => {write!(writer, "{}", e)?;}
                    _ => {}
                };
                
                // match self.parse_go(input) {
                //     Ok(control) if self.position.is_some() => {
                //         // println!("the received contro  ller is -------- {}", control.depth());
                //         // self.update_controller(control);
                //         // println!("the newly saved controller has a depth of {}", self.controller.lock().unwrap().depth());
                //         // let controller = Arc::clone(&self.controller);

                //         table.increase_age();
                //         let mut board = self.position.clone().unwrap();
                        
                //         // thread::scope(|s| {
                //             //     let mut bb = board.clone();
                //             //     s.spawn(move || {
                //                 //         negamax[0].iterative_deepening(7, &mut bb);
                //                 //     });
                //                 // });
                                
                //                 let result = thread::spawn(move || {
                //             // let mut negamax = (0..1).map(|i| NegaMax::new(controller.clone(), table.get(), i)).collect::<Vec<_>>();
                //             // let depth = controller.lock().unwrap().depth();
                //             // negamax[0].iterative_deepening(depth, &mut board, &tb);
                //             // println!("done done >>>>");
                //             // write!(writer, "{}", board.to_string()).unwrap();
                //             board
                //         }).join().unwrap();

                //         write!(writer, "{}", result.to_string()).unwrap();


                //     }
                //     Err(e) => {write!(writer, "{}", e)?;}
                //     _ => {}
                // }
            }
            Some("quit") => { 
                self.stop.store(true, Ordering::SeqCst);
                return Ok(false);
             }
            Some("isready") => {writeln!(writer, "readyok")?;}
            Some("uci") => {
            for data in Self::identify() {
                    writeln!(writer, "{}", data)?;
                }
            }
            Some("d") => {writeln!(writer, "{}", self.position.as_ref().unwrap().to_string())?;},
            Some("stop") => {
                self.stop.store(true, Ordering::SeqCst);
                return Ok(false);
            },
            Some("setoption") => {
                if input.next() == Some("name") {
                    let mut option_name = String::new();
                    let mut option_value = String::new();

                    let mut error_found = false;
                    let mut found_value_str = false;

                    while let Some(value) = input.next() {
                        if (value == "value" && found_value_str) || (value == "name") {
                            error_found = true;
                            break;
                        }
                        if value == "value" { found_value_str = true; continue; }

                        if !found_value_str {
                            option_name = format!("{} {}", option_name, value);
                            let _ = option_name.trim();
                        }

                        if found_value_str {
                            option_value = format!("{} {}", option_value, value);
                            let _ = option_value.trim();

                        }
                    }

                    if !error_found {
                        self.options.push((option_name, option_value));
                    }
                    
                    writeln!(writer, "error found! Please ensure that the provided 'name' and 'value' match the UCI name/value recommendations")?;
                }
            }
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
}