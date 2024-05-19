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

use std::{borrow::Cow, sync::Arc};

use board::fen::FEN;
// use bishop::Bishop;
pub use bitboard::Bitboard;
use constants::PIECE_ATTACKS;
// use crate::constants::

use crate::{bit_move::BitMove, board::{board_state::BoardState, piece::Piece}, color::Color, constants::{CMK_POSITION, KILLER_POSITION, RANK_4, START_POSITION, TRICKY_POSITION}, move_type::MoveType, moves::Moves, squares::{Square, BISHOP_RELEVANT_BITS, SQUARE_NAMES}};




fn main() {
    let board = BoardState::parse_fen("r3k2r/p11pqpb1/bn2pnp1/2pPN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq c6 0 1 ").unwrap();

    // let xx = Arc::new(&board);
    println!("{}", board.to_string());
    let move_list =board.gen_movement();

    for index in 0..move_list.count() {
        let bmove = move_list.list[index];
        let resulting_board = board.make_move(bmove, MoveType::AllMoves);
        println!(" making the move {}", bmove.to_string());
        println!("{:?}", resulting_board.to_string());
    }

    // for bit_move in move_list {
    //     // let resulting_board = board.make_move(bit_move, MoveType::AllMoves);
    //     println!(" making the move {}", bit_move.to_string());
    //     // println!("{:?}", resulting_board.to_string());
    // }

    println!("{}", move_list);
}
