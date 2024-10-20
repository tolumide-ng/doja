// use std::{num::NonZero, sync::{Arc, Mutex}, thread};

// use crate::{board::position::Position, search::{alpha_beta::NegaMax, control::Control}, syzygy::probe::TableBase, tt::table::TTable};


// pub(crate) struct Scale;

// impl Scale {
//     pub fn run(threads_num: NonZero<usize>, board: &mut Position, depth: usize) {
//         let controller = Arc::new(Mutex::new(Control::default()));

//         let table = TTable::default();
//         let tb = TableBase::default();
    
//         let mut negamax_thread = (0..threads_num.into()).map(|i| NegaMax::new(controller.clone(), table.get(), i)).collect::<Vec<_>>();
        

//         match threads_num.into() {
//             0 => negamax_thread[0].iterative_deepening(depth as u8, board, &tb),
//             _ => thread::scope(|s| {
//                     for td in negamax_thread.iter_mut() {
//                         let mut bb = board.clone();
//                         s.spawn(move || {
//                             td.iterative_deepening(depth as u8, &mut bb, &tb);
//                         });
//                     }
//                 })
//         }
//     }
// }