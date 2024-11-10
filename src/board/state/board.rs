use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

use crate::bitboard::Bitboard;
use crate::board::piece::Piece::*;
use crate::board::{castling::Castling, piece::Piece};
use crate::color::Color::*;
use crate::move_logic::move_stack::MoveStack;
use crate::move_scope::MoveScope;
use crate::squares::Square::*;
use crate::{
    board::piece_map::PieceMap,
    color::Color,
    constants::{
        BLACK_KING_CASTLING_MASK, BLACK_QUEEN_CASTLING_MASK, CASTLING_TABLE, OCCUPANCIES,
        PIECE_ATTACKS, RANK_4, RANK_5, WHITE_KING_CASTLING_MASK, WHITE_QUEEN_CASTLING_MASK,
        ZOBRIST,
    },
    move_logic::{
        bitmove::{
            Move,
            MoveType::{self, *},
        },
        move_action::MoveAction,
    },
    squares::Square,
    zobrist::START_POSITION_ZOBRIST,
};

#[cfg(test)]
#[path = "./tests.rs"]
mod tests;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Board {
    pub(crate) turn: Color,
    // change this name eventually to piece_map everywhere too
    pub(crate) board: PieceMap,
    pub(crate) castling_rights: Castling,
    pub(crate) enpassant: Option<Square>,
    pub(crate) occupancies: [u64; OCCUPANCIES], // 0-white, 1-black, 2-both
    // castling_table: [u8; TOTAL_SQUARES],
    pub(crate) hash_key: u64,
    // fifty move rule counter
    pub(crate) fifty: [u8; 2],
}

impl Board {
    pub fn new() -> Board {
        Self {
            board: PieceMap::new(),
            turn: Color::White,
            enpassant: None,
            castling_rights: Castling::all(),
            occupancies: [0; OCCUPANCIES],
            hash_key: START_POSITION_ZOBRIST,
            fifty: [0, 0],
            // prev: None, //  castling_table: CASTLING_TABLE,
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

    /// Looking at this is not the best implementation
    /// Any changes here should affect the desired color, and the occupancy for both colors
    /// Avoid using it till the issues listed above fixed, AND WRITE TESTS!
    pub(crate) fn set_occupancy(&mut self, color: Color, occupancy: u64) {
        match color {
            Color::White => self.occupancies[color] |= occupancy,
            Color::Black => self.occupancies[color] |= occupancy,
            _ => {}
        }
        self.occupancies[Color::Both] |= occupancy;
    }

    pub(crate) fn remove_piece(&mut self, piece: Piece, sq: Square) {
        let sq_mask = 1u64 << u64::from(sq);
        self.occupancies[piece.color()] &= !sq_mask;
        self.occupancies[Both] &= !sq_mask;
        *self[piece] &= !sq_mask;
    }

    pub(crate) fn add_piece(&mut self, piece: Piece, sq: Square) {
        let sq_mask = 1u64 << u64::from(sq);
        self.occupancies[piece.color()] |= sq_mask;
        self.occupancies[Both] |= sq_mask;
        *self[piece] |= sq_mask;
    }

    pub(crate) fn get_occupancy(&self, color: Color) -> u64 {
        self.occupancies[color]
    }

    /// Returns all possible attacks that can capture the provided square (sq)
    pub(crate) fn get_all_attacks(&self, sq: Square) -> u64 {
        let sq_mask = 1u64 << u64::from(sq);

        let attacking_pawns = (PIECE_ATTACKS.pawn_attacks[Black][sq] & *self[BP])
            | (PIECE_ATTACKS.pawn_attacks[White][sq] & *self[WP]);
        let attacking_knights = PIECE_ATTACKS.knight_attacks[sq] & (*self[WN] | *self[BN]);
        let diagonal_attackers = *self[WQ] | *self[WB] | *self[BQ] | *self[BB];
        let diagonal_attacks =
            PIECE_ATTACKS.nnbishop_attacks(sq_mask, self.occupancies[Both]) & diagonal_attackers;
        let orthogonal_attackers = *self[WQ] | *self[WR] | *self[BQ] | *self[BR];
        let orthogonal_attacks =
            PIECE_ATTACKS.nnrook_attacks(sq_mask, self.occupancies[Both]) & orthogonal_attackers;
        let attacking_kings = PIECE_ATTACKS.king_attacks[sq] & (*self[WK] | *self[BK]);

        attacking_pawns
            | attacking_knights
            | attacking_kings
            | diagonal_attacks
            | orthogonal_attacks
    }

    /// Given the current pieces on the board, is this square under attack by the given side (color)
    /// Getting attackable(reachable) spots from this square, it also means this square can be reached from those
    /// squares
    pub(crate) fn is_square_attacked(&self, sq_64: u64, attacker: Color) -> bool {
        // bitboard with only the square's bit set
        let sq_mask = 1u64 << sq_64;
        let sq: Square = Square::from(sq_64);

        let pawn_attackers = self[Piece::pawn(attacker)]; // get the occupancy for the attacking pawns
        if (PIECE_ATTACKS.pawn_attacks[attacker][sq] & *pawn_attackers) != 0 {
            return true;
        }

        let knights = self[Piece::knight(attacker)]; // knight occupancy for the attacking side
        if (PIECE_ATTACKS.knight_attacks[sq] & *knights) != 0 {
            return true;
        }

        let king = self[Piece::king(attacker)];
        if (PIECE_ATTACKS.king_attacks[sq] & *king) != 0 {
            return true;
        }

        let bishops_queens = *self[Piece::queen(attacker)] | *self[Piece::bishop(attacker)];
        if (PIECE_ATTACKS.nnbishop_attacks(sq_mask, self.occupancies[Color::Both]) & bishops_queens)
            != 0
        {
            return true;
        }

        let rooks_queens = *self[Piece::queen(attacker)] | *self[Piece::rook(attacker)];
        if (PIECE_ATTACKS.nnrook_attacks(sq_mask, self.occupancies[Color::Both]) & rooks_queens)
            != 0
        {
            // println!("{}", Bitboard::from(PIECE_ATTACKS.nnrook_attacks(sq_mask, self.occupancies[Color::Both]) & rooks_queens));
            return true;
        }
        // println!("eeee");

        false
    }

    // pub(crate) fn get_square_attacks(&self, sq_64: u64, stm: Color) {
    //     let b = self;
    //     let mvs = self.gen_movement();
    //     if b.turn != stm {}
    // }

    /// Target for single pawn pushes (black or white)
    /// Returns a value with all the bits set when the pawns of that specific color are pushed
    fn single_push_targets(&self, color: Color) -> u64 {
        let empty = !self.occupancies[Color::Both];
        let color_pawns = self[Piece::pawn(color)];

        match color {
            Color::White => color_pawns.north() & empty,
            _ => color_pawns.south() & empty,
        }
    }

    /// Double push for pawns(black or white)
    /// https://www.chessprogramming.org/Pawn_Pushes_(Bitboards)
    fn double_push_targets(&self, color: Color) -> u64 {
        let single_push = Bitboard::from(self.single_push_targets(color));
        if color == Color::Black {
            return single_push.south() & !self.occupancies[Color::Both] & RANK_5;
        }

        single_push.north() & !self.occupancies[Color::Both] & RANK_4
    }

    /// Returns a value that has the squares containing only eligible pawns of `color` that can make double push moves
    fn pawns_able_to_double_push(&self, color: Color) -> u64 {
        let empty = !self.occupancies[Color::Both];
        if color == Color::Black {
            let empty_rank6 = Bitboard::from(empty & RANK_5).north() & empty;
            return Bitboard::from(empty_rank6).north() & *self[Piece::BP];
        }
        let empty_rank_3 = Bitboard::from(empty & RANK_4).south() & empty;
        return Bitboard::from(empty_rank_3).south() & *self[Piece::WP];
    }

    /// Returns all possible movements for pawns of a specific color.
    /// If the double argument is true, then only double pawns moves are returned in the result
    pub(crate) fn get_pawn_movement<T: MoveAction>(
        &self,
        color: Color,
        double: bool,
        mv_list: &mut MoveStack<T>,
    ) {
        match double {
            true => {
                let mut src2 = self.pawns_able_to_double_push(color);
                let mut target2 = self.double_push_targets(color);

                // let length = target2.count_ones() as usize; // because doubles cannot be promoted

                while src2 != 0 {
                    let src = src2.trailing_zeros() as u8;
                    let target = target2.trailing_zeros() as u8;

                    // let piece = Piece::pawn(color);
                    let mv = Move::new(src, target, DoublePush);
                    mv_list.push(T::create(mv));

                    // if mv.get_src().to_string() == "f7" && mv.tgt().to_string() == "g8" {
                    //     println!("---------------------------************---------------------------************---------------------------************  eleniyan {}", mv);
                    // }
                    // println!("the pushed move is >>>>>>>>>>>>>>>>>>>>>>> {}", mv.to_string());

                    src2 &= src2 - 1;
                    target2 &= target2 - 1;
                }
            }
            false => {
                let mut single_push_targets = self.single_push_targets(color);

                while single_push_targets != 0 {
                    let target_sq = single_push_targets & (!single_push_targets + 1);
                    let src_sq = match color {
                        Color::White => Bitboard::from(target_sq).south(),
                        _ => Bitboard::from(target_sq).north(),
                    };

                    let t_sq = target_sq.trailing_zeros();
                    let s_sq = src_sq.trailing_zeros();
                    // let piece = Piece::pawn(color);
                    let move_promotes = match color {
                        Color::White => t_sq >= Square::A8 as u32 && t_sq <= Square::H8 as u32,
                        _ => t_sq >= Square::A1 as u32 && t_sq <= Square::H1 as u32,
                    };

                    if move_promotes {
                        mv_list.push(T::create(Move::new(
                            s_sq as u8,
                            t_sq as u8,
                            PromotedToBishop,
                        )));
                        mv_list.push(T::create(Move::new(
                            s_sq as u8,
                            t_sq as u8,
                            PromotedToQueen,
                        )));
                        mv_list.push(T::create(Move::new(
                            s_sq as u8,
                            t_sq as u8,
                            PromotedToKnight,
                        )));
                        mv_list.push(T::create(Move::new(s_sq as u8, t_sq as u8, PromotedToRook)));
                    } else {
                        mv_list.push(T::create(Move::new(s_sq as u8, t_sq as u8, Quiet)));
                    }

                    single_push_targets &= single_push_targets - 1;
                }
            }
        }
    }

    /// shows what squares this color's pawns (including the src square) can attack
    pub(crate) fn get_pawn_attacks<T: MoveAction>(&self, color: Color, mv_list: &mut MoveStack<T>) {
        let piece = Piece::pawn(color);
        let mut color_pawns = *self[piece]; // pawns belonging to this color

        while color_pawns != 0 {
            let src: u32 = color_pawns.trailing_zeros();
            let targets = PIECE_ATTACKS.pawn_attacks[!color][Square::from(src as u64)];
            let mut captures = targets & self.occupancies[!color];

            while captures != 0 {
                let target: u64 = captures.trailing_zeros() as u64;

                let can_promote = match color {
                    Color::White => target >= Square::A8 as u64 && target <= Square::H8 as u64,
                    _ => target >= Square::A1 as u64 && target <= Square::H1 as u64,
                };

                if can_promote {
                    mv_list.push(T::create(Move::new(
                        src as u8,
                        target as u8,
                        CaptureAndPromoteToBishop,
                    )));
                    mv_list.push(T::create(Move::new(
                        src as u8,
                        target as u8,
                        CaptureAndPromoteToRook,
                    )));
                    mv_list.push(T::create(Move::new(
                        src as u8,
                        target as u8,
                        CaptureAndPromoteToKnight,
                    )));
                    mv_list.push(T::create(Move::new(
                        src as u8,
                        target as u8,
                        CaptureAndPromoteToQueen,
                    )));
                } else {
                    mv_list.push(T::create(Move::new(src as u8, target as u8, Capture)));
                }
                captures &= captures - 1;
            }
            color_pawns &= color_pawns - 1;
        }

        if let Some(enpass) = self.enpassant {
            let enpass_mask = 1u64 << u64::from(enpass);

            let (enpass_right_attack, enpass_left_attack) = match color {
                Color::White => {
                    let enpass_right_attack = Bitboard::from(enpass_mask).south_west();
                    let enpass_left_attack = Bitboard::from(enpass_mask).south_east();
                    (enpass_right_attack, enpass_left_attack)
                }
                _ => {
                    let enpass_right_attack = Bitboard::from(enpass_mask).north_east();
                    let enpass_left_attack = Bitboard::from(enpass_mask).north_west();
                    (enpass_right_attack, enpass_left_attack)
                }
            };

            // ensures that this exists (not pushed outside the board)
            if enpass_right_attack != 0 {
                if (enpass_right_attack & *self[piece]) != 0 {
                    let source = enpass_right_attack.trailing_zeros();
                    let bmove = Move::new(source as u8, enpass as u8, Enpassant);
                    mv_list.push(T::create(bmove));
                }
            }
            if enpass_left_attack != 0 {
                if (enpass_left_attack & *self[piece]) != 0 {
                    let source = enpass_left_attack.trailing_zeros();
                    let bmove = Move::new(source as u8, enpass as u8, Enpassant);
                    mv_list.push(T::create(bmove));
                }
            }
        }
    }

    pub(crate) fn get_castling<T: MoveAction>(&self, color: Color, mv_list: &mut MoveStack<T>) {
        // let mut move_list = Vec::with_capacity(2);

        match color {
            Color::White => {
                if (self.castling_rights & Castling::WHITE_KING) != Castling::NONE {
                    let f1g1_empty = (self.occupancies[Color::Both] & 0x60u64) == 0;
                    let e1f1g1_attacked = self.is_square_attacked(u64::from(Square::E1), !color)
                        || self.is_square_attacked(u64::from(Square::F1), !color)
                        || self.is_square_attacked(u64::from(Square::G1), !color);

                    if f1g1_empty && !e1f1g1_attacked {
                        mv_list.push(T::create(Move::new(
                            Square::E1 as u8,
                            Square::G1 as u8,
                            Castling,
                        )));
                    }
                }

                if (self.castling_rights & Castling::WHITE_QUEEN) != Castling::NONE {
                    let b1c1d1_empty = (self.occupancies[Color::Both] & 0xe_u64) == 0;
                    let e1c1d1_attacked = self.is_square_attacked(u64::from(Square::E1), !color)
                        || self.is_square_attacked(u64::from(Square::C1), !color)
                        || self.is_square_attacked(u64::from(Square::D1), !color);

                    if b1c1d1_empty && !e1c1d1_attacked {
                        mv_list.push(T::create(Move::new(
                            Square::E1 as u8,
                            Square::C1 as u8,
                            Castling,
                        )));
                    }
                }
            }
            Color::Black => {
                if (self.castling_rights & Castling::BLACK_KING) != Castling::NONE {
                    let f8g8_empty = (self.occupancies[Color::Both] & 0x6000000000000000u64) == 0;
                    let e8f8g8_attacked = self.is_square_attacked(u64::from(Square::E8), !color)
                        || self.is_square_attacked(u64::from(Square::F8), !color)
                        || self.is_square_attacked(u64::from(Square::G8), !color);

                    if f8g8_empty && !e8f8g8_attacked {
                        mv_list.push(T::create(Move::new(
                            Square::E8 as u8,
                            Square::G8 as u8,
                            Castling,
                        )));
                    }
                }

                if (self.castling_rights & Castling::BLACK_QUEEN) != Castling::NONE {
                    let b8c8d8_empty = (self.occupancies[Color::Both] & 0xe00000000000000u64) == 0;
                    let e8d8c8_attacked = self.is_square_attacked(u64::from(Square::E8), !color)
                        || self.is_square_attacked(u64::from(Square::D8), !color)
                        || self.is_square_attacked(u64::from(Square::C8), !color);

                    if b8c8d8_empty && !e8d8c8_attacked {
                        mv_list.push(T::create(Move::new(
                            Square::E8 as u8,
                            Square::C8 as u8,
                            Castling,
                        )));
                    }
                }
            }
            _ => {}
        }
    }

    pub(crate) fn get_sliding_and_leaper_moves<const T: u8, U: MoveAction>(
        &self,
        piece: Piece,
        mv_list: &mut MoveStack<U>,
    ) {
        // let mut move_list: Vec<Move> = vec![];

        let color = piece.color();
        let mut pieces_on_board = self[piece];

        while pieces_on_board.not_zero() {
            let square = pieces_on_board.trailing_zeros() as u64;
            // assert_eq!(square, pieces_on_board.trailing_zeros() as u64);
            pieces_on_board.pop_bit(square);
            let src = Square::from(square);

            // generates a bitboard(u64) where only this src square is set to 1
            let sq_bits = 1u64 << src as u64;
            let (attacks, occupancies) = match piece {
                Piece::WN | Piece::BN => {
                    (PIECE_ATTACKS.knight_attacks[src], !self.occupancies[color])
                } // leapers
                Piece::WB | Piece::BB => (
                    PIECE_ATTACKS.nnbishop_attacks(sq_bits, self.occupancies[Color::Both]),
                    !self.occupancies[color],
                ),
                Piece::WR | Piece::BR => (
                    PIECE_ATTACKS.nnrook_attacks(sq_bits, self.occupancies[Color::Both]),
                    !self.occupancies[color],
                ),
                Piece::WQ | Piece::BQ => {
                    let queen_attacks = PIECE_ATTACKS
                        .nnbishop_attacks(sq_bits, self.occupancies[Color::Both])
                        | PIECE_ATTACKS.nnrook_attacks(sq_bits, self.occupancies[Color::Both]);
                    (queen_attacks, !self.occupancies[color])
                }
                Piece::WK | Piece::BK => {
                    (PIECE_ATTACKS.king_attacks[src], !self.occupancies[color])
                }
                _ => unreachable!(),
            };

            // we're getting !self.occupancies[color]s because our knight hould be able to make both quiet or capture moves (on the opponent)
            let mut targets = Bitboard::from(attacks & occupancies);

            let source = src as u32;

            while targets.not_zero() {
                let target = targets.trailing_zeros() as u64;
                // capture move // there is an opponent on the target square
                let opponent_on_target =
                    Bitboard::from(self.occupancies[!color]).get_bit(target) != 0;
                let mvt = if opponent_on_target { Capture } else { Quiet };

                targets.pop_bit(target);

                if (mvt == Capture && T == MoveScope::QUIETS)
                    || (mvt == Quiet && T == MoveScope::CAPTURES)
                {
                    continue;
                }
                mv_list.push(U::create(Move::new(source as u8, target as u8, mvt)));
            }
        }
    }

    pub(crate) fn possibly_zugzwang(&self) -> bool {
        self.occupancies[self.turn] ^ (*self.board[Piece::pawn(self.turn)] & *self.board[Piece::king(self.turn)]) == 0
    }

    /// If the king of the side-to-move is in check
    pub(crate) fn stm_in_check(&self) -> bool {
        self.is_square_attacked(self[Piece::king(self.turn)].get_lsb1().unwrap(), !self.turn)
    }

    /// This is important for validation of a tt_move by the move-picker
    /// Important for validating a move provided by the client
    /// We can go a step further and remove all validations on the `make_move` methoid, if we're sure that this is up to paar with the current available validation
    /// TODO!: Consider implementing a better validation to check if the stm would still be in check (assuming they were before), or if the stm will now become in check
    /// after this move, if any of both cases is true, just return false 
    pub(crate) fn is_pseudo_legal(&self, mv: &Move) -> bool {
        let src = mv.src();
        let tgt = mv.tgt();
        let moving_piece = self.piece_at(mv.src());
        
        if moving_piece.is_none() { return false; }
        if moving_piece.unwrap().color() != self.turn { return false; }
        if mv.get_enpassant() && self.enpassant.is_some_and(|enp| enp != tgt) { return false; }
        if mv.get_double_push() && self.piece_at(tgt).is_some() { return false; }
        if mv.get_double_push() {
            if self.turn == White && self.piece_at(Square::from(src as u64 + 8)).is_some() {return false;}
            if self.turn == Black {
                let Some(mid) = u64::from(src).checked_sub(8) else { return false; };
                if self.piece_at(Square::from(mid)).is_some() {return false;}
            }
        }
        if mv.is_capture() && self.get_piece_at(tgt, !self.turn).is_some() {return false;}
        if mv.get_castling() && self.validate_castling_move(mv).is_none() { return false; }
        if tgt < A2 && src > H7 && mv.get_promotion().is_none() { return false; }

        let mut mvl = MoveStack::<Move>::default();
        if moving_piece.unwrap() == Piece::pawn(self.turn) {
            if mv.get_capture() {
                self.gen_movement::<{MoveScope::ALL}, Move>(&mut mvl);
                return mvl.contains(&mv);
            }
            self.gen_movement::<{MoveScope::ALL}, Move>(&mut mvl);
        }

        self.gen_movement::<{MoveScope::ALL}, Move>(&mut mvl);
        
        return mvl.contains(&mv); 
    }

    /// T denotes whether you want to generate Quiet(= 0), Captures(= 1), or All(= 2) moves
    pub(crate) fn gen_movement<const T: u8, U: MoveAction>(
        &self,
        mut move_list: &mut MoveStack<U>,
    ) {
        let color = self.turn;

        if T == MoveScope::CAPTURES || T == MoveScope::ALL {
            self.get_pawn_attacks(color, &mut move_list);
        }

        if T == MoveScope::QUIETS || T == MoveScope::ALL {
            self.get_pawn_movement::<U>(color, true, &mut move_list);
            self.get_pawn_movement::<U>(color, false, &mut move_list);
            self.get_castling::<U>(color, &mut move_list);
        }

        self.get_sliding_and_leaper_moves::<T, U>(Piece::knight(color), &mut move_list);
        self.get_sliding_and_leaper_moves::<T, U>(Piece::bishop(color), &mut move_list);
        self.get_sliding_and_leaper_moves::<T, U>(Piece::rook(color), &mut move_list);
        self.get_sliding_and_leaper_moves::<T, U>(Piece::queen(color), &mut move_list);
        self.get_sliding_and_leaper_moves::<T, U>(Piece::king(color), &mut move_list);
    }

    /// turn: The turn of the attacker
    /// e.g. If white pawn just made an enpassant move, then we know we should deduct the tgt
    pub(crate) fn enpass_tgt(tgt: Square, turn: Color) -> u64 {
        match turn {
            Color::Black => tgt as u64 + 8,
            _ => tgt as u64 - 8,
        }
    }

    /// Returns the rook source and target
    pub(crate) fn validate_castling_move(&self, mv: &Move) -> Option<(Square, Square)> {
        let king_side_mask = 0b1001u64;
        let queen_side_mask = 0b10001u64;

        match mv.get_target() {
            Square::G1 => {
                // white castles king side
                let shifted_occupancy = self.occupancies[Both] >> (E1 as u64);
                let mask = ((1 << ((H1 - E1) + 1)) - 1) as u64;
                let result = shifted_occupancy & mask;

                let cannot_castle = (self.castling_rights.bits() & WHITE_KING_CASTLING_MASK) == 0;
                if ((result | king_side_mask) != king_side_mask) || cannot_castle {
                    return None;
                }

                return Some((H1, F1));
            }
            Square::G8 => {
                // black castles king side
                let shifted_occupancy = self.occupancies[Color::Both] >> (E8 as u64);
                let mask = ((1 << ((H8 - E8) + 1)) - 1) as u64;
                let result = shifted_occupancy & mask;

                let cannot_castle = (self.castling_rights.bits() & BLACK_KING_CASTLING_MASK) == 0;

                if ((result | king_side_mask) != king_side_mask) || cannot_castle {
                    return None;
                }

                return Some((H8, F8));
            }
            Square::C1 => {
                // white castles queen side
                let shifted_occupancy = self.occupancies[Color::Both] >> (A1 as u64);
                let mask = ((1 << (E1 - A1) + 1) - 1) as u64;

                let result = shifted_occupancy & mask;
                let cannot_castle = (self.castling_rights.bits() & WHITE_QUEEN_CASTLING_MASK) == 0;

                if ((result | queen_side_mask) != queen_side_mask) || cannot_castle {
                    // println!("the mask is {:05b}", mask);
                    return None;
                }

                return Some((A1, D1));
            }
            Square::C8 => {
                // black castles queen side
                let shifted_occupancy = self.occupancies[Color::Both] >> (A8 as u64);
                let mask = ((1 << (E8 - A8) + 1) - 1) as u64;

                let result = shifted_occupancy & mask;
                let cannot_castle = (self.castling_rights.bits() & BLACK_QUEEN_CASTLING_MASK) == 0;

                if (result | queen_side_mask) != queen_side_mask || cannot_castle {
                    return None;
                }

                return Some((A8, D8));
            }
            x => unreachable!("Not a valid castling target {x}"),
        }
    }

    pub(crate) fn make_move(&self, bit_move: Move, scope: MoveScope) -> Option<Self> {
        let mut board = self.clone();
        // println!("RECEIVED {}", board.to_string());

        match scope {
            MoveScope::AllMoves => {
                // if bit_move.get_double_push() {
                //     println!("input is ((((((((((((((((((((((((((((((((((((((((((((((( {} >>>>> {}", bit_move.to_string(), bit_move.get_double_push());
                // }
                let from = bit_move.get_src(); // initial position of the piece
                let to = bit_move.get_target(); // target position of the piece

                let Some(piece) = self.piece_at(from) else {
                    return None;
                }; // the piece trying to move
                let turn = self.turn;

                if *(self[piece]) & (1 << (from as u64)) == 0 || turn != piece.color() {
                    return None;
                }

                // move piece
                board[piece].pop_bit(from.into());
                board[piece].set_bit(to.into());
                board.hash_key ^= ZOBRIST.piece_keys[piece][from];
                board.hash_key ^= ZOBRIST.piece_keys[piece][to];

                if Piece::WP == piece || Piece::BP == piece || bit_move.get_capture() {
                    board.fifty = [0, 0];
                }

                // Removes the captured piece from the the captured piece bitboard
                if bit_move.get_capture() && !bit_move.get_enpassant() {
                    // there would usually only be a maximum of 2 captures each, consider unrolling this for loop (what did I mean here by 2???????)
                    let target_pieces = Piece::all_pieces_for(!turn);
                    // let mut found = false;
                    for p in target_pieces {
                        if board[p].get_bit(to.into()) != 0 {
                            board[p].pop_bit(to.into());
                            board.hash_key ^= ZOBRIST.piece_keys[p][to];
                            // found = true;
                            break;
                        }
                    }
                    // this is an illegal move, the victim is not on the target square
                    // if !bit_move.get_enpassant() && !found {
                    //     println!("the mv is >>>>> {}, the board now \n {}", bit_move.to_string(), self.board.to_string());
                    //     return None }
                }

                if let Some(promoted_to) = bit_move.get_promotion() {
                    // if this piece is eligible for promotion, the new type it's vying for
                    let promoted_to = Piece::from((promoted_to, turn));
                    board[piece].pop_bit(to.into());
                    board.hash_key ^= ZOBRIST.piece_keys[piece][to];
                    board[promoted_to].set_bit(to.into());
                    board.hash_key ^= ZOBRIST.piece_keys[promoted_to][to];
                }

                if bit_move.get_enpassant() {
                    // let enpass_target = match board.turn {Color::Black => to as u64 + 8, _ => to as u64 -  8};
                    let enpass_target = Self::enpass_tgt(to, board.turn);
                    board[Piece::pawn(!turn)].pop_bit(enpass_target);
                    board.hash_key ^=
                        ZOBRIST.piece_keys[Piece::pawn(!turn)][enpass_target as usize];
                }

                if let Some(enpass) = board.enpassant {
                    // remove the enpassant from the zobrist_hash if it was there before (this move definitely resulted in an existing enpassant been removed)
                    board.hash_key ^= ZOBRIST.enpassant_keys[enpass as usize];
                }
                board.enpassant = None;

                if bit_move.move_type() == MoveType::DoublePush {
                    // let enpass_target = match board.turn {Color::Black => to as u64 + 8, _ => to as u64 -  8};
                    let enpass_target = Self::enpass_tgt(to, board.turn);
                    // println!("this is a double push to >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> {} {} -->> {}",bit_move.src(), to, Square::from(enpass_target));
                    board.enpassant = Some(enpass_target.into());
                    // double move results in an enpassant, add it to the hash key
                    board.hash_key ^= ZOBRIST.enpassant_keys[enpass_target as usize];
                }

                if bit_move.get_castling() {
                    if let Some((src, tgt)) = self.validate_castling_move(&bit_move) {
                        board[Piece::rook(turn)].pop_bit(src.into());
                        board[Piece::rook(turn)].set_bit(tgt.into());
                        board.hash_key ^= ZOBRIST.piece_keys[Piece::rook(turn)][src as usize];
                        board.hash_key ^= ZOBRIST.piece_keys[Piece::rook(turn)][tgt as usize];
                    } else {
                        return None;
                    };
                }

                let old_castling =
                    usize::from_str_radix(&board.castling_rights.bits().to_string(), 10).unwrap();
                board.hash_key ^= ZOBRIST.castle_keys[old_castling];
                let castle_one = board.castling_rights.bits() & CASTLING_TABLE[from];
                let castle_two = castle_one & CASTLING_TABLE[to];
                board.castling_rights = Castling::from(castle_two);
                let new_castling =
                    usize::from_str_radix(&board.castling_rights.bits().to_string(), 10).unwrap();
                board.hash_key ^= ZOBRIST.castle_keys[new_castling];

                board.occupancies[Color::White] = *board[Piece::WP]
                    | *board[Piece::WB]
                    | *board[Piece::WK]
                    | *board[Piece::WN]
                    | *board[Piece::WQ]
                    | *board[Piece::WR];
                board.occupancies[Color::Black] = *board[Piece::BP]
                    | *board[Piece::BB]
                    | *board[Piece::BK]
                    | *board[Piece::BN]
                    | *board[Piece::BQ]
                    | *board[Piece::BR];
                board.occupancies[Color::Both] =
                    board.occupancies[Color::White] | board.occupancies[Color::Black];

                // // println!("[[[[[[[[[[[[[[[[[[the move is >>>>>>>>>>>>>>]]]]]]]]]]]]]]]]]] {:?}", bit_move.to_string());
                // println!("BECAME***** {}", board.to_string());

                // is this an illegal move?
                // if board.is_square_attacked(board[Piece::king(turn)].get_lsb1().unwrap(), !board.turn) {
                let opponent = !board.turn;
                if board.is_square_attacked(board[Piece::king(turn)].get_lsb1().unwrap(), opponent)
                {
                    return None;
                }

                board.turn = !board.turn;
                board.hash_key ^= ZOBRIST.side_key;

                if Piece::WP == piece || Piece::BP == piece || bit_move.get_capture() {
                    board.fifty = [0, 0];
                } else {
                    board.fifty[turn] += 1;
                }

                // // *self = board;

                // if bit_move.get_double_push() {
                //     if board.enpassant.is_none() {
                //         println!("xxxxxxxx >>>>> {} --{} {:?} {} {}", bit_move, bit_move.to_string(), board.enpassant, bit_move.src(), bit_move.tgt());
                //         let enpass_target = Self::enpass_tgt(to, !board.turn);
                //         println!("this is a double push to >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> {} {} -->> {} \n\n\n",bit_move.src(), to, Square::from(enpass_target));
                //     }
                //     // if bit_move.to_string() == "a2a4" {
                //     //     println!("the double move here is >>>> done");
                //     //     println!("the board now is >>>>>>>>>>>>>>>>>>>>>>>>> {}\n\n\n\n", board.to_string());
                //     // }
                // }

                Some(board)
            }

            MoveScope::CapturesOnly => {
                if bit_move.get_capture() {
                    return self.make_move(bit_move, MoveScope::AllMoves);
                } else {
                    return None;
                }
            }
            _ => None,
        }
    }

    pub(crate) fn piece_at(&self, sq: Square) -> Option<Piece> {
        let sq_mask = 1 << sq as u64;
        let white_pieces = self.occupancies[0];
        // Deduct's the victim's color (The assumption here, is that if the piece is not white, then it must be black)
        let is_white = (white_pieces & sq_mask) != 0;

        if self.enpassant.is_some_and(|pre_enp_sq| pre_enp_sq == sq) {
            let enp_mask = 1u64 << Self::enpass_tgt(sq, self.turn);
            if (self.occupancies[!self.turn] & enp_mask) != 0 {
                return Some(Piece::pawn(!self.turn));
            }
        }

        // If the color is not white, then it will be black
        let range = match is_white {
            true => 0..6,
            false => 6..12,
        };

        for i in range {
            if (*self.board[i] & sq_mask) != 0 {
                return Some(Piece::from(i as u8));
            }
        }

        None
    }

    pub(crate) fn get_piece_at(&self, sq: Square, color: Color) -> Option<Piece> {
        let range = match color {
            White => 0..6,
            _ => 6..12,
        };

        if self.enpassant.is_some_and(|pre_enp_sq| pre_enp_sq == sq) {
            let enp_mask = 1u64 << Self::enpass_tgt(sq, self.turn);
            if (self.occupancies[!self.turn] & enp_mask) != 0 {
                return Some(Piece::pawn(!self.turn));
            }
        }

        for p in range {
            if self.board[p].get_bit(sq.into()) != 0 {
                return Some(Piece::from(p as u8));
            }
        }
        None
    }

    /// color: your opponent's/target's color
    pub(crate) fn get_move_capture(&self, mv: Move) -> Option<Piece> {
        let target = mv.get_target();
        if mv.get_enpassant() {
            let victim = Square::from(match self.turn {
                Color::Black => target as u64 + 8,
                _ => target as u64 - 8,
            });
            return self.get_piece_at(victim, !self.turn);
        }
        if mv.get_capture() {
            return self.get_piece_at(mv.get_target(), !self.turn);
        }
        None
    }

    pub(crate) fn set_zobrist(&mut self, key: u64) {
        self.hash_key = key;
    }

    /// Generates the zobrist hash for this board
    pub(crate) fn hash_key(&self) -> u64 {
        let mut final_key = 0u64;

        for piece in Piece::ascii_pieces() {
            // bitboard containing all pieces of this type
            let mut bitboard = *self[piece];

            while bitboard != 0 {
                let sq = Square::from(u64::from(bitboard.trailing_zeros()));
                final_key ^= ZOBRIST.piece_keys[piece][sq];

                // pop LS1B
                bitboard &= bitboard - 1;
            }
        }

        if let Some(enpass) = self.enpassant {
            final_key ^= ZOBRIST.enpassant_keys[enpass];
            // println!("I see {}", enpass as usize);
        }

        let index = usize::from_str_radix(&self.castling_rights.bits().to_string(), 10).unwrap();
        final_key ^= ZOBRIST.castle_keys[index];

        if self.turn == Color::Black {
            final_key ^= ZOBRIST.side_key
        };

        final_key
    }

    /// Returns the least valuable attacker based on the provided mask (attackers)
    pub(crate) fn get_lva(&self, attackers: u64, stm: Color) -> Option<(Piece, Square)> {
        let range = if stm == White { 0..6 } else { 6..12 };
        for piece in range {
            let bits = *self[piece] & attackers;
            if bits != 0 {
                return Some((
                    Piece::from(piece as u8),
                    Square::from(bits.trailing_zeros() as u64),
                ));
            }
        }
        None
    }
}

impl Deref for Board {
    type Target = PieceMap;

    fn deref(&self) -> &Self::Target {
        &self.board
    }
}

impl DerefMut for Board {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.board
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.board.to_string())?;
        writeln!(f, "    Side:       {:?}", self.turn)?;
        writeln!(f, "    Enpass:     {:?}", self.enpassant)?;
        writeln!(f, "    Castling:   {}", self.castling_rights.to_string())?;
        writeln!(f, "    Hashkey:    {0:x}", self.hash_key)
    }
}
