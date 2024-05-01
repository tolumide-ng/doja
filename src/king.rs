use crate::{constants::{NOT_A_FILE, NOT_H_FILE}, squares::Square, BitBoard};

pub struct King;


impl King {
    pub fn mask_knight_attacks(square: u64) -> BitBoard {

        let mut bitboard = BitBoard::new();
        let mut attack = BitBoard::new();

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

        attack
    }


    pub fn init_leapers_attack() -> Vec<BitBoard> {
        let mut attacks = Vec::with_capacity(64);

        for i in 0..64_u64 {
            attacks.push(Self::mask_knight_attacks(i))
        }
        attacks
    }
}