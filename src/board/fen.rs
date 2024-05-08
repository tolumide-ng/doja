use thiserror::Error;
use crate::{board::{castling::Castling, occupancies}, color::Color, constants::PLAYER_PIECES, squares::{Square, SQUARE_NAMES}};

use super::{board_state::BoardState, piece::Piece};


#[derive(Error, Debug)]
pub enum FENError {
    #[error("Not Enough Ranks in FEN string: (expected 8, found {rank}")]
    NotEnoughRanks { rank: u8 },
    #[error("Not Enough Files on Rank {rank}: (expected 8 files, found {file})")]
    NotEnoughFiles {rank: u8, file: u8},
    #[error("At least 4 blocks expected, found only {blocks}")]
    NotEnoughBlocks {blocks: u8},
    #[error("Incomplete Enpassant")]
    IncompletedEnpassant,
    #[error("Enpassant Invalid")]
    EnpassantInvalid,
}

pub trait FEN {
    fn parse_fen(fen: &str) -> Result<BoardState, FENError> {
        let mut board = BoardState::new();

        let fen_blocks = fen.split_whitespace().collect::<Vec<_>>();
        
        if fen_blocks.len() < 4 {
            return Err(FENError::NotEnoughBlocks { blocks: fen_blocks.len() as u8 })
        }

        let total_ranks = fen_blocks[0].split("/").collect::<Vec<_>>().len();
        if total_ranks != 8 {
            return Err(FENError::NotEnoughRanks {rank: total_ranks as u8});
        }

        let pieces = fen_blocks[0].chars().collect::<Vec<char>>();
        let mut square: u64 = 0;



        for char in pieces {
            if char == '/' {
                if square % 8 != 0 {
                    let rank = (square % 8) as u8;
                    let file = (square as u8 - (8 * rank)) - 1;
                    return Err(FENError::NotEnoughFiles { rank: rank-1, file });
                }
                continue;
            }
            
            if char.is_ascii_digit() {
                square += char.to_digit(10).unwrap() as u64;
                continue;
            }


            
            if char.is_ascii_alphabetic() {
                let piece = Piece::from(char);
                board[piece as usize].set_bit(square);
            }
            
            if char.is_ascii_whitespace() {
                break;
            }
            
            square +=1;
        }
        
        println!("for square @ {}", square);

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
        for (white, black) in Piece::white_pieces().into_iter().zip(Piece::black_pieces()) {
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

        
        Ok(board)
    }
}