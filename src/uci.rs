use crate::{board::board_state::BoardState, move_type::MoveType};

pub(crate) struct UCI;


impl UCI {
    pub(crate) fn parse(board: &BoardState, mv: String) -> Option<BoardState> {
        let board_moves = board.gen_movement();

        for bmove in board_moves {
            if bmove.to_string().trim() == mv.trim() {
                return board.make_move(bmove, MoveType::AllMoves);
            }
        }

        
        None
    }
}