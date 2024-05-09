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
    /// Returns: (attacks, bitboard)
    fn init_sliders_attacks(bishop: bool) -> (Vec<Vec<u64>>, Vec<u64>) {
        let mut bishop_attacks: Vec<Vec<u64>> = vec![vec![0; 512]; 64];
        let mut rook_attacks: Vec<Vec<u64>> = vec![vec![0; 4096]; 64];
        let mut bishop_bitboards: Vec<u64> = vec![0; 64];
        let mut rook_bitboards: Vec<u64> = vec![0; 64];

        for sq in 0..64_usize {
            bishop_bitboards[sq] = Bishop::bitboard_bishop_attack(sq as u64).into();
            rook_bitboards[sq] = Rook::bitboard_rook_attacks(sq as u64).into();

            // init current bitboard
            let attack_bitboard = match bishop {
                true => bishop_bitboards[sq],
                false => rook_bitboards[sq]
            };

            // init relevant occupancy bit count
            let relevant_bits_count = Bitboard::from(attack_bitboard).count_bits();
            
            let occupany_indices = 1 << relevant_bits_count;
            
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
                        let magic_index = (*occupancy).wrapping_mul(ROOK_MAGIC_NUMBERS[sq]) >> (64 - ROOK_RELEVANT_BITS[sq]);
                        rook_attacks[sq][magic_index as usize] = DynamicAttacks::rookie(sq as u64, *occupancy).into();
                    }
                }
            }
        }

        match bishop {
            true => (bishop_attacks, bishop_bitboards),
            false => (rook_attacks, rook_bitboards)
        }
    }

    pub(crate) fn get_bishop_attacks(sq: Square, occupancy: u64) -> u64 {
        let (bishop_attacks, bishop_bitboards) = Self::init_sliders_attacks(true);

        let mut occ = occupancy;
        let sq = sq as usize;


        occ &= bishop_bitboards[sq];
        occ = occ.wrapping_mul(BISHOP_MAGIC_NUMBERS[sq]);
        occ >>= 64 - BISHOP_RELEVANT_BITS[sq];

        return bishop_attacks[sq][occ as usize]
    }

    pub(crate) fn get_rook_attacks(sq: Square, occupancy: u64) -> u64 {
        let (rook_attacks, rook_bitboards) = Self::init_sliders_attacks(false);

        let mut occ = occupancy;
        let sq = sq as usize;
        // get bishop attacks assuming current board occupancy
        occ &= rook_bitboards[sq];
        occ = occ.wrapping_mul(ROOK_MAGIC_NUMBERS[sq]);
        occ >>= 64 - ROOK_RELEVANT_BITS[sq];

        rook_attacks[sq][occ as usize]
    }

    pub(crate) fn get_queen_attacks(sq: Square, occupancy: u64) -> u64 {
        let bishop_attacks = Self::get_bishop_attacks(sq, occupancy);
        let rook_attacks = Self::get_rook_attacks(sq, occupancy);
        
        let queen_attacks = bishop_attacks | rook_attacks;

        queen_attacks
    }
}