use std::time::Instant;

use crate::constants::TRICKY_POSITION;
use crate::move_type::MoveType;
use crate::board::board_state::BoardState;
use crate::board::fen::FEN;

pub(crate) struct Perft;

impl Perft {
    // #[inline(always)]
    pub(crate) fn driver(depth: usize, nodes: &mut usize, board: BoardState) {
        if depth == 0 {
            *nodes += 1;
            return;
        }

        let move_list =board.gen_movement();

        for index in 0..move_list.count() {
            let bmove = move_list.list[index];
            let legal_move = board.make_move(bmove, MoveType::AllMoves);
            if let Some(new_board) = legal_move {                
                Perft::driver(depth-1, nodes, new_board);
            }
        }
    }

    // pub(crate) fn cummulative_nodes() {}


    // this is still really slow for a depth of 5 and 6. NEED TO IMPROVE
    pub(crate) fn start(depth: usize) {
        println!("STARTED!!");
        let mut nodes = 0;
        let instant = Instant::now();
        let board = BoardState::parse_fen(TRICKY_POSITION).unwrap();
        println!("{}", board.to_string());
        Self::test(depth, &mut nodes, board);


        let elapsed = instant.elapsed();
        println!("\n\n");
        println!("      Depth: {depth}");
        println!("      Nodes: {nodes}");
        println!("      Time: {}ms", elapsed.as_millis());
        println!("      Done!!!");
        
    }

    pub(crate) fn test(depth: usize, nodes: &mut usize, board: BoardState) {
        let move_list = board.gen_movement();

        for index in 0..move_list.count() {
            let bmove = move_list.list[index];
            let legal_move = board.make_move(bmove, MoveType::AllMoves);
            if let Some(new_board) = legal_move {
                let cummulative_nodes = *nodes;
                Perft::driver(depth-1, nodes, new_board);
                let old_nodes = *nodes - cummulative_nodes;
                if let Some(p) = bmove.get_promotion() {
                    println!("{}{}{}: {:?},", bmove.get_src(), bmove.get_target(), p, old_nodes);
                } else {
                    // println!("SRC---->>>> {}        TARGET----->>>>> {}          [[[[[[CAPTURE]]]]]] {}", bmove.get_src(), bmove.get_target(), bmove.get_capture());
                    // println!("      move: {}{}     nodes: {:?}", bmove.get_src(), bmove.get_target(), old_nodes);
                    println!("{}{}: {:?},", bmove.get_src(), bmove.get_target(), old_nodes);
                }
            }
            // println!("\n\n\n\n");
        }
    }
}