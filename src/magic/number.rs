use std::ops::Deref;

use crate::{bishop::Bishop, rook::Rook, squares::{BISHOP_RELEVANT_BITS, ROOK_RELEVANT_BITS}, Bitboard, magic::attacks::DynamicAttacks};


#[derive(Debug)]
pub struct Magic(u32);

//// https://www.chessprogramming.org/Looking_for_Magics
impl Magic {
    pub fn new() -> Self {
        Self(1804289383)
    }

    pub fn random_u32(&mut self) -> u32 {
        // XOR shift algorithm
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 17;
        self.0 ^= self.0 << 5;

        self.0
    }


    //// https://www.chessprogramming.org/Looking_for_Magics
    fn get_random_u64(&mut self) -> u64 {
        let u1 = (self.random_u32() as u64) & 0xFFFF;
        let u2 = (self.random_u32() as u64) & 0xFFFF;
        let u3 = (self.random_u32() as u64) & 0xFFFF;
        let u4 = (self.random_u32() as u64) & 0xFFFF;

        u1 | u2 << 16 | u3 << 32 | u4 << 48
    }


    fn random_u64(&self) -> u64 {
        let u1 = rand::random::<u64>() & 0xFFFF;
        let u2 = rand::random::<u64>() & 0xFFFF;
        let u3 = rand::random::<u64>() & 0xFFFF;
        let u4 = rand::random::<u64>() & 0xFFFF;

        u1 | (u2 << 16) | (u3 << 32) | (u4 << 48)
    }

    /// Generate magic number
    /// https://www.chessprogramming.org/Looking_for_Magics
    fn random_u64_fewbits(&mut self) -> u64 {
        self.random_u64() & self.random_u64() & self.random_u64()
    }

    fn generate_magic_number(&mut self) -> u64 {
        self.get_random_u64() & self.get_random_u64() & self.get_random_u64()
    }


    pub(crate) fn find_magic(&mut self, sq: u64, relevant_bits: u32, bishop: bool) -> u64 {
        let mut b: Vec<u64> = vec![0; 4096]; // occupancies
        let mut attacks: Vec<u64> = vec![0; 4096];
        let mut used_attacks: Vec<u64> = Vec::with_capacity(4096);

        let bitboard = match bishop {
            true => Bishop::bitboard_bishop_attack(sq),
            false => Rook::bitboard_rook_attacks(sq)
        };
        let n = bitboard.count_ones();

        for i in 0..(1<<n) {
            // println!("the i is {} and the n is {}", i, n);
            let i = i as usize;
            b[i] = bitboard.index_to_u64(i, n);
            attacks[i] = match bishop {
                true => DynamicAttacks::bishop(sq, b[i]).into(),
                false => DynamicAttacks::rookie(sq, b[i]).into(),
            };
        }

        for _ in 0..100000000 {
            let magic = self.random_u64_fewbits();
            let magic_bitboard = bitboard.wrapping_mul(magic) & 0xFF00000000000000;
            if Bitboard::from(magic_bitboard).count_ones() < 6 {continue};
            // unsafe { ptr::write_bytes(used_attacks.as_mut_ptr(), 0, 4096) };
            used_attacks = vec![0; 4096];
            let mut i =0; let mut fail = false;
            while !fail && i < (1 << n) {
                // println!("xov2 {}", b[i]);
                let j = ((b[i].wrapping_mul(magic)) >> (64-relevant_bits)) as usize;
                if used_attacks[j]== 0 {used_attacks[j] = attacks[i]}
                else if used_attacks[j] != attacks[i] {fail=true};
                i+=1;
            }
            if !fail {return magic}
        }
        return 0
    }


    pub(crate) fn finding_finder(&mut self) {
        let mut rook = Vec::with_capacity(64);
        let mut bishop = Vec::with_capacity(64);

        for sq in 0..64 {
            // println!("rookie == {:#?}", self.find_magic(sq, ROOK_RELEVANT_BITS[sq as usize], false))
            rook.push(self.find_magic(sq, ROOK_RELEVANT_BITS[sq as usize], false));
        }


        println!("{:#?}", rook);

        println!("length is {}", rook.len());

        println!(":::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::");

        for sq in 0..64 {
            // println!("bishop = {:#?}", self.find_magic(sq, BISHOP_RELEVANT_BITS[sq as usize], true))
            bishop.push(self.find_magic(sq, BISHOP_RELEVANT_BITS[sq as usize], true));
        }

        println!("<><> \n {:#?}", bishop);
    }



    /// Find appropriate magic number
    /// https://www.chessprogramming.org/Looking_for_Magics
    pub(crate) fn find_magic_number(&mut self, sq: u64, relevant_bits: u32, bishop: bool) -> u64 {
        let mut occupancies: Vec<u64> = Vec::with_capacity(4096);
        let mut attacks: Vec<u64> = Vec::with_capacity(4096);
        let mut used_attacks: Vec<u64> = Vec::with_capacity(4096);
        
        let attack_bitboard = match bishop {
            true => Bishop::bitboard_bishop_attack(sq),
            false => Rook::bitboard_rook_attacks(sq)
        };

        // init occupancy indices
        let occupancy_indices: u64 = 1 << relevant_bits;

        // loop over occupancy indices
        for i in 0..occupancy_indices {
            let index = i as usize;
            occupancies.insert(index, attack_bitboard.set_occupancy(i, relevant_bits).into());

            let indexed_attacks = match bishop {
                true => DynamicAttacks::bishop(sq, occupancies[index]).into(),
                false => DynamicAttacks::rookie(sq, occupancies[index]).into()
            };
            attacks.insert(index, indexed_attacks);

        }


        // test magic number
        for _ in 0..100000000 {
            // generate magic number candidate
            let magic_number = self.generate_magic_number();
            // skip inappropriate magic numbers
            let magic_attack = (*attack_bitboard).wrapping_mul(magic_number) & 0xFF00000000000000u64;
            if Bitboard::from(magic_attack).count_bits() < 6 {continue};
            // unsafe { ptr::write_bytes(used_attacks.as_mut_ptr(), 0, 4096) };
            used_attacks.drain(..);
            for _ in 0..4096 {used_attacks.push(0)};
            // println!("the used_attacks.len {}", used_attacks.len());
            
            let mut fail = false; let mut i: usize = 0;
            while !fail && i < occupancy_indices.try_into().unwrap() {
                let magic_index = (occupancies[i] * magic_number) as usize >> (64-relevant_bits);

                // if magic index works
                // if used_attacks.get(magic_index).is_some_and(|v| *v == 0) {
                if used_attacks[magic_index] == 0 {
                    used_attacks[magic_index] = attacks[i];
                } else if attacks.get(i).is_some_and(|v| *v != used_attacks[magic_index]) {
                // } else if used_attacks.get(magic_index).is_some_and(|v| attacks.get(i).is_some_and(|vv| v != vv)) {
                    // i.e. used_attacks[magic_index] != attacks[i]
                    fail = true;
                }

                // if magic number works, return it
                if !fail {
                    return magic_number;
                }

                i+=1;
                // if magic number doesn't work
            }
        }
        
        println!(" Magic number fails!");
        return 0;
    }


    pub(crate) fn init_magic_numbers(&mut self) {
        let mut rook = Vec::with_capacity(64);
        let mut bishop = Vec::with_capacity(64);

        // loop over 64 board squares
        for square in 0..64 {
            // init rook magic numbers
            // println!("self is {:?}", self);
            rook.push(self.find_magic_number(square, ROOK_RELEVANT_BITS[square as usize], false));
        }
        // println!("{:#?}", rook);
        println!("________________________________________________________ \n");

        for square in 0..64 {
                bishop.push(self.find_magic_number(square, BISHOP_RELEVANT_BITS[square as usize], true));
            }
        }
        // println!("{:#?}", bishop);
}


impl Deref for Magic {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


impl From<u32> for Magic {
    fn from(value: u32) -> Self {
        Magic(value)
    }
}