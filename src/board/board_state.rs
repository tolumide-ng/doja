use std::{fmt::Display, ops::{Deref, DerefMut}};

use crate::{board::board::Board, color::Color, constants::{FILE, OCCUPANCIES, PLAYERS_COUNT, RANK, SQUARES}, king::King, knight::Knight, magic::plain::PlainAttacks, pawn::Pawn, squares::Square, Bitboard};

use super::{castling::Castling, fen::FEN, piece::Piece};

pub struct BoardState {
    board: Board,
    turn: Color,
    enpassant: Option<Square>,
    castling_rights: Castling,
    occupancies: [u64; OCCUPANCIES]
}


impl BoardState {
    pub fn new() -> BoardState {
        Self { board: Board::new(), turn: Color::White, enpassant: None, castling_rights: Castling::NONE, occupancies: [0; OCCUPANCIES] }
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
        self.occupancies[color as usize] |= occupancy;
    }

    pub(crate) fn get_occupancy(&self, color: Color) -> u64 {
        self.occupancies[color as usize]
    }

    /// Given the current pieces on the board, is this square under attack by the given side (color)
    pub(crate) fn is_square_attacked(&self, sq: u64, i_am: Color) -> bool {
        let square = Square::from(sq);

        // let knight_attacks = u64::from(Knight::bitboard_knight_attacks(sq));
        // let king_attacks = u64::from(King::bitboard_knight_attacks(sq));
        // let bishop_attacks = u64::from(PlainAttacks::get_bishop_attacks(square, self.get_occupancy(Color::Both)));
        // let rook_attacks = u64::from(PlainAttacks::get_rook_attacks(square, self.get_occupancy(Color::Both)));
        // let queen_attacks = u64::from(PlainAttacks::get_queen_attacks(square, self.get_occupancy(Color::Both)));


        // Attacks by black pawn (can an attack by any black pawn on the board reach this sq)
        if i_am == Color::Black && u64::from(Pawn::mask_pawn_attacks(Color::White, sq)) & u64::from(self[Piece::BP as usize]) !=0 {return true};
        // Attacks by white pawn (can an attack by a white pawn reach this position)
        if i_am == Color::White && u64::from(Pawn::mask_pawn_attacks(Color::Black, sq)) & u64::from(self[Piece::WP as usize]) != 0 {return true};

        let knight_attacks = u64::from(Knight::bitboard_knight_attacks(sq));
        // if there is a knight on this square, can it attack any of my knights(black) on the board
        if i_am == Color::Black && (knight_attacks & u64::from(self[Piece::BN as usize]) != 0) {return true};
        // if there is a knight on this square, can it attack any of my knights(white) on the board
        if i_am == Color::White && (knight_attacks & u64::from(self[Piece::WN as usize]) != 0) {return true};

        let king_attacks = King::mask_king_attacks(sq);
        if i_am == Color::Black && (king_attacks & u64::from(self[Piece::BK as usize])) != 0 {return true}
        if i_am == Color::White && (king_attacks & u64::from(self[Piece::BK as usize])) != 0 {return true}

        let bishop_attacks = PlainAttacks::get_bishop_attacks(sq, self.get_occupancy(Color::Black));
        if i_am == Color::Black && (bishop_attacks & u64::from(self[Piece::BB as usize])) != 0 {return true}
        if i_am == Color::White && (bishop_attacks & u64::from(self[Piece::WB as usize])) != 0 {return true}
        false
    }

    // print all the squares that the current color can attack or move to.
    pub(crate) fn get_possible_destination_squares_for(&self, side: Color) -> Bitboard {
        let mut sample_bitboard = Bitboard::new();

        for sq in 0..(SQUARES as u64) {
            if self.is_square_attacked(sq, side) {
                println!("sql is >>>>>> {sq}");
                sample_bitboard.set_bit(sq)
            }
        }

        sample_bitboard
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