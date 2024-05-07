use std::ops::MulAssign;

use crate::{bishop::Bishop, rook::Rook, squares::{Square, BISHOP_MAGIC_NUMBERS, BISHOP_RELEVANT_BITS, ROOK_MAGIC_NUMBERS, ROOK_RELEVANT_BITS}, Bitboard};

use super::attacks::DynamicAttacks;


pub(crate) struct PlainAttacks {
    rook_attacks: Vec<Vec<u64>>,
    bishop_attacks: Vec<Vec<u64>>,
    rook_bitboards: Vec<u64>,
    bishop_bitboards: Vec<u64>
}

impl PlainAttacks {
    pub(crate) fn init_sliders_attacks(bishop: bool) -> Self {
        let mut bishop_attacks: Vec<Vec<u64>> = vec![vec![0; 512]; 64];
        let mut rook_attacks: Vec<Vec<u64>> = vec![vec![0; 4096]; 64];
        let mut bishop_bitboards: Vec<u64> = vec![0; 64];
        let mut rook_bitboards: Vec<u64> = vec![0; 64];


        for sq in 0..64_usize {
            bishop_bitboards[sq] = Bishop::bitboard_bishop_attack(sq as u64).into();
            rook_bitboards[sq] = Rook::bitboard_rook_attacks(sq as u64).into();

            // println!("for sq {}", sq.to_string());
            // println!("the bishop bitboard here is {}", Bitboard::from(bishop_bitboards[sq]).to_string());
            // println!("\n\n\n");

            // init current bitboard
            let attack_bitboard = match bishop {
                true => bishop_bitboards[sq],
                false => rook_bitboards[sq]
            };

            // init relevant occupancy bit count
            let relevant_bits_count = Bitboard::from(attack_bitboard).count_bits();
            
            let occupany_indices = 1 << relevant_bits_count;
            
            // println!("OCC {}", occupany_indices);
            // loop over occupancy indices
            for index in 0..occupany_indices {
                match bishop {
                    true => {
                        let occupancy = Bitboard::from(attack_bitboard).set_occupancy(index, relevant_bits_count);
                        let magic_index = (*occupancy).wrapping_mul(BISHOP_MAGIC_NUMBERS[sq]) >> (64 - BISHOP_RELEVANT_BITS[sq]);
                        bishop_attacks[sq][magic_index as usize] = DynamicAttacks::bishop(sq as u64, *occupancy).into();

                    }
                    false => {
                        let occupancy = Bitboard::from(attack_bitboard).set_occupancy(index, relevant_bits_count);
                        // println!("the rookie bitboard index {index} {:#?}", occupancy.to_string());
                        // println!(":::XXXXXX:::: {:#?}", ROOK_RELEVANT_BITS[sq]);
                        // println!("::::::::::::: {}", *occupancy);
                        // println!("--------------------- {:0b}", ROOK_MAGIC_NUMBERS[sq]);
                        // println!("LLLLLLLLLLLLL {}", (64 - ROOK_RELEVANT_BITS[sq]));
                        let magic_index = (*occupancy).wrapping_mul(ROOK_MAGIC_NUMBERS[sq]) >> (64 - ROOK_RELEVANT_BITS[sq]);
                        rook_attacks[sq][magic_index as usize] = DynamicAttacks::rookie(sq as u64, *occupancy).into();
                    }
                }
            }


        }

        Self { rook_attacks, bishop_attacks, rook_bitboards, bishop_bitboards }
    }

    pub(crate) fn get_bishop_attacks(&self, sq: Square, occupancy: u64) -> u64 {
        let mut occ = occupancy;
        let sq = sq as usize;


        occ &= self.bishop_bitboards[sq];
        occ = occ.wrapping_mul(BISHOP_MAGIC_NUMBERS[sq]);
        occ >>= 64 - BISHOP_RELEVANT_BITS[sq];

        return self.bishop_attacks[sq][occ as usize]
    }

    pub(crate) fn get_rook_attacks(&self, sq: Square, occupancy: u64) -> u64 {
        let mut occ = occupancy;
        let sq = sq as usize;
        // get bishop attacks assuming current board occupancy
        occ &= self.rook_bitboards[sq];
        occ = occ.wrapping_mul(ROOK_MAGIC_NUMBERS[sq]);
        occ >>= 64 - ROOK_RELEVANT_BITS[sq];

        self.rook_attacks[sq][occ as usize]
    }
}