use crate::{constants::{NOT_A_FILE, NOT_H_FILE}, squares::Square, Mask};

pub struct King;


impl King {
    pub fn mask_knight_attacks(square: u64) -> Mask {

        let mut mask = Mask::new();
        let mut attack = Mask::new();

        mask.set_bit(square);

        if (mask.0 >> 8) != 0 {
            attack.0 |= mask.0 >> 8;
        }

        if ((mask.0 >> 9) & NOT_H_FILE) != 0 {
            attack.0 |= mask.0 >> 9; 
        }

        if ((mask.0 >> 7) & NOT_A_FILE) != 0 {
            attack.0 |= mask.0 >> 7; 
        }

        if ((mask.0 >> 1) & NOT_H_FILE) != 0 {
            attack.0 |= mask.0 >> 1; 
        }




        if (mask.0 << 8) != 0 {
            attack.0 |= mask.0 << 8;
        }

        if ((mask.0 << 9) & NOT_A_FILE) != 0 {
            attack.0 |= mask.0 << 9; 
        }

        if ((mask.0 << 7) & NOT_H_FILE) != 0 {
            attack.0 |= mask.0 << 7; 
        }

        if ((mask.0 << 1) & NOT_A_FILE) != 0 {
            attack.0 |= mask.0 << 1; 
        }

        attack
    }


    pub fn init_leapers_attack() -> Vec<Mask> {
        let mut attacks = Vec::with_capacity(64);

        for i in 0..64_u64 {
            attacks.push(Self::mask_knight_attacks(i))
        }
        attacks
    }
}