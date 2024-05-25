/**
 * https://www.chessprogramming.org/Kogge-Stone_Algorithm
 * */


 
use crate::shift::Shift;

pub struct KoggeStone;

impl KoggeStone {
    fn rotate_left(x: u64, s: i8) -> u64 {
        if s >= 0 {
            // return bits.rotate_left(s as u32)
            return (x << s) | (x >> (64-s))
        }
        return x.rotate_right((-s) as u32);
    }
    fn rotate_right(x: u64, s: i8) -> u64 {(x >> s) | (x << (64-s))}

    pub(crate) fn occluded_fill(gen: u64, pro: u64, shift: Shift) -> u64 {
        let mut pro = pro; let mut gen = gen;
        let r = shift.amount;
        
        pro &= shift.mask;

        // becnause rust only supports rotate_left and right by u32
        gen |= pro & Self::rotate_left(gen, r);
        pro &= Self::rotate_left(pro, r);
        gen |= pro & Self::rotate_left(gen, 2*r);
        pro &= Self::rotate_left(pro, 2*r);
        gen |= pro & Self::rotate_left(gen, 4*r);
        gen
    }

    pub(crate) fn shift_one(b: u64, shift: Shift) -> u64 {
        let r = shift.amount;
        Self::rotate_left(b, r) & shift.mask
    }

    pub(crate) fn sliding_attacks(sliders: u64, empty: u64, shift: Shift) -> u64 {
        let fill = Self::occluded_fill(sliders, empty, shift);
        Self::shift_one(fill, shift)
    }

    
}