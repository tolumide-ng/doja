use crate::{color::Color, constants::{NOT_A_FILE, NOT_H_FILE}, Bitboard};


pub struct Pawn;

impl Pawn {
    /// result attacks bitboard
    pub fn bitboard_pawn_attacks(color: Color, square: u64) -> Bitboard {
        let mut attacks = Bitboard::new();

        // piece board
        let mut bitboard = Bitboard::new();
        bitboard.set_bit(square);
         
        match color {
            Color::Black => {
                if ((bitboard.0 << 7) & NOT_H_FILE) != 0 {
                    attacks.0 |= bitboard.0 << 7;
                }

                if ((bitboard.0 << 9) & NOT_A_FILE) != 0 {
                    attacks.0 |= bitboard.0 << 9;
                }
            }
            Color::White => {
                if ((bitboard.0 >> 7) & NOT_A_FILE) != 0 {
                    attacks.0 |= bitboard.0 >> 7;
                }

                if ((bitboard.0 >> 9) & NOT_H_FILE) != 0{
                    attacks.0 |= bitboard.0 >> 9;
                }
            }
            Color::Both => {}
        }
        attacks
    }

    pub fn init_leapers_attack() -> Vec<Vec<Bitboard>> {
        let mut attacks: Vec<Vec<Bitboard>> = vec![vec![Bitboard::new(); 8*8]; 2];

        for i in 0..64 {
            attacks[Into::<usize>::into(Color::White)][i] = Pawn::bitboard_pawn_attacks(Color::White, i as u64);
            attacks[Into::<usize>::into(Color::Black)][i] = Pawn::bitboard_pawn_attacks(Color::Black, i as u64);
        }
        attacks
    }
}
