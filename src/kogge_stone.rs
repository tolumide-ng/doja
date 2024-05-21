/**
 * https://www.chessprogramming.org/Kogge-Stone_Algorithm
 * */

use std::ops::{BitAnd, BitAndAssign, BitOrAssign};

use crate::{color::Color, constants::{AVOID_WRAP, NOT_A_FILE, NOT_H_FILE, PIECE_ATTACKS, SHIFT}, shift::Shift, Bitboard};

pub struct KoggeStone;

impl KoggeStone {
    /// gen is a bitboard(u64) containing all the attack pieces (e.g. black rooks) of the attacker
    /// vertical south fill
    pub(crate) fn sout_fill(gen: u64) -> u64 {
        let mut gen = gen | (gen >> 8);
        gen |= gen >> 16;
        gen |= gen >> 32;
        gen
    }

    /// Vertical nort fill
    fn nort_fill(gen: u64) -> u64 {
        let mut gen = gen | gen << 8;
        gen |= gen << 16;
        gen |= 32;
        gen
    }

    /// Explicit propagators as compile time constants to manage the A-, H-file wraps
    fn east_fill(gen: u64) -> u64 {
        let pr0 = NOT_A_FILE;
        let pr1 = pr0 & (pr0 << 1);
        let pr2 = pr1 & (pr1 << 2);
        
        let mut gen = gen;
        gen |= pr0 & (gen << 1);
        gen |= pr1 & (gen << 2);
        gen |= pr2 & (gen << 4);
        gen
    }

    fn no_ea_fill(gen: u64) -> u64 {
        let pr0 = NOT_A_FILE;
        let pr1 = pr0 & (pr0 << 9);
        let pr2 = pr1 & (pr1 << 18);
        
        let mut gen= gen;
        gen |= pr0 & (gen << 9);
        gen |= pr1 & (gen << 18);
        gen |= pr2 & (gen << 36);
        gen
    }

    fn so_ea_fill(gen: u64) -> u64 {
        let pr0 = NOT_A_FILE;
        let pr1 = pr0 & (pr0 >> 7);
        let pr2 = pr1 & (pr1 >> 14);
        let mut gen = gen;
        gen |= pr0 & (gen >> 7);
        gen |= pr1 & (gen >> 14);
        gen |= pr2 & (gen >> 28);
        gen
    }

    fn west_fll(gen: u64) -> u64 {
        let pr0 = NOT_H_FILE;
        let pr1 = pr0 & (pr0 >> 1);
        let pr2 = pr1 & (pr1 >> 2);
        let mut gen = gen;
        gen |= pr0 & (gen >> 1);
        gen |= pr1 & (gen >> 2);
        gen |= pr2 & (gen >> 4);
        gen
    }

    fn so_we_fill(gen: u64) -> u64 {
        let pr0 = NOT_H_FILE;
        let pr1 = pr0 & (pr0 >> 9);
        let pr2 = pr1 & (pr1 >> 18);
        let mut gen = gen;
        gen |= pr0 & (gen >> 9);
        gen |= pr1 & (gen >> 18);
        gen |= pr2 & (gen >> 16);
        gen
    }

    fn no_we_fill(gen: u64) -> u64 {
        let pr0 = NOT_H_FILE;
        let pr1 = pr0 & (pr0 << 7);
        let pr2 = pr1 & (pr1 << 14);
        let mut gen = gen;
        gen |= pr0 & (gen << 7);
        gen |= pr1 & (gen << 14);
        gen |= pr2 & (gen << 28);
        gen
    }


    // Occluded fills include sliders, but exclude blockers
    /// pro is empty (all the empty bits are set to 1 and the filled bits set to 0)
    fn sout_occl(gen: u64, pro: u64) -> u64 {
        let mut pro = gen;
        let mut gen = gen;
        gen.bitor_assign(pro.bitand(gen >> 8));
        pro.bitand_assign(pro >> 8);
        gen.bitor_assign(pro.bitand(gen >> 16));
        pro.bitand_assign(pro >> 16);
        gen.bitor_assign(pro.bitand(gen >> 32));
        gen
    }

    fn nort_occl(gen: u64, pro: u64) -> u64 {
        let (mut gen, mut pro) = (gen, pro);
        gen |= pro & (gen << 8);
        pro &= pro << 8;
        gen |= pro & (gen << 16);
        pro &= pro << 16;
        gen |= pro & (gen << 32);
        gen
    }

    fn east_occl(gen: u64, pro: u64) -> u64 {
        let mut gen = gen; let mut pro = pro;
        pro &= NOT_A_FILE;
        gen |= pro & (gen << 1);
        pro &= pro << 1;
        gen |= pro & (gen << 2);
        pro &= pro << 2;
        gen |= pro & (gen << 4);
        gen
    }

    fn no_ea_occl(gen: u64, pro: u64) -> u64 {
        let mut pro = pro; let mut gen = gen;
        pro &= NOT_A_FILE;
        gen |= pro & (gen << 9);
        pro &= pro << 9;
        gen |= pro & (gen << 18);
        pro &= pro << 18;
        gen |= pro & (gen << 36);
        gen
    }

    fn so_ea_occl(gen: u64, pro: u64) -> u64 {
        let mut pro = pro; let mut gen = gen;
        pro &= NOT_A_FILE;
        gen |= pro & (gen >> 7);
        pro &= pro >> 7;
        gen |= pro & (gen >> 14);
        pro &= pro >> 14;
        gen |= pro & (gen >> 28);
        gen
    }

    fn west_occl(gen: u64, pro: u64) -> u64 {
        let mut gen = gen; let mut pro = pro;
        pro &= NOT_H_FILE;
        gen |= pro & (gen >> 1);
        pro &= pro >> 1;
        gen |= pro & (gen >> 2);
        pro &= pro >> 2;
        gen |= pro & (gen >> 4);
        gen
    }

    fn so_we_occl(gen: u64, pro: u64) -> u64 {
        let mut gen = gen; let mut pro = pro;
        pro &= NOT_H_FILE;
        gen |= pro & (gen >> 9);
        pro &= pro >> 9;
        gen |= pro & (gen >> 18);
        pro &= pro >> 18;
        gen |= pro & (gen >> 36);
        gen
    }

    fn no_we_occl(gen: u64, pro: u64) -> u64 {
        let mut pro = pro; let mut gen = gen;
        pro &= NOT_H_FILE;
        gen |= pro & (gen << 7);
        pro &= pro << 7;
        gen |= pro & (gen << 14);
        pro &= pro << 14;
        gen |= pro & (gen << 28);
        gen
    }

    fn south(bits: u64) -> u64 { bits >> 8 }
    fn north(bits: u64) -> u64 { bits << 8 }
    fn east(bits: u64) -> u64 { (bits << 1) & NOT_A_FILE  }
    fn north_east(bits: u64) -> u64 { (bits << 9) & NOT_A_FILE}
    fn south_east(bits: u64) -> u64 { (bits >> 7) & NOT_A_FILE}
    fn west(bits: u64) -> u64 { (bits >> 1) & NOT_H_FILE}
    fn south_west(bits: u64) -> u64 { (bits >> 9) & NOT_H_FILE}
    fn north_west(bits: u64) -> u64 { (bits << 7) & NOT_H_FILE}


    pub(crate) fn south_attacks(rooks: u64, empty: u64) -> u64 {
        Self::south(Self::sout_occl(rooks, empty))
    }
    pub(crate) fn nort_attacks(rooks: u64, empty: u64) -> u64 {
        Self::north(Self::nort_occl(rooks, empty))
    }
    pub(crate) fn east_attacks(rooks: u64, empty: u64) -> u64 {
        Self::east(Self::east_occl(rooks, empty))
    }
    // pub(crate) fn no_east_attacks(bishops: u64, empty: u64) -> u64 {
    //     Self::north_east(Self::no_ea_occl(bishops, empty))
    // }
    pub(crate) fn so_east_attacks(bishops: u64, empty: u64) -> u64 {
        Self::south_east(Self::so_ea_occl(bishops, empty))
    }
    pub(crate) fn west_attacks(rooks: u64, empty: u64) -> u64 {
        Self::west(Self::west_occl(rooks, empty))
    }
    pub(crate) fn so_we_attacks(bishops: u64, empty: u64) -> u64 {
        Self::south_west(Self::so_we_occl(bishops, empty))
    }
    pub(crate) fn no_we_attacks(bishops: u64, empty: u64) -> u64 {
        Self::north_west(Self::no_we_occl(bishops, empty))
    }

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