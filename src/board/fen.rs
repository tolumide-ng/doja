use thiserror::Error;
use crate::{board::{castling::Castling, piece::Piece}, color::Color, squares::{Square, SQUARE_NAMES}};

use crate::board::state::board::Board;


#[derive(Error, Debug, PartialEq, Eq)]
pub enum FENError {
    #[error("Malformed Ranks in FEN string: (expected 8, found {rank}")]
    MalformedRanks { rank: u8 },
    #[error("Malformed Files on Rank {rank}: (expected 8 files, found {file})")]
    MalformedFiles {rank: u8, file: u8},
    #[error("At least 4 blocks expected, found only {blocks}")]
    NotEnoughBlocks {blocks: u8},
    #[error("Incomplete Enpassant")]
    IncompletedEnpassant,
    #[error("Enpassant Invalid")]
    EnpassantInvalid,
}

pub trait FEN {
    fn parse_fen(fen: &str) -> Result<Board, FENError> {
        let mut board = Board::new();

        let fen_blocks = fen.split_whitespace().collect::<Vec<_>>();
        if fen_blocks.len() < 4 {
            return Err(FENError::NotEnoughBlocks { blocks: fen_blocks.len() as u8 })
        }

        let total_ranks = fen_blocks[0].split("/").collect::<Vec<_>>().len();
        if total_ranks != 8 {
            return Err(FENError::MalformedRanks {rank: total_ranks as u8});
        }

        let pieces = fen_blocks[0].chars().collect::<Vec<char>>();
        let mut square: u64 = 64;
        
        for char in pieces {
            let file = ((64-square)%8) as u8;
            
            if char == '/' {
                if square % 8 != 0 {
                    let rank = (square % 8) as u8;
                    return Err(FENError::MalformedFiles { rank, file });
                }
                continue;
            }
            
            if char.is_ascii_digit() {
                square -= char.to_digit(10).unwrap() as u64;
                continue;
            }


            
            if char.is_ascii_alphabetic() {
                let rank = (square -1) / 8;
                let piece = Piece::from(char);
                let sq = ((rank * 8) + file as u64) as u64;
                board[piece].set_bit(sq);
            }
            
            if char.is_ascii_whitespace() {
                break;
            }
            
            square -=1;
        }
        
        let turn = fen_blocks[1];
        let color = Color::from(turn);

        let castling = fen_blocks[2];
        let rights = Castling::from(castling);

        let mut enpass: Option<Square> = None;

        if fen_blocks[3] != "-" {
            let sq = fen_blocks[3].to_ascii_uppercase();
            let position = SQUARE_NAMES.iter().position(|s| *s == sq);
            if let Some(index) = position {
                enpass = Some(Square::from(index as u64));
            }
        }

        // setup occupancies
        for (white, black) in Piece::all_pieces_for(Color::White).into_iter().zip(Piece::all_pieces_for(Color::Black)) {
            board.set_occupancy(Color::White, board[white as usize].into());
            board.set_occupancy(Color::Black, board[black as usize].into());
        }

        let both = board.get_occupancy(Color::White) | board.get_occupancy(Color::Black);
        board.set_occupancy(Color::Both, both);



        // if fen_blocks[3] == "-" {
        //     if fen_blocks.len() < 6 {
        //         return Err(FENError::IncompletedEnpassant);
        //     }

        //     // let (rank, file) = (fen_blocks[4], fen_blocks[5]);
        //     match (u64::from_str_radix(fen_blocks[4], 10), u64::from_str_radix(fen_blocks[5], 10)) {
        //         (Ok(rank), Ok(file)) => {
        //             enpass = Some(Square::from((rank * 8) + file));
        //         }
        //         _ => {
        //             return Err(FENError::EnpassantInvalid)
        //         }
        //     }
        // } else {
        //     enpass = Some(Square::NoSquare)
        // }

        board.set_turn(color);
        board.set_enpassant(enpass);
        board.set_castling(rights);
        let zobrist_key = board.hash_key();
        board.set_zobrist(zobrist_key);

        
        Ok(board)
    }
}




#[cfg(test)]
mod fen_tests {
    use crate::{board::{state::board::Board, castling::Castling, fen::FENError}, color::Color};

    use super::FEN;

            // rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
    struct TestStruct;
    impl FEN for TestStruct {}

    #[cfg(test)]
    mod should_fail_if {
        use super::*;

        #[test]
        fn the_files_are_not_complete() {
            let fen= "rnbqkbnr/ppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

            let result = TestStruct::parse_fen(fen);
            assert_eq!(result.unwrap_err(), FENError::MalformedFiles { rank: 1, file: 7 });
        }

        #[test]
        fn the_ranks_are_incomplete() {
            let fen = "rnbqkbnr/pppppppp/8/8/8/8/RNBQKBNR w KQkq - 0 1";

            let result = TestStruct::parse_fen(fen);
            assert_eq!(result.unwrap_err(), FENError::MalformedRanks { rank: 7 });
        }

        #[test]
        fn the_fen_blocks_are_incomplete() {
            let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

            let result: Result<Board, FENError> = TestStruct::parse_fen(fen);
            assert_eq!(result.unwrap_err(), FENError::NotEnoughBlocks { blocks: 1 });
        }

        #[test]
        #[should_panic(expected = "Unrecognized castling character 0")]
        fn it_contains_invalid_castling_str() {
            let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w 0 1";

            let _ = TestStruct::parse_fen(fen).unwrap();
        }

        #[test]
        #[should_panic(expected = "Invalid Piece character provide x")]
        fn the_fen_contains_an_invalid_piece_char() {
            let fen = "rnbqkbnr/pppppxpp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
            TestStruct::parse_fen(fen).unwrap();
        }

        #[test]
        fn the_ranks_exceeds_8() {
            let fen = "rnbqkbnr/ppppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
            let result = TestStruct::parse_fen(fen);
            assert_eq!(result.unwrap_err(), FENError::MalformedFiles { rank: 7, file: 1 });
        }

        #[test]
        fn the_files_exceed_8() {
            let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR/pppppppp w KQkq - 0 1";
            let result = TestStruct::parse_fen(fen);
            assert_eq!(result.unwrap_err(), FENError::MalformedRanks { rank: 9 });
        }
    }


    #[test]
    fn should_return_a_valid_board() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let board = Board::parse_fen(fen).unwrap();
        assert_eq!(board.castling_rights, Castling::from("KQkq"));
        assert_eq!(board.turn, Color::White);
        assert_eq!(board.enpassant, None);
        assert_eq!(board.hash_key, 0xE0AC430339C6FB3E);
    }

}