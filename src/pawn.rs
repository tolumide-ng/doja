use crate::{color::Color, constants::{NOT_A_FILE, NOT_H_FILE}, BitBoard};


pub struct Pawn;

impl Pawn {
    /// result attacks bitboard
    pub fn mask_pawn_attacks(color: Color, square: u64) -> BitBoard {
        let mut attacks = BitBoard::new();

        // piece board
        let mut bit_board = BitBoard::new();
        bit_board.set_bit(square);
         
        match color {
            Color::Black => {
                if ((bit_board.0 << 7) & NOT_H_FILE) != 0 {
                    attacks.0 |= bit_board.0 << 7;
                }

                if ((bit_board.0 << 9) & NOT_A_FILE) != 0 {
                    attacks.0 |= bit_board.0 << 9;
                }
            }
            Color::White => {
                if ((bit_board.0 >> 7) & NOT_A_FILE) != 0 {
                    attacks.0 |= bit_board.0 >> 7;
                }

                if ((bit_board.0 >> 9) & NOT_H_FILE) != 0{
                    attacks.0 |= bit_board.0 >> 9;
                }
            }
        }
        attacks
    }

    pub fn init_leapers_attack() -> Vec<Vec<BitBoard>> {
        let mut attacks: Vec<Vec<BitBoard>> = vec![vec![BitBoard::new(); 8*8]; 2];

        for i in 0..64 {
            attacks[Into::<usize>::into(Color::White)][i] = Pawn::mask_pawn_attacks(Color::White, i as u64);
            attacks[Into::<usize>::into(Color::Black)][i] = Pawn::mask_pawn_attacks(Color::Black, i as u64);
        }
        attacks
    }
}
