use crate::{squares::Square, BitBoard};

pub struct Rook;

impl Rook {
    pub fn mask_rook_attacks(square: u64) -> BitBoard {
        let mut attack = BitBoard::new();

        let target_rank = square / 8;
        let target_file  = square %8;


        // top
        if target_rank > 0 {
            let mut rank = target_rank - 1;
            while rank > 0 {
                attack.0 |= 1 << ((rank * 8) + target_file);
                rank -=1;
            }
        }


        // bottom
        let mut rank = target_rank + 1;
        while rank <= 6 {
            attack.0 |= 1 << ((8 * rank) + target_file);
            rank +=1;
        }


        //right
        let mut file = target_file +1;
        while file <= 6 {
            attack.0 |= 1 << ((target_rank * 8) + file);
            file +=1;
        }

        // left
        if target_file > 0 {
            let mut file = target_file -1;
            while file > 0 {
                attack.0 |= 1<<((target_rank *8) + file);
                file-=1;
            }

        }

        attack
    }


    pub fn init_leapers_attack() -> Vec<BitBoard> {
        let mut attacks = Vec::with_capacity(64);
        for i in 0..64_u64 {
            attacks.push(Self::mask_rook_attacks(i));
        }
        attacks
    }
}