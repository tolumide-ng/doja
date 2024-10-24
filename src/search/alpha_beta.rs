// use std::{sync::{Arc, Mutex}, time::Instant};

// use crate::{move_logic::bitmove::Move, board::{piece::Piece, position::Position, state::board::Board}, constants::{DEPTH_REDUCTION_FACTOR, FULL_DEPTH_MOVE, MATE_SCORE, MATE_VALUE, MAX_PLY, REDUCTION_LIMIT, TOTAL_PIECES, TOTAL_SQUARES, VAL_WINDOW, ZOBRIST}, move_scope::MoveScope, syzygy::probe::TableBase, tt::{flag::HashFlag, tpt::TPT}};
// use super::time_control::TimeControl;
// use crate::constants::INFINITY;
// use crate::move_logic::move_list::Moves;


// /// Sometimes you can figure out what kind of node you are dealing with early on. If the first move you search fails high (returns a score greater than or equal to beta).
// /// you've vlearly got a beta node. If the first move fails low(returns a score lesser than or equal to alpha), assuming that your move ordering is pretty good, you
// /// probably have an alpha mode. If the first move returns a score between alpha and beta, you probably have a PV node.
// /// Ofcourse, you could be wrong in two of tyhe case. Once you fail high, you return beta, so you can't make a mistake about that, 
// #[derive(Debug)]
// pub struct NegaMax<'a, T: TimeControl> {
//     nodes: u64,
//     ply: usize,
//     follow_pv: bool,
//     score_pv: bool,
//     controller: Arc<Mutex<T>>,
//     /// Transposition table
//     tt: TPT<'a>,
//     repetition_index: usize,
//     repetition_table: [u64; 500],
    
//     /// MOVE ORDERING HEURISTICS
//     /// It is a path-dependent move ordering technique. It considers moves that caused a beta-cutoff in a sibling node as killer moves and orders them high on the list
//     /// When a node fails high, a the quiet move that caused a cutoff is stored in a table indexed by ply.
//     killer_moves: [[u16; 64]; 2],
//     /// https://www.chessprogramming.org/History_Heuristic
//     history_moves: [[u32; TOTAL_SQUARES]; TOTAL_PIECES], //[[target_sq; 64]; moving_piece];
//     /// The Principal variation (PV) is a sequence of moves that programs consider best and therefore expect to be played. All the nodes included by the PV are PV-nodes
//     /// [Principal Variation](https://www.chessprogramming.org/Principal_Variation)
//     pv_table: [[Move; MAX_PLY]; MAX_PLY],
//     pv_length: [usize; MAX_PLY],
//     name: usize, 
//     dd: u8,
// }


// impl<'a, T> NegaMax<'a, T> where T: TimeControl {
//     pub(crate) fn new(controller: Arc<Mutex<T>>, tt: TPT<'a>, name: usize) -> Self {
//         let x = Self {
//             killer_moves: [[0; 64]; 2], 
//             history_moves: [[0; 64]; 12], 
//             pv_length: [0; 64], 
//             pv_table: [[Move::default(); 64]; 64], 
//             nodes: 0, ply: 0, follow_pv: false, score_pv: false, controller,
//             tt,
//             repetition_index: 0,
//             repetition_table: [0; 500],
//             name,
//             // ss: [SearchE::default(); MAX_DEPTH],
//             dd: 0
//         };

//         x
//     }

//     pub(crate) fn iterative_deepening(&mut self, limit: u8, board: &mut Position, tb: &TableBase) {
//         let mut alpha = -INFINITY;
//         let mut beta = INFINITY;

//         // println!("\n\n\n\n");


//         for depth in 1..=(limit) {
//             self.dd = depth;
//             let start_time = Instant::now();
//             // return 0 if time is up
//             // if self.controller.as_ref().lock().unwrap().stopped() { break; }

//             self.follow_pv = true;
//             // println!("!!!!<<<before>> {depth} prev best {:?}", self.pv_table[0][self.ply].to_string());
//             // for count in 0..self.pv_length[0] as usize {
//             //     print!("[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[-->>>]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]] {}, \n", Move::from(self.pv_table[0][count]))
//             // }
//             // if depth <= 3 {
//             //     println!("3333--3333--3333--3333--3333--3333--3333--3333--3333--3333--3333--3333--3333--3333--3333--3333--3333--3333--3333--3333");
//             // }
//             let score = self.negamax::<true>(alpha, beta, depth, board, tb, None);
//             if (score <= alpha) || (score >= beta) {
//                 // Aspiration window
//                 alpha = -INFINITY; // We fell outside the window, so try again with a
//                 beta = INFINITY; //  full-width window (and the same depth).
//                 continue;
//             }
            
//             alpha = score - VAL_WINDOW; // set up the window for the next iteration
//             beta = score + VAL_WINDOW;

//             if depth == limit {
//                 if score > -MATE_VALUE && score < -MATE_SCORE {
//                     println!("info score mate {} depth {} nodes {} time {}ms pv", (-(score + MATE_VALUE)/2) -1, depth, self.nodes, start_time.elapsed().as_millis());
//                     println!("MATE IN {}", (MATE_VALUE - (score + 1)/2));
//                 } else if (score > MATE_SCORE) && score < MATE_VALUE {
//                     println!("info score mate {} depth {} nodes {} time {}ms pv", ((MATE_VALUE - score)/2) + 1, depth, self.nodes, start_time.elapsed().as_millis());
//                     println!("MATED IN {}", (MATE_VALUE + score)/2);
//                 } else {
//                     println!("info score cp->{} depth===>{} nodes {} time {}ms pv", score, depth, self.nodes, start_time.elapsed().as_millis());
//                 }
    
//                 for count in 0..self.pv_length[0] as usize {
//                     print!("-->>> {}, ", Move::from(self.pv_table[0][count]))
//                 }
    
//                 // println!("");
//                 println!("\n----index {}---------------------- {:#?}ms", self.name, start_time.elapsed().as_millis());
//                 println!("=======------------------- {:#?}s \n", start_time.elapsed().as_secs());
//                 // println!("{:?}", self.pv_table);
//             }
            

//         }

//     }
    
//     // This method is currently VERY SLOW once the depth starts approaching 8, please work to improve it
//     pub(crate) fn run(controller: Arc<Mutex<T>>, tt: TPT<'a>, depth: u8, board: &mut Position, name: usize, tb: &TableBase) {
//         let mut negamax = Self::new(controller, tt, name);
//         negamax.iterative_deepening(depth, board, tb);
//     }

    
//     fn enable_pv_scoring(&mut self, moves: &Moves) {
//         // disable following pv
//         self.follow_pv = false;

//         for mv in moves.into_iter() {
//             // if this move is the best move at that specific ply(self.ply), then enable `score_pv`, and `follow_pv`
//             if self.pv_table[0][self.ply] == mv {
//                 self.score_pv = true;
//                 self.follow_pv = true;
//             }
//         }
//     }


//     /// mv: Move (please remove the mut later, and find a abtter way to write this)
//     /// In Move ordering, killer moves usually come right after the has move, and (good) captures.
//     ///     - In many positions, ther is only a small set of moves creating a threat or defending
//     ///       against it(threats). Those that cannot create, or defend against a threat might(should)
//     ///       be refuted.
//     pub(crate) fn score_move(&mut self, board: &Board, mv: Move, best_move: Option<Move>) -> i32 {
//         if let Some(b_mv) = best_move { if b_mv == mv { return 50_000 }}

//         if self.score_pv {
//             if self.pv_table[0][self.ply] == mv {
//                 // println!("(((((((((((((((((((((((((((((THE PREV IS NOW?????))))))))))))))))))))))))))))) {:?}", mv.to_string());
//                 self.score_pv = false;
//                 return 20_000;
//             }
//         }
//         match mv.get_capture() {
//             true => {
//                 let src = board.get_piece_at(mv.get_src(), board.turn).unwrap();
//                 let tgt = board.get_move_capture(mv).unwrap();
//                 let ll = src.get_mvv_lva(&tgt) + 10_000;
                
//                 return ll
//             }
//             false => {
//                 // if mv.get_promotion().is_some() {
//                 //     return 100_000
//                 // }
//                 if *mv == self.killer_moves[0][self.ply] { return 9_000 }
//                 if *mv == self.killer_moves[1][self.ply] { return 8_000 }

//                 let piece = board.get_piece_at(mv.get_src(), board.turn).unwrap();
//                 return self.history_moves[piece][mv.get_target()] as i32;
//             }
//         }
//         // 0
//     }

//     /// todo! add target on the Move, so that this cmp method can be implenented directly on Moves(MvList), that way
//     /// we wouldn't need this one anymore
//     pub(crate) fn sort_moves(&mut self, board: &Board, mv_list: Moves, best_move: Option<Move>, depth: u8) -> Vec<Move> {
//         // println!("------------------------------------------------------------------------- {depth}");
//         let mut sorted_moves: Vec<Move> = Vec::with_capacity(mv_list.count_mvs());
//         // if depth == 2 {
//         //     if (board.get_occupancy(Color::White) & 1 << Square::E2 as u64) == 0 && (board.get_occupancy(Color::Black) & 1 << Square::A6 as u64) == 0  {
//         //         if (board.get_occupancy(Color::Black) & 1 << Square::B4 as u64) == 0 && (board.get_occupancy(Color::White) & 1 << Square::C3 as u64) == 0  {
//         //             println!("provided best mv is {:?}", best_move);
//         //             println!("{}", board);
//         //             // let mut ap = Vec::new();
//         //             for x in mv_list {
//         //                 print!("{}, ", x.to_string());
//         //             }
//         //             println!("\ndone ======================================================>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>\n\n");
//         //         }
//         //     }
//         // }
//         sorted_moves.extend_from_slice(&mv_list.list[..mv_list.count_mvs()]);
//         let mut w_scores = mv_list.into_iter().map(|m| (self.score_move(board, m, best_move), m)).collect::<Vec<_>>();
//         w_scores.sort_by(|a, b| b.0.cmp(&a.0));

//         sorted_moves.sort_by(|a, b| self.score_move(board, *b, best_move).cmp(&self.score_move(board, *a, best_move)));
//         // println!("vcalled {depth}");
//         // if depth == 2 {
//         //     if (board.get_occupancy(Color::White) & 1 << Square::E2 as u64) == 0 && (board.get_occupancy(Color::Black) & 1 << Square::A6 as u64) == 0  {
//         //         if (board.get_occupancy(Color::Black) & 1 << Square::B4 as u64) == 0 && (board.get_occupancy(Color::White) & 1 << Square::C3 as u64) == 0  {
//         //             println!("the sorted mvs now is");
//         //             for (s, x) in w_scores {
//         //                 print!("{s}={}, ", x.to_string());
//         //             }
        
//         //             println!("\n\n\n\n\n")
//         //         }
//         //     }
//         // }
//         return sorted_moves
//     }




//   /// https://www.chessprogramming.org/Quiescence_Search
//     fn quiescence(&mut self, mut alpha: i32, beta: i32, mut board: &mut Position, xm: Option<Move>) -> i32 {
//         // this action will be performed every 2048 nodes
//         // if (self.nodes & NODES_2047) == 0 { self.controller.as_ref().lock().unwrap().communicate(); }
//         self.nodes+=1;
        
//         let eval = board.evaluate();

//         // if let Some(parent_m) = xm{
//         //     if parent_m.to_string() == String::from("e6d5x") {
//         //         println!("<><><><> AT THIS alpha {alpha} beta={beta}, and eval={eval}");
//         //     }
//         // }
//         // let eval = Evaluation::evaluate(board);

//         // if let Some(the_m) = xm {
//         //     if the_m.to_string() == String::from("g7f6x") {
//         //         println!("the result of this evaluation is >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> eval={eval}, alpha={alpha}, and beta={beta}");
//         //     }
//         // }        
//         if self.ply > MAX_PLY - 1 {
//             return eval;
//         }
        

//         if eval >= beta { return beta; }
//         if eval > alpha { alpha = eval;}

//         // if self.ply > 0 && (self.is_repetition(board) || board.fifty.iter().any(|&p| p >= 50)) {
//         //     return 0 // draw
//         // }

//         // let king_square = u64::from(board[Piece::king(board.turn)].trailing_zeros());
//         // let king_in_check = board.is_square_attacked(king_square, !board.turn);

//         // let mut best_move: Option<Move> = None;

//         // let tt_data = self.tt.probe(board.hash_key);
//         // if let Some(data) =  tt_data {
//         //     let score = data.score(self.ply);

//         //     match data.flag {
//         //         HashFlag::Exact => return score,
//         //         HashFlag::LowerBound if score >= beta => return beta,
//         //         HashFlag::UpperBound if score <= alpha => return alpha, // Based on the transposition table, this path doesn't get any better
//         //         _ => best_move = data.mv,
//         //     }
//         // }


//         // let stand_pat = board.evaluate();
//         // let eval = if !king_in_check {
//         //     if let Some(data) = tt_data {
//         //         let score = data.score(self.ply);
//         //         self.ss[self.ply].eval = if data.eval() == -INFINITY {stand_pat} else {data.eval()};

//         //         match data.flag {
//         //             HashFlag::Exact => return score,
//         //             HashFlag::LowerBound if score > self.ss[self.ply].eval => return beta,
//         //             HashFlag::UpperBound if score < self.ss[self.ply].eval => return alpha, 
//         //             _ => self.ss[self.ply].eval,
//         //         }
//         //     } else {
//         //         self.ss[self.ply].eval = stand_pat;
//         //         stand_pat
//         //     }
//         // } else {
//         //     self.ss[self.ply].eval = -INFINITY;
//         //     -INFINITY
//         // };

//         // let prev_alpha = alpha;
//         // // standing pat
//         // // alpha = std::cmp::max(alpha, eval);
//         // // if eval >= beta { return beta; }


//         // if stand_pat >= beta { return beta; } // node (move) fails high
//         // if stand_pat > alpha { alpha = stand_pat; } // found a better score

//         // let mut allow_quiet_moves = false;

//         // // If the king is in check, allow all possible searches to get him out of there
//         // if king_in_check { allow_quiet_moves = true; }

//         // let mut best_score = stand_pat;
//         // let mut moves_made = 0;


//         let sorted_moves = self.sort_moves(board, board.gen_movement::<{ MoveScope::ALL }>().into_iter(), None, 0);
//         // if let Some(mk) = xm {
//         //     println!("HERE--HERE_----------------------------------- {}", mk);
//         //     if mk.to_string() == String::from("e6d5x") {println!("HERE--HERE")}}
//         // until every capture has been examined
//         for mv in sorted_moves {
//             // if self.controller.as_ref().lock().unwrap().stopped() { return 0}

//             // if !allow_quiet_moves && mv.get_capture() == false { continue; }

//             // if let Some(mk) = xm {
//             //     if [String::from("f3f6x"), String::from("d2c3x")].contains(&mk.to_string()) {
//             //         if [String::from("g7f6x")].contains(&mv.to_string()) {
//             //         println!("|||||||||||||||||||||||||||||||||r4sult   alpha-->{alpha}, beta is -->>{beta}, xm is {}, and mv is {}||||", mk.to_string(), mv.to_string());
//             //             // if (board.get_occupancy(Color::White) & 1 << Square::E2 as u64) == 0 && (board.get_occupancy(Color::Black) & 1 << Square::A6 as u64) == 0  {
//             //             //     if (board.get_occupancy(Color::Black) & 1 << Square::B4 as u64) == 0 && (board.get_occupancy(Color::White) & 1 << Square::C3 as u64) == 0  {
//             //             //         if (board.get_occupancy(Color::White) & 1 << Square::D2 as u64) == 0 && (board.get_occupancy(Color::White) & 1 << Square::C3 as u64) == 0  {
//             //             //             if (board.get_occupancy(Color::Black) & 1 << Square::E6 as u64) == 0 && (board.get_occupancy(Color::White) & 1 << Square::D5 as u64) == 0  {
//             //             //                 println!("\n\n**************************************************************************************************{}", board.hash_key);
//             //             //                 // println!("&&&&&&&&&&&&&&&&&&&&&&$$$$$$$$$$$$$$$$$$$&&&&&&&&&&&&&&&&&&&&&&$$$$$$$$$$$$$$$$$$$&&&&&&&&&&&&&&&&&&&&&&$$$$$$$$$$$$$$$$$$$&&&&&&&&&&&&&&&&&&&&&&$$$$$$$$$$$$$$$$$$$");
//             //             //                 // println!("{}", board.to_string());
//             //             //             }
//             //             //         }
//             //             //     }
//             //             // }
//             //         }
//             //     }
//             // }

//             if board.make_move_nnue(mv, MoveScope::CapturesOnly) {
//                 // moves_made += 1;
//                 self.ply += 1;
//                 // self.repetition_index += 1;
//                 // self.repetition_table[self.repetition_index] = board.hash_key;

                
//                 let score = -self.quiescence(-beta, -alpha, &mut board, Some(mv));
                
//                 self.ply -=1;
//                 // self.repetition_index-=1;
//                 board.undo_move(true);

//                 // if let Some(mk) = xm {
//                 //     if [String::from("f3f6x"), String::from("d2c3x")].contains(&mk.to_string()) {
//                 //         println!("r4sult for xxx is score-->>{score} alpha-->{alpha}, beta is -->>{beta}, mv  {}||||", mv.to_string());
//                 //         if [String::from("g7f6x")].contains(&mv.to_string()) {
//                 //             // if (board.get_occupancy(Color::White) & 1 << Square::E2 as u64) == 0 && (board.get_occupancy(Color::Black) & 1 << Square::A6 as u64) == 0  {
//                 //             //     if (board.get_occupancy(Color::Black) & 1 << Square::B4 as u64) == 0 && (board.get_occupancy(Color::White) & 1 << Square::C3 as u64) == 0  {
//                 //             //         if (board.get_occupancy(Color::White) & 1 << Square::D2 as u64) == 0 && (board.get_occupancy(Color::White) & 1 << Square::C3 as u64) == 0  {
//                 //             //             if (board.get_occupancy(Color::Black) & 1 << Square::E6 as u64) == 0 && (board.get_occupancy(Color::White) & 1 << Square::D5 as u64) == 0  {
//                 //             //                 println!("\n\n**************************************************************************************************{}", board.hash_key);
//                 //             //                 // println!("&&&&&&&&&&&&&&&&&&&&&&$$$$$$$$$$$$$$$$$$$&&&&&&&&&&&&&&&&&&&&&&$$$$$$$$$$$$$$$$$$$&&&&&&&&&&&&&&&&&&&&&&$$$$$$$$$$$$$$$$$$$&&&&&&&&&&&&&&&&&&&&&&$$$$$$$$$$$$$$$$$$$");
//                 //             //                 // println!("{}", board.to_string());
//                 //             //             }
//                 //             //         }
//                 //             //     }
//                 //             // }

//                 //         }
//                 //     }
//                 // }

//                 if score > alpha {
//                     alpha = score;

//                     if score >= beta {
//                         return beta;
//                     }
//                 }
//                 // alpha = score;
//                 // if score > alpha { best_move = Some(mv);  alpha = score; }
//                 // if score >= beta { alpha = beta; break; }
//             }
//         }

//         // Copy of Viridthias' approach
//         // if king_in_check && moves_made == 0 { return -5000 }
        
//         // return 0 if time is up
//         // if !self.controller.as_ref().lock().unwrap().stopped() { 
//         //     let flag = if best_score >= beta {HashFlag::LowerBound} else if best_score > prev_alpha {HashFlag::Exact} else {HashFlag::UpperBound};
//         //     self.tt.record(board.hash_key, 0, alpha, eval, self.ply, flag, 0, best_move);
//         // }

//         return alpha
//     }

//     fn is_repetition(&self, board: &Board) -> bool {
//         for i in 0..self.repetition_index {
//             if self.repetition_table[i] == board.hash_key {
//                 return true
//             }
//         }
//         return false;
//     }

//     /// nmfp: Null Move forward prunning
//     /// https://web.archive.org/web/20040427014629/http://brucemo.com/compchess/programming/nullmove.htm
//     /// "If I do nothing here, can the opponent do anything?"
//     /// Returns the score, only if the score is greater than beta.
//     /// This means that even if we "skip" our play, and allow the opponent to play (instead of us),
//     /// They still won't be better off than they were before we skipped our play
//     fn make_null_move(&mut self, beta: i32, depth: u8, mut nmfp_board: &mut Position, tb: &TableBase) -> Option<i32> {
//             // nmfp: null-move forward prunning (board)
//             self.ply += 1;
//             self.repetition_index+=1;
//             self.repetition_table[self.repetition_index] = nmfp_board.hash_key;

//             // update the zobrist hash accordingly, since this mutating actions do not direcly update the zobrist hash
//             let old_hashkey = nmfp_board.hash_key;
//             let old_enpassant = nmfp_board.enpassant;

//             if let Some(enpass_sq) = nmfp_board.enpassant {
//                 // we know that we're going to remove the enpass if it's available (see 4 lines below), so we remove it from the hashkey if it exists here
//                 nmfp_board.set_zobrist(nmfp_board.hash_key ^ ZOBRIST.enpassant_keys[enpass_sq]);
//             }
//             nmfp_board.set_turn(!nmfp_board.turn);
//             nmfp_board.set_enpassant(None);
//             nmfp_board.set_zobrist(nmfp_board.hash_key ^ ZOBRIST.side_key); // side about to move
//             nmfp_board.nnue_push();
            
//             let score = -self.negamax::<false>(-beta, -beta+1, depth-1-DEPTH_REDUCTION_FACTOR, &mut nmfp_board, tb, None);

//             self.ply -= 1;
//             self.repetition_index-=1;
//             nmfp_board.nnue_pop();
            
//             // recent change for undo-move in order to avoid cloning the board
//             nmfp_board.set_turn(!nmfp_board.turn);
//             nmfp_board.set_enpassant(old_enpassant);
//             nmfp_board.set_zobrist(old_hashkey);
//             // return 0 if time is up
//             // if self.controller.as_ref().lock().unwrap().stopped() { return None}

//             if score >= beta {
//                 return Some(beta)
//             }
            
//             return None;
//     }


    
//     /// https://www.chessprogramming.org/Alpha-Beta#Negamax_Framework
//     fn negamax<const ROOT: bool>(&mut self, mut alpha: i32, beta: i32, depth: u8, mut board: &mut Position, tb: &TableBase, mmm: Option<Move>) -> i32 {

//         self.pv_length[self.ply] = self.ply;
//         let mut hash_flag = HashFlag::UpperBound; // alpha
//         if (self.ply > 0 && self.is_repetition(board)) || board.fifty.iter().any(|&p| p >= 50) {
//             return 0 // draw
//         }

        
//         let pv_node = (beta - alpha) > 1; // is the current move a PV node ((Pedro Castro) -> CodeMK)
        
//         let mut score = 0;
//         let mut best_move: Option<Move> = None;
        
//         let mut probe_tt = || -> Option<i32> {
//             if let Some(entry) =  self.tt.probe(board.hash_key) {
//                 if entry.depth >= depth {
//                     best_move = entry.mv;
//                     let entry_score = entry.score(self.ply);
//                     match entry.flag {
//                         HashFlag::Exact => return Some(entry_score),
//                         HashFlag::LowerBound if entry_score >= beta => return Some(beta),
//                         HashFlag::UpperBound if entry_score <= alpha =>  return Some(alpha),
//                         _ => { return None }
//                     }
//                 }
//             }
//             None
//         };

//         // if we had cached the score for this move before, we return it, and confirm that the current node is not a PV node(principal variation)
//         let tt_score = probe_tt();

//         if self.ply > 0 && tt_score.is_some() && !pv_node { return tt_score.unwrap() }
        
        
        
//         if depth == 0 {
//             let rss = self.quiescence(alpha, beta, board, mmm);
//             // if let Some(parent_m) = mmm {
//             //     if parent_m.to_string() == String::from("e6d5x") && depth == 0 {
//             //         println!("<><><><> AT THIS DEPTH OF {depth} probing -->> {:?}", rss);
//             //     }
//             // }
//             // if let Some(mk) = mmm {
//             //     if mk.to_string() == String::from("e6d5x") {
//             //         if (board.get_occupancy(Color::White) & 1 << Square::E2 as u64) == 0 && (board.get_occupancy(Color::Black) & 1 << Square::A6 as u64) == 0  {
//             //             if (board.get_occupancy(Color::Black) & 1 << Square::B4 as u64) == 0 && (board.get_occupancy(Color::White) & 1 << Square::C3 as u64) == 0  {
//             //                 if (board.get_occupancy(Color::White) & 1 << Square::D2 as u64) == 0 && (board.get_occupancy(Color::White) & 1 << Square::C3 as u64) == 0  {
//             //                     if (board.get_occupancy(Color::Black) & 1 << Square::E6 as u64) == 0 && (board.get_occupancy(Color::White) & 1 << Square::D5 as u64) == 0  {
//             //                         println!("\ndone ======================================================>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>{}", board.hash_key);
//             //                         // println!("&&&&&&&&&&&&&&&&&&&&&&$$$$$$$$$$$$$$$$$$$&&&&&&&&&&&&&&&&&&&&&&$$$$$$$$$$$$$$$$$$$&&&&&&&&&&&&&&&&&&&&&&$$$$$$$$$$$$$$$$$$$&&&&&&&&&&&&&&&&&&&&&&$$$$$$$$$$$$$$$$$$$");
//             //                         // println!("{}", board.to_string());
//             //                         println!("r4sult for e6d5 is {rss} alpha-->{alpha}, beta is -->>{beta}, depth----<<<<{depth}, mmm is {}", mk.to_string());
//             //                     }
//             //                 }
//             //             }
//             //         }
//             //     }
//             // }
//             return rss;
//         }

//         if self.ply > MAX_PLY -1 {
//             return board.evaluate();
//         }

//         self.nodes+=1;
        
//         let king_square = u64::from(board[Piece::king(board.turn)].trailing_zeros());
//         // is king in check
//         let king_in_check = board.is_square_attacked(king_square, !board.turn);
//         let depth = if king_in_check {depth +1} else {depth};
//         let mut legal_moves = 0;
        
//         // evaluation pruning / static null-move pruning
//         // if depth < 3 && !pv_node && !king_in_check && (beta-1).abs() > -INFINITY + 100 {
//         //     let eval = board.evaluate();
//         //     let eval_margin = 120 * depth as i32;
//         //     if eval - eval_margin >= beta {
//         //         return eval - eval_margin
//         //     }
//         // }
        
//         // Null-Move Forward Pruning
//         // Null-move forward pruning is a step you perform prior to searching any of the moves.  You ask the question, "If I do nothing here, can the opponent do anything?"
//         // In order to test this, we allow the opponent play this turn(even though its ours), if they play and we're not in harms way (greater than beat), then we're good.
//         // -- "Null-move forward pruning is not used, at least in endgames.  If you do try to use it in endgames, very bad things will happen very often."
//         let null_move_forward_pruning_conditions = depth >= (DEPTH_REDUCTION_FACTOR + 1) && !king_in_check && self.ply> 0;
//         // added 1 to the depth_reduction factor to be sure, there is atleast one more depth that would be checked
        
//         if null_move_forward_pruning_conditions {
//             if let Some(beta) = self.make_null_move(beta, depth, &mut board, tb) {
//                 return beta;
//             };
//         }

        
//         let moves = board.gen_movement::<{ MoveScope::ALL }>().into_iter();
//         if self.follow_pv {
//             self.enable_pv_scoring(&moves);
//         }

//         // println!("\n\n");
//         // if depth > 0 {
//         //     // for x in moves.into_iter() {
//         //     //     print!("--||{:?}||--", x.to_string());
//         //     // }
//         //     println!("****************************************************************************************************** {depth}")
//         // }
        
//         // for mv in moves 
//         let sorted_moves = self.sort_moves(board, moves, best_move, depth);

//         // println!("\n\n");
//         // if depth <= 3 {
//         //     for x in sorted_moves.iter() {
//         //         print!("--||{:?}||--", x.to_string());
//         //     }
//         // }
        
//         // https://www.chessprogramming.org/Principal_Variation_Search#Pseudo_Code
//         let mut moves_searched = 0;

//         let sl = sorted_moves.len();

//         // if depth == 1 {
//         //     if let Some(mmmm) = mmm {
//         //         if mmmm.to_string() == String::from("d2c3x") {
//         //             println!("===[[[[[[[[[[[[[d2c3x]]]]]]]]]]]]]=== {}", sl);
//         //             for mm in &sorted_moves {
//         //                 print!("{}, ", mm.to_string());
//         //             }
//         //         }
//         //     }
//         // }

//         let mut best_move: Option<Move> = None;
        
//         // loop through hte moves
//         for (count, mv) in sorted_moves.iter().enumerate() {
//             let mv = *mv;
//             let legal_move = board.make_move_nnue(mv, MoveScope::AllMoves);
            
//             if !legal_move { continue; }
            
//             // if depth == 1 {
//             //     if let Some(mmmm) = mmm {
//             //         if mmmm.to_string() == String::from("f3f6x") {
//             //             println!("**f3f6** {} ll={}>> {count}", mv.to_string(), sl);
//             //         }
//             //         if mmmm.to_string() == String::from("d2c3x") {
//             //             println!("\n ===d2c3x=== {} ll={} moves_searched-->> {moves_searched}, count={count}", mv.to_string(), sl);
//             //         }
//             //     }
//             // }
//             self.ply +=1;
//             self.repetition_index+=1;
//             self.repetition_table[self.repetition_index] = board.hash_key;
//             legal_moves += 1;


//             if depth == 1 && self.dd == 4 {
//                 // if let Some(mmmm) = mmm {
//                 // //     if mmmm.to_string() == String::from("f3f6x") {
//                 // //         println!("<<**f3f6**>> score={score}, depth={depth}, alpha={alpha}, beta={beta} mv={}", mv.to_string())
//                 // //     }
//                 //     // if mmmm.to_string() == String::from("d2c3x") {
//                 //     //     println!("((((((((((((({}))))))))))))) moves_searched={moves_searched} score={score}, depth={depth}, alpha={alpha}, beta={beta} mv={}", mmmm.to_string(), mv.to_string())
//                 //     // }
//                 // }
//             }




//             // https://www.chessprogramming.org/Principal_Variation_Search#Pseudo_Code
//             let score = match moves_searched {
//                 0 => {
//                     // full depth search
//                     let xxx = -self.negamax::<false>(-beta, -alpha, depth-1, &mut board, tb, Some(mv));
                    
//                     // if depth == 1 {
//                     //     if let Some(mmmm) = mmm {
//                     //         if mmmm.to_string() == String::from("d2c3x") {
//                     //             println!("---------->>>>>d2c3x<<<<<-------- score==>{xxx}, alpha->{alpha}, beta->{beta}");
//                     //         }}}

//                     xxx
//                 },
//                 _ => {
//                     // https://web.archive.org/web/20150212051846/http://www.glaurungchess.com/lmr.html
//                     // condition for Late Move Reduction
//                     let not_tactical_mv = !king_in_check && mv.get_promotion().is_none() && !mv.get_capture();


//                     let mut value =  if (moves_searched >= FULL_DEPTH_MOVE) && (depth >= REDUCTION_LIMIT) && not_tactical_mv {
//                         -self.negamax::<false>(-alpha+1, -alpha, depth-2, &mut board, tb, Some(mv))
//                     } else {
//                         alpha +1 // Hack to ensure that full-depth search is done
//                     };

//                     if value > alpha {
//                         value = -self.negamax::<false>(-alpha+1, -alpha, depth-1, &mut board, tb, Some(mv));
//                         if (value > alpha) && (value < beta) {
//                             value = -self.negamax::<false>(-beta, -alpha, depth-1, &mut board, tb, Some(mv));
//                         }
//                     }
//                     value
//                 }
//             };

//             // if depth == 1 && self.dd == 4 {
//             //     if let Some(mmmm) = mmm {
//             //     //     if mmmm.to_string() == String::from("f3f6x") {
//             //     //         println!("<<**f3f6**>> score={score}, depth={depth}, alpha={alpha}, beta={beta} mv={}", mv.to_string())
//             //     //     }
//             //         if mmmm.to_string() == String::from("d2c3x") {
//             //             println!("<<==={}===>> moves_searched={moves_searched} score={score}, depth={depth}, alpha={alpha}, beta={beta} mv={}", mmmm.to_string(), mv.to_string())
//             //         }
//             //     }
//             // }


//             // if mv.to_string() == String::from("f3f6x") {
//             //     println!("f3f6x @ depth={depth} >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> {:?}", score);
//             // }

//             // if mv.to_string() == String::from("d2c3x") {
//             //     println!("d2c3x @depth={depth} !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!! {:?}", score);
//             // }

            
//             board.undo_move(true);
            
            
            
//             self.ply -=1;
//             self.repetition_index-=1;
//             moves_searched += 1;
//             // return 0 if time is up
//             // if self.controller.as_ref().lock().unwrap().stopped() { return 0}
            
            
//             // if [String::from("d2c5"), String::from("d2c3x")].contains(&mv.to_string()) && (depth <= 3 || depth == 2) {
//             //     println!(">>>>>>>>>>>>>>>>>>>>>> depth==>>{depth}, score==>>{score}, mvstr==>>{}, alpha**==>>{alpha}, beta ==>>{beta}", mv.to_string());
//             // }

//             // if depth <= 3 {
//             //     if (board.get_occupancy(Color::White) & 1 << Square::E2 as u64) == 0 && (board.get_occupancy(Color::Black) & 1 << Square::A6 as u64) == 0  {
//             //         if (board.get_occupancy(Color::Black) & 1 << Square::B4 as u64) == 0 && (board.get_occupancy(Color::White) & 1 << Square::C3 as u64) == 0  {
//             //             println!(">>>>>>>>>>>>>>> {}--> {}", mv.to_string(), score);
//             //         }
//             //     }
//             // }

//             // if depth == 1 {
//             //     if let Some(mmmm) = mmm {
//             //         if mmmm.to_string() == String::from("d2c3x") {
//             //             println!("---------->>>>>d2c3x<<<<<-------- score==>{score}, alpha->{alpha}, beta->{beta}");
//             //         }}}

            
//             if score > alpha {
//                 hash_flag = HashFlag::Exact;
//                 best_move = Some(mv);

//                 if !mv.get_capture() {
//                     // store history moves
//                     unsafe {
//                         *((*(self.history_moves.as_mut_ptr().add(board.piece_at(mv.get_src()).unwrap() as usize))).as_mut_ptr().add(mv.get_target() as usize)) += depth as u32; 
//                     }
//                 }

//                 alpha = score;

//                 // write PV move
//                 // Traingular PV-Table
//                 self.pv_table[self.ply][self.ply] =  mv;
//                 // if [2, 3].contains(&depth) {
//                 //     println!("winner is >>>>>>> depth-->>|| {}, and the mv -->> {}, with score of {}", depth, mv.to_string(), score);
//                 // }

//                for j in (self.ply+1)..self.pv_length[self.ply+1] {
//                     // copy move from deeper ply into current ply's line
//                     self.pv_table[self.ply][j] = self.pv_table[self.ply+1][j];
//                 }
//                 self.pv_length[self.ply] = self.pv_length[self.ply + 1];

//                 if score >= beta {
//                     // if [String::from("d2c5"), String::from("d2c3x")].contains(&mv.to_string()) && (depth <= 3 || depth == 2) {
//                     //     println!("GREATER THAN BETA??? depth==>>{depth}, score==>>{score}, mvstr==>>{}", mv.to_string());
//                     // }
//                     self.tt.record(board.hash_key, depth, beta, INFINITY, self.ply, HashFlag::LowerBound, 0, best_move);
//                     if !mv.get_capture() { // quiet move (non-capturing quiet move that beats the opponent)
//                         self.killer_moves[1][self.ply] = self.killer_moves[0][self.ply];
//                         self.killer_moves[0][self.ply] = (*mv).into();
//                     }
//                     // node/move fails high
//                     return beta
//                 }
//             }

//             // fail-hard beta cutoff
//             // if score >= beta {
//             //     // if [String::from("d2c5"), String::from("d2c3x")].contains(&mv.to_string()) && (depth <= 3 || depth == 2) {
//             //     //     println!("GREATER THAN BETA??? depth==>>{depth}, score==>>{score}, mvstr==>>{}", mv.to_string());
//             //     // }
//             //     self.tt.record(board.hash_key, depth, beta, INFINITY, self.ply, HashFlag::LowerBound, 0, Some(mv));
//             //     if !mv.get_capture() { // quiet move (non-capturing quiet move that beats the opponent)
//             //         self.killer_moves[1][self.ply] = self.killer_moves[0][self.ply];
//             //         self.killer_moves[0][self.ply] = (*mv).into();
//             //     }
//             //     // node/move fails high
//             //     return beta
//             // }

//             // if depth == 1 {
//             //     if let Some(mmmm) = mmm {
//             //         println!("depth>>>{depth}, parent :::>>{}<< the move {}, score==> {}, alpha={}, beta={}", mmmm, mv.to_string(), score, alpha, beta);
//             //     } else {
//             //         println!("depth>>>{depth}, the move {}, score==> {}, alpha={}, beta={}", mv.to_string(), score, alpha, beta);
//             //     }
//             // }
            
//             // // best score so far
//             // if score > alpha {
//             //     hash_flag = HashFlag::Exact;
                
//             //     // if !mv.get_capture() {
//             //     //     // store history moves
//             //     //     unsafe {
//             //     //         *((*(self.history_moves.as_mut_ptr().add(board.piece_at(mv.get_src()).unwrap() as usize))).as_mut_ptr().add(mv.get_target() as usize)) += depth as u32; 
//             //     //     }
//             //     // }
//             //     alpha = score; // PV move (position)


//             //     // write PV move
//             //     // Traingular PV-Table
//             //     self.pv_table[self.ply][self.ply] =  mv;
//             //     // if [2, 3].contains(&depth) {
//             //     //     println!("winner is >>>>>>> depth-->>|| {}, and the mv -->> {}, with score of {}", depth, mv.to_string(), score);
//             //     // }

//             //    for j in (self.ply+1)..self.pv_length[self.ply+1] {
//             //         // copy move from deeper ply into current ply's line
//             //         self.pv_table[self.ply][j] = self.pv_table[self.ply+1][j];
//             //     }
//             //     self.pv_length[self.ply] = self.pv_length[self.ply + 1];

//             // }
//         }

//         // println!("\n\n");


//         // we don't have any legal moves to make in the current position
//         if legal_moves == 0 {
//             // is king in check
//             if king_in_check {
//                 return -MATE_VALUE + (self.ply) as i32;
//             }
//             // king is not in check and there are not legal moves
//             return 0 // stalemate | draw
//         }

//         self.tt.record(board.hash_key, depth, alpha, INFINITY, self.ply, hash_flag, 0, best_move);
//         return alpha
//     }
// }