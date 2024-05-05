use crate::{bishop::Bishop, knight::Knight, pawn::Pawn, BitBoard};

pub struct DynamicAttacks;

// https://www.chessprogramming.net/generating-magic-multipliers/
impl DynamicAttacks {
    /// AND_RESULT
    pub fn bishop(square: u64, block: u64) -> BitBoard {
          let mut attack = BitBoard::new();

        let target_rank = square / 8;
        let target_file = square % 8;


        // Generate bishop attacks
        // mask relevant bishop occupancy bits
        let (mut rank, mut file) = (target_rank+1, target_file+1);
        while rank <= 7 && file <=7 { // bottom right            
            attack.0 |= 1 << (rank * 8 + file);
            if (1 << ((rank * 8) + file)) & block != 0 {break} //  AND_RESULT
            rank+=1; file+=1;
            // println!("|||||||| {}", rank * 8 + file);
        }


        if target_rank > 0 && target_file > 0 { // this helps us avoid overflow (subtracting from 0)
            // mask relevant bishop occupancy bits
            let (mut rank, mut file) = (target_rank, target_file);
            while rank > 0 && file > 0 { // top left
                rank-=1; file-=1;
                 attack.0 |= 1 << (rank * 8 + file);
                 if (1 << ((rank * 8) + file)) & block != 0 {break} //  AND_RESULT (if this spot is already filled)
             }
         }


         if target_file > 0 {
             let (mut rank, mut file) = (target_rank, target_file);
             while rank < 7 && file > 0 { // bottom left
                rank+=1; file-=1;
                 attack.0 |= 1 << (rank * 8 + file);
                 if (1 << ((rank * 8) + file)) & block != 0 {break} //  AND_RESULT
             }
         }

        //  println!("the target rank is {target_rank}");

         if target_rank > 0 {
             let (mut rank, mut file) = (target_rank, target_file);
             while rank > 0 && file < 7 { // top right
                rank-=1; file+=1;
                 attack.0 |= 1 << (rank * 8 + file);
                 if (1 << ((rank * 8) + file)) & block != 0 {break} //  AND_RESULT
             }
         }

        attack
    }


    pub fn rookie(square: u64, block: u64) -> BitBoard {
        let mut attack = BitBoard::new();

        let target_rank = square / 8;
        let target_file  = square %8;


        // top
        if target_rank > 0 {
            let mut rank = target_rank - 1;
            while rank >= 0 {
                attack.0 |= 1 << ((rank * 8) + target_file);
                if 1 << ((rank * 8) + target_file) & block != 0 {break}
                if rank == 0 {break}
                rank -=1;
            }
        }


        // bottom
        let mut rank = target_rank + 1;
        while rank <= 7 {
            attack.0 |= 1 << ((8 * rank) + target_file);
            if 1 << ((8 * rank) + target_file) & block != 0 {break}
            rank +=1;
        }


        //right
        let mut file = target_file +1;
        while file <= 7 {
            attack.0 |= 1 << ((target_rank * 8) + file);
            if 1 << ((target_rank * 8) + file) & block != 0 {break}
            file +=1;
        }

        // left
        if target_file > 0 {
            let mut file = target_file - 1;
            while file >= 0 {
                attack.0 |= 1<<((target_rank *8) + file);
                if 1<<((target_rank *8) + file) & block != 0 {break}
                if file == 0 {break}
                file-=1;
            }

        }
        attack
    }

    fn init_leapers() {
        let bishop = Bishop::init_leapers_attack();
        let knight = Knight::init_leapers_attack();
        let pawn = Pawn::init_leapers_attack();
    }

    fn init_sliders_attacks(bishop: bool) {
        // loop over 64 board squares
        for sq in 0..64 {
            // bish
        }

        // init bishop & rook masks
    }

}





#[cfg(test)]
mod tests {
    use super::*;
}