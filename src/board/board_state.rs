use std::{fmt::Display, ops::{Deref, DerefMut}};

use crate::{bit_move::BitMove, board::board::Board, color::Color, constants::{NOT_A_FILE, NOT_H_FILE, OCCUPANCIES, PIECE_ATTACKS, RANK_4, RANK_5, SQUARES}, squares::Square, Bitboard};

use super::{castling::Castling, fen::FEN, piece::Piece};

pub struct BoardState {
    turn: Color,
    pub board: Board,
    castling_rights: Castling,
    enpassant: Option<Square>,
    occupancies: [u64; OCCUPANCIES], // 0-white, 1-black, 2-both
}


impl BoardState {
    pub fn new() -> BoardState {
        Self { board: Board::new(), turn: Color::White, enpassant: None, castling_rights: Castling::NONE, occupancies: [0; OCCUPANCIES], }
    }

    pub(crate) fn set_turn(&mut self, turn: Color) {
        self.turn = turn
    }

    pub(crate) fn set_enpassant(&mut self, enpassant: Option<Square>) {
        self.enpassant = enpassant;
    }

    pub(crate) fn set_castling(&mut self, castling: Castling) {
        self.castling_rights = castling
    }

    pub(crate) fn set_occupancy(&mut self, color: Color, occupancy: u64) {
        self.occupancies[color] |= occupancy;
    }

    pub(crate) fn get_occupancy(&self, color: Color) -> u64 {
        self.occupancies[color]
    }

    /// Given the current pieces on the board, is this square under attack by the given side (color)
    /// Getting attackable(reachable) spots from this square, it also means this square can be reached from those
    /// attackable spots 
    pub(crate) fn is_square_attacked(&self, sq: u64, i_am: Color) -> bool {
        let index = sq as usize;

        // Attacks by black pawn (can an attack by any black pawn on the board reach this sq)
        if i_am == Color::Black && PIECE_ATTACKS.pawn_attacks[Color::White as usize][index] & u64::from(self[Piece::BP as usize]) !=0 {return true};
        // Attacks by white pawn (can an attack by a white pawn reach this position)
        if i_am == Color::White && PIECE_ATTACKS.pawn_attacks[Color::Black as usize][index] & u64::from(self[Piece::WP as usize]) != 0 {return true};

        let knight_attacks = PIECE_ATTACKS.knight_attacks[index];
        // if there is a knight on this square, can it attack any of my knights(black) on the board
        if i_am == Color::Black && (knight_attacks & u64::from(self[Piece::BN as usize]) != 0) {return true};
        // if there is a knight on this square, can it attack any of my knights(white) on the board
        if i_am == Color::White && (knight_attacks & u64::from(self[Piece::WN as usize]) != 0) {return true};

        let king_attacks = PIECE_ATTACKS.king_attacks[index];
        if i_am == Color::Black && (king_attacks & u64::from(self[Piece::BK as usize])) != 0 {return true}
        if i_am == Color::White && (king_attacks & u64::from(self[Piece::WK as usize])) != 0 {return true}

        let bishop_attacks = PIECE_ATTACKS.get_bishop_attacks(sq, self.get_occupancy(Color::Both));
        if i_am == Color::Black && (bishop_attacks & u64::from(self[Piece::BB as usize])) != 0 {return true}
        if i_am == Color::White && (bishop_attacks & u64::from(self[Piece::WB as usize])) != 0 {return true}

        let rook_attacks = PIECE_ATTACKS.get_rook_attacks(sq, self.get_occupancy(Color::Both));
        if i_am == Color::Black && (rook_attacks & u64::from(self[Piece::BR as usize])) != 0 {return true}
        if i_am == Color::White && (rook_attacks & u64::from(self[Piece::WR as usize])) != 0 {return true}

        let queen_attacks = PIECE_ATTACKS.get_queen_attacks(sq, self.get_occupancy(Color::Both));
        if i_am == Color::Black && (queen_attacks & u64::from(self[Piece::BQ as usize])) != 0 {return true}
        if i_am == Color::White && (queen_attacks & u64::from(self[Piece::WQ as usize])) != 0 {return true}

        false
    }

    // print all the squares that the current color can attack or move to.
    pub(crate) fn get_possible_destination_squares_for(&self, side: Color) -> Bitboard {
        let mut sample_bitboard = Bitboard::new();

        for sq in 0..(SQUARES as u64) {
            if self.is_square_attacked(sq, side) {
                sample_bitboard.set_bit(sq)
            }
        }

        sample_bitboard
    }


    fn white_pawn_east_attacks(&self, pawns: bool) -> u64 { match pawns {
        true => self.board[Piece::WP].north_east(), false => {Bitboard::from(self.occupancies[Color::White]).north_east()} }}

    fn white_pawn_west_attacks(&self, pawns: bool) -> u64 { match pawns {
        true => self.board[Piece::WP].north_west(), false => {Bitboard::from(self.occupancies[Color::White]).north_west()} }}

    fn black_pawn_east_attacks(&self, pawns: bool) -> u64 { match pawns {
        true => self.board[Piece::BP].south_east(), false => {Bitboard::from(self.occupancies[Color::Black]).south_east()} }}

    fn black_pawn_west_attacks(&self, pawns: bool) -> u64 { match pawns {
        true => self.board[Piece::BP].south_west(), false => Bitboard::from(self.occupancies[Color::Black]).south_west() }}


    /// shows the position where the color's pawns can be attacked from
    pub(crate) fn pawn_any_attack(&self, color: Color, pawns: bool) -> u64{
        if color == Color::Black {
            return self.black_pawn_east_attacks(pawns) | self.black_pawn_west_attacks(pawns)
        }
        self.white_pawn_east_attacks(pawns) | self.white_pawn_west_attacks(pawns)
    }

    /// Shows the possible double pawn attacks for color
    pub(crate) fn pawn_double_attack(&self, color: Color) -> u64 {
        if color == Color::Black {
            return self.black_pawn_east_attacks(true) & self.black_pawn_west_attacks(true)
        }
        self.white_pawn_east_attacks(true) & self.white_pawn_west_attacks(true)
    }

    pub(crate) fn pawn_single_attack(&self, color: Color) -> u64 {
        if color == Color::Black {
            return self.black_pawn_east_attacks(true) ^ self.black_pawn_west_attacks(true)
        }
        self.white_pawn_east_attacks(true) ^ self.white_pawn_west_attacks(true)
    }

    /// https://www.chessprogramming.org/Pawn_Attacks_(Bitboards)
    pub(crate) fn safe_pawn_squares(&self, color: Color) -> u64 {
        let bpawn_double_attacks = self.pawn_double_attack(Color::Black);
        let wpawn_double_attacks = self.pawn_double_attack(Color::White);

        if color == Color::Black {
            let wpawn_any_attacks = self.pawn_any_attack(Color::White, true);
            let bpawn_odd_attacks = self.pawn_single_attack(Color::Black);
            return bpawn_double_attacks | !wpawn_any_attacks | (bpawn_odd_attacks ^ !wpawn_double_attacks);
        }

        let wpawn_odd_attacks = self.pawn_single_attack(Color::White);
        let bpawn_any_attacks = self.pawn_any_attack(Color::Black, true);
        wpawn_double_attacks | !bpawn_any_attacks | (wpawn_odd_attacks & !bpawn_double_attacks)
    }

    pub(crate) fn pawns_able_2capture_east(&self, color: Color) -> u64 {
        if color == Color::Black {
            return *self[Piece::BP] & self.white_pawn_west_attacks(false)
        }
        *self.board[Piece::WP] & self.black_pawn_west_attacks(false)
    }
    pub(crate) fn pawns_able_2capture_west(&self, color: Color) -> u64 {
        if color == Color::Black {
            return *self[Piece::BP] & self.white_pawn_east_attacks(false)
        }
        *self[Piece::WP] & self.black_pawn_east_attacks(false)
    }

    /// Returns the squares of this color capable of capturing other squares
    pub(crate) fn pawns_able_2capture_any(&self, color: Color) -> u64 {
        if color == Color::Black {
            return *self[Piece::BP] & self.pawn_any_attack(Color::White, false)
        }
        *self[Piece::WP] & self.pawn_any_attack(Color::Black, false)
    }

    // pub(crate) fn pawn_captures(&self, color: Color) -> u64 {}

    
    /// Push pawn(black or white) by one
    pub(crate) fn single_push_targets(&self, color: Color) -> u64 {
        if color == Color::Black {
            return self[Piece::BP].south() & !self.occupancies[Color::Both]
        }

        self[Piece::WP].north() & !self.occupancies[Color::Both]
    }
    
    /// Double push for pawns(black or white)
    /// https://www.chessprogramming.org/Pawn_Pushes_(Bitboards)
    pub(crate) fn double_push_targets(&self, color: Color) -> u64 {
        let single_push = Bitboard::from(self.single_push_targets(color));
        if color == Color::Black {
            return single_push.south() & !self.occupancies[Color::Both] & RANK_5
        }

        single_push.north() & !self.occupancies[Color::Both] & RANK_4
    }

    pub(crate) fn pawns_able_to_2push(&self, color: Color) -> u64 {
        if color == Color::White {
            return Bitboard::from(!self.occupancies[Color::Both]).south() & *self[Piece::WP]    
        }
        Bitboard::from(!self.occupancies[Color::Both]).north() & *self[Piece::BP]
    }

    pub(crate) fn pawns_able_to_double_push(&self, color: Color) -> u64 {
        let empty = !self.occupancies[Color::Both];
        if color == Color::Black {
            let empty_rank6 = Bitboard::from(empty & RANK_5).north() & empty;
            return Bitboard::from(empty_rank6).north() & *self[Piece::BP]
        }
        let empty_rank_3 = Bitboard::from(empty & RANK_4).south() & empty;
        return Bitboard::from(empty_rank_3).south() & *self[Piece::WP];
    }


    pub(crate) fn get_pawn_movement(&self, color: Color, double: bool) {
        match double {
            true => {
                let mut src2 = self.pawns_able_to_double_push(color);
                let mut target2 = self.double_push_targets(color);
                
                while src2 !=0 {
                    let sindex = src2.trailing_zeros() as u16;
                    let tindex = target2.trailing_zeros() as u16;

                    src2 &= src2 -1;
                    target2 &= target2 -1;
                    let piece = if color == Color::Black {Piece::BP} else {Piece::WP};
                    let m=BitMove::new(sindex, tindex, piece);
                    println!("from = {:?}  ----->>>> to = {:?} ||||| becomes {:?} \n", m.get_src(), m.get_target(), m.get_piece());
                }
            }
            false => {
                let mut src = self.pawns_able_to_2push(color);
                let mut target = self.single_push_targets(color);
                while src != 0 {
                    let sindex = src.trailing_zeros() as u16;
                    let tindex = target.trailing_zeros() as u16;
        
                    src &= src -1;
                    target &= target - 1;
                    let piece = if color == Color::Black {Piece::BP} else {Piece::WP};
                    let m=BitMove::new(sindex, tindex, piece);
                    println!("from = {:?}  ----->>>> to = {:?} ||||| becomes {:?} \n", m.get_src(), m.get_target(), m.get_piece());
                }
            }
        }

    }


    /// shows what squares this color's pawns (including the src square) can attack
    pub(crate) fn get_pawn_attacks(&self, color: Color) {
        let mut  piece = if color == Color::Black {Piece::BP} else {Piece::WP};
        match color {
            Color::Black => {
                let mut capture = self.pawns_able_2capture_any(Color::Black);
                // println!()
                while capture != 0 {
                    let src = capture.trailing_zeros() as u16;
                    let left_target = ((capture >> 9).trailing_zeros()) as u64;
                    let right_target = (capture >> 7).trailing_zeros() as u64; 
    
                    let attacker_exists = Bitboard::from(self.occupancies[Color::White]).get_bit_by_square(left_target.into());
                    // println!("does attacker exist?>> {attacker_exists}");

                    if attacker_exists != 0 {
                        let m = BitMove::new(src, left_target as u16, piece);
                        println!("from = {:?}  ----->>>> to = {:?} ||||| becomes {:?} \n", m.get_src(), m.get_target(), m.get_piece());
                    }
    
                    let right_attacker_exists = Bitboard::from(self.occupancies[Color::White]).get_bit_by_square(right_target.into());
                    if right_attacker_exists != 0 {
                        let m = BitMove::new(src, right_target as u16, piece);
                        println!("from = {:?}  ----->>>> to = {:?} ||||| becomes {:?} \n", m.get_src(), m.get_target(), m.get_piece());
                    }
                    capture &= capture-1;
                }
            }, 
            Color::White => {
                let mut capture = self.pawns_able_2capture_any(Color::White);
                while capture != 0 {
                    let src = capture.trailing_zeros() as u16;
                    let left_target = ((capture << 9).trailing_zeros()) as u64;
                    let right_target = (capture << 7).trailing_zeros() as u64;
    
    
                    let attacker_exists = Bitboard::from(self.occupancies[Color::Black]).get_bit_by_square(left_target.into());
                    // println!("attacker_exists===={}", attacker_exists);
                    if attacker_exists != 0 {
                        let m = BitMove::new(src, left_target as u16, piece);
                        println!("from = {:?}  ----->>>> to = {:?} ||||| becomes {:?} \n", m.get_src(), m.get_target(), m.get_piece());
                    }
    
                    let right_attacker_exists = Bitboard::from(self.occupancies[Color::Black]).get_bit_by_square(right_target.into());
                    if right_attacker_exists != 0 {
                        let m = BitMove::new(src, right_target as u16, piece);
                        println!("from = {:?}  ----->>>> to = {:?} ||||| becomes {:?} \n", m.get_src(), m.get_target(), m.get_piece());
                    }
                    capture &= capture-1;
                }
            },
            _ => panic!("Unrecognized player")
        };

    }


    fn enemy_or_empty(&self, color: Color) -> u64 {
        match color {
            Color::White => {
                return !*self[Piece::WP] & !*self[Piece::WB] & !*self[Piece::WK] & !*self[Piece::WN] & !*self[Piece::WQ] & !*self[Piece::WR]
            },
            Color::Black => {
                return !*self[Piece::BP] & !*self[Piece::BB] & !*self[Piece::BK] & !*self[Piece::BN] & !*self[Piece::BQ] & !*self[Piece::BR]
            },
            _ => {0}
        }
    }


}


impl FEN for BoardState {}

impl Deref for BoardState {
    type Target = Board;

    fn deref(&self) -> &Self::Target {
        &self.board    
    }
}

impl DerefMut for BoardState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.board
    }
}


impl Display for BoardState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!("{}", self.board.to_string());
        println!("    Side:       {:?}", self.turn);
        println!("    Enpass:     {:?}", self.enpassant);
        println!("    Castling:   {}", self.castling_rights.to_string());

        writeln!(f, "")
    }
}