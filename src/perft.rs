use std::time::Instant;

use crate::constants::TRICKY_POSITION;
use crate::board::state::board::Board;
use crate::move_logic::move_list::Moves;
use crate::move_scope::MoveScope;

pub(crate) struct Perft;

impl Perft {
    // #[inline(always)]
    pub(crate) fn driver(depth: usize, nodes: &mut usize, board: Board) {
        if depth == 0 {
            *nodes += 1;
            return;
        }

        let mut move_list =Moves::new();
        board.gen_movement::<{ MoveScope::ALL }>(&mut move_list);

        for index in 0..move_list.count_mvs() {
            let bmove = move_list.list[index];
            let legal_move = board.make_move(bmove, MoveScope::AllMoves);
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
        let board = Board::try_from(TRICKY_POSITION).unwrap();
        println!("{}", board.to_string());
        Self::test(depth, &mut nodes, board);


        let elapsed = instant.elapsed();
        println!("\n\n");
        println!("      Depth: {depth}");
        println!("      Nodes: {nodes}");
        println!("      Time: {}ms", elapsed.as_millis());
        println!("      Done!!!");
        
    }

    pub(crate) fn test(depth: usize, nodes: &mut usize, board: Board) {
        let mut move_list = Moves::new();
        board.gen_movement::<{ MoveScope::ALL }>(&mut move_list);

        for index in 0..move_list.count_mvs() {
            let bmove = move_list.list[index];
            let legal_move = board.make_move(bmove, MoveScope::AllMoves);
            if let Some(new_board) = legal_move {
                let cummulative_nodes = *nodes;
                Perft::driver(depth-1, nodes, new_board);
                let old_nodes = *nodes - cummulative_nodes;
                if let Some(p) = bmove.get_promotion() {
                    println!("{}{}{}: {:?},", bmove.get_src(), bmove.get_target(), p, old_nodes);
                } else {
                    println!("{}{}: {:?},", bmove.get_src(), bmove.get_target(), old_nodes);
                }
            }
            // println!("\n\n\n\n");
        }
    }
}