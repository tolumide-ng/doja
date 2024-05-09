use crate::{color::Color, constants::{NOT_A_FILE, NOT_H_FILE}, Bitboard};


pub struct Pawn;

impl Pawn {
    /// ::::Generate Pawn Attacks::::
    /// Returns all the positions a pawn(at this position, with this color)
    /// can attack (move to from it's current position (square))
    pub fn mask_pawn_attacks(color: Color, square: u64) -> Bitboard {
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
            attacks[Color::White as usize][i] = Pawn::mask_pawn_attacks(Color::White, i as u64);
            attacks[Color::Black as usize][i] = Pawn::mask_pawn_attacks(Color::Black, i as u64);

        }
        attacks
    }
}
