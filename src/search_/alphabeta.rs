use crate::board::position::Position;
use crate::move_type::MoveType::*;



pub(crate) enum NodeType {
    // i.e PV-nodes: Nodes whose evaluation value lies between the bounds defined by the alpha and beta values.
    Type1Node,
    /// i.e. Cut-nodes: Nodes whose evaluation value result in an alpha or beta cut-off
    Type2Node,
    /// All-nodes: Nodes whose evaluation value does not result in an update of the alpha or beta value, and does not 
    /// result in a alpha or beta cut-off
    Type3Node,
}

fn alphabeta_max(board: &mut Position, depth: usize, alpha: &mut i32, beta: &mut i32) -> i32 {
    if depth == 0 {
        // rteurn evalPosition(board);
    }
    
    let mut score: i32 = 0;

    let mvs = board.gen_movement();
    for mv in mvs {
        board.make_move(mv, AllMoves);
        score = alphabeta_min(board, depth-1, alpha, beta);
        board.undo_move(true);
    }

    if score >= *beta {
        return *beta; // beta cut-off
    }

    if score > *alpha {
        *alpha = score;
    }
    
    return *alpha;
}


fn alphabeta_min(board: &mut Position, depth: usize, alpha: &mut i32, beta: &mut i32) -> i32 {
    if depth == 0 { 
        // return evalPosition(board)
    }

    let mut score = 0;
    let mvs = board.gen_movement();
    for mv in mvs {
        board.make_move(mv, AllMoves);
        score = alphabeta_max(board, depth-1, alpha, beta);
        board.undo_move(true);
    }

    // we only want scores greater than alpha, since this isn't we can go ahead and ignore this node
    if score <= *alpha { return *alpha } // alpha cut-off
    if score < *beta {
        *beta = score;
    }

    *beta
}