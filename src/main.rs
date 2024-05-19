mod bitboard;
mod squares;
mod color;
mod board;
mod constants;
mod magic;
mod moves;
mod random_magic;
mod piece_attacks;
mod bit_move;
mod move_type;

use std::{borrow::Cow, io, sync::Arc};

use board::fen::FEN;
// use bishop::Bishop;
pub use bitboard::Bitboard;
// use crate::constants::

use crate::{bit_move::BitMove, board::{board_state::BoardState, piece::Piece}, color::Color, constants::{CMK_POSITION, KILLER_POSITION, RANK_4, START_POSITION, TRICKY_POSITION}, move_type::MoveType, moves::Moves, squares::{Square, BISHOP_RELEVANT_BITS, SQUARE_NAMES}};




fn main() {
    let board = BoardState::parse_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBbPPP/R3K2R w KQkq - 0 1 ").unwrap();

    // let xx = Arc::new(&board);
    println!("{}", board.to_string());
    let move_list =board.gen_movement();

    for index in 0..move_list.count() {
        let bmove = move_list.list[index];
        let resulting_board = board.make_move(bmove, MoveType::AllMoves);
        println!(" making the move {}", bmove.to_string());
        if let Some(new_board) = resulting_board {
            println!("{:?}", new_board.to_string());
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
        }
    }


    println!("{}", move_list);
}
