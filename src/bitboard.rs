use std::{fmt::Display, ops::{Deref, DerefMut}};

use crate::constants::{NOT_A_FILE, NOT_H_FILE};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Bitboard(pub u64);

impl Bitboard {
    /// to get the square of a set bit, simply use Rust's (*.trailing_zeros()) function
    pub fn new() -> Self {
        Self(0)
    }

    /// Returns true if this bitboard is 0, otherwise returns false
    pub fn is_zero(&self) -> bool {
        **self == 0
    }

    /// Returns true if this bitboard is not 0, otherwise returns false
    pub fn not_zero(&self) -> bool {
        **self != 0
    }

    /// Returns the bit at the index `square`
    pub fn get_bit(&self, square: u64) -> u64 {
        // ((self.0 & 1 << square) != 0) as u64
        (self.0 >> square) & 1
    }

    /// Removes the bit at index `square`
    /// should be renamed to (pop_bit_at)
    pub fn pop_bit(&mut self, square: u64) {
        if self.get_bit(square.into()) != 0 {
            self.0 ^= 1 << square;
        }
    }

    //// Sets the bit at index `square` in self
    pub fn set_bit(&mut self, square: u64) {
        self.0 |= 1 << square;
    }

    /// Returns the least significant bit on this bitboard, 
    /// or returns None otherwise
    #[inline]
    pub(crate) fn get_lsb1(&self) -> Option<u64> {
        if self.0 == 0 {
            // return -1
            return None
        }

        // let x = self.0 & -self.0;
        // let lsb = (self.0 as i64 & -(self.0 as i64)) -1
        // asset_eq(self.get_lsb1(), lsb);
        Some(self.trailing_zeros() as u64)
    }

    /// One shift only
    pub(crate) fn south(&self) -> u64 { **self >> 8 }
    /// One shift only
    pub(crate) fn north(&self) -> u64 { **self << 8 }
    // /// Post-shift mask
    pub(crate) fn east(&self) -> u64 { (**self << 1) & NOT_A_FILE  }
    pub(crate) fn north_east(&self) -> u64 { (**self << 9) & NOT_A_FILE}
    pub(crate) fn north_west(&self) -> u64 { (**self << 7) & NOT_H_FILE}
    pub(crate) fn south_east(&self) -> u64 { (**self >> 7) & NOT_A_FILE}
    pub(crate) fn south_west(&self) -> u64 { (**self >> 9) & NOT_H_FILE}
    pub(crate) fn west(&self) -> u64 { (**self >> 1) & NOT_H_FILE}

    pub(crate) fn set_occupancy(&self, index: u64, bits_in_bitboard: u32) -> Bitboard {
        let mut attack_bitboard: Bitboard = self.clone();
        let mut occupancy = 0u64;
        
        // loop over the range of bits within attack bitboard
        for count in 0..bits_in_bitboard {
            // get the index of the least significant first bit(LS1B) in the attack bitboard
            let square = attack_bitboard.get_lsb1().unwrap();
            // then pop the it
            attack_bitboard.pop_bit(square);
            // make sure the occupancy is on the board
            if (index & (1<<count)) != 0 {
                occupancy |= 1u64 << square;
            }
        }
        
        occupancy.into()
    }
}

/// Returns a stringified u64 with all 64 bits being represented.
fn format_u64(input: u64) -> String {
    format!("{:064b}", input)
}

impl Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // println!("TOLUMIDE SHOPEIN");
        writeln!(f, "---------------------------")?;
        writeln!(f, "")?;
        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = (rank * 8) + file;

                if file == 0 {
                    write!(f, "{}  ", rank+1)?;
                }
                
                write!(f, " {} ", self. get_bit(square))?;
                // print!(" {} ", 8 * rank + file);
            }
            writeln!(f, "")?;
        }
        writeln!(f, "    \n    a  b  c  d  e  f  g  h\n")?;
        writeln!(f, "Bitboard: {}", self.0)?;
        writeln!(f, "")?;
        Ok(())
    }
}


impl Deref for Bitboard {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Bitboard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


impl From<u64> for Bitboard {
    fn from(value: u64) -> Self {
        Self(value)
    }
}


impl From<Bitboard> for u64 {
    fn from(value: Bitboard) -> Self {
        value.0
    }
}



#[cfg(test)]
mod bitboard_tests {
    use super::*;

    const BOARD: u64 = 0x4000020880300;

    #[test]
    fn basic_immutable_operations_on_bitboard() {
        let bitboard = Bitboard::from(BOARD);

        assert_eq!(bitboard.is_zero(), false);
        assert_eq!(bitboard.not_zero(), true);

        assert_eq!(bitboard.get_bit(7), 0);
        assert_eq!(bitboard.get_bit(8), 1);
        assert_eq!(bitboard.get_bit(9), 1);
        assert_eq!(bitboard.get_bit(11), 0);
        assert_eq!(bitboard.get_bit(23), 1);
        assert_eq!(bitboard.get_bit(56), 0);


        assert_eq!(bitboard.count_ones(), 6);
        assert_eq!(bitboard.get_lsb1(), Some(8));
    }

    #[test]
    fn bitboard_navigation() {
        let d3: u64 = 0x80000; let e2: u64 = 0x1000;
        
        

        let h1: u64 = 0x80; let g1= 0x40u64; let h8 = 0x8000000000000000u64; 
        let h7 = 0x80000000000000u64; let g8 = 0x4000000000000000u64;

        let a8 =0x100000000000000u64; let b8 = 0x200000000000000u64;
        let a7 = 0x1000000000000u64; let b7 = 0x2000000000000u64;

        let a1 =0x1u64; let a2= 0x100u64; let b1 = 0x2u64;

        let g3: u64 = 0x400000; let h2 = 0x8000u64;
        let h4 = 0x80000000u64; let f4 = 0x20000000u64;
        let f2 = 0x2000; let g2 = 0x4000u64; let h3 = 0x800000u64;
        let g4 = 0x40000000u64; let f3 = 0x200000u64; let f1 = 0x20u64;


        //south_west
        let g7: u64 = 0x40000000000000; let h6: u64 = 0x800000000000; let b2 = 0x200u64;
        
        
        assert_eq!(Bitboard::from(d3).south_east(), e2);
        assert_eq!(Bitboard::from(g3).south_east(), h2);
        assert_eq!(Bitboard::from(g7).south_east(), h6);
        assert_eq!(Bitboard::from(a1).south_east(), 0);
        assert_eq!(Bitboard::from(h1).south_east(), 0);
        assert_eq!(Bitboard::from(h8).south_east(), 0);
        assert_eq!(Bitboard::from(a8).south_east(), b7);
        
        
        assert_eq!(Bitboard::from(a1).south_west(), 0);
        assert_eq!(Bitboard::from(h1).south_west(), 0);
        assert_eq!(Bitboard::from(a8).south_west(), 0);
        assert_eq!(Bitboard::from(h8).south_west(), g7);
        assert_eq!(Bitboard::from(h2).south_west(), g1);
        assert_eq!(Bitboard::from(g2).south_west(), f1);



        // north west
        assert_eq!(Bitboard::from(g3).north_west(), f4);
        assert_eq!(Bitboard::from(h8).north_west(), 0);
        assert_eq!(Bitboard::from(a8).north_west(), 0);
        assert_eq!(Bitboard::from(a1).north_west(), 0);
        assert_eq!(Bitboard::from(h8).north_west(), 0);
        assert_eq!(Bitboard::from(h1).north_west(), g2);

        //north_east
        assert_eq!(Bitboard::from(g3).north_east(), h4);
        assert_eq!(Bitboard::from(h1).north_east(), 0);
        assert_eq!(Bitboard::from(h8).north_east(), 0);
        assert_eq!(Bitboard::from(a1).north_east(), b2);
        assert_eq!(Bitboard::from(a8).north_east(), 0);


        // north_one
        assert_eq!(Bitboard::from(a1).north(), a2);
        assert_eq!(Bitboard::from(h1).north(), h2);
        assert_eq!(Bitboard::from(a8).north(), 0);
        assert_eq!(Bitboard::from(h8).north(), 0);
        assert_eq!(Bitboard::from(g3).north(), g4);
        assert_eq!(Bitboard::from(b1).north(), b2);
        assert_eq!(Bitboard::from(f2).north(), f3);


        // west_one
        assert_eq!(Bitboard::from(a1).south(), 0);
        assert_eq!(Bitboard::from(h1).south(), 0);
        assert_eq!(Bitboard::from(a8).south(), a7);
        assert_eq!(Bitboard::from(h8).south(), h7);
        assert_eq!(Bitboard::from(g3).south(), g2);



        // east 1
        assert_eq!(Bitboard::from(a1).east(), b1);
        assert_eq!(Bitboard::from(h1).east(), 0);
        assert_eq!(Bitboard::from(a8).east(), b8);
        assert_eq!(Bitboard::from(h8).east(), 0);
        assert_eq!(Bitboard::from(g3).east(), h3);



        // east 1
        assert_eq!(Bitboard::from(a1).west(), 0);
        assert_eq!(Bitboard::from(h1).west(), g1);
        assert_eq!(Bitboard::from(a8).west(), 0);
        assert_eq!(Bitboard::from(h8).west(), g8);
        assert_eq!(Bitboard::from(g3).west(), f3);

    }


    #[cfg(test)]
    mod mutable_board_methods {
        use crate::squares::Square;

        use super::*;

        #[test]
        fn should_pop_the_bit_if_it_exists() {
            let mut bitboard = Bitboard::from(BOARD);

            let sq = Square::A2 as u64;
            assert_eq!(bitboard.get_bit(sq), 1);

            bitboard.pop_bit(sq);
            assert_ne!(bitboard.get_bit(sq), 1);

            let sq = Square::G3 as u64;
            assert_eq!(bitboard.get_bit(sq), 0);

            bitboard.pop_bit(sq);
            assert_ne!(bitboard.get_bit(sq), 1);
        }


        #[test]
        fn should_set_bit() {
            let mut bitboard = Bitboard::from(BOARD);

            let sq = Square::G3 as u64;
            assert_eq!(bitboard.get_bit(sq), 0);

            bitboard.set_bit(sq);
            assert_eq!(bitboard.get_bit(sq), 1);

            assert_eq!(bitboard.get_bit(Square::A2 as u64), 1);

            bitboard.pop_first_bit();
            assert_eq!(bitboard.get_bit(Square::A2 as u64), 0);
        }
    }

}