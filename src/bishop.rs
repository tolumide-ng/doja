use crate::BitBoard;

pub struct Bishop;


impl Bishop {
    pub fn mask_bishop_attack(square: u64) -> BitBoard {
        let mut attack = BitBoard::new();

        let target_rank = square / 8;
        let target_file = square % 8;


        println!("target_rank is {target_rank}, and target_file is {target_file}");

        // mask relevant bishop occupancy bits
        let (mut rank, mut file) = (target_rank+1, target_file+1);
        while rank <= 6 && file <=6 {
            // bottom right
            attack.0 |= 1 << (rank * 8 + file);
            rank+=1; file+=1;
        }


        if target_rank > 0 && target_file > 0 { // this helps us avoid overflow (subtracting from 0)
             // mask relevant bishop occupancy bits
             let (mut rank, mut file) = (target_rank-1, target_file-1);
             while rank >= 1 && file >= 1 {
                 // top left
                 attack.0 |= 1 << (rank * 8 + file);
                 rank-=1; file-=1;
             }
         }


         if target_file > 0 {
             let (mut rank, mut file) = (target_rank+1, target_file-1);
             while rank <= 6 && file >= 1 {
                 // bottom left
                 attack.0 |= 1 << (rank * 8 + file);
                 rank+=1; file-=1;
             }
         }


         if target_rank > 0 {
             let (mut rank, mut file) = (target_rank-1, target_file+1);
             while rank >= 1 && file <= 6 {
                 // top right
                 attack.0 |= 1 << (rank * 8 + file);
     
                 rank-=1; file+=1;
             }
         }



        attack
    }


    pub fn init_leapers_attack() -> Vec<BitBoard> {
        let mut attacks = Vec::with_capacity(64);


        for i in 0..64_u64 {
            attacks.push(Self::mask_bishop_attack(i));
        }

        attacks
    }
}