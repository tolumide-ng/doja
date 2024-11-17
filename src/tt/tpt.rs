use std::sync::atomic::Ordering;

use crate::move_logic::bitmove::Move;

use super::{entry::{to_tt, TTData, TTEntry}, flag::HashFlag, table::TOTAL_SIZE};

/// TPT => Transposition Table (derived)
/// This is the exposed derivation of the original Transposition Table (TT)
#[derive(Debug)]
pub(crate) struct TPT<'a> {
    pub(crate) table : &'a [TTEntry],
    pub(crate) age: u8
}


impl<'a> TPT<'a> {
    const  TT_REPLACE_OFFSET: u8 = 4;

    pub(crate) fn record(&self, zobrist_key: u64, depth: u8, score: i32, eval: i32, ply: usize, flag: HashFlag, mv: Option<Move>, pv: bool) {
        let age = 0;
        // let index = zobrist_key & (TOTAL_SIZE as u64 -1);
        let index = zobrist_key as usize % TOTAL_SIZE;
        let old =  TTData::from(self.table[index].smp_data.load(Ordering::Relaxed));
        let mut best_mv = mv;


        if self.age != old.age || old.key != zobrist_key ||  depth + Self::TT_REPLACE_OFFSET + 2 * u8::from(pv) > old.depth || flag == HashFlag::Exact {
            if old.mv.is_some_and(|_| best_mv.is_none()) {
                best_mv = old.mv;
            }

            unsafe {
                let ptr = self.table.as_ptr();
                (*ptr.add(index)).write(zobrist_key, age, depth, to_tt(score, ply), eval as i16, best_mv, flag);
           }
        }
    }

    pub(crate) fn probe(&self, zobrist_key: u64) -> Option<TTData> { 
        // let index = zobrist_key & (TOTAL_SIZE as u64 -1);
        let index = zobrist_key as usize % TOTAL_SIZE;
        let entry = &self.table[index];
        // we can turst the #[default] implementation to work without any issue because the default key is 0,
        // and that would likely not match any zobtist key

        let atomic_data = entry.smp_data.load(Ordering::Relaxed);
        if atomic_data == 0 {return None};

        let data = TTData::from(atomic_data);
        if zobrist_key ^ u64::from(data) == entry.smp_key.load(Ordering::Relaxed) {
            return Some(data)
        }

        None
    }
}