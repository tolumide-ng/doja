use std::{fmt::Display, ops::Deref};

use crate::{constants::{NOT_A_FILE, NOT_H_FILE}, squares::{Square, BIT_TABLE}};



#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Bitboard(pub u64);

impl Bitboard {
    pub fn new() -> Self {
        Self(0)
    }

    /// First shifts the binary representation of 1 to the left by value (u64),
    /// generating a mask with the `square-th` bit set.
    /// It then compares the mask with this bitboard using the `&` operator, effectively
    /// comparing the value at the `square-th` position on both.
    /// we then finally shift the result to the right by `square`, so we can get a 1 or 0 \
    /// NB: this is same as (self.0 >> square) & 1 meaning shift self to the right
    /// by `square` compare with 1 using the `&` operator, then return the result
    pub fn get_bit_by_square(&self, square: Square) -> u64 {
        // let value = (self.0 >> square) & 1;
        let value: u64 = square.into();
        (self.0 & (1 << value)) >> value
    }


    /// Returns true if this bitboard is 0, otherwise returns false
    pub fn is_zero(&self) -> bool {
        **self == 0
    }

    /// Returns true if this bitboard is not 0, otherwise returns false
    pub fn not_zero(&self) -> bool {
        **self != 0
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
            self.0 ^= 1 << square;
        }
    }

    /// shifts the binary representation of 1 to the right by square (u64)
    /// this creates a mask with only the `square-th` bit set i.e. if value
    /// of the square is 6, then this (1) would become (0010 0000)
    /// and then BitOrAssigns the mask to self.0
    /// |= means Bitwise OR and assignment
    /// this means that if the other positions in the target bitboard are 1,
    /// the zeros on this new bitboard cannot override them 
    /// since 0 | 1 is 1 and 1 | 0 is also 0
    /// e.g
    /// 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00100000 (mask)
    /// 00000000 00000000 00000001 00010000 00000000 00000000 00000000 00000000 (self)
    /// becomes (|=)
    /// 00000000 00000000 00000001 00010000 00000000 00000000 00000000 00100000 (mask)
    pub fn set_bit(&mut self, square: u64) {
        self.0 |= 1 << square;
    }


    /// Counts the number of bits(1's) in a bitboard \
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
        let mut bitboard = self.clone();
        let mut result = 0_u64;

        for i in 0..bits {
            let j = bitboard.pop_first_bit();
            if index & (1<<i) !=0 {result |= 1 << j}
        }
        result
    }

    /// One shift only
    pub(crate) fn south(&self) -> u64 { **self >> 8 }
    /// One shift only
    pub(crate) fn north(&self) -> u64 { **self << 8 }
    // /// Post-shift mask
    pub(crate) fn east(&self) -> u64 { (**self << 1) & NOT_A_FILE  }
    pub(crate) fn north_east(&self) -> u64 { (**self << 9) & NOT_A_FILE}
    pub(crate) fn south_east(&self) -> u64 { (**self >> 7) & NOT_A_FILE}
    pub(crate) fn west(&self) -> u64 { (**self >> 1) & NOT_H_FILE}
    pub(crate) fn south_west(&self) -> u64 { (**self >> 9) & NOT_H_FILE}
    pub(crate) fn north_west(&self) -> u64 { (**self << 7) & NOT_H_FILE}
    // Pre-shifts
    pub(crate) fn pre_east(&self) -> u64 { (**self & NOT_H_FILE) << 1  }
    pub(crate) fn pre_north_east(&self) -> u64 { (**self & NOT_H_FILE) << 9}
    pub(crate) fn pre_south_east(&self) -> u64 { (**self & NOT_H_FILE) >> 7}
    pub(crate) fn pre_west(&self) -> u64 { (**self & NOT_A_FILE) >> 1}
    pub(crate) fn pre_south_west(&self) -> u64 { (**self & NOT_A_FILE) >> 9}
    pub(crate) fn pre_north_west(&self) -> u64 { (**self & NOT_A_FILE) << 7}

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

// pub fn string_u64(input: u64) -> String {
//     let mut s = String::new();
//     let format_in = format_u64(input);
//     for x in 0..8 {
//         let slice = &format_in[x * 8..((x * 8) + 8)];
//         s += slice;
//         s += "\n";
//     }
//     s
// }

impl Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!("---------------------------");
        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = (rank * 8) + file;

                if file == 0 {
                    print!("{}  ", rank+1);
                }
                
                print!(" {} ", self. get_bit(square));
                // print!(" {} ", 8 * rank + file);
            }
            println!("");
        }
        println!("    \n    a  b  c  d  e  f  g  h\n");
        println!("Bitboard: {}", self.0);
        Ok(())
        // let s = &string_u64(reverse_bytes(self.0));
        // f.pad(s)
    }
}

pub fn reverse_bytes(b: u64) -> u64 {
    let mut m: u64 = 0;
    m |= (reverse_byte(((b >> 56) & 0xFF) as u8) as u64) << 56;
    m |= (reverse_byte(((b >> 48) & 0xFF) as u8) as u64) << 48;
    m |= (reverse_byte(((b >> 40) & 0xFF) as u8) as u64) << 40;
    m |= (reverse_byte(((b >> 32) & 0xFF) as u8) as u64) << 32;
    m |= (reverse_byte(((b >> 24) & 0xFF) as u8) as u64) << 24;
    m |= (reverse_byte(((b >> 16) & 0xFF) as u8) as u64) << 16;
    m |= (reverse_byte(((b >> 8) & 0xFF) as u8) as u64) << 8;
    m |= reverse_byte((b & 0xFF) as u8) as u64;
    m
}

pub fn reverse_byte(b: u8) -> u8 {
    let m: u8 = ((0b0000_0001 & b) << 7)
        | ((0b0000_0010 & b) << 5)
        | ((0b0000_0100 & b) << 3)
        | ((0b0000_1000 & b) << 1)
        | ((0b0001_0000 & b) >> 1)
        | ((0b0010_0000 & b) >> 3)
        | ((0b0100_0000 & b) >> 5)
        | ((0b1000_0000 & b) >> 7);
    m
}


impl Deref for Bitboard {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
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