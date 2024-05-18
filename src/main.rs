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

use board::fen::FEN;
// use bishop::Bishop;
pub use bitboard::Bitboard;
use constants::PIECE_ATTACKS;
// use crate::constants::

use crate::{bit_move::NBitMove, board::{board_state::BoardState, piece::Piece}, color::Color, constants::{CMK_POSITION, KILLER_POSITION, RANK_4, START_POSITION, TRICKY_POSITION}, squares::{Square, BISHOP_RELEVANT_BITS, SQUARE_NAMES}};




fn main() {
    // let board = BoardState::parse_fen(TRICKY_POSITION).unwrap();
    let board = BoardState::parse_fen("r2rk2r/p1ppqpb1/bn2pnp1/3PN3/Pp2P3/2N2Q1p/PPPBr1bP/R3K2R b KQkq a3 0 1 ").unwrap();

    println!("{}", board.to_string());
    // board.double_push_targets(Color::White);

    // println!("{}", Bitboard::from(RANK_4).to_string());

    // println!("{}", board[6].to_string());
    // println!("{}", Bitboard::from(board.pawns_able_to_double_push(Color::White)).to_string());
    // println!("{}", Bitboard::from(board.double_push_targets(Color::White)).to_string());
    
    // println!("{}", Bitboard::from(board.pawns_able_to_2push(Color::Black)).to_string());
    // println!("{}", Bitboard::from(board.single_push_targets(Color::Black)).to_string());
    // let x = board.get_pawn_movement(Color::Black, false);
    // println!("{}", Bitboard::from(board.pawn_single_attack(Color::White)).to_string());
    // println!("{}", Bitboard::from(board.pawns_able_2capture_any(Color::Black)).to_string());
    // let x = board.pawn_any_attack(Color::Black) & *board[Piece::WP];
    // println!("{}", Bitboard::from(x).to_string());
    // board.get_pawn_attacks(Color::Black);
    // board.get_pawn_movement(Color::Black, true);
    // board.get_pawn_movement(Color::Black, false);

    board.gen_movement(Color::White);

    let mv = NBitMove::new(32u32, 20u32, Piece::BN, Piece::WB, false, true, false, true);

    println!("{:?}", mv.get_src());
    println!("{:?}", mv.get_target());
    println!("{:?}", mv.get_promotion());
    println!("{:?}", mv.get_piece());
    println!("{:?}", mv.get_capture());
    println!("{:?}", mv.get_double_push());
    println!("{:?}", mv.get_enpassant());
    println!("{:?}", mv.get_castling());

    // println!("{:#?}", Bitboard::from(0b0011_1111).to_string());

    // println!("{}", Bitboard::from(0x7F_7F_7F_7F_7F_7F));
    
    // println!("{}", board.get_possible_destination_squares_for(Color::White));
    // let x = Bitboard::from(0b1111111100000000);
    // println!("{}", x.to_string());

    // println!("{}", board.to_string());
    // println!("{}", board.get_possible_destination_squares_for(Color::Black).to_string());


    // let point = board[Piece::BP as usize];
    // println!("{}", point.to_string());
    
    
    // let mut occ = Bitboard::new();
    // occ.set_bit(Square::G4.into());

    // let x = PIECE_ATTACKS.get_queen_attacks(Square::D4.into(), occ.into());
    // println!("{}", Bitboard::from(x));

    // let pa =Pawn::bitboard_pawn_attacks(Color::White, Square::B3.into());
    // println!("{}", pa.to_string());


    // let pw = u64::from(pa);
    // println!("{}", Bitboard::from(pw & u64::from(point)).to_string());

    // println!("{}", Bitboard::from(board.get_occupancy(Color::White)));
    // // println!("{}", Bitboard::from(board.get_occupancy(Color::Both)));git 
}
