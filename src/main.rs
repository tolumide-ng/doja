mod bitboard;
mod squares;
mod color;
mod shift;
mod board;
mod constants;
mod magic;
mod moves;
mod random_magic;
mod piece_attacks;
mod bit_move;
mod move_type;
mod perft;
 mod kogge_stone;

use std::{borrow::Cow, io, ops::Shl, sync::Arc};

use board::fen::FEN;
// use bishop::Bishop;
pub use bitboard::Bitboard;
use constants::NOT_H_FILE;
use kogge_stone::KoggeStone;
use perft::Perft;
// use crate::constants::

use crate::{bit_move::BitMove, board::{board_state::BoardState, piece::Piece}, color::Color, constants::{CMK_POSITION, KILLER_POSITION, NOT_A_FILE, PIECE_ATTACKS, RANK_4, START_POSITION, TRICKY_POSITION}, move_type::MoveType, moves::Moves, shift::Shift, squares::{Square, BISHOP_RELEVANT_BITS, SQUARE_NAMES}};




fn main() {
    // let board = BoardState::parse_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1 ").unwrap();
    // let mv = BitMove::new(Square::D2 as u32, Square::C1 as u32, Piece::WB, None, false, false, false, false);
    // let dd = board.make_move(mv, MoveType::AllMoves).unwrap();
    // println!("{}", dd.to_string());
    // println!("{}", board.is_square_attacked(31, Color::Black));
    // // // println!("{}", BitMove::new(Square::A2, Square::A4, Piece::pawn(Color::White), None, false, true, false, false));
    // // let double = BitMove::new(Square::A2 as u32, Square::A4 as u32, Piece::WP, None, false, true, false, false);
    // // let enpass_move = BitMove::new(Square::B4 as u32, Square::A3 as u32, Piece::BP, None, true, false, true, false);

    // // let new_board = board.make_move(double, MoveType::AllMoves).unwrap();
    // // let new_new_board = new_board.make_move(enpass_move, MoveType::AllMoves).unwrap();
    // println!("{}", board.to_string());
    // // println!("{}", new_board.to_string());
    // // println!("{}", new_new_board.to_string());
    
    Perft::start(7);
    
    // // let x = 61u64;
    // // let xb = 1 << x;


    // let movess = board.get_pawn_attacks(Color::White);
    // movess.iter().for_each(|x| {
    //     println!("{}", x);
    // });

    // // let plays = board.get_pawn_attacks(Color::Black);
    // let castles = board.get_castling(Color::White);
    // println!("{}", board.to_string());
    // castles.iter().for_each(|x| {
    //     println!("{}", x);
    // });

    // println!("{}", Bitboard::from(0x800000).to_string());

    // plays.iter().for_each(|x| println!("from={} to={} enpass={} promo={:?}", x.get_src(), x.get_target(), x.get_enpassant(), x.get_promotion()));




    // println!("{} {}", u64::from(Square::D4), Square::D4 as u64);
    // let x = 1 << u64::from(Square::C2);
    // println!("{}", Bitboard::from(x).to_string());
    // println!("move");
    // let xx = Bitboard::south_west1(x);
    // println!("{}", Bitboard::from(xx).to_string());
    // let xxx = Square::from(xx.trailing_zeros() as u64);
    // println!("after {:?}", xxx);
    // let xxtt = xx.trailing_zeros() as u64;
    // let xxot = xxtt as u32;
    // println!("after {:?}", Square::from(xxot as u64));



    // let mask = 1 << Square::A5 as u64;
    // println!("{}", Bitboard::from(mask).to_string());

    // let new_mask = (mask  >> 9u64) & NOT_H_FILE;
    // println!("{}", Bitboard::from(new_mask).to_string());


    // println!("{0:b}", !0b0010100u8);
    // let xx = BoardState::parse_fen(START_POSITION).unwrap();
    // println!("{}", xx.to_string());

    // // let moves = xx.get_pawn_movement(Color::Black, false);
    // println!("::::::::: {:?}", xx.get_pawn_movement(Color::White, false).iter().for_each(|m| {
    //         println!("FROM =={}  TO =={} CAPTURE=={}", m.get_src(), m.get_target(), m.get_capture());
    //         println!("{:?}", m);
    //     }));
        // let xxx = BitMove::from(1032);
        // println!("FROM =={}  TO =={} CAPTURE=={}", xxx.get_src(), xxx.get_target(), xxx.get_capture());
    // println!("{:?}", moves);






    // let sw = Bitboard::from(Bitboard::from(0x210041200).south_west());
    // let pre_sw = Bitboard::from(Bitboard::from(0x210041200).pre_south_west());

    // println!("{}", sw.to_string());
    // println!("{}", pre_sw.to_string());

    // let no_attacks = !self.is_square_attacked(Square::E1.into(), !color) && !self.is_square_attacked(Square::F1.into(), !color);

    // let points = 1u64 << Square::D5 as u64 | 1u64 << Square::G4 as u64;

    // println!("the points are {}", Bitboard::from(points).to_string());

    // let lexd = Bitboard::from(1u64.shl(Square::D4 as u64));
    // println!("{}", lexd.to_string());
    // println!("{}", Bitboard::from(Square::D4 as u64).to_string());

    // let xo = Bitboard::from(PIECE_ATTACKS.nnbishop_attacks(points, 0));
    // println!("{}", xo.to_string());

    
    
}
