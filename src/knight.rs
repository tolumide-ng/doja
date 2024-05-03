use crate::{constants::{NOT_AB_FILE, NOT_A_FILE, NOT_GH_FILE, NOT_H_FILE}, BitBoard};

pub struct Knight;

impl Knight {
    pub fn mask_knight_attacks(square: u64) -> BitBoard {
        let mut attack = BitBoard::new();

        
        let mut bitboard = BitBoard::new();
        bitboard.set_bit(square);
        
        println!("the board before \n {:#?}", bitboard.to_string());


        if ((*bitboard >> 17) & NOT_H_FILE) != 0 {
            attack.0 |= *bitboard >> 17;
        }

        if((*bitboard >> 15) & NOT_A_FILE) !=0 {
            attack.0 |= *bitboard >> 15;
        }

        if((*bitboard >> 10) & NOT_GH_FILE) != 0 { 
            attack.0 |= *bitboard >> 10;
        }

        if((*bitboard >> 6) & NOT_AB_FILE) != 0 {
            attack.0 |= *bitboard >> 6;
        }

        if((*bitboard << 6) & NOT_GH_FILE) != 0 {
            attack.0 |= *bitboard << 6;
        }
        
        if((*bitboard << 10) & NOT_AB_FILE) != 0 { 
            attack.0 |= *bitboard << 10;
        }

        if((*bitboard << 15) & NOT_H_FILE) !=0 {
            attack.0 |= *bitboard << 15;
        }

        if ((*bitboard << 17) & NOT_A_FILE) != 0 {
            attack.0 |= *bitboard << 17;
        }

        
        
        attack
    }



    pub fn init_leapers_attack() -> Vec<BitBoard> {
        // result attacks bitbord
        let mut attacks: Vec<BitBoard> =  Vec::with_capacity(8*8);

        for i in 0..64_u64 {
            attacks.push(Self::mask_knight_attacks(i));
        }

        attacks
    }
}