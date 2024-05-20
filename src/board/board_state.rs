use std::{fmt::Display, ops::{BitAnd, Deref, DerefMut}, sync::Arc};

use bitflags::Flags;

use crate::{bit_move::BitMove, board::board::Board, color::Color, constants::{A1_E1_IS_FILLED, A8_E8_IS_FILLED, BLACK_KING_CASTLING_CELLS, BLACK_QUEEN_CASTLING_CELLS, E1_F1_FILLED, E8_F8_IS_FILLED, NOT_A_FILE, NOT_H_FILE, OCCUPANCIES, PIECE_ATTACKS, RANK_1, RANK_4, RANK_5, RANK_8, SQUARES, WHITE_KING_CASTLING_CELLS, WHITE_QUEEN_CASTLING_CELLS}, move_type::{self, MoveType}, moves::Moves, piece_attacks, squares::Square, Bitboard};

use super::{castling::Castling, fen::FEN, piece::Piece};


#[derive(Debug, Clone)]
pub struct BoardState {
    turn: Color,
    pub board: Board,
    castling_rights: Castling,
    enpassant: Option<Square>,
    occupancies: [u64; OCCUPANCIES], // 0-white, 1-black, 2-both
    // prev: Option<Arc<BoardState>>
}


const PROMOTABLE_TARGETS: usize = 4;

impl BoardState {
    pub fn new() -> BoardState {
        Self { board: Board::new(), turn: Color::White, enpassant: None, castling_rights: Castling::all(), occupancies: [0; OCCUPANCIES] }
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
    /// attackable spots (i_am is hte attacker)
    pub(crate) fn is_square_attacked(&self, sq: u64, i_am: Color) -> bool {
        let index = sq as usize;

        // Attacks by black pawn (can an attack by any black pawn on the board reach this sq)
        if i_am == Color::Black && PIECE_ATTACKS.pawn_attacks[Color::White][index] & u64::from(self[Piece::BP]) !=0 {return true};
        // Attacks by white pawn (can an attack by a white pawn reach this position)
        if i_am == Color::White && PIECE_ATTACKS.pawn_attacks[Color::Black][index] & u64::from(self[Piece::WP]) != 0 {return true};

        let knight_attacks = PIECE_ATTACKS.knight_attacks[index];
        // if there is a knight on this square, can it attack any of my knights(black) on the board
        if i_am == Color::Black && (knight_attacks & u64::from(self[Piece::BN]) != 0) {return true};
        // if there is a knight on this square, can it attack any of my knights(white) on the board
        if i_am == Color::White && (knight_attacks & u64::from(self[Piece::WN]) != 0) {return true};

        let king_attacks = PIECE_ATTACKS.king_attacks[index];
        if i_am == Color::Black && (king_attacks & u64::from(self[Piece::BK])) != 0 {return true}
        if i_am == Color::White && (king_attacks & u64::from(self[Piece::WK])) != 0 {return true}

        let bishop_attacks = PIECE_ATTACKS.get_bishop_attacks(sq, self.get_occupancy(Color::Both));
        if i_am == Color::Black && (bishop_attacks & u64::from(self[Piece::BB])) != 0 {return true}
        if i_am == Color::White && (bishop_attacks & u64::from(self[Piece::WB])) != 0 {return true}

        let rook_attacks = PIECE_ATTACKS.get_rook_attacks(sq, self.get_occupancy(Color::Both));
        if i_am == Color::Black && (rook_attacks & u64::from(self[Piece::BR])) != 0 {return true}
        if i_am == Color::White && (rook_attacks & u64::from(self[Piece::WR])) != 0 {return true}

        let queen_attacks = PIECE_ATTACKS.get_queen_attacks(sq, self.get_occupancy(Color::Both));
        if i_am == Color::Black && (queen_attacks & u64::from(self[Piece::BQ])) != 0 {return true}
        if i_am == Color::White && (queen_attacks & u64::from(self[Piece::WQ])) != 0 {return true}

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
        if color == Color::Black {
            return *self[Piece::BP] & self.pawn_any_attack(Color::White, false)
        }
        *self[Piece::WP] & self.pawn_any_attack(Color::Black, false)
    }

    
    /// Push pawn(black or white) by one
    fn single_push_targets(&self, color: Color) -> u64 {
        if color == Color::Black {
            return self[Piece::BP].south() & !self.occupancies[Color::Both]
        }

        self[Piece::WP].north() & !self.occupancies[Color::Both]
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
        if color == Color::White {
            return Bitboard::from(!self.occupancies[Color::Both]).south() & *self[Piece::WP]    
        }
        Bitboard::from(!self.occupancies[Color::Both]).north() & *self[Piece::BP]
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
                    move_list.push(BitMove::new(sindex, tindex, piece, None, false, true, false, false));

                    src2 &= src2 -1;
                    target2 &= target2 -1;
                }
                move_list
            }
            false => {
                let mut src = self.pawns_able_2push(color);
                let mut targets = self.single_push_targets(color);

                let mut length = targets.count_ones() as usize; // because doubles cannot be promoted
                let promotable = if color == Color::White {(targets & RANK_8).count_ones()} else {(targets & RANK_1).count_ones()} as usize;
                length = (length - promotable) + (promotable * PROMOTABLE_TARGETS);

                let mut move_list: Vec<BitMove> = Vec::with_capacity(length);
                
                while src != 0 {
                    let sindex = src.trailing_zeros();
                    let tindex = targets.trailing_zeros();
                    
                    let move_promotes = tindex >= Square::A8 as u32 && tindex <= Square::H8 as u32;
                    let piece = Piece::pawn(color);

                    if move_promotes {
                        move_list.push(BitMove::new(sindex, tindex, piece, Some(Piece::bishop(color)), false, false, false, false));
                        move_list.push(BitMove::new(sindex, tindex, piece, Some(Piece::queen(color)), false, false, false, false));
                        move_list.push(BitMove::new(sindex, tindex, piece, Some(Piece::knight(color)), false, false, false, false));
                        move_list.push(BitMove::new(sindex, tindex, piece, Some(Piece::rook(color)), false, false, false, false));
                    } else {
                        move_list.push(BitMove::new(sindex, tindex, piece, None, false, false, false, false));
                    }

                    src &= src -1;
                    targets &= targets - 1;
                }
                move_list
            }
        }
    }


    /// shows what squares this color's pawns (including the src square) can attack
    pub(crate) fn get_pawn_attacks(&self, color: Color) -> Vec<BitMove> {
        let piece = Piece::pawn(color);
        
        let mut capture = self.pawns_able_2capture_any(color);
        
        let mut length = capture.count_ones() as usize; // because doubles cannot be promoted
        let promotable = if color == Color::White {(capture & RANK_8).count_ones()} else {(capture & RANK_1).count_ones()} as usize;
        length = (length - promotable) + (promotable * PROMOTABLE_TARGETS);
        let mut move_list: Vec<BitMove> = Vec::with_capacity(length);


        // println!()
        while capture != 0 {
            let src = capture.trailing_zeros();
            let left_target = if color == Color::Black {(capture >> 9).trailing_zeros()} else {(capture << 7).trailing_zeros()};
            let right_target = if color == Color::Black {(capture >> 7).trailing_zeros()} else {(capture << 9).trailing_zeros()};
            
            let left_attacker_exists = Bitboard::from(self.occupancies[!color]).get_bit_by_square((left_target as u64).into());
            let right_attacker_exists = Bitboard::from(self.occupancies[!color]).get_bit_by_square((right_target as u64).into());

            // do we have enpassant captures
            let (left_is_enpassant, right_is_enpassant) = match self.enpassant {
                Some(enpassant) => {
                    (enpassant == (left_target as u64).into(), enpassant == (right_target as u64).into())}
                None => (false, false)
            };
            
            
            let (left_promotes, right_promotes) = match color {
                Color::White => (
                    left_attacker_exists != 0 && (left_target >= Square::A8 as u32 && left_target <= Square::H8 as u32), 
                    right_attacker_exists != 0 && (right_target >= Square::A8 as u32 && right_target <= Square::H8 as u32)
                ),
                Color::Black => {
                        ( left_attacker_exists != 0 &&  (left_target >= Square::A1 as u32 && left_target <= Square::H1 as u32),
                            right_attacker_exists != 0 && right_target >= Square::A1 as u32 && right_target <= Square::H1 as u32)
                }, 
                _ => {unreachable!("")}};


            // attacking the target on the LHS will result in this pawn's promotion (enpassant cannot occur at this point anymore)
            [(left_promotes, left_target), (right_promotes, right_target)].iter().filter(|(p, _)| *p == true).for_each(|(_, target)| {
                move_list.push(BitMove::new(src, *target, piece, Some(Piece::bishop(color)), true, false, false, false));
                move_list.push(BitMove::new(src, *target, piece, Some(Piece::queen(color)), true, false, false, false));
                move_list.push(BitMove::new(src, *target, piece, Some(Piece::knight(color)), true, false, false, false));
                move_list.push(BitMove::new(src, *target, piece, Some(Piece::rook(color)), true, false, false, false));
            });
            

            let left_enpassant_move = (left_attacker_exists == 0 && !left_promotes && left_is_enpassant);
            if (left_attacker_exists != 0 && !left_promotes) || left_enpassant_move {
                move_list.push(BitMove::new(src, left_target, piece, None, true, false, left_is_enpassant, false));
            }

            let right_enpassant_move = left_attacker_exists == 0 && !left_promotes && left_is_enpassant;
            if (right_attacker_exists != 0 && !right_promotes) || right_enpassant_move {
                move_list.push(BitMove::new(src, right_target, piece, None, true, false, right_is_enpassant, false));
            }
            capture &= capture-1;
        }

        move_list

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

    fn get_castling(&self, color: Color) -> Vec<BitMove> {
        let mut move_list = Vec::with_capacity(2);

        match color {
            Color::White => {
                // rank 1 cell 4,5,6,7
                let king_castling_cells = self.occupancies[Color::Both] & WHITE_KING_CASTLING_CELLS;
                let only_expected_cells_are_filled = (king_castling_cells ^ E1_F1_FILLED) == 0;

                let src = Square::E1 as u32;

                // king castling is available and the square between king and rook (f and g are empty)
                if (self.castling_rights & Castling::WHITE_KING) != Castling::NONE && only_expected_cells_are_filled {
                    let no_attacks = !self.is_square_attacked(Square::E1.into(), !color) && !self.is_square_attacked(Square::F1.into(), !color);
                    if no_attacks {
                        move_list.push(BitMove::new(src, Square::G1 as u32, Piece::WK, None, false, false, false, true));
                    }
                }

                let queen_castling_cells = self.occupancies[Color::Both] & WHITE_QUEEN_CASTLING_CELLS;
                let only_expected_cells_are_filled = (queen_castling_cells ^ A1_E1_IS_FILLED) == 0;
                if (self.castling_rights & Castling::WHITE_QUEEN) != Castling::NONE && only_expected_cells_are_filled {
                    let no_attacks = !self.is_square_attacked(Square::E1.into(), !color) && !self.is_square_attacked(Square::D1.into(), !color);
                    if no_attacks {
                        move_list.push(BitMove::new(src, Square::C1 as u32, Piece::WK, None, false, false, false, true));
                    }
                }
            }
            Color::Black => {
                let king_castling_cells = self.occupancies[Color::Both] & BLACK_KING_CASTLING_CELLS;
                let only_expected_cells_are_filled = (king_castling_cells ^ E8_F8_IS_FILLED) == 0;

                let src = Square::E8 as u32;


                // king castling is available and the square between king and rook (f and g are empty)
                if (self.castling_rights & Castling::BLACK_KING) != Castling::NONE && only_expected_cells_are_filled {
                    let no_attacks = !self.is_square_attacked(Square::E8.into(), !color) && !self.is_square_attacked(Square::F8.into(), !color);
                    if no_attacks {
                        move_list.push(BitMove::new(src, Square::G8 as u32, Piece::BK, None, false, false, false, true));
                    }
                }

                let queen_castling_cells = self.occupancies[Color::Both] & BLACK_QUEEN_CASTLING_CELLS;
                // println!("{}", Bitboard::from(queen_castling_cells).to_string());
                let only_expected_cells_are_filled = (queen_castling_cells ^ A8_E8_IS_FILLED) == 0;
                if (self.castling_rights & Castling::BLACK_QUEEN) != Castling::NONE && only_expected_cells_are_filled {
                    let no_attacks = !self.is_square_attacked(Square::E8.into(), !color) && !self.is_square_attacked(Square::D8.into(), !color);
                    if no_attacks {
                        move_list.push(BitMove::new(src, Square::C8 as u32, Piece::BK, None, false, false, false, true));
                    }
                }
            }
            _ => {}
        }

        move_list
    }


    fn get_sliding_and_leaper_moves(&self, color: Color, piece: Piece) -> Vec<BitMove> {
        // let knight = if color == Color::White {Piece::WN} else {Piece::BN};
        let mut move_list: Vec<BitMove> = vec![];
        
        let mut pieces_on_board = self[piece];

        while pieces_on_board.not_zero() {
            let square = pieces_on_board.get_lsb1().unwrap();
            pieces_on_board.pop_bit(square);
            let src = Square::from(square);
            
            let (attacks, occupancies) = match piece {
                Piece::WN | Piece::BN => (PIECE_ATTACKS.knight_attacks[src], !self.occupancies[color]),
                Piece::WB | Piece::BB => (PIECE_ATTACKS.get_bishop_attacks(square, self.occupancies[Color::Both]), !self.occupancies[color]),
                Piece::WR | Piece::BR  => (PIECE_ATTACKS.get_rook_attacks(square, self.occupancies[Color::Both]), !self.occupancies[color]),
                Piece::WQ | Piece::BQ => (PIECE_ATTACKS.get_queen_attacks(square, self.occupancies[Color::Both]), !self.occupancies[color]),
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
        move_list.add_many(&self.get_sliding_and_leaper_moves(color, if color == Color::Black {Piece::BN}else {Piece::WN}));
        move_list.add_many(&self.get_sliding_and_leaper_moves(color, if color == Color::Black {Piece::BB}else {Piece::WB}));
        move_list.add_many(&self.get_sliding_and_leaper_moves(color, if color == Color::Black {Piece::BR}else {Piece::WR}));
        move_list.add_many(&self.get_sliding_and_leaper_moves(color, if color == Color::Black {Piece::BQ}else {Piece::WQ}));
        move_list.add_many(&self.get_sliding_and_leaper_moves(color, if color == Color::Black {Piece::BK}else {Piece::WK}));

        // move_list.list.setle

        // generate knight squares

        move_list
    }


    pub(crate) fn make_move(&self, bit_move: BitMove, move_type: MoveType) -> Option<Self> {
        let mut board = self.clone();
        let turn = board.turn;
        // board.prev = Some(Arc::new(self));

        match move_type {
            MoveType::AllMoves => {
                let src = bit_move.get_src();
                let target = bit_move.get_target();
                let prev_piece = bit_move.get_piece();
                
                let new_piece = match bit_move.get_promotion() {
                    Some(p) => p, None => prev_piece
                };
                
                // let promoted_piece = bit_move.get_promotion();

                
                // Removes the captured piece from the the captured piece bitboard
                if bit_move.get_capture() {
                    let target_pieces = board[Piece::pawn(!turn).into()..Piece::king(!turn).into()].iter_mut();
                    // let target_pieces = match board.turn {Color::Black => {board[Piece::WP.into()..=Piece::WK.into()].iter_mut()}, _ => {board[Piece::BP.into()..=Piece::BK.into()].iter_mut()}};
    
                    for bitboard in target_pieces {
                        if bitboard.get_bit(target.into()) != 0 {
                            bitboard.pop_bit(target.into());
                            break;
                        };
                    }
                }
                
                board[new_piece].pop_bit(src.into());
                board[new_piece].set_bit(target.into());
                
                
                if bit_move.get_enpassant() {
                    let enpass_target = match board.turn {Color::Black => target as u64 + 8, _ => target as u64 -  8};
                    board[Piece::pawn(!turn)].pop_bit(enpass_target);
                }

                board.enpassant = None;

                if bit_move.get_double_push() {
                    let enpass_target = match board.turn {Color::Black => target as u64 + 8, _ => target as u64 -  8};
                    board.enpassant = Some(enpass_target.into());
                }

                if bit_move.get_castling() {
                    match target {
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

                let castling_rights = board.new_castling_rights(src, target);
                board.castling_rights = castling_rights;

                board.occupancies[Color::White] = *board[Piece::WP] | *board[Piece::WB] | *board[Piece::WK] | *board[Piece::WN] | *board[Piece::WQ] | *board[Piece::WR];
                board.occupancies[Color::Black] = *board[Piece::BP] | *board[Piece::BB] | *board[Piece::BK] | *board[Piece::BN] | *board[Piece::BQ] | *board[Piece::BR];
                board.occupancies[Color::Both] = board.occupancies[Color::White] | board.occupancies[Color::Black];

                
                // is this an illegal move?
                if board.is_square_attacked(board[Piece::king(turn)].get_lsb1().unwrap(), !turn) {
                    return None;
                }


                board.turn = !board.turn;
            }

            MoveType::CapturesOnly => {}
        }

        Some(board)
    }


    pub(crate) fn new_castling_rights(&mut self, from: Square, to: Square) -> Castling {
        let new_mask = from.castling_mask() | to.castling_mask();
        let existing_rights = self.castling_rights.bits() & new_mask;
        let new_rights = self.castling_rights.bits().bitand(!existing_rights);
        Castling::from(new_rights)
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