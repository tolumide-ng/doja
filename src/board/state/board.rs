use std::{fmt::Display, ops::{Deref, DerefMut}};

use crate::{bit_move::{Move, MoveType::{self, *}}, board::piece_map::PieceMap, color::Color, constants::{BLACK_KING_CASTLING_MASK, BLACK_QUEEN_CASTLING_MASK, CASTLING_TABLE, OCCUPANCIES, PIECE_ATTACKS, RANK_4, RANK_5, WHITE_KING_CASTLING_MASK, WHITE_QUEEN_CASTLING_MASK, ZOBRIST}, moves::Moves, squares::Square, zobrist::START_POSITION_ZOBRIST};

use crate::board::{castling::Castling, fen::FEN, piece::Piece};
use crate::bitboard::Bitboard;
use crate::squares::Square::*;
use crate::color::Color::*;
use crate::move_scope::{MoveScope, *};

#[cfg(test)]
#[path ="./tests.rs"]
mod tests;


#[derive(Debug, Clone, PartialEq, Eq)]
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
        Self { board: PieceMap::new(), turn: Color::White, enpassant: None, castling_rights: Castling::all(), 
            occupancies: [0; OCCUPANCIES], hash_key: START_POSITION_ZOBRIST, fifty: [0, 0],
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

    pub(crate) fn set_occupancy(&mut self, color: Color, occupancy: u64) {
        match color {
            Color::White => self.occupancies[color] |= occupancy, 
            Color::Black => self.occupancies[color] |= occupancy,
            _ => {}
        }
        self.occupancies[Color::Both] |= occupancy;
    }

    pub(crate) fn reset_occupancy_to(&mut self, color: Color, occupancy: u64) {
        match color {
            Color::White => self.occupancies[color] = occupancy, 
            Color::Black => self.occupancies[color] = occupancy,
            _ => {}
        }
        self.occupancies[Color::Both] = occupancy;
    }

    pub(crate) fn get_occupancy(&self, color: Color) -> u64 {
        self.occupancies[color]
    }

    /// Given the current pieces on the board, is this square under attack by the given side (color)
    /// Getting attackable(reachable) spots from this square, it also means this square can be reached from those
    /// squares
    pub(crate) fn is_square_attacked(&self, sq_64: u64, attacker: Color) -> bool {
        // bitboard with only the square's bit set
        let sq_mask = 1u64 << sq_64;
        let sq: Square = Square::from(sq_64);

        let pawn_attackers = self[Piece::pawn(attacker)]; // get the occupancy for the attacking pawns
        if  (PIECE_ATTACKS.pawn_attacks[attacker][sq] & *pawn_attackers) != 0 { return true }
        
        let knights = self[Piece::knight(attacker)]; // knight occupancy for the attacking side
        if (PIECE_ATTACKS.knight_attacks[sq] & *knights) != 0 { return true }


        let king = self[Piece::king(attacker)];
        if (PIECE_ATTACKS.king_attacks[sq] & *king) != 0 { return true }

        let bishops_queens = *self[Piece::queen(attacker)] | *self[Piece::bishop(attacker)];
        if (PIECE_ATTACKS.nnbishop_attacks(sq_mask, self.occupancies[Color::Both]) & bishops_queens) != 0 { return true }
        // println!("dddd");

        let rooks_queens = *self[Piece::queen(attacker)] | *self[Piece::rook(attacker)];
        if (PIECE_ATTACKS.nnrook_attacks(sq_mask, self.occupancies[Color::Both]) & rooks_queens) != 0 { return true }
        // println!("eeee");
        
        false
    }

    
    /// Target for single pawn pushes (black or white)
    /// Returns a value with all the bits set when the pawns of that specific color are pushed
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

    /// Returns a value that has the squares containing only eligible pawns of `color` that can make double push moves
    fn pawns_able_to_double_push(&self, color: Color) -> u64 {
        let empty = !self.occupancies[Color::Both];
        if color == Color::Black {
            let empty_rank6 = Bitboard::from(empty & RANK_5).north() & empty;
            return Bitboard::from(empty_rank6).north() & *self[Piece::BP]
        }
        let empty_rank_3 = Bitboard::from(empty & RANK_4).south() & empty;
        return Bitboard::from(empty_rank_3).south() & *self[Piece::WP];
    }

    /// Returns all possible movements for pawns of a specific color.
    /// If the double argument is true, then only double pawns moves are returned in the result
    pub(crate) fn get_pawn_movement(&self, color: Color, double: bool) -> Vec<Move> {
        match double {
            true => {
                let mut src2 = self.pawns_able_to_double_push(color);
                let mut target2 = self.double_push_targets(color);
                
                let length = target2.count_ones() as usize; // because doubles cannot be promoted
                let mut move_list: Vec<Move> = Vec::with_capacity(length);

                
                while src2 !=0 {
                    let src = src2.trailing_zeros() as u8;
                    let target = target2.trailing_zeros() as u8;

                    
                    // let piece = Piece::pawn(color);
                    let xx = Move::new(src, target, DoublePush);
                    move_list.push(xx);

                    src2 &= src2 -1;
                    target2 &= target2 -1;
                }
                move_list
            }
            false => {
                let mut single_push_targets = self.single_push_targets(color);
                let mut move_list: Vec<Move>  = Vec::with_capacity(single_push_targets.count_ones() as usize);

                while single_push_targets != 0 {
                    let target_sq = single_push_targets & (!single_push_targets + 1);
                    let src_sq = match color {Color::White => Bitboard::from(target_sq).south(), _ => Bitboard::from(target_sq).north()};

                    let t_sq = target_sq.trailing_zeros();
                    let s_sq = src_sq.trailing_zeros();
                    // let piece = Piece::pawn(color);
                    let move_promotes = match color {
                        Color::White => {t_sq >= Square::A8 as u32 && t_sq <= Square::H8 as u32}
                        _ => {t_sq >= Square::A1 as u32 && t_sq <= Square::H1 as u32}
                    };


                    if move_promotes {
                        move_list.push(Move::new(s_sq as u8, t_sq as u8, PromotedToBishop));
                        move_list.push(Move::new(s_sq as u8, t_sq as u8, PromotedToQueen));
                        move_list.push(Move::new(s_sq as u8, t_sq as u8, PromotedToKnight));
                        move_list.push(Move::new(s_sq as u8, t_sq as u8, PromotedToRook));
                    } else {
                        move_list.push(Move::new(s_sq as u8, t_sq as u8, Quiet));
                    }

                    single_push_targets &= single_push_targets - 1;
                }

                move_list
            }
        }
    }


    /// shows what squares this color's pawns (including the src square) can attack
    pub(crate) fn get_pawn_attacks(&self, color: Color) -> Vec<Move> {
        let piece = Piece::pawn(color);
        let mut mv_list: Vec<Move> = vec![];
        let mut color_pawns = *self[piece]; // pawns belonging to this color
        
        while color_pawns != 0 {
            let src: u32 = color_pawns.trailing_zeros();
            let targets = PIECE_ATTACKS.pawn_attacks[!color][Square::from(src as u64)];            
            let mut captures = targets & self.occupancies[!color];

            while captures != 0 {
                let target: u64 = captures.trailing_zeros() as u64;
                
                let can_promote = match color {
                    Color::White => {target >= Square::A8 as u64 && target <= Square::H8 as u64}
                    _ => {target >= Square::A1 as u64 && target <= Square::H1 as u64}
                };

                if can_promote {
                    mv_list.push(Move::new(src as u8, target as u8, CaptureAndPromoteToBishop));
                    mv_list.push(Move::new(src as u8, target as u8, CaptureAndPromoteToRook));
                    mv_list.push(Move::new(src as u8, target as u8, CaptureAndPromoteToKnight));
                    mv_list.push(Move::new(src as u8, target as u8, CaptureAndPromoteToQueen));

                } else {
                    mv_list.push(Move::new(src as u8, target as u8, Capture));

                }
                captures &= captures-1;
            }    
            color_pawns &= color_pawns-1;
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
                    mv_list.push(bmove);
                }
            }
            if enpass_left_attack != 0 {
                if (enpass_left_attack & *self[piece]) != 0 {
                    let source = enpass_left_attack.trailing_zeros();    
                    let bmove = Move::new(source as u8, enpass as u8, Enpassant);
                    mv_list.push(bmove);
                }
            }
        }

        mv_list
    }


    pub(crate) fn get_castling(&self, color: Color) -> Vec<Move> {
        let mut move_list = Vec::with_capacity(2);

        match color {
            Color::White => {
                if (self.castling_rights & Castling::WHITE_KING) != Castling::NONE {
                    let f1g1_empty = (self.occupancies[Color::Both] & 0x60u64) == 0;
                    let e1f1g1_attacked = self.is_square_attacked(u64::from(Square::E1), !color) || self.is_square_attacked(u64::from(Square::F1), !color) || self.is_square_attacked(u64::from(Square::G1), !color);
                    
                    if f1g1_empty && !e1f1g1_attacked {
                        move_list.push(Move::new(Square::E1 as u8, Square::G1 as u8, Castling));
                    }
                }

                if (self.castling_rights & Castling::WHITE_QUEEN) != Castling::NONE {
                    let b1c1d1_empty = (self.occupancies[Color::Both] & 0xe_u64) == 0;
                    let e1c1d1_attacked = self.is_square_attacked(u64::from(Square::E1), !color) || self.is_square_attacked(u64::from(Square::C1), !color)  || self.is_square_attacked(u64::from(Square::D1), !color);

                    if b1c1d1_empty && !e1c1d1_attacked {
                        move_list.push(Move::new(Square::E1 as u8, Square::C1 as u8, Castling));
                    }
                }
            }
            Color::Black => {
                if (self.castling_rights & Castling::BLACK_KING) != Castling::NONE {
                    let f8g8_empty = (self.occupancies[Color::Both] & 0x6000000000000000u64) == 0;
                    let e8f8g8_attacked = self.is_square_attacked(u64::from(Square::E8), !color) || self.is_square_attacked(u64::from(Square::F8), !color) || self.is_square_attacked(u64::from(Square::G8), !color);

                    if f8g8_empty && !e8f8g8_attacked {
                        move_list.push(Move::new(Square::E8 as u8, Square::G8 as u8, Castling));
                    }
                }

                if (self.castling_rights & Castling::BLACK_QUEEN) != Castling::NONE {
                    let b8c8d8_empty = (self.occupancies[Color::Both] & 0xe00000000000000u64) == 0;
                    let e8d8c8_attacked = self.is_square_attacked(u64::from(Square::E8), !color) || self.is_square_attacked(u64::from(Square::D8), !color) || self.is_square_attacked(u64::from(Square::C8), !color);

                    if b8c8d8_empty && !e8d8c8_attacked {
                        move_list.push(Move::new(Square::E8 as u8, Square::C8 as u8, Castling));
                    }
                }
            }
            _ => {}
        }

        move_list
    }


    pub(crate) fn get_sliding_and_leaper_moves(&self, piece: Piece) -> Vec<Move> {
        let mut move_list: Vec<Move> = vec![];
        
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
            // we're getting !self.occupancies[color]s because our knight hould be able to make both quiet or capture moves (on the opponent)
            let mut targets = Bitboard::from(attacks & occupancies);

            let source = src as u32;

            while targets.not_zero() {
                let target = targets.trailing_zeros() as u64;
                // let target = targets.trailing_zeros() as u64;
                // capture move // there is an opponent on the target square
                let opponent_on_target = Bitboard::from(self.occupancies[!color]).get_bit(target) != 0;
                let mvt = if opponent_on_target {Capture} else {Quiet};
                move_list.push(Move::new(source as u8, target as u8, mvt));

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
        move_list.add_many(&self.get_sliding_and_leaper_moves(Piece::knight(color)));
        move_list.add_many(&self.get_sliding_and_leaper_moves(Piece::bishop(color)));
        move_list.add_many(&self.get_sliding_and_leaper_moves(Piece::rook(color)));
        move_list.add_many(&self.get_sliding_and_leaper_moves(Piece::queen(color)));
        move_list.add_many(&self.get_sliding_and_leaper_moves(Piece::king(color)));


        move_list
    }

    /// Returns the rook source and target
    pub(crate) fn validate_castling_move(&self, mv: &Move) -> Option<(Square, Square)> {
        let king_side_mask = 0b1001u64;
        let queen_side_mask = 0b10001u64;

        match mv.get_target() {
            Square::G1 => { // white castles king side
                let shifted_occupancy = self.occupancies[Both] >> (E1 as u64);
                let mask = ((1 << ((H1 - E1) + 1)) - 1) as u64;
                let result = shifted_occupancy & mask;

                let cannot_castle = (self.castling_rights.bits() & WHITE_KING_CASTLING_MASK) == 0;
                if ((result | king_side_mask) != king_side_mask) || cannot_castle {
                    return None;
                }

                return Some((H1, F1));
            }
            Square::G8 => { // black castles king side
                let shifted_occupancy = self.occupancies[Color::Both] >> (E8 as u64);
                let mask = ((1 << ((H8 - E8) + 1)) - 1) as u64;
                let result = shifted_occupancy & mask;

                let cannot_castle = (self.castling_rights.bits() & BLACK_KING_CASTLING_MASK) == 0;

                if ((result | king_side_mask) != king_side_mask) || cannot_castle {
                    return None;
                }

                return Some((H8, F8));
            }
            Square::C1 => { // white castles queen side
                let shifted_occupancy = self.occupancies[Color::Both] >> (A1 as u64);
                let mask = ((1 << (E1  - A1) + 1) - 1) as u64;

                let result = shifted_occupancy & mask;
                let cannot_castle = (self.castling_rights.bits() & WHITE_QUEEN_CASTLING_MASK) == 0;
                
                if ((result | queen_side_mask) != queen_side_mask) || cannot_castle {
                    // println!("the mask is {:05b}", mask);
                    return None;
                }

                return Some((A1, D1));
            }
            Square::C8 => { // black castles queen side
                let shifted_occupancy = self.occupancies[Color::Both] >> (A8 as u64);
                let mask = ((1 << (E8  - A8) + 1) - 1) as u64;

                let result = shifted_occupancy & mask;
                let cannot_castle = (self.castling_rights.bits() & BLACK_QUEEN_CASTLING_MASK) == 0;

                if (result | queen_side_mask) != queen_side_mask || cannot_castle {
                    return None;
                }

                return Some((A8, D8));
            }
            x => unreachable!("Not a valid castling target {x}")
        }
    }


    pub(crate) fn make_move(&self, bit_move: Move, scope: MoveScope) -> Option<Self> {
        let mut board = self.clone();
        
        match scope {
            MoveScope::AllMoves => {
                let from = bit_move.get_src(); // initial position of the piece
                let to = bit_move.get_target(); // target position of the piece

                let Some(piece) = self.piece_at(from) else {return None}; // the piece trying to move
                let turn = self.turn;
                
                if *(self[piece]) & (1 << (from as u64)) == 0 || turn != piece.color() {return None}
                
                // move piece
                board[piece].pop_bit(from.into());
                board[piece].set_bit(to.into());
                board.hash_key ^= ZOBRIST.piece_keys[piece][from];
                board.hash_key ^= ZOBRIST.piece_keys[piece][to];
                
                if Piece::WP == piece || Piece::BP == piece || bit_move.get_capture() {
                    board.fifty = [0, 0];
                }
                
                
                // Removes the captured piece from the the captured piece bitboard
                if bit_move.get_capture() {
                    // there would usually only be a maximum of 2 captures each, consider unrolling this for loop (what did I mean here by 2???????)
                    let target_pieces = Piece::all_pieces_for(!turn);
                   for p in target_pieces {
                        if board[p].get_bit(to.into()) != 0 {
                            board[p].pop_bit(to.into());
                            board.hash_key ^= ZOBRIST.piece_keys[p][to];
                            break;
                        }
                    }
                }
                
                if let Some(promoted_to) = bit_move.get_promotion() { // if this piece is eligible for promotion, the new type it's vying for
                    let promoted_to = Piece::from((promoted_to, turn));
                    board[piece].pop_bit(to.into());
                    board.hash_key ^= ZOBRIST.piece_keys[piece][to];
                    board[promoted_to].set_bit(to.into());
                    board.hash_key ^= ZOBRIST.piece_keys[promoted_to][to];
                }

                
                
                if bit_move.get_enpassant() {
                    let enpass_target = match board.turn {Color::Black => to as u64 + 8, _ => to as u64 -  8};
                    board[Piece::pawn(!turn)].pop_bit(enpass_target);
                    board.hash_key ^= ZOBRIST.piece_keys[Piece::pawn(!turn)][enpass_target as usize];
                }
                
                if let Some(enpass) = board.enpassant {
                    // remove the enpassant from the zobrist_hash if it was there before (this move definitely resulted in an existing enpassant been removed)
                    board.hash_key ^= ZOBRIST.enpassant_keys[enpass as usize];
                }
                board.enpassant = None;

                if bit_move.move_type() == MoveType::DoublePush {
                    let enpass_target = match board.turn {Color::Black => to as u64 + 8, _ => to as u64 -  8};
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

                let old_castling = usize::from_str_radix(&board.castling_rights.bits().to_string(), 10).unwrap();
                board.hash_key ^= ZOBRIST.castle_keys[old_castling];
                let castle_one = board.castling_rights.bits() & CASTLING_TABLE[from];
                let castle_two = castle_one & CASTLING_TABLE[to];
                board.castling_rights = Castling::from(castle_two);
                let new_castling = usize::from_str_radix(&board.castling_rights.bits().to_string(), 10).unwrap();
                board.hash_key ^= ZOBRIST.castle_keys[new_castling];

                board.occupancies[Color::White] = *board[Piece::WP] | *board[Piece::WB] | *board[Piece::WK] | *board[Piece::WN] | *board[Piece::WQ] | *board[Piece::WR];
                board.occupancies[Color::Black] = *board[Piece::BP] | *board[Piece::BB] | *board[Piece::BK] | *board[Piece::BN] | *board[Piece::BQ] | *board[Piece::BR];
                board.occupancies[Color::Both] = board.occupancies[Color::White] | board.occupancies[Color::Black];
                
            
                // is this an illegal move?
                if board.is_square_attacked(board[Piece::king(turn)].get_lsb1().unwrap(), !board.turn) {
                    return None;
                }
                
                board.turn = !board.turn;
                board.hash_key ^= ZOBRIST.side_key;

                if Piece::WP == piece || Piece::BP == piece || bit_move.get_capture() {
                    board.fifty = [0, 0];
                } else {
                    board.fifty[turn] +=1;
                }

                // *self = board;

                Some(board)
            }

            MoveScope::CapturesOnly => {
                if bit_move.get_capture() {
                    return self.make_move(bit_move, MoveScope::AllMoves);
                } else {
                    return None;
                }
            }
        }
    }

    pub(crate) fn piece_at(&self, sq: Square) -> Option<Piece> {
        let sq = 1 << sq as u64;
        let white = (self.occupancies[0] & sq) != 0;
        let black = (self.occupancies[1] & sq) != 0;

        match (white, black) {
            (true, false) => {
                for i in 0..=5 {
                    if (*self.board[i] & sq) != 0 {
                        return Some(Piece::from(i as u8))
                    }
                }
            }
            (false, true) => {
                for i in 6..=11 {
                    if (*self.board[i] & sq) != 0 {
                        return Some(Piece::from(i as u8))
                    }
                }
            }
            _ => {}
        }

        None
    }
    
    pub(crate) fn get_piece_at(&self, sq: Square, color: Color) -> Option<Piece> {
        let target_pieces = Piece::all_pieces_for(color);
        for p in target_pieces {
            if self.board[p].get_bit(sq.into()) != 0 {
                return Some(p);
            }
        }
        None
    }

    /// color: your opponent's/target's color
    pub(crate) fn get_move_capture(&self, mv: Move, color: Color) -> Option<Piece> {
        let target = mv.get_target();
        if mv.get_enpassant() {
            let victim = Square::from(match self.turn {Color::Black => target as u64 + 8, _ => target as u64 -  8});
            return self.get_piece_at(victim, color)
        }
        if mv.get_capture() {
            return self.get_piece_at(mv.get_target(), color)
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
                bitboard &= bitboard -1;
            }
        }

        if let Some(enpass) = self.enpassant {
            final_key ^= ZOBRIST.enpassant_keys[enpass];
            // println!("I see {}", enpass as usize);
        }

        let index = usize::from_str_radix(&self.castling_rights.bits().to_string(), 10).unwrap();
        final_key ^= ZOBRIST.castle_keys[index];

        if self.turn == Color::Black {final_key ^= ZOBRIST.side_key};

        final_key
    }
}


impl FEN for Board {}

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