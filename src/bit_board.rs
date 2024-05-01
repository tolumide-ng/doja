use std::fmt::Display;

use crate::squares::Square;



#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct BitBoard(pub u64);

impl BitBoard {
    pub fn new() -> Self {
        Self(0)
    }

    /// First shifts the binary representation of 1 to the left by value (u64),
    /// generating a mask with the `square-th` bit set.
    /// It then compares the mask with the bitboard using the `&` operator, effectively
    /// comparing the value at the `square-th` position on both.
    /// we then finally shift the result to the right by `square`, so we can get a 1 or 0 \
    /// NB: this is same as (self.0 >> square) & 1 meaning shift the bitboard to the right
    /// by `square` compare with 1 using the `&` operator, then return the result
    pub fn get_bit_by_square(&self, square: Square) -> u64 {
        // let value = (self.0 >> square) & 1;
        let value: u64 = square.into();
        (self.0 & (1 << value)) >> value
    }

    /// shifts the bit_board to the right by square_value (u64) 
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
    pub fn pop_bit(&mut self, square: Square) {
        let value: u64 = square.into();

        if self.get_bit(square.into()) == 1 {
            self.0 ^= 1 << value;
        }
    }

    /// shifts the binary representation of 1 to the right by square (u64)
    /// this creates a mask with only the `square-th` bit set i.e. if value
    /// of the square is 6, then this (1) would become (10 0000)
    /// and then assigns this mask to the bitboard
    /// |= means Bitwise OR and assignment
    /// this means that if the other positions in the target bitboard are 1,
    /// the zeros on this new mask cannot override them 
    /// since 0 | 1 is 1 and 1 | 0 is also 0
    pub fn set_bit(&mut self, square: u64) {
        self.0 |= 1 << square;
    }

}

impl Display for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rank in 0..8 {
            for file in 0..8 {
                let square = (rank * 8) + file;

                if file == 0 {
                    // let index = rank-8;
                    print!("{}  ", 8-rank);
                }
                
                print!(" {} ", self.get_bit(square));
            }
            println!("");
        }
        println!("    a  b  c  d  e  f  g  h\n");
        write!(f, "Bitboard: {}", self.0)
    }
}