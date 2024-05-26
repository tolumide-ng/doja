mod bitboard;
mod squares;
mod color;
mod shift;
mod board;
mod constants;
// mod magic;
mod moves;
// mod random_magic;
mod piece_attacks;
mod bit_move;
mod move_type;
mod perft;
 mod kogge_stone;
 mod uci;


use bit_move::BitMove;
use bitboard::Bitboard;
use board::{board_state::BoardState, fen::FEN, piece::Piece};
use constants::{START_POSITION, TRICKY_POSITION};
use perft::Perft;
use squares::Square;
use uci::UCI;




fn main() {
    // Perft::start(5);
    // UCI::parse(String::from("e2c4"));
    // println!("{}", Bitboard::from(0xf0000).to_string())
    // let mv = BitMove::new(Square::A1 as u32, Square::B2 as u32, Piece::WB, None, false, false, false, false);

    let board = BoardState::parse_fen("r3k2r/pPppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1 ").unwrap();

    if let Some(n_board) = UCI::parse(&board, String::from("b7b8q")) {
        println!("move works");
        println!("{}", n_board.to_string());
    } else {
        println!("Illegal move");
    }
}

