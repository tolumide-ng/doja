use std::{fmt::Display, ops::{BitAnd, BitXor, Deref, DerefMut}, sync::Arc};

use bitflags::Flags;

use crate::{bit_move::BitMove, board::board::Board, color::Color, constants::{A1_E1_IS_FILLED, A8_E8_IS_FILLED, BLACK_KING_CASTLING_CELLS, BLACK_QUEEN_CASTLING_CELLS, CASTLING_TABLE, E1_F1_FILLED, E8_F8_IS_FILLED, NOT_A_FILE, NOT_H_FILE, OCCUPANCIES, PIECE_ATTACKS, RANK_1, RANK_4, RANK_5, RANK_8, SQUARES, WHITE_KING_CASTLING_CELLS, WHITE_QUEEN_CASTLING_CELLS}, move_type::{self, MoveType}, moves::Moves, perft::{CAPTURES, CASTLES, ENPASSANT, PROMOTIONS}, piece_attacks, squares::Square, Bitboard};

use super::{castling::Castling, fen::FEN, piece::{self, Piece}};


#[derive(Debug, Clone)]
pub struct BoardState {
    pub(crate) turn: Color,
    pub board: Board,
    pub(crate) castling_rights: Castling,
    enpassant: Option<Square>,
    occupancies: [u64; OCCUPANCIES], // 0-white, 1-black, 2-both
    castling_table: [u8; SQUARES],
    // prev: Option<Arc<BoardState>>
}


const PROMOTABLE_TARGETS: usize = 4;

impl BoardState {
    pub fn new() -> BoardState {
        Self { board: Board::new(), turn: Color::White, enpassant: None, castling_rights: Castling::all(), 
            occupancies: [0; OCCUPANCIES], castling_table: CASTLING_TABLE 
        }
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

    // / Given the current pieces on the board, is this square under attack by the given side (color)
    // / Getting attackable(reachable) spots from this square, it also means this square can be reached from those
    // / attackable spots

    pub(crate) fn is_square_attacked(&self, sq_64: u64, attacker: Color) -> bool {
        // bitboard with only the square's bit set
        let sq_mask = 1u64 << sq_64;
        let sq = Square::from(sq_64);

        
        let pawn_attackers = self[Piece::pawn(attacker)];
        if (PIECE_ATTACKS.pawn_attacks[attacker][sq] & *pawn_attackers) != 0 { return true }
        // println!("aaaaaa");

        let knights = self[Piece::knight(attacker)];
        if (PIECE_ATTACKS.knight_attacks[sq] & *knights) != 0 { return true }
        // println!("bbbb");

        let king = self[Piece::king(attacker)];
        if (PIECE_ATTACKS.king_attacks[sq] & *king) != 0 { return true }
        // println!("cccc");

        let bishops_queens = *self[Piece::queen(attacker)] | *self[Piece::bishop(attacker)];
        if (PIECE_ATTACKS.nnbishop_attacks(sq_mask, self.occupancies[Color::Both]) & bishops_queens) != 0 { return true }
        // println!("dddd");

        let rooks_queens = *self[Piece::queen(attacker)] | *self[Piece::rook(attacker)];
        if (PIECE_ATTACKS.nnrook_attacks(sq_mask, self.occupancies[Color::Both]) & rooks_queens) != 0 { return true }
        // println!("eeee");
        
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


    fn attacksTo() {}


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

    fn pawns_able_2capture_east(&self, color: Color) -> u64 {
        if color == Color::Black {
            return *self[Piece::BP] & self.white_pawn_west_attacks(false)
        }
        *self.board[Piece::WP] & self.black_pawn_west_attacks(false)
    }
    fn pawns_able_2capture_west(&self, color: Color) -> u64 {
        if color == Color::Black {
            return *self[Piece::BP] & self.white_pawn_east_attacks(false)
        }
        *self[Piece::WP] & self.black_pawn_east_attacks(false)
    }

    /// Returns the squares of this color capable of capturing other squares
    fn pawns_able_2capture_any(&self, color: Color) -> u64 {
        *self[Piece::pawn(color)] & self.pawn_any_attack(!color, false)
        // if color == Color::Black {
        //     return *self[Piece::BP] & self.pawn_any_attack(Color::White, false)
        // }
        // *self[Piece::WP] & self.pawn_any_attack(Color::Black, false)
    }

    
    /// Target for single pawn pushes (black or white)
    fn single_push_targets(&self, color: Color) -> u64 {
        let empty = !self.occupancies[Color::Both];
        let color_pawns = self[Piece::pawn(color)];
        
        match color {
            Color::White => {color_pawns.north() & empty},
            _ => {color_pawns.south() & empty}
        }
    }
    
    /// Double push for pawns(black or white)
    /// https://www.chessprogramming.org/Pawn_Pushes_(Bitboards)
    fn double_push_targets(&self, color: Color) -> u64 {
        let single_push = Bitboard::from(self.single_push_targets(color));
        if color == Color::Black {
            return single_push.south() & !self.occupancies[Color::Both] & RANK_5
        }

        single_push.north() & !self.occupancies[Color::Both] & RANK_4
    }

    fn pawns_able_2push(&self, color: Color) -> u64 {
        let empty = Bitboard::from(!self.occupancies[Color::Both]);
        
        match color {
            Color::White => empty.south() & *self[Piece::pawn(color)],
            _ => empty.north() & *self[Piece::pawn(color)]
        }
    }

    fn pawns_able_to_double_push(&self, color: Color) -> u64 {
        let empty = !self.occupancies[Color::Both];
        if color == Color::Black {
            let empty_rank6 = Bitboard::from(empty & RANK_5).north() & empty;
            return Bitboard::from(empty_rank6).north() & *self[Piece::BP]
        }
        let empty_rank_3 = Bitboard::from(empty & RANK_4).south() & empty;
        return Bitboard::from(empty_rank_3).south() & *self[Piece::WP];
    }

    pub(crate) fn get_pawn_movement(&self, color: Color, double: bool) -> Vec<BitMove> {
        match double {
            true => {
                let mut src2 = self.pawns_able_to_double_push(color);
                let mut target2 = self.double_push_targets(color);
                
                let length = target2.count_ones() as usize; // because doubles cannot be promoted
                let mut move_list: Vec<BitMove> = Vec::with_capacity(length);

                
                while src2 !=0 {
                    let sindex = src2.trailing_zeros() as u32;
                    let tindex = target2.trailing_zeros() as u32;

                    
                    let piece = Piece::pawn(color);
                    let xx = BitMove::new(sindex, tindex, piece, None, false, true, false, false);
                    move_list.push(xx);
                    // if Square::from(sindex as u64) == Square::G2 && Square::from(tindex as u64) == Square::G4 {
                    //     print!("{xx:?}")
                    // }

                    src2 &= src2 -1;
                    target2 &= target2 -1;
                }
                move_list
            }
            false => {
                // let psrc = self.pawns_able_2push(color);

                let mut move_list: Vec<BitMove>  = vec![];
                let mut single_push_targets = self.single_push_targets(color);

                // println!("{}", Bitboard::from(psrc).to_string());
                // println!("{}", Bitboard::from(single_push_targets).to_string());

                while single_push_targets != 0 {
                    // println!("----->>>>>>   {}", Square::from(single_push_targets.trailing_zeros() as u64));
                    // let target_sq = single_push_targets.trailing_zeros() as u64;
                    let target_sq = single_push_targets & (!single_push_targets + 1);
                    let src_sq = match color {Color::White => Bitboard::south_one(target_sq), _ => Bitboard::north_one(target_sq)};

                    // println!()

                    let t_sq = target_sq.trailing_zeros();
                    let s_sq = src_sq.trailing_zeros();
                    // println!("src {}, target {}", Square::from(s_sq as u64), Square::from(t_sq as u64));
                    
                    // let move_promotes = t_sq >= Square::A8 as u32 && t_sq <= Square::H8 as u32;
                    
                    let piece = Piece::pawn(color);
                    let move_promotes = match color {
                        Color::White => {t_sq >= Square::A8 as u32 && t_sq <= Square::H8 as u32}
                        _ => {t_sq >= Square::A1 as u32 && t_sq <= Square::H1 as u32}
                    };


                    // if Square::from(s_sq as u64) == Square::G2 && color == Color::Black {
                    //     println!("can you promote?????? {}", move_promotes);
                    // }


                    if move_promotes {
                        move_list.push(BitMove::new(s_sq, t_sq, piece, Some(Piece::bishop(color)), false, false, false, false));
                        move_list.push(BitMove::new(s_sq, t_sq, piece, Some(Piece::queen(color)), false, false, false, false));
                        move_list.push(BitMove::new(s_sq, t_sq, piece, Some(Piece::knight(color)), false, false, false, false));
                        move_list.push(BitMove::new(s_sq, t_sq, piece, Some(Piece::rook(color)), false, false, false, false));
                    } else {
                        move_list.push(BitMove::new(s_sq, t_sq, piece, None, false, false, false, false));
                    }

                    single_push_targets &= single_push_targets - 1;
                }

                move_list
            }
        }
    }


    /// shows what squares this color's pawns (including the src square) can attack
    pub(crate) fn get_pawn_attacks(&self, color: Color) -> Vec<BitMove> {
        let piece = Piece::pawn(color);
        let mut mv_list: Vec<BitMove> = vec![];

        // current position of all pawn pieces of this color on the board
        let mut color_pawns = *self[piece];
        // println!("{}", Bitboard::from(color_pawns).to_string());
        // println!("{}", Bitboard::from(self.occupancies[!color]).to_string());
        
        while color_pawns != 0 {
            let src: u32 = color_pawns.trailing_zeros();
            let targets = PIECE_ATTACKS.pawn_attacks[!color][Square::from(src as u64)];            
            let mut captures = targets & self.occupancies[!color];

            // println!("sq is {:?}", Square::from(src as u64));
            // println!("targets {}", Bitboard::from(targets).to_string());
            
            // println!("captures {}", captures.count_ones());
            // println!("targets {}", targets.count_ones());
            // println!("------++++------++++------++++------++++------++++------++++");

            while captures != 0 {
                let target: u64 = captures.trailing_zeros() as u64;
                
                let can_promote = match color {
                    Color::White => {target >= Square::A8 as u64 && target <= Square::H8 as u64}
                    _ => {target >= Square::A1 as u64 && target <= Square::H1 as u64}
                };

                // if Square::from(src as u64) == Square::B4 && Square::from(target as u64) == Square::A3 {
                //     println!("|||||||||||{can_promote:?}")
                // }

                if can_promote {
                    mv_list.push(BitMove::new(src, target as u32, piece, Some(Piece::bishop(color)), true, false, false, false));
                    mv_list.push(BitMove::new(src, target as u32, piece, Some(Piece::rook(color)), true, false, false, false));
                    mv_list.push(BitMove::new(src, target as u32, piece, Some(Piece::knight(color)), true, false, false, false));
                    mv_list.push(BitMove::new(src, target as u32, piece, Some(Piece::queen(color)), true, false, false, false));

                } else {
                    mv_list.push(BitMove::new(src, target as u32, piece, None, true, false, false, false));

                }
                captures &= captures-1;
            }    
            color_pawns &= color_pawns-1;
        }


        // let enpassant = if let Some(enpass) = self.enpassant {enpass as u64 == target} else {false};
        // color_pawns &= !(1 << src);

        if let Some(enpass) = self.enpassant {
            let enpass_mask = 1u64 << u64::from(enpass);

            // confirm that there is nothing on our enpassant
            // if enpass_mask & self.occupancies[Color::Both] != 0 {
            //     return mv_list;
            // }

            let (enpass_right_attack, enpass_left_attack) = match color {
                Color::White => {
                    let enpass_right_attack = Bitboard::south_west1(enpass_mask);
                    let enpass_left_attack = Bitboard::south_east1(enpass_mask);
                    (enpass_right_attack, enpass_left_attack)
                }
                _ => {
                    let enpass_right_attack = Bitboard::north_east1(enpass_mask);
                    let enpass_left_attack = Bitboard::north_west1(enpass_mask);
                    (enpass_right_attack, enpass_left_attack)
                }
            };
            
            
            // ensures that this exists (not pushed outside the board)
            if enpass_right_attack != 0 {
                // println!("{}", self[piece].to_string());
                
                // let color_pawn_can_attack = enpass_right_attack & *self[piece];
                if (enpass_right_attack & *self[piece]) != 0 {
                    let source = enpass_right_attack.trailing_zeros();
                    // println!("rrr src = {:?}", Square::from(source as u64));
                    // println!("rrr target = {:?}", Square::from(enpass as u64));
                    // if Square::from(source as u64) == Square::B4 && Square::from(enpass as u64) == Square::A3 {
                    //     println!("XXXXXXX")
                    // }
                    
                    let bmove = BitMove::new(source, enpass as u32, piece, None, true, false, true, false);
                    mv_list.push(bmove);
                }
            }
            if enpass_left_attack != 0 {
                if (enpass_left_attack & *self[piece]) != 0 {
                    let source = enpass_left_attack.trailing_zeros();
                    // println!("llll src = {:?}", Square::from(source as u64));
                    // println!("llll target = {:?}", Square::from(enpass));
                    // if Square::from(source as u64) == Square::B4 && Square::from(enpass as u64) == Square::A3 {
                    //     println!("<<<|||||||>>>")
                    // }
    
                    let bmove = BitMove::new(source, enpass as u32, piece, None, true, false, true, false);
                    mv_list.push(bmove);
                }
            }
        }

        mv_list
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

    pub(crate) fn get_castling(&self, color: Color) -> Vec<BitMove> {
        let mut move_list = Vec::with_capacity(2);

        match color {
            Color::White => {
                if (self.castling_rights & Castling::WHITE_KING) != Castling::NONE {
                    let f1g1_empty = (self.occupancies[Color::Both] & 0x60u64) == 0;
                    // let e1f1_attacked = self.is_square_attacked(u64::from(Square::E1), !color) || self.is_square_attacked(u64::from(Square::F1), !color);
                    let e1f1g1_attacked = self.is_square_attacked(u64::from(Square::E1), !color) || self.is_square_attacked(u64::from(Square::F1), !color) || self.is_square_attacked(u64::from(Square::G1), !color);
                    
                    if f1g1_empty && !e1f1g1_attacked {
                    // if f1g1_empty && !e1f1_attacked {
                        // println!("we are not under attack, so we can create this >>>>>>>>>>>>>>>>>>>> {} {}", self.is_square_attacked(u64::from(Square::F1), !color), u64::from(Square::F1));
                        // println!("{}", self.to_string());
                        // println!("<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<||||||||||||||||||||>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> \n\n\n\n\n");

                        move_list.push(BitMove::new(Square::E1 as u32, Square::G1 as u32, Piece::WK, None, false, false, false, true));
                    }
                }

                if (self.castling_rights & Castling::WHITE_QUEEN) != Castling::NONE {
                    let b1c1d1_empty = (self.occupancies[Color::Both] & 0xe_u64) == 0;
                    // let make sure the king(e1) and d1 squares are not under attack
                    // let e1d1_attacked = self.is_square_attacked(u64::from(Square::E1), !color) || self.is_square_attacked(u64::from(Square::D1), !color);
                    let e1c1d1_attacked = self.is_square_attacked(u64::from(Square::E1), !color) || self.is_square_attacked(u64::from(Square::C1), !color)  || self.is_square_attacked(u64::from(Square::D1), !color);

                    // if b1c1d1_empty && !e1d1_attacked {
                    if b1c1d1_empty && !e1c1d1_attacked {
                        move_list.push(BitMove::new(Square::E1 as u32, Square::C1 as u32, Piece::WK, None, false, false, false, true));
                    }
                }
            }
            Color::Black => {
                if (self.castling_rights & Castling::BLACK_KING) != Castling::NONE {
                    let f8g8_empty = (self.occupancies[Color::Both] & 0x6000000000000000u64) == 0;
                    // let e8f8_attacked = self.is_square_attacked(u64::from(Square::E8), !color) || self.is_square_attacked(u64::from(Square::F8), !color);
                    let e8f8g8_attacked = self.is_square_attacked(u64::from(Square::E8), !color) || self.is_square_attacked(u64::from(Square::F8), !color) || self.is_square_attacked(u64::from(Square::G8), !color);

                    // if f8g8_empty && !e8f8_attacked {
                    if f8g8_empty && !e8f8g8_attacked {
                        // println!("............................................e8g8............................................................................");
                        // println!("{}", self.to_string());
                        // println!("........................................................................................................................\n\n\n\n");

                        move_list.push(BitMove::new(Square::E8 as u32, Square::G8 as u32, Piece::BK, None, false, false, false, true));
                    }
                }

                if (self.castling_rights & Castling::BLACK_QUEEN) != Castling::NONE {
                    let b8c8d8_empty = (self.occupancies[Color::Both] & 0xe00000000000000u64) == 0;
                    // let e8d8_attacked = self.is_square_attacked(u64::from(Square::E8), !color) || self.is_square_attacked(u64::from(Square::D8), !color);
                    let e8d8c8_attacked = self.is_square_attacked(u64::from(Square::E8), !color) || self.is_square_attacked(u64::from(Square::D8), !color) || self.is_square_attacked(u64::from(Square::C8), !color);

                    if b8c8d8_empty && !e8d8c8_attacked {
                    // if b8c8d8_empty && !e8d8_attacked {
                        move_list.push(BitMove::new(Square::E8 as u32, Square::C8 as u32, Piece::BK, None, false, false, false, true));
                    }
                }
            }
            _ => {}
        }

        move_list
    }


    pub(crate) fn get_sliding_and_leaper_moves(&self, color: Color, piece: Piece) -> Vec<BitMove> {
        // let knight = if color == Color::White {Piece::WN} else {Piece::BN};
        // println!("{}", self[piece].to_string());
        let mut move_list: Vec<BitMove> = vec![];
        
        let mut pieces_on_board = self[piece];

        while pieces_on_board.not_zero() {
            let square = pieces_on_board.get_lsb1().unwrap();
            pieces_on_board.pop_bit(square);
            let src = Square::from(square);

        //     let bishops_queens =  *self[Piece::bishop(attacker)];
        // if (PIECE_ATTACKS.nnbishop_attacks(sq_bit, self.occupancies[Color::Both]) & *self[Piece::bishop(attacker)]) != 0 { return true }
        
        // generates a bitboard(u64) where only this index of this square is set
        let sq_bits = 1u64 << src as u64;
        // 1u64.shl(Square::E4 as u64)
            // let xo = Bitboard::from(PIECE_ATTACKS.nnbishop_attacks(1u64.shl(Square::E4 as u64), 0));
            
            let (attacks, occupancies) = match piece {
                Piece::WN | Piece::BN => (PIECE_ATTACKS.knight_attacks[src], !self.occupancies[color]),
                Piece::WB | Piece::BB => (PIECE_ATTACKS.nnbishop_attacks(sq_bits, self.occupancies[Color::Both]), !self.occupancies[color]),
                Piece::WR | Piece::BR  => (PIECE_ATTACKS.nnrook_attacks(sq_bits, self.occupancies[Color::Both]), !self.occupancies[color]),
                Piece::WQ | Piece::BQ => {
                    let queen_attacks = PIECE_ATTACKS.nnbishop_attacks(sq_bits, self.occupancies[Color::Both]) | PIECE_ATTACKS.nnrook_attacks(sq_bits, self.occupancies[Color::Both]);
                    (queen_attacks, !self.occupancies[color])
                },
                Piece::WK | Piece::BK => (PIECE_ATTACKS.king_attacks[src], !self.occupancies[color]),
                _ => unreachable!()
            };

            // let attacks = attack_map[src];
            // we're getting !self.occupancies[color] because our knight should be able to make both quiet or capture moves (on the opponent)
            let mut targets = Bitboard::from(attacks & occupancies);

            // println!("{}", targets.to_string());
            let source = src as u32;

            while targets.not_zero() {
                let target = targets.get_lsb1().unwrap();
                // capture move // there is an opponent on the target square
                let opponent_on_target = Bitboard::from(self.occupancies[!color]).get_bit(target) != 0;
                move_list.push(BitMove::new(source, target as u32, piece, None, opponent_on_target, false, false, false));

                targets.pop_bit(target);
            }
        }

        move_list
    }

    pub(crate) fn gen_movement(&self) -> Moves {
        let color = self.turn;

        let mut move_list = Moves::new();

        move_list.add_many(&self.get_pawn_attacks(color));
        move_list.add_many(&self.get_pawn_movement(color, true));
        move_list.add_many(&self.get_pawn_movement(color, false));
        move_list.add_many(&self.get_castling(color));
        move_list.add_many(&self.get_sliding_and_leaper_moves(color, Piece::knight(color)));
        move_list.add_many(&self.get_sliding_and_leaper_moves(color, Piece::bishop(color)));
        move_list.add_many(&self.get_sliding_and_leaper_moves(color, Piece::rook(color)));
        move_list.add_many(&self.get_sliding_and_leaper_moves(color, Piece::queen(color)));
        move_list.add_many(&self.get_sliding_and_leaper_moves(color, Piece::king(color)));


        move_list
    }


    pub(crate) fn make_move(&self, bit_move: BitMove, move_type: MoveType) -> Option<Self> {
        let mut board = self.clone();
        let turn = board.turn;
        // board.prev = Some(Arc::new(self));
        // if bit_move.get_src() == Square::E1 && bit_move.get_target() == Square::C1 {
        //     println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@");
        //     println!("{}", board.to_string());
        //     println!("\n\n");
        // }

        match move_type {
            MoveType::AllMoves => {
                let from = bit_move.get_src(); // initial position of the piece
                let to = bit_move.get_target(); // target position of the piece
                let piece = bit_move.get_piece(); // the piece trying to move
                
                
                // move piece
                board[piece].pop_bit(from.into());
                board[piece].set_bit(to.into());


                // Removes the captured piece from the the captured piece bitboard
                if bit_move.get_capture() {
                    // there would usually only be a maximum of 2 captures each, consider unrolling this for loop
                    let target_pieces = Piece::all_pieces_for(!turn);
    
                    for p in target_pieces {
                        if board[p].get_bit(to.into()) != 0 {
                            board[p].pop_bit(to.into());
                            // if let Ok(mut val) = CAPTURES.lock() {
                            //     *val +=1;
                            // }
                            break;
                        }
                    }
                }

                
                if let Some(promoted_to) = bit_move.get_promotion() { // if this piece is eligible for promotion, the new type it's vying for
                    board[piece].pop_bit(to.into());
                    board[promoted_to].set_bit(to.into());
                    // if let Ok(mut val) = PROMOTIONS.lock() {
                    //     *val +=1;
                    // }
                }
                
                
                if bit_move.get_enpassant() {
                    let enpass_target = match board.turn {Color::Black => to as u64 + 8, _ => to as u64 -  8};
                    board[Piece::pawn(!turn)].pop_bit(enpass_target);
                    // if let Ok(mut val) = ENPASSANT.lock() {
                    //     *val +=1;
                    // }
                }

                board.enpassant = None;

                if bit_move.get_double_push() {
                    let enpass_target = match board.turn {Color::Black => to as u64 + 8, _ => to as u64 -  8};
                    board.enpassant = Some(enpass_target.into());
                }

                if bit_move.get_castling() {
                    // if [Square::E8, Square::E1].contains(&from) &&
                    // [Square::G1, Square::C1, Square::G8, Square::C8].contains(&to) {
                        
                    //     // if let Ok(mut val) = CASTLES.lock() {
                    //     //     *val +=1;
                    //     // }
                    // }
                    match to {
                        Square::G1 => { // white castles king side
                            board[Piece::WR].pop_bit(Square::H1.into());
                            board[Piece::WR].set_bit(Square::F1.into());
                        }
                        Square::G8 => { // black castles king side
                            board[Piece::BR].pop_bit(Square::H8.into());
                            board[Piece::BR].set_bit(Square::F8.into());
                        }
                        Square::C1 => { // white castles queen side
                            board[Piece::WR].pop_bit(Square::A1.into());
                            board[Piece::WR].set_bit(Square::D1.into());
                        }
                        Square::C8 => { // black castles queen side
                            board[Piece::BR].pop_bit(Square::A8.into());
                            board[Piece::BR].set_bit(Square::D8.into());
                        }
                        x => unreachable!("Not a valid castling target {x}")
                    }
                }

                // let castling_rights = board.new_castling_rights(from, to);
                // board.castling_rights = castling_rights;
                let castle_one = board.castling_rights.bits() & board.castling_table[from];
                let castle_two = castle_one & board.castling_table[to];
                board.castling_rights = Castling::from(castle_two);
                // board.castling_rights &= board.castling_table[to];

                // if bit_move.get_src() == Square::H8 && bit_move.get_target() == Square::H7 {
                //     println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@");
                //     println!("{}", board.to_string());
                //     println!("{}", board.castling_rights);
                //     println!("\n\n");
                // }


                board.occupancies[Color::White] = *board[Piece::WP] | *board[Piece::WB] | *board[Piece::WK] | *board[Piece::WN] | *board[Piece::WQ] | *board[Piece::WR];
                board.occupancies[Color::Black] = *board[Piece::BP] | *board[Piece::BB] | *board[Piece::BK] | *board[Piece::BN] | *board[Piece::BQ] | *board[Piece::BR];
                board.occupancies[Color::Both] = board.occupancies[Color::White] | board.occupancies[Color::Black];

                
                // is this an illegal move?
                if board.is_square_attacked(board[Piece::king(turn)].get_lsb1().unwrap(), !board.turn) {
                    return None;
                } else {
                    // if bit_move.get_src() == Square::E1 && bit_move.get_target() == Square::G1 {
                    //     println!("*****************:::::*****************:::::");
                    //     println!("{}", board.to_string());
                    // }
                }

                board.turn = !board.turn;

            }

            MoveType::CapturesOnly => {}
        }

        Some(board)
    }


    pub(crate) fn new_castling_rights(&mut self, from: Square, to: Square) -> Castling {
        // if from == Square::D2 && to == Square::C1 {
        //     println!("*****************:::::*****************:::::");
        //     println!("{}", self.to_string());
        // }

        let new_mask = from.castling_mask() | to.castling_mask();
        let existing_rights = self.castling_rights.bits() & new_mask;
        let new_rights = self.castling_rights.bits().bitand(!existing_rights);
        let nr  = Castling::from(new_rights);
        // if from == Square::D2 && to == Square::C1 {
        //     println!("*****************:::****************************::*****************:::::");
        //     println!("{}", nr);
        // }

        return nr
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