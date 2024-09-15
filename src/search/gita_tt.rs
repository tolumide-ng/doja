use crate::bit_move::Move;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HashFlag {
    #[default]
    Exact =0, 
    Alpha, 
    Beta
}


#[derive(Debug, Clone, Copy)]
pub(crate) struct TTEntry {
    key: u64, depth: usize, flags: HashFlag, value: i32, best: Option<Move>
}


const MAX_SIZE: usize = 500;

#[derive(Debug, Clone)]
pub(crate) struct TTable {
    table: [TTEntry; MAX_SIZE],
    entries: usize,
}



impl TTable {
    pub(crate) fn probe(&self, depth: usize, alpha: i32, beta: i32, zobrist: u64) -> Option<i32> {
        let index = (zobrist as usize) % self.entries;
        let phashe = &self.table[index];

        if phashe.key == zobrist {
            if phashe.depth >= depth {
                if phashe.flags == HashFlag::Exact { return Some(phashe.value); }
                if phashe.flags == HashFlag::Alpha && phashe.value <= alpha { return Some(alpha); }
                if phashe.flags == HashFlag::Beta && phashe.value >= beta { return Some(beta); }
            }

            // return u32::from(phashe.best.unwrap()) as i32
        }
        return None
    }

    pub(crate) fn record(&mut self, depth: usize, value: i32, zobrist: u64, flag: HashFlag, best: Option<Move>) {
        let index = (zobrist as usize) % self.entries;
        let phashe = &self.table.as_mut_ptr();

        unsafe {
            (*phashe.add(index)).key = zobrist;
            (*phashe.add(index)).best = best;
            (*phashe.add(index)).value = value;
            (*phashe.add(index)).depth = depth;
            (*phashe.add(index)).flags = flag;
        }
    }
}