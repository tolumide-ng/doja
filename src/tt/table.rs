use super::{entry::TTEntry, tpt::TPT};


/**
 * Transposition Table
 * https://web.archive.org/web/20071031100051/http://www.brucemo.com/compchess/programming/hashing.htm
 * 
 * keys:
 * PV -- means Principal Variation
 */


 /// 4MegaByte
//  pub(crate) const TOTAL_SIZE: usize = 0x400000;
// pub(crate) const TOTAL_SIZE: usize = 10 * 1024 * 1024 * 1024; // 300MB
pub(crate) const TOTAL_SIZE: usize = 0x10000; // 1MB





/// Transposition Table
#[derive(Debug)]
pub(crate) struct TTable {
    table: Vec<TTEntry>,
}




// const TT_ENTRY: Option<TTEntry> = None;
impl Default for TTable {
   fn default() -> Self {
        // let table: Box<[TTEntry; TOTAL_SIZE]> = Box::new(core::array::from_fn(|_| TTEntry::default()));
        // let max = TOTAL_SIZE / std::mem::size_of::<TTEntry>();
        let max = TOTAL_SIZE;
        // println!("SIZE OF IS {}", std::mem::size_of::<TTEntry>());
        let table = (0..max).map(|_| TTEntry::default()).collect::<Vec<_>>();
        Self { table, }
   }
}


impl TTable {
   pub(crate) fn get(&self) -> TPT {
        TPT { table: &self.table }
   }
}
