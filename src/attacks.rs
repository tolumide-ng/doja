use crate::BitBoard;

pub struct DynamicAttacks;

impl DynamicAttacks {
    pub fn dynamic_bishpp_attacks(square: u64, block: u64) -> BitBoard {
          let mut attack = BitBoard::new();

        let target_rank = square / 8;
        let target_file = square % 8;


        // println!("target_rank is {target_rank}, and target_file is {target_file}");

        // Generate bishop attacks

        // mask relevant bishop occupancy bits
        let (mut rank, mut file) = (target_rank+1, target_file+1);
        while rank <= 7 && file <=7 {
            // bottom right
            if 1 << ((rank * 8) + file) & block != 0 {break}
            
            attack.0 |= 1 << (rank * 8 + file);
            rank+=1; file+=1;
        }


        if target_rank > 0 && target_file > 0 { // this helps us avoid overflow (subtracting from 0)
            // top left
             // mask relevant bishop occupancy bits
             let (mut rank, mut file) = (target_rank-1, target_file-1);
             while rank >= 1 && file >= 1 {
                if (1 << ((rank * 8) + file)) & block != 0 {break}
                 attack.0 |= 1 << (rank * 8 + file);
                 rank-=1; file-=1;
             }
         }


         if target_file > 0 {
             let (mut rank, mut file) = (target_rank+1, target_file-1);
             while rank <= 7 && file >= 1 {
                 // bottom left
                 if (1 << ((rank * 8) + file)) & block != 0 {break}
                 attack.0 |= 1 << (rank * 8 + file);
                 rank+=1; file-=1;
             }
         }


         if target_rank > 0 {
             let (mut rank, mut file) = (target_rank-1, target_file+1);
             while rank >= 1 && file <= 7 {
                 // top right
                 if (1 << ((rank * 8) + file)) & block != 0 {break}
                 attack.0 |= 1 << (rank * 8 + file);
     
                 rank-=1; file+=1;
             }
         }



        attack
    }
}