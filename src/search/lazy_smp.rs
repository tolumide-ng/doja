use std::{num::NonZero, sync::{Arc, Mutex}, thread};

use crate::{board::{fen::FEN, position::Position, state::board::Board}, constants::TRICKY_POSITION, search::{alpha_beta::NegaMax, control::Control}, syzygy::probe::TableBase, tt::table::TTable};


pub(crate) struct LazySMP;

impl LazySMP {
    pub fn run(threads_num: usize, board: Position) {
        let board = Position::with(Board::parse_fen(TRICKY_POSITION).unwrap());
        // println!("**********************BEFORE*****************************");
        println!("{}", board.to_string());
        
        let bb = board.clone();
        // NegaMax::run(controller, &tt, 1, &mut board);
        
        println!("num of cpus {:?}", std::thread::available_parallelism().unwrap_or(NonZero::<usize>::new(1).unwrap()));
        // let tt = TTable::default();
        let controller = Arc::new(Mutex::new(Control::default()));
        let board = Position::with(Board::parse_fen(TRICKY_POSITION).unwrap());
        let threads = std::thread::available_parallelism().unwrap_or(NonZero::<usize>::new(1).unwrap()).get();
        let depth = 10;
        // let mut bb = board.clone();
        let table = TTable::default();
        let tb = TableBase::default();
    
        let mut negamax_thread = (0..threads).map(|i| NegaMax::new(controller.clone(), table.get(), i)).collect::<Vec<_>>();
        
    
        thread::scope(|s| {
            for td in negamax_thread.iter_mut() {
                let mut bb = board.clone();
                s.spawn(move || {
                    td.iterative_deepening(7, &mut bb, &tb);
                });
            }
        });
    }
}