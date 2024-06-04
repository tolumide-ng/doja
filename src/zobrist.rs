use bitflags::Flags;

use crate::{board::{board_state::BoardState, piece::Piece}, color::Color, constants::{RANDOM_STATE_SEED, TOTAL_PIECES, TOTAL_SQUARES}, squares::Square, utils::prng::PRNG};


type PieceKeys = [[u64; TOTAL_SQUARES]; TOTAL_PIECES];

pub(crate) const START_POSITION_ZOBRIST: u64 = 0xb1fd47c5345382d7;

#[derive(Debug)]
pub struct Zobrist {
    // Zobrist key for each piece on each square on the baord
    pub(crate) piece_keys: PieceKeys,
    // Zobrist key for each enpassant capturable file
    pub(crate) enpassant_keys: [u64; TOTAL_SQUARES],
    pub(crate) castle_keys: [u64; 16],
    pub(crate) side_key: u64,
}


impl Zobrist {
    #[cold]
    pub(crate) fn init_zobrist() -> Self {
        let mut piece_keys: PieceKeys = [[0; TOTAL_SQUARES]; TOTAL_PIECES];
        
        let mut rand = PRNG::new(RANDOM_STATE_SEED);
        for piece in Piece::ascii_pieces() {
            for sq in (0..TOTAL_SQUARES).rev() {
                // fill a table of random numbers/bitstrings   
                piece_keys[piece][sq] = rand.get_random_u64();
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


    // pub(crate) fn hash_key(&self, board: &BoardState) -> u64 {
    //     let mut final_key = 0u64;

    //      for piece in Piece::ascii_pieces() {
    //         // bitboard containing all pieces of this type
    //         let mut bitboard = *board[piece];

    //         while bitboard != 0 {
    //             let sq = Square::from(u64::from(bitboard.trailing_zeros()));
    //             final_key ^= self.piece_keys[piece][sq];

    //             // pop LS1B
    //             bitboard &= bitboard -1;
    //         }
    //     }
    //     println!("before >>>>>>>>> {0:x}", final_key);

    //     let index = usize::from_str_radix(&board.castling_rights.bits().to_string(), 10).unwrap();
    //     final_key ^= self.castle_keys[index];

    //     println!("before color||| {0:x}", final_key);

    //     if board.turn == Color::Black {final_key ^= self.side_key};

    //     println!("{0:x}", final_key);

    //     final_key
    // }
}