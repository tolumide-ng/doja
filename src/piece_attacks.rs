use crate::{color::Color, constants::{NOT_AB_FILE, NOT_A_FILE, NOT_GH_FILE, NOT_H_FILE, PLAYERS_COUNT, SQUARES}, squares::{BISHOP_MAGIC_NUMBERS, BISHOP_RELEVANT_BITS, ROOK_MAGIC_NUMBERS, ROOK_RELEVANT_BITS}, Bitboard};

pub struct PieceAttacks {
    pub(crate) king_attacks: [u64; SQUARES],
    pub(crate) knight_attacks: [u64; SQUARES],
    pub(crate) bishop_masks: [u64; SQUARES],
    pub(crate) rook_masks: [u64; SQUARES],
    pub(crate) pawn_attacks: [[u64; SQUARES]; PLAYERS_COUNT]
}


impl PieceAttacks {
    // Generates all the possible piece attacks, which are then accessible from the struct
    pub(crate) fn new() -> Self {
        let mut king_attacks: [u64; SQUARES] = [0; SQUARES];
        let mut knight_attacks: [u64; SQUARES] = [0; SQUARES];
        let mut bishop_masks: [u64; SQUARES] = [0; SQUARES];
        let mut rook_masks: [u64; SQUARES] = [0; SQUARES];
        let mut pawn_attacks: [[u64; SQUARES]; PLAYERS_COUNT] = [[0; SQUARES]; PLAYERS_COUNT];


        for sq in 0..64u64 {
            let index = sq as usize;
            king_attacks[index] = Self::mask_king_attacks(sq);
            knight_attacks[index] = Self::mask_knight_attacks(sq);
            bishop_masks[index] = Self::mask_bishop_attacks(sq);
            rook_masks[index] = Self::mask_rook_attacks(sq);
            pawn_attacks[Color::White as usize][index] = Self::mask_pawn_attacks(Color::White, sq);
            pawn_attacks[Color::Black as usize][index] = Self::mask_pawn_attacks(Color::Black, sq);
        }



        Self { king_attacks, knight_attacks, bishop_masks, pawn_attacks, rook_masks }
    }

    fn mask_king_attacks(square: u64) -> u64 {
        let mut bitboard = Bitboard::new();
        bitboard.set_bit(square);
        let mut attack = 0u64;

        if (*bitboard >> 8) != 0 { attack |= *bitboard >> 8 }
        if ((*bitboard >> 9) & NOT_H_FILE) != 0 {attack |= *bitboard >> 9}
        if ((*bitboard >> 7) & NOT_A_FILE) != 0 {attack |= *bitboard >> 7}
        if ((*bitboard >> 1) & NOT_H_FILE) != 0 {attack |= *bitboard >> 1}

        if (*bitboard << 8) != 0 {attack |= *bitboard << 8}
        if ((*bitboard << 9) & NOT_A_FILE) != 0 {attack |= *bitboard << 9}
        if ((*bitboard << 7) & NOT_H_FILE) != 0 {attack |= *bitboard << 7}
        if ((*bitboard << 1) & NOT_A_FILE) != 0 {attack |= *bitboard << 1}

        attack
    }


    /// Generate knight attacks from this square
    fn mask_knight_attacks(square: u64) -> u64 {
        let mut attack = 0u64;
        let mut bitboard = Bitboard::new();
        bitboard.set_bit(square);


        if (*bitboard >> 17)  & NOT_H_FILE   != 0 {attack |= *bitboard >> 17}
        if ((*bitboard >> 15) & NOT_A_FILE)  !=0 {attack |= *bitboard >> 15}
        if ((*bitboard >> 10) & NOT_GH_FILE) != 0 {attack |= *bitboard >> 10}
        if ((*bitboard >> 6)  & NOT_AB_FILE) != 0 {attack |= *bitboard >> 6}

        if ((*bitboard << 6)  & NOT_GH_FILE) != 0 { attack |= *bitboard << 6 }
        if ((*bitboard << 10) & NOT_AB_FILE) != 0 { attack |= *bitboard << 10 }
        if ((*bitboard << 15) & NOT_H_FILE)  !=0 { attack |= *bitboard << 15}
        if ((*bitboard << 17) & NOT_A_FILE)  != 0 { attack |= *bitboard << 17 }
        
        attack
    }

    /// Generate bishop attacks from this square
    fn mask_bishop_attacks(sq: u64) -> u64 {
        let mut attack = 0u64;
        let target_rank = sq / 8;
        let target_file = sq % 8;


        // bitboard relevant bishop occupancy bits
        let (mut rank, mut file) = (target_rank+1, target_file+1); // bottom right
        while rank <= 6 && file <=6 { attack |= 1 << (rank * 8 + file); rank+=1; file+=1; }

        if target_rank > 0 && target_file > 0 { // this helps us avoid overflow (subtracting from 0)
            // bitboard relevant bishop occupancy bits
            let (mut rank, mut file) = (target_rank-1, target_file-1); // top left
            while rank >= 1 && file >= 1 {  attack |= 1 << (rank * 8 + file); rank-=1; file-=1; }
        }

        if target_file > 0 {
            let (mut rank, mut file) = (target_rank+1, target_file-1); // bottom left
            while rank <= 6 && file >= 1 { attack |= 1 << (rank * 8 + file); rank+=1; file-=1; }
        }

        if target_rank > 0 {
            let (mut rank, mut file) = (target_rank-1, target_file+1); // top right
            while rank >= 1 && file <= 6 { attack |= 1 << (rank * 8 + file); rank-=1; file+=1; }
        }
        
        attack
    }


    /// Generate pawn attacks from this square
    /// positions this pawn can move to
    fn mask_pawn_attacks(color: Color, sq: u64) -> u64 {
        let mut attacks = 0u64;
        let mut bitboard = Bitboard::new(); // piece board
        bitboard.set_bit(sq);
         
        match color {
            Color::Black => {
                if ((*bitboard << 7) & NOT_H_FILE) != 0 { attacks |= *bitboard << 7 }
                if ((*bitboard << 9) & NOT_A_FILE) != 0 { attacks |= *bitboard << 9 }
            }
            Color::White => {
                if ((*bitboard >> 7) & NOT_A_FILE) != 0 { attacks |= *bitboard >> 7 }
                if ((*bitboard >> 9) & NOT_H_FILE) != 0{ attacks |= *bitboard >> 9 }
            }
            Color::Both => {}
        }
        attacks
    }


    /// Generate rook attachs from this square
    fn mask_rook_attacks(sq: u64) -> u64 {
        let mut attack = 0u64;
        let target_rank = sq / 8;
        let target_file  = sq % 8;

        if target_rank > 0 { // top
            let mut rank = target_rank - 1;
            while rank > 0 { 
                attack |= 1 << ((rank * 8) + target_file); 
                rank -=1 
            }
        }

        let mut rank = target_rank + 1; // bottom
        while rank <= 6 { 
            attack |= 1 << ((8 * rank) + target_file);
            rank +=1;
        }

        let mut file = target_file +1; //right
        while file <= 6 {
            attack |= 1 << ((target_rank * 8) + file);
            file +=1;
        }

        if target_file > 0 {
            let mut file = target_file -1; // left
            while file > 0 {
                attack |= 1<<((target_rank *8) + file);
                file-=1;
            }
        }

        attack
    }


    
    /// Gets the bishoop attacks from position sq,
    /// while stopping if there is any peice blocking the attack direction
    /// https://www.chessprogramming.net/generating-magic-multipliers/
    pub(crate) fn get_bishop_attacks_on_the_fly(&self, sq: u64, block: u64) -> u64 {
        let mut attack = 0;
        let target_rank = sq / 8; let target_file = sq % 8;

        // Generate bishop attacks
        // bitboard relevant bishop occupancy bits
        let (mut rank, mut file) = (target_rank, target_file);
        while rank < 7 && file <7 { // bottom right            
            rank+=1; file+=1;
            attack |= 1 << (rank * 8 + file);
            if (1 << ((rank * 8) + file)) & block != 0 {break} //  AND_RESULT
        }

        if target_rank > 0 && target_file > 0 { // this helps us avoid overflow (subtracting from 0)
            // bitboard relevant bishop occupancy bits
            let (mut rank, mut file) = (target_rank, target_file);
            while rank > 0 && file > 0 { // top left
                rank-=1; file-=1;
                 attack |= 1 << (rank * 8 + file);
                 if (1 << ((rank * 8) + file)) & block != 0 {break} //  AND_RESULT (if this spot is already filled)
             }
         }

         if target_file > 0 {
             let (mut rank, mut file) = (target_rank, target_file);
             while rank < 7 && file > 0 { // bottom left
                rank+=1; file-=1;
                 attack |= 1 << (rank * 8 + file);
                 if (1 << ((rank * 8) + file)) & block != 0 {break} //  AND_RESULT
             }
         }

         if target_rank > 0 {
             let (mut rank, mut file) = (target_rank, target_file);
             while rank > 0 && file < 7 { // top right
                rank-=1; file+=1;
                 attack |= 1 << (rank * 8 + file);
                 if (1 << ((rank * 8) + file)) & block != 0 {break} //  AND_RESULT
             }
         }

        attack
    }
    

    // https://www.chessprogramming.net/generating-magic-multipliers/
    pub(crate) fn get_rook_attacks_on_the_fly(&self, sq: u64, block: u64) -> u64 {
        let mut attack = 0u64;
        let target_rank = sq / 8; let target_file  = sq %8;

        if target_rank > 0 { 
            let mut rank = target_rank; // top
            while rank > 0 {
                rank -=1;
                attack |= 1 << ((rank * 8) + target_file);
                if 1 << ((rank * 8) + target_file) & block != 0 {break}
                if rank == 0 {break}
            }
        }

        let mut rank = target_rank; // bottom
        while rank < 7 {
            rank +=1;
            attack |= 1 << ((8 * rank) + target_file);
            if 1 << ((8 * rank) + target_file) & block != 0 {break}
        }

        let mut file = target_file;
        while file < 7 { //right
            file +=1;
            attack |= 1 << ((target_rank * 8) + file);
            if 1 << ((target_rank * 8) + file) & block != 0 {break}
        }

        if target_file > 0 {
            let mut file = target_file; // left
            while file > 0 {
                file-=1;
                attack |= 1<<((target_rank *8) + file);
                if 1<<((target_rank *8) + file) & block != 0 {break}
                if file == 0 {break}
            }
        }

        attack
    }


    fn init_sliders_attacks(&self, bishop: bool) -> Vec<Vec<u64>> {
        let mut bishop_attacks: Vec<Vec<u64>> = vec![vec![0; 512]; 64];
        let mut rook_attacks: Vec<Vec<u64>> = vec![vec![0; 4096]; 64];


        for sq in 0..64_usize {
            // init current bitboard
            let attack_bitboard = match bishop {
                true => self.bishop_masks[sq],
                false => self.rook_masks[sq]
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
                        bishop_attacks[sq][magic_index as usize] = self.get_bishop_attacks_on_the_fly(sq as u64, *occupancy).into();

                    }
                    false => {
                        let occupancy = Bitboard::from(attack_bitboard).set_occupancy(index, relevant_bits_count);
                        let magic_index = (*occupancy).wrapping_mul(ROOK_MAGIC_NUMBERS[sq]) >> (64 - ROOK_RELEVANT_BITS[sq]);
                        rook_attacks[sq][magic_index as usize] = self.get_rook_attacks_on_the_fly(sq as u64, *occupancy).into();
                    }
                }
            }
        }

        match bishop {
            true => bishop_attacks,
            false => rook_attacks
        }
    }


    pub(crate) fn get_bishop_attacks(&self, sq: u64, occupancy: u64) -> u64 {
        let bishop_attacks = self.init_sliders_attacks(true);

        let mut occ = occupancy;
        let sq = sq as usize;

        occ &= self.bishop_masks[sq];
        occ = occ.wrapping_mul(BISHOP_MAGIC_NUMBERS[sq]);
        occ >>= 64 - BISHOP_RELEVANT_BITS[sq];

        return bishop_attacks[sq][occ as usize]
    }

    pub(crate) fn get_rook_attacks(&self, sq: u64, occupancy: u64) -> u64 {
        let rook_attacks = self.init_sliders_attacks(false);

        let mut occ = occupancy;
        let sq = sq as usize;
        occ &= self.rook_masks[sq]; // get bishop attacks assuming current board occupancy
        occ = occ.wrapping_mul(ROOK_MAGIC_NUMBERS[sq]);
        occ >>= 64 - ROOK_RELEVANT_BITS[sq];

        rook_attacks[sq][occ as usize]
    }

    pub(crate) fn get_queen_attacks(&self, sq: u64, occupancy: u64) -> u64 {
        let bishop_attacks = self.get_bishop_attacks(sq, occupancy);
        let rook_attacks = self.get_rook_attacks(sq, occupancy);
        
        let queen_attacks = bishop_attacks | rook_attacks;

        queen_attacks
    }

}