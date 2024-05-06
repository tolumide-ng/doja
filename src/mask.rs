use std::{fmt::Display, ops::Deref};

use crate::squares::{Square, BIT_TABLE};



#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Mask(pub u64);

impl Mask {
    pub fn new() -> Self {
        Self(0)
    }

    /// First shifts the binary representation of 1 to the left by value (u64),
    /// generating a mask with the `square-th` bit set.
    /// It then compares the mask with self using the `&` operator, effectively
    /// comparing the value at the `square-th` position on both.
    /// we then finally shift the result to the right by `square`, so we can get a 1 or 0 \
    /// NB: this is same as (self.0 >> square) & 1 meaning shift self to the right
    /// by `square` compare with 1 using the `&` operator, then return the result
    pub fn get_bit_by_square(&self, square: Square) -> u64 {
        // let value = (self.0 >> square) & 1;
        let value: u64 = square.into();
        (self.0 & (1 << value)) >> value
    }

    /// shifts self to the right by square_value (u64) 
    /// and compares the value at the LSB (Least Significant Byte)
    /// with 1 ---->> if the value at LSB is 1, then return 1 else
    /// it returns 0
    pub fn get_bit(&self, square: u64) -> u64 {
        let value = (self.0 >> square) & 1;
        value
    }

    /// ^= (BitXorAssign): 
    /// comparing 0 and 0 returns 0, comparing 1 and 1 return 0,
    /// comparing 1 and 0 returns 1, comparing 0 and 1 returns 1 \
    /// Checks if the bit at the target position is 1, if the value
    /// is 1, then it uses the BitXorAssign operator described above
    pub fn pop_bit(&mut self, square: u64) {
        if self.get_bit(square.into()) != 0 {
            self.0 ^= (1 << square);
        }
    }

    /// shifts the binary representation of 1 to the right by square (u64)
    /// this creates a mask with only the `square-th` bit set i.e. if value
    /// of the square is 6, then this (1) would become (10 0000)
    /// and then assigns the new mask to self.0
    /// |= means Bitwise OR and assignment
    /// this means that if the other positions in the target mask are 1,
    /// the zeros on this new mask cannot override them 
    /// since 0 | 1 is 1 and 1 | 0 is also 0
    pub fn set_bit(&mut self, square: u64) {
        self.0 |= 1 << square;
    }


    /// Counts the number of bits(1's) in a mask \
    /// e.g given 0b00011100 \
    /// this would return 3
    #[inline]
    pub(crate) fn count_bits(&self) -> u32 {
        self.count_ones()
    }

    /// Returns the Least Significant first Bit
    /// e.g given 0b00010000
    /// the LSB would be 5
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

    /// https://www.chessprogramming.org/Looking_for_Magics
    pub(crate) fn pop_first_bit(&mut self) -> u64 {
        let b = self.0 ^ (self.0 - 1);
        let fold = (b as u32) ^ (b >> 32) as u32;
        self.0 &= self.0 - 1;
        BIT_TABLE[(fold.wrapping_mul(0x783a9b23) >> 26) as usize]
    }

    /// https://www.chessprogramming.org/Looking_for_Magics
    /// Does exactly the same as set_occupancy, but this uses Tord Romstad's approach
    pub(crate) fn index_to_u64(&self, index: usize, bits: u32) -> u64 {
        let mut mask = self.clone();
        let mut result = 0_u64;

        for i in 0..bits {
            let j = mask.pop_first_bit();
            if index & (1<<i) !=0 {result |= 1 << j}
        }
        result
    }



    pub(crate) fn set_occupancy(&self, index: u64, bits_in_mask: u32) -> Mask {
        let mut attack_mask: Mask = self.clone();
        let mut occupancy = 0u64;
        
        // loop over the range of bits within attack mask
        for count in 0..bits_in_mask {
            // get the index of the least significant first bit(LS1B) in the attack mask
            let square = attack_mask.get_lsb1().unwrap();
            // then pop the it
            attack_mask.pop_bit(square.into());
            // make sure the occupancy is on the board
            if (index & (1<<count)) != 0 {
                occupancy |= 1u64 << square;
            }
        }
        
        occupancy.into()
    }

}

impl Display for Mask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rank in 0..8 {
            for file in 0..8 {
                let square = (rank * 8) + file;

                if file == 0 {
                    // let index = rank-8;
                    print!("{}  ", 8-rank);
                }
                
                print!(" {} ", self. get_bit(square));
            }
            println!("");
        }
        println!("    \n    a  b  c  d  e  f  g  h\n");
        write!(f, "Bitboard: {}", self.0)
    }
}

impl Deref for Mask {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


impl From<u64> for Mask {
    fn from(value: u64) -> Self {
        Self(value)
    }
}


impl From<Mask> for u64 {
    fn from(value: Mask) -> Self {
        value.0
    }
}