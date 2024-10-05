use crate::{bitboard::Bitboard, constants::{A_FILE, B_FILE, C_FILE, D_FILE, E_FILE, FILE, F_FILE, G_FILE, H_FILE, RANK, RANK_1, RANK_2, RANK_3, RANK_4, RANK_5, RANK_6, RANK_7, RANK_8}};



pub struct EvaluationMasks {
    pub(crate) file_masks: [u64; 64],
    pub(crate) rank_masks: [u64; 64],
    pub(crate) isolated_masks: [u64; 64],
    pub(crate) white_passed_masks: [u64; 64],
    pub(crate) black_passed_masks: [u64; 64],
}

impl EvaluationMasks {
    fn set_file_rank_mask(sq_file: Option<u8>, sq_rank: Option<u8>) -> u64 {
        let mut bitboard = Bitboard::new();
        // array containing masks for each filled rank e.g the first item here means rank 1 is filled
        let rank_masks = [RANK_1, RANK_2, RANK_3, RANK_4, RANK_5, RANK_6, RANK_7, RANK_8];
        // array containing masks for each filled file e.g the first item here means the A file is filled
        let file_masks = [A_FILE, B_FILE, C_FILE, D_FILE, E_FILE, F_FILE, G_FILE, H_FILE];
        
        if let Some(file) = sq_file {
           *bitboard |= file_masks[file as usize];
       } else if let Some(rank) = sq_rank {
            *bitboard |= rank_masks[rank as usize];
        }

        *bitboard
    }


    pub(crate) fn init() -> Self {
        let mut file_masks = [0; 64];
        let mut rank_masks = [0; 64];
        let mut isolated_masks = [0; 64];
        let mut white_passed_masks = [0; 64];
        let mut black_passed_masks = [0; 64];

        for rank in 0..RANK as u8 {
            for file in 0..FILE as u8 {
                let square = ((rank * 8) + file) as usize;
                file_masks[square] |= Self::set_file_rank_mask(Some(file), None);
                rank_masks[square] |= Self::set_file_rank_mask(None, Some(rank));

                
                if file > 0 {
                    isolated_masks[square] |= Self::set_file_rank_mask(Some(file - 1), None);
                    white_passed_masks[square] |= Self::set_file_rank_mask(Some(file-1), None);
                    black_passed_masks[square] |= Self::set_file_rank_mask(Some(file-1), None);
                }
                
                white_passed_masks[square] |= Self::set_file_rank_mask(Some(file), None);
                black_passed_masks[square] |= Self::set_file_rank_mask(Some(file), None);

                
                if file < (FILE-1)as u8 {
                    isolated_masks[square] |= Self::set_file_rank_mask(Some(file + 1), None);
                    white_passed_masks[square] |= Self::set_file_rank_mask(Some(file + 1), None);
                    black_passed_masks[square] |= Self::set_file_rank_mask(Some(file + 1), None);
                }

                
                let mut white = 0u64;
                let mut black = 0u64;
                for i in 0..=rank {
                    // let remove_rank_sq = ((i + 1) * 8) + file;
                    if i < 8 {
                        white |= Self::set_file_rank_mask(None, Some(i));
                    }
                    if i > 0 {
                        black |= Self::set_file_rank_mask(None, Some(i-1));
                    }
                }
                white_passed_masks[square] &= !white;
                black_passed_masks[square] &= black;

                // println!("for square {}", Square::from(square as u64));
                // println!("{}", Bitboard::from(black_passed_masks[square]));
            }
        }

        Self { file_masks, rank_masks, isolated_masks, white_passed_masks, black_passed_masks }
    }
}