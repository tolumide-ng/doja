use std::{fmt::Display, ops::{Deref, DerefMut}};

use crate::{board::board::Board, color::Color, squares::Square};

use super::castling::Castling;

pub struct BoardState {
    board: Board,
    turn: Color,
    enpassant: Square,
    castling_rights: Castling
}


impl BoardState {
    pub fn new() -> BoardState {
        let castling = Castling::WHITE_KING | Castling::BLACK_KING | Castling::WHITE_QUEEN | Castling::BLACK_QUEEN;
        Self { board: Board::new(), turn: Color::White, enpassant: Square::E3, castling_rights: castling }
    }
}

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