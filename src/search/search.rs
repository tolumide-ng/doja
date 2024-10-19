use crate::{bit_move::{Move, MoveType}, bitboard::Bitboard, board::{piece::Piece, position::{self, Position}, state::board::Board}, color::Color, constants::{params::MAX_DEPTH, DEPTH_REDUCTION_FACTOR, FULL_DEPTH_MOVE, INFINITY, MATE_VALUE, MAX_PLY, MVV_LVA, PIECE_ATTACKS, PLAYERS_COUNT, REDUCTION_LIMIT, TOTAL_SQUARES, VAL_WINDOW, ZOBRIST}, move_scope::MoveScope, moves::Moves, squares::Square, tt::{flag::HashFlag, tpt::TPT}};
use crate::board::piece::Piece::*;
use crate::color::Color::*;
use crate::search::heuristics::pv_table::PVTable;

use super::heuristics::{history::HistoryHeuristic, killer_moves::KillerMoves};

/// The number of nodes you can actually cut depends on:
/// 1. How well written your alpha-beta program is
/// 2. How well ordered your game-tree is (i.e the next moves on the board) --- GOOD MOVE ORDERING IS IMPORTANT
///     If you always put the best possible move first, you elimiate the most nodes.
/// 
/// GOALS:
///  1. [-] AlphaBeta 
///  2. [-] Quiescence Search
///         a. [x] Standing Pat
///         b. [-] Delta Pruning
///             i. https://www.chessprogramming.org/Delta_Pruning
///             ii. https://www.chessprogramming.org/CPW-Engine_quiescence (check how CPW implements its beta pruning)
///  3. [x] Late Move Reduction
///  4. [x] MVV_LVA - (Most Viable Victim -- Least Viable Attacker)
///  5. [x] Move Ordering
///  6. [x] Transposition Table
///  7. [x] Null move forward Prunning
///  8. [x] Principal Variation Node
///  9. [x] Killer Moves
/// 10. [x] History Moves 
/// 11. [x] Aspiration Window
/// 12. [x] Iterative Deepening
/// 13. [x] PV-Table
/// 14. [x] Repetitions https://www.chessprogramming.org/Repetitions
/// This implementation is a fail-soft implementation (meaning we have to keep track of the best score)[XX] 
/// - fail-hard for now
pub(crate) struct Search<'a> {
    nodes:  usize,
    ply: usize,
    pv_table: PVTable,
    /// The Killer Move is a quiet move which caused a beta-cutoff in a sibling Cut-node,
    killer_moves: KillerMoves,
    /// Previosyly successful moves in a particular positioin that resulted in a beta-cutoff
    history_table: HistoryHeuristic,
    tt: TPT<'a>,
    limit: u8
}


impl<'a> Search<'a> {
    pub(crate) fn new(tt: TPT<'a>) -> Self {
        Self { nodes: 0, ply: 0, pv_table: PVTable::new(), killer_moves: KillerMoves::new(),
            history_table: HistoryHeuristic::new(), tt, limit: 0 }
    }

    pub(crate) fn iterative_deepening(&mut self, limit: u8, position: &mut Position) {
        let mut alpha = -INFINITY;
        let mut beta = INFINITY;

        
        for depth in 1..=limit {
            self.limit = depth;
            println!("RUNNING ::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::<<>>::::::::::: {depth}");
            println!("{}", position.to_string());
            self.ply = 0;
            let score = self.alpha_beta(alpha, beta, depth, position);
            println!("the score is {score} and nodes {}", self.nodes);
            if score <= alpha || score >= beta { // aspiration window
                println!("(((((((((((((((((((((((((((should not be here))))))))))))))))))))))))))) score={score}, alpha={alpha}, and beta={beta}");
                // We fell outside the window, so try agin with a full-width window (and the same depth)
                alpha = -INFINITY; beta = INFINITY;
                // for i in self.pv_table.get_pv(0) {
                //     print!("-->> {}", Move::from(*i));
                // }
                // println!("\n");
                continue;
            }

            alpha = score - VAL_WINDOW;
            beta = score + VAL_WINDOW;

            println!("MOVES ARE :::: with length of {}", self.pv_table.len(0));
            for i in self.pv_table.get_pv(0) {
                print!("-->> {}", Move::from(*i));
            }
            println!("\n")
        }
    }


    fn get_lva(attackers: u64, board: &Board, stm: Color) -> Option<(Piece, Square)> {
        println!("::::::::::::::::------------------- {:?}", stm);
        println!("SIDE TO MOVE {:?}", stm);
        
        let range = if stm == White {0..6} else {6..12};
        for piece in range {
            let bits = *board[piece] & attackers;
            if bits != 0 {
                return Some((Piece::from(piece as u8), Square::from(bits.trailing_zeros() as u64)))
            }
        }

        None
    }

    /// https://www.chessprogramming.net/static-exchange-evaluation-in-chess/
    pub(crate) fn static_exchange_eval(position: &Position, mv: &Move, threshold: i32) -> bool {
        println!("STATIC EXCHANGE EVAL");
        let src = mv.get_src();
        let tgt = mv.get_target();
        let mt = mv.move_type();

        // Castling cannot have a bad SEE, since all squares the king passes through are not under attack
        if mt == MoveType::Castling { return true }

        let piece_at_tgt = position.get_piece_at(tgt, !position.turn).unwrap();

        let mut move_value = match mv.get_capture() {
            true if mv.get_enpassant() == true => Piece::pawn(Color::White).piece_value(),
            true => piece_at_tgt.piece_value(),
            false => 0
        };

        // Piece being swapped off is the promoted piece
        let victim = if let Some(piece) = mv.get_promotion() {
            let new_piece = Piece::from((piece, position.turn));
            // We're promoting our piece to new type (hence adding it below), and removing the promoted pawn (both colors have the same value)
            move_value += new_piece.piece_value() - Piece::pawn(Color::White).piece_value();
            new_piece
        } else {
            position.piece_at(src).unwrap()
        };

        // Lose if the balance is already in our opponent's favour, and it's their turn
        let mut balance = move_value - threshold;
        if balance < 0 { return false }

        // Win if the balance is still in our favor even if we lose the capturing piece
        // This ensures a positive SEE in case of even trades
        balance -= victim.piece_value();
        if balance >= 0 { return true }

        let diaginal_sliders = *position[WB] | *position[BB] | *position[WQ] | *position[BQ];
        let orthogonal_sliders = *position[WR] | *position[BR] | *position[WQ] | *position[BQ];
        
        // let mut occs = (position.occupancies[Both] & !(1 << src as u64)) | (1 << tgt as u64);
        println!("this move is {}", mv.to_string());
        let mut local_board = position.board.clone();
        // println!("{}", local_board);
        // Removes the moving piece from the src and set it at the tgt
        let piece_at_src = local_board.piece_at(src).unwrap();
        match mv.get_enpassant() {
            true => {
                local_board.remove_piece(piece_at_src, src);
                local_board.remove_piece(piece_at_tgt, (Board::enpass_tgt(tgt, local_board.turn)).into())
            },
            false => {
                local_board.remove_piece(piece_at_src, src);
                local_board.remove_piece(piece_at_tgt, tgt);
            },
        }
        // Add the moved piece to the new position
        local_board.add_piece(victim, tgt);


        // Get all pieces covering the exchange square and start exchanging
        let mut attackers = local_board.get_all_attacks(tgt);
        // println!("--->>> times {}", position.to_string());
        let mut stm = !local_board.turn;

        // println!("total attackers for mv {:?} is {}", mv.to_string(), Bitboard::from(attackers));
        // if tgt == Square::H3 {
        //     println!("{}", Bitboard(local_board.occupancies[Both]));
        // }

        loop {
            // SEE terminates when no recapture is possible
            let our_attackers = attackers & *local_board[stm];

            println!("WE ARE {stm:?} {}", Bitboard::from(our_attackers));
            if our_attackers == 0 { break }

            println!("stm ---->>>> {:?}", stm);

            // Get the least valuable attacker and simulate the recapture
            let (attacking_piece, attacking_sq) = Self::get_lva(our_attackers, &local_board, stm).unwrap();
            local_board.remove_piece(attacking_piece, attacking_sq);

            // Diagonal recaptures uncover bishops/queens
            if [Piece::pawn(stm), Piece::bishop(stm), Piece::queen(stm)].contains(&attacking_piece) {
                attackers |= PIECE_ATTACKS.nnbishop_attacks(tgt.into(), local_board.occupancies[Both]) & diaginal_sliders;
            }

            // Orthognal recpatures uncover rooks/queens
            if [Piece::rook(stm), Piece::queen(stm)].contains(&attacking_piece) {
                attackers |= PIECE_ATTACKS.nnrook_attacks(tgt.into(), local_board.occupancies[Both]) & orthogonal_sliders;
            }

            attackers &= local_board.occupancies[Both];

            // Negamax the balance, cutoff if losing out attacker would still win the exchange
            stm = !stm;
            balance = -balance - 1 - attacking_piece.piece_value();

            if balance >= 0 {
                // If the recapturing piece is a king, and the opponent has another attacker,
                // a positrive balance should not translate to an exchange win.
                if attacking_piece == Piece::king(!stm) && ((attackers & *local_board[stm]) != 0) {
                    return local_board.turn == stm
                }
                break;
            }
        }
        // We win the exchange if we are not the one who should recapture
        local_board.turn != stm
    }


    /// Static Exchange Evaluation
    pub(crate) fn see(position: &Position, mv: &Move) -> i32 {
        let board = position.board;
        let sq = mv.get_target();


        let mut value = board.get_move_capture(*mv).unwrap().piece_value();
        let Some(mut board) = board.make_move(*mv, MoveScope::CapturesOnly) else { return i32::MIN };
        let mut us = false;


        loop {
            let mvs = board.gen_movement();
            let mut attacking_mvs = (mvs.into_iter().collect::<Vec<_>>()[..mvs.count_mvs()]).iter().filter_map(|m|{
                let mut tgt = m.get_target() as u64;
                if m.get_enpassant() { tgt = match board.turn {Color::Black => tgt + 8, _ => tgt -  8}; }
                
                let can_capture = m.get_capture() && tgt == (sq).into();
                if can_capture {Some((*m, position.get_piece_at(m.get_src(), board.turn).unwrap()))} else { None }
            }).collect::<Vec<_>>();

            if attacking_mvs.len() == 0 || board.piece_at(sq).is_none() {
                break;
            }

            // Sort attackers by least valueable attacker first (LVA)
            attacking_mvs.sort_by_key(|(_mv, attacker)| *attacker as usize);
            
            let (mv, ..) = attacking_mvs[0];

            // the piece that we would be removing here (piece_just_captured)
            let piece_to_remove = board.piece_at(sq).unwrap().piece_value();
            

            if let Some(b) = board.make_move(mv, MoveScope::CapturesOnly) {
                board = b;
                if us { value += piece_to_remove } else { value -= piece_to_remove }
                us = !us;
            } else {
                // this would only hapen is one of the kings is in-check
                if us { value += 2_000 } else {value = i32::MIN }
                break; 
            }
        }
        value
    }

    fn get_sorted_moves(&self, board: &Position) -> Vec<(Move, i32)> {
        let mvs = board.gen_movement();
        let mut mvs = (mvs.collect::<Vec<_>>())[0..mvs.count_mvs()].to_vec();
        //  now sort those moves and return the sorted moves
        // PV-nodes and expected Cut-nodes must be searched (give them more priority)

        
        let mut sorted_mvs = Vec::with_capacity(mvs.len());
        if let Some(pv_mv) = self.pv_table.get_pv(self.ply).get(0) {
            if let Some(pos) = mvs.iter().position(|&m| *m == *pv_mv) {
                sorted_mvs.push((mvs[pos], i32::MAX));
                mvs.remove(pos);
            }
        }

        
        let [mut captures, mut non_captures] = mvs.iter().fold([Vec::new(), Vec::new()], |mut acc, mv| {
            if mv.get_capture() {
                let r = if Self::static_exchange_eval(board, mv, 0) {20_000} else{10_000} ;
                acc[0].push((*mv, r)) 
            } else { acc[1].push((*mv, 0)) }
            [acc[0].clone(), acc[1].clone()]
        }); // use Self::see at this level, and filter out bad moves already
        
        
        
        // Sort by MVV-LVA table
        // let [mut captures, mut non_captures] = mvs.iter().fold([Vec::new(), Vec::new()], |mut acc, mv| {
        //     if mv.get_capture() {
        //         acc[0].push(
        //             (*mv, 
        //                 MVV_LVA[(board.piece_at(mv.get_src()).unwrap() as usize) % 6]
        //                 [(board.get_move_capture(*mv).unwrap() as usize) % 6])
        //             ) 
        //     } else { acc[1].push((*mv, 0)) }
        //     [acc[0].clone(), acc[1].clone()]
        // }); // use Self::see at this level, and filter out bad moves already

        // if captures.iter().find(|x| x.to_string() == "c3d2x").is_some() {
        //     println!("the board here is {}", board.to_string());
        // }
        
        // captures.sort_by_key(|mv| {
        //     return Self::see(board, mv);
        // });
        captures.sort_by_key(|mv| {
            return mv.1
        });

        captures.reverse();

        sorted_mvs.append(&mut captures);

        non_captures.sort_by_key(|mv| {
            if self.killer_moves.is_killer(self.ply, &mv.0) {
                return 1
            } 
            0
        });

        non_captures.reverse();

        sorted_mvs.append(&mut non_captures);
        
        return sorted_mvs
    }

    fn is_repetition(position: &Position, key: u64) -> bool {
        let len = position.history_len();

        if len == 0 { return false }

        // subtracting 1 from len because we don't care about the opponent's (the person who played last's) game
        // stepping by 2 because we don't care about the opponent's key positional history in this case
        for index in (0..len-1).rev().step_by(2) {
            if let Some(history) = position.history_at(index) { 
                if history.hash() == key { return true }
             } else { continue }
        } 

        false
    }


    // In addition, we a score to return in case there are no captures available to be played. -->> static evaluation
    /// At the beginning of quiescence, the position's evaluation is used to establish a lower-bound on the score.
    /// If the lower bound from the stand pat(static evaluation) is always greater than or equal to beta, we can return the stand-pat(fail-soft)
    /// or beta(fail-hard) as a lower bound. Otherwise, the search continues
    /// https://www.chessprogramming.org/Quiescence_Search
    fn quiescence(&mut self, mut alpha: i32, beta: i32, position: &mut Position) -> i32 {
        self.nodes+=1;
        
        let stand_pat = position.evaluate();
        if self.ply >= MAX_DEPTH { return stand_pat }
        // check if it's a draw
        if self.ply > 0 && (Self::is_repetition(&position, position.hash_key) || position.fifty.iter().any(|&p| p >= 50)) {
            return 0 // draw
        }

        let in_check = Self::in_check(&position, position.turn);
        // conditions
            // 1. is king in check
                //  if the stm is in check, the position is not quiet, and there is a threat that needs to be resolved. In that case, 
                // all evastions to the check are searched. Stand pat is not allowed if we are in check.
                // So, if the king of the stm is in check - WE MUST SEARCH EVERY MOVE IN THE POSITION, RATHER THAN ONLY CAPTURES.
                    // - LIMIT THE GENERATION OF CHECKS TO THE FIRST X PLIES OF QUIESCENCE (AND USE "DELTA PRUNNING" TO AVOID LONG FRUITLESS SEARCHES TO GET OUT OF BEEN IN CHECK)
        
        // beta cutoff
        if !in_check && stand_pat >= beta { return beta }
        if !in_check && stand_pat > alpha { alpha = stand_pat }
        
        // Probe the Transposition Table here
        let moves = self.get_sorted_moves(&position).iter().map(|x| x.0).collect::<Vec<_>>();
        let captures_only = !in_check;
        let mut best_score = -INFINITY;

        for mv in moves.into_iter() {
            if captures_only && !mv.get_capture() { continue; }

            if position.make_move_nnue(mv, MoveScope::AllMoves) {
                self.ply += 1;
                // if position.make_move_nnue(mv, MoveScope::AllMoves) {}
                let score = -self.quiescence(-beta, -alpha, position);
                position.undo_move(true);

                self.ply -= 1;
    
                if score > best_score {
                    best_score = score;
                    
                    if score > alpha { best_score = score; alpha = score; }
                    if score >= beta {alpha = beta; break}
                }
            }


        }


        if in_check && best_score == -INFINITY {
            return  - 5000;
        }
        
        alpha
    }

    fn in_check(position: &Position, color: Color) -> bool {
        let king_square = u64::from(position[Piece::king(color)].trailing_zeros());
        position.is_square_attacked(king_square, !color)
    }

    /// NB: This method is not pure, and would update the provided Move if the conditions are satisified
    fn probe_tt(&self, key: u64, depth: Option<u8>, alpha: i32, beta: i32, best_move: &mut Option<Move>) -> Option<i32>  {
        if let Some(entry) = self.tt.probe(key) {
            if depth.is_some_and(|d| d != entry.depth) { return None };
            best_move.replace(entry.mv?);
            let entry_score = entry.score(self.ply);
            let value = match entry.flag {
                HashFlag::Exact => Some(entry_score),
                HashFlag::LowerBound if entry_score >= beta => Some(beta),
                HashFlag::UpperBound if entry_score <= alpha => Some(alpha),
                _ => None
            };
            return value
        }
        None
    }

    /// nmfp: Null Move forward prunning
    /// https://web.archive.org/web/20040427014629/http://brucemo.com/compchess/programming/nullmove.htm
    /// "If I do nothing here, can the opponent do anything?"
    fn make_null_move(&mut self, beta: i32, depth: u8, mut position: &mut Position) -> Option<i32> {
        self.ply += 1;
        
        let old_hashkey = position.hash_key;
        let old_enpassant = position.enpassant;

        if let Some(enpass_sq) = position.enpassant {
                // we know that we're going to remove the enpass if it's available (see 4 lines below), so we remove it from the hashkey if it exists here
            position.set_zobrist(position.hash_key ^ ZOBRIST.enpassant_keys[enpass_sq]);
        }

        position.set_turn(!position.turn);
        position.set_enpassant(None);
        position.set_zobrist(position.hash_key ^ ZOBRIST.side_key); // side about to move
        position.nnue_push();

        let score = -self.alpha_beta(-beta, -beta+1, depth-1-DEPTH_REDUCTION_FACTOR, &mut position);

        // reverse all actions, after we're done
        self.ply -= 1;
        position.nnue_pop();

        // recent change for undo move in order to avoid cloning the board
        position.set_turn(!position.turn);
        position.set_enpassant(old_enpassant);
        position.set_zobrist(old_hashkey);

        if score > beta {
            return Some(beta);
        }

        None
    }

    pub(crate) fn alpha_beta(&mut self, mut alpha: i32, beta: i32, depth: u8, mut position: &mut Position) -> i32 {
        let mut hash_flag = HashFlag::UpperBound;

        if Self::is_repetition(&position, position.hash_key) || position.fifty.iter().any(|&s| s >= 50) {
            if depth == 2 && self.limit == 2 {
                println!("<<<<<<<<<<<<<<<<<<<<<<<<<it's a repetition>>>>>>>>>>>>>>>>>>>>>>>>>");
            }
            return 0; // draw
        }

        let mut best_mv: Option<Move> = None;

        let tt_hit = self.probe_tt(position.hash_key, Some(depth), alpha, beta, &mut best_mv);
        // When beta - alpha > 1, it indicates that there is a significant gap between the two bounds. This gap suggests that there are possible values for the evaluation score that have not yet been fully explored or are still uncertain.
        // The search can continue to explore more moves because the values returned by the evaluated moves could potentially fall within this range, providing room for a better evaluation.
        let explore_more_moves = (beta - alpha) > 1;

        if self.ply > 0 && tt_hit.is_some() && !explore_more_moves { 
            if depth == 2 && self.limit == 2 {
                println!("[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[USING TT TABLE HERE]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]");
            }
            return tt_hit.unwrap() }
        if depth == 0 { return self.quiescence(alpha, beta, position) }
        if self.ply > MAX_PLY - 1 { 
            if depth == 2 && self.limit == 2 {
                println!("((((((((((((((((((((((((((((EXCEEDED MAX_PLY))))))))))))))))))))))))))))");
            }
            return position.evaluate() }


        self.nodes += 1;

        let stm_in_check = Self::in_check(position, position.turn);
        // let depth = if stm_in_check { depth + 1} else { depth };
        let mut mvs_searched = 0;

        let null_move_forward_pruning_conditions = depth >= (DEPTH_REDUCTION_FACTOR + 1) && !stm_in_check && self.ply > 0;

        if null_move_forward_pruning_conditions {
            if let Some(beta) = self.make_null_move(beta, depth, position) {
                if depth == 2 && self.limit == 2 {
                    println!("-----------------------------------------------------------AFTER NULL-MOVE FP-----------------------------------------------------------");
                }
                return beta;
            }
        }

        let mut best_score = -INFINITY;
        let mvs = self.get_sorted_moves(&position);


        let mvs = mvs.iter().map(|x| x.0).collect::<Vec<_>>();


        if self.limit == 3 && depth == 3 {
            for mv in &mvs {
                print!("::: {} ----->>>> ", mv.to_string())
            }
        }


        
        for (index, mv) in mvs.iter().enumerate() {
            // println!("TOTAL MOVES IS {} index is {index} _---and now we're running>>>>>> {} limit->{}, depth-->{} stm is {:?}", mvs.len(), mv.to_string(), self.limit, depth, position.turn);
            if (mv.to_string() == "e2a6x" || mv.to_string() == "g2h3x") && self.limit == 3 && depth == 3  {
                println!("\n\n <<BEFORE>><<BEFORE>><<BEFORE>><<BEFORE>>for mv: {}, with score => xxx, alpha={alpha}, and beta={beta}", mv.to_string());
            }

            if position.make_move_nnue(*mv, MoveScope::AllMoves) {
                self.ply += 1;
                // mvs_searched = moves_searched;
                // if (mv.to_string() == "e2a6x" || mv.to_string() == "g2h3x") && self.limit == 2 && depth == 2  {
                //     println!("\n\n *******************for mv: {}, with score => xxx, alpha={alpha}, and beta={beta}", mv.to_string());
                // }

                // let score = -self.alpha_beta(-beta, -alpha, depth -1, &mut position);
                let score = match mvs_searched {
                    0 => -self.alpha_beta(-beta, -alpha, depth -1, &mut position),
                    _ => {
                        // https://web.archive.org/web/20150212051846/http://www.glaurungchess.com/lmr.html
                        // condition for Late Move Reduction
                        let non_tatcital_mv = !stm_in_check && mv.get_promotion().is_none() && !mv.get_capture();

                        let mut value = if (mvs_searched as u8 >= FULL_DEPTH_MOVE) && (depth >= REDUCTION_LIMIT) && non_tatcital_mv {
                            -self.alpha_beta(-(alpha + 1), -alpha, depth-2, position)
                        } else {
                            alpha + 1 // Hack to ensure that full-depth search is done
                        };

                        if value > alpha {
                            value = -self.alpha_beta(-(alpha - 1), -alpha, depth-1, position);

                            if (value > alpha) && value < beta {
                                value = -self.alpha_beta(-beta, -alpha, depth-1, position);
                            }
                        }

                        value
                    }
                };
                
                mvs_searched += 1;

                let zobrist_key = position.hash_key;
                position.undo_move(true);
                self.ply -= 1;
                let moved_piece = position.piece_at(mv.get_src()).unwrap();

                // if (mv.to_string() == "e2a6x" || mv.to_string() == "g2h3x") && self.limit == 2 && depth == 2  {
                //     println!("\n\n *******************for mv: {}, with score => {}, alpha={alpha}, and beta={beta}", mv.to_string(), score);
                // }


                if score >= beta {
                    self.tt.record(zobrist_key, depth, beta, INFINITY, self.ply, hash_flag, 0, best_mv); 
                    if !mv.get_capture() {
                        self.killer_moves.store(depth as usize, mv);
                    }
                    return beta;
                }

                // if score > best_score {
                //     best_score = score;
                    
                if depth == 3 && self.limit == 3 {
                    println!("----------------------------<<<<<<<<<<<<<<ON WHATEVER>>>>>>>>>>>>>>------------for mv: {}, with score => {score}, alpha={alpha}, and beta={beta}", mv.to_string());
                }
                if score > alpha {
                    // println!("previous_best_score==>> {best_score}, depth = {depth} --->>>> the move is {:?}, and the score is {score}, alpha is {alpha}, and beta={beta}", mv.to_string());
                    best_score = score;
                    
                    best_mv = Some(*mv);
                    hash_flag = HashFlag::Exact;
                    self.pv_table.store_pv(self.ply, mv);
                    alpha = score;

                    if !mv.get_capture() {
                        self.history_table.update(moved_piece, mv.get_src(), depth);
                    }
                }
                // }
            }
        }

        if mvs_searched == 0 {
            if stm_in_check {
                return -MATE_VALUE + self.ply as i32;
            }
            // king is not in check, but there are no legal moves
            return 0; // stalemate/draw
        }

        self.tt.record(position.hash_key, depth, best_score, INFINITY, self.ply, hash_flag, 0, best_mv);
        alpha
    }




}
