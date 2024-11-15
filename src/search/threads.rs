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

use crate::{board::position::Position, constants::MAX_PLY, move_logic::bitmove::Move, tt::{flag::HashFlag, table::TTable, tpt::TPT}};

use super::{heuristics::{capture_history::CaptureHistory, continuation_history::ContinuationHistory, countermove::CounterMove, history::HistoryHeuristic, killer_moves::KillerMoves, pv::PVTable}, stack::{Stack, StackItem}};

pub(crate) struct Thread<'a> {
    pub(crate) ss: [StackItem; MAX_PLY + 10],
    
    pub(crate) eval: i32,
    pub(crate) depth: usize,
    // In the case of depth limited search, this is provided by the client
    limit: u8,
    nodes: usize,
    // ply: usize, // already on the board

    pub(crate) history_table: HistoryHeuristic,
    caphist: CaptureHistory,
    conthist: ContinuationHistory,
    counter_mvs: CounterMove,
    /// The Killer Move is a quiet move which caused a beta-cutoff in a sibling Cut-node,
    killer_moves: KillerMoves,
    tt: TPT<'a>,
    
    pv_table: PVTable,
    thread_id: usize
}

impl<'a> Thread<'a> {
    pub(crate) fn new(limit: u8, tt: TPT<'a>, thread_id: usize) -> Self {
        Self { ss: [StackItem::default(); MAX_PLY + 10], 
            eval: 0, depth: 0, limit, nodes: 0,
            history_table: HistoryHeuristic::new(), caphist: CaptureHistory::default(), 
            conthist: ContinuationHistory::new(), counter_mvs: CounterMove::new(), 
            killer_moves: KillerMoves::new(), tt, pv_table: PVTable::default(), thread_id }
    }

    pub(crate) fn update_stats(&mut self, position: &Position, best_mv: &Option<Move>, quiets: &Vec<Move>, captures: &Vec<(Move, HashFlag)>, depth: u8) {
        self.caphist.update_many(&position, depth, captures);

        if best_mv.is_some_and(|m| m.is_quiet()) {
            self.conthist.update_many(&position, &quiets, depth, best_mv);
            self.counter_mvs.add_many(&position, quiets);
        }
    }


}