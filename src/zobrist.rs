use crate::{board::piece::Piece, constants::{RANDOM_STATE_SEED, TOTAL_PIECES, TOTAL_SQUARES}};
use crate::utils::rand::Rand;



#[derive(Debug)]
pub struct Zobrist {
    pub(crate) piece_keys: [[u64; TOTAL_PIECES]; TOTAL_SQUARES],
    pub(crate) enpassant_keys: [u64; TOTAL_SQUARES],
    pub(crate) castle_keys: [u64; 16],
    pub(crate) side_key: u64,
}


impl Zobrist {
    #[cold]
    pub(crate) fn init_zobrist() -> Self {
        let mut piece_keys: [[u64; TOTAL_PIECES]; TOTAL_SQUARES] = [[0; TOTAL_PIECES]; TOTAL_SQUARES];
        
        let mut rand = Rand::new(RANDOM_STATE_SEED);
        for piece in Piece::ascii_pieces() {
            for sq in 0..TOTAL_SQUARES {
                // fill a table of random numbers/bitstrings   
                piece_keys[sq][piece] = rand.get_random_u64();
            }
        }

        let mut enpassant_keys: [u64; TOTAL_SQUARES] = [0; TOTAL_SQUARES];
        for sq in 0..TOTAL_SQUARES {
            enpassant_keys[sq] = rand.get_random_u64();
        }
        
        let mut castle_keys: [u64; 16] = [0; 16];
        for index in 0..16 {
            castle_keys[index] = rand.get_random_u64();
        }

        let side_key = rand.get_random_u64();

        Self {piece_keys, enpassant_keys, castle_keys, side_key}
    }    
}