use std::{fmt::Display, ops::{Deref, DerefMut}};

use crate::{board::board::Board, color::Color, constants::{OCCUPANCIES, PLAYERS_COUNT}, squares::Square, Bitboard};

use super::{castling::Castling, fen::FEN};

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