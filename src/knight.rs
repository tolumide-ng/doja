use crate::{constants::{NOT_AB_FILE, NOT_A_FILE, NOT_GH_FILE, NOT_H_FILE}, Mask};

pub struct Knight;

impl Knight {
    pub fn mask_knight_attacks(square: u64) -> Mask {
        let mut attack = Mask::new();

        
        let mut mask = Mask::new();
        mask.set_bit(square);
        
        println!("the board before \n {:#?}", mask.to_string());


        if ((*mask >> 17) & NOT_H_FILE) != 0 {
            attack.0 |= *mask >> 17;
        }

        if((*mask >> 15) & NOT_A_FILE) !=0 {
            attack.0 |= *mask >> 15;
        }

        if((*mask >> 10) & NOT_GH_FILE) != 0 { 
            attack.0 |= *mask >> 10;
        }

        if((*mask >> 6) & NOT_AB_FILE) != 0 {
            attack.0 |= *mask >> 6;
        }

        if((*mask << 6) & NOT_GH_FILE) != 0 {
            attack.0 |= *mask << 6;
        }
        
        if((*mask << 10) & NOT_AB_FILE) != 0 { 
            attack.0 |= *mask << 10;
        }

        if((*mask << 15) & NOT_H_FILE) !=0 {
            attack.0 |= *mask << 15;
        }

        if ((*mask << 17) & NOT_A_FILE) != 0 {
            attack.0 |= *mask << 17;
        }

        
        
        attack
    }



    pub fn init_leapers_attack() -> Vec<Mask> {
        // result attacks bitbord
        let mut attacks: Vec<Mask> =  Vec::with_capacity(8*8);

        for i in 0..64_u64 {
            attacks.push(Self::mask_knight_attacks(i));
        }

        attacks
    }
}