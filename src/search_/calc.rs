// common evaluated qualities of a board position are:
// 1. material (i.e the pieces still on the board)
// 2. piece mobility
// 3. king safety
// 4. pawn structure
// 5. ability to castle

use crate::board::{piece::Piece, state::board::Board};

const MATERIAL: [i32; 10] = [100, 320, 330, 500, 900, -100, -320, -330, -500, -900];
const MVV_LVA_ATTACKER: [i32; 6] = [-1, -3, -3, -5, -9, -10]; // p, n, r, b, q, k // Moving Piece
const MVV_LVA_VICTIM: [i32; 5] = [10, 30, 30, 50, 90]; // p, n, r, b, q // Taken Piece

impl Piece {
    // Estimates how favourable a board position is for two players
    pub(crate) fn eval_position(board: &Board) -> i32 {
        let mut score = 0;
        for (index, board) in (*board.board).iter().enumerate() {
            let count = board.count_ones(); // counts the number of this piece on the board
            score += MATERIAL[index] * count as i32;
        }
        score
    }

    // /// MVV-LVA (Most Valuable Victim--Least Valuable Attacker)
    // pub(crate) fn prioritize(&self, victim: Self) -> i32 {
    //     let lva = MVV_LVA_ATTACKER[*self as usize];
    //     let mvv = MVV_LVA_VICTIM[victim as usize];

    // }
}


struct Heuristics;

impl Heuristics {
    pub(crate) fn order_moves() {}
}


// MVV-LVA (Most Valuable Victim -- Least Valuable Aggressor)