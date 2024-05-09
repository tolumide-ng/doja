use crate::{constants::{NOT_A_FILE, NOT_H_FILE}, squares::Square, Bitboard};

pub struct King;


impl King {
    /// Generate a mask with all the positions(squars) a King on this square can attack
    pub fn mask_king_attacks(square: u64) -> u64 {

        let mut bitboard = Bitboard::new();
        let mut attack = Bitboard::new();

        bitboard.set_bit(square);

        if (bitboard.0 >> 8) != 0 {
            attack.0 |= bitboard.0 >> 8;
        }

        if ((bitboard.0 >> 9) & NOT_H_FILE) != 0 {
            attack.0 |= bitboard.0 >> 9; 
        }

        if ((bitboard.0 >> 7) & NOT_A_FILE) != 0 {
            attack.0 |= bitboard.0 >> 7; 
        }

        if ((bitboard.0 >> 1) & NOT_H_FILE) != 0 {
            attack.0 |= bitboard.0 >> 1; 
        }




        if (bitboard.0 << 8) != 0 {
            attack.0 |= bitboard.0 << 8;
        }

        if ((bitboard.0 << 9) & NOT_A_FILE) != 0 {
            attack.0 |= bitboard.0 << 9; 
        }

        if ((bitboard.0 << 7) & NOT_H_FILE) != 0 {
            attack.0 |= bitboard.0 << 7; 
        }

        if ((bitboard.0 << 1) & NOT_A_FILE) != 0 {
            attack.0 |= bitboard.0 << 1; 
        }

        u64::from(attack)
    }


    pub fn init_leapers_attack() -> Vec<u64> {
        let mut attacks = Vec::with_capacity(64);

        for i in 0..64_u64 {
            attacks.push(Self::mask_king_attacks(i))
        }
        attacks
    }
}