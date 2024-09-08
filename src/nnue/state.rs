use std::alloc::{self, alloc_zeroed, Layout};

use crate::{board::{state::board_state::BoardState, piece::Piece}, color::Color, squares::Square};
use crate::color::Color::*;

use super::{accumulator::Accumulator, commons::{Eval, HIDDEN, MAX_DEPTH, QA, QAB, SCALE}, net::{nnue_index, squared_crelu, MODEL}};

// Used for turning features on/off
pub(super) const ON: bool = true;
pub(super) const OFF: bool = true;


/// NNUEStack is a stack of accumulators. updated along the search tree
#[derive(Debug, Clone)]
pub(crate) struct NNUEState {
    accumulator_stack: [Accumulator; MAX_DEPTH + 1],
    // index of the current accumulator
    current_acc: usize,
}


impl NNUEState {
    pub fn from_board(board: &BoardState) -> Box<Self> {
        let mut boxed: Box<Self> = unsafe {
            let layout = Layout::new::<Self>();
            let ptr = alloc_zeroed(layout);

            if ptr.is_null() {
                alloc::handle_alloc_error(layout);
            }

            Box::from_raw(ptr.cast())
        };

        // init with feature biases and add in all features of the board
        boxed.accumulator_stack[0] = Accumulator::default();

        let mut board_sqs = board.get_occupancy(Color::Both);
        while board_sqs != 0 {
            let sq = board_sqs.trailing_zeros() as u64;
            let white = board.get_occupancy(Color::White) & (1u64 << sq);
            let sq = Square::from(sq);
            let color = if white != 0 { White } else { Black };

            boxed.manual_update::<ON>(board.get_piece_at(sq, color).unwrap(), sq);
            board_sqs &= board_sqs - 1;
        }

        // while black_sqs != 0 {
        //     let sq = Square::from(black_sqs.trailing_zeros() as u64);
        //     boxed.manual_update::<ON>(board.get_piece_at(sq, Color::Black).unwrap(), sq);
        //     black_sqs &= black_sqs -1;
        // }

        // while white_sqs != 0 {
        //     let sq = Square::from(white_sqs.trailing_zeros() as u64);
        //     boxed.manual_update::<ON>(board.get_piece_at(sq, Color::White).unwrap(), sq);
        //     white_sqs &= white_sqs -1;
        // }
        
        boxed
    }


    /// Refresh the accumulator stack to the given board
    pub fn refresh(&mut self, board: &BoardState) {
        // reset the accumualtor stack
        self.current_acc = 0;
        self.accumulator_stack[self.current_acc] = Accumulator::default();

        // Update the first accumulator
        
        let mut black_sqs = board.get_occupancy(Color::Black);
        let mut white_sqs = board.get_occupancy(Color::White);

        while black_sqs != 0 {
            let sq = Square::from(black_sqs.trailing_zeros() as u64);
            self.manual_update::<ON>(board.get_piece_at(sq, Color::Black).unwrap(), sq);
            black_sqs &= black_sqs -1;
        }

        while white_sqs != 0 {
            let sq = Square::from(white_sqs.trailing_zeros() as u64);
            self.manual_update::<ON>(board.get_piece_at(sq, Color::White).unwrap(), sq);
            white_sqs &= white_sqs -1;
        }

    }
    
    /// Add a new accumulator to the stack by copying the previous top
    pub(crate) fn push(&mut self) {
        self.accumulator_stack[self.current_acc + 1] = self.accumulator_stack[self.current_acc];
        self.current_acc += 1;
    }

    /// Pop the top of the accumualtor stack
    pub(crate) fn pop(&mut self) {
        self.current_acc -=1;
    }


    pub(crate) fn manual_update<const ON: bool>(&mut self, piece: Piece, sq: Square) {
        self.accumulator_stack[self.current_acc].update_weights::<ON>(nnue_index(piece, sq));
    }

    /// Efficiently update accumulator for a quiet move (that is, only changes from/to features)
    pub(crate) fn move_update(&mut self, piece: Piece, from: Square, to: Square) {
        let from_idx = nnue_index(piece, from);
        let to_idx = nnue_index(piece, to);

        self.accumulator_stack[self.current_acc].add_sub_weights(from_idx, to_idx);
    }


    /// Evaluate the nn from the current accumualtor
    /// Concatenates the accumualtors based on the side to move, computes the activation function
    /// with Squared CReLu and muiltiplies activation by weight. The result is the sum of all these
    /// with the bias.
    /// Since we are squaring activations, we need extra quantization pass with QA.
    pub(crate) fn evaluate(&self, side: Color) -> Eval {
        let acc = &self.accumulator_stack[self.current_acc];

        let (us, them) = match side {
            Color::White => (acc.white.iter(), acc.black.iter()),
            _  => (acc.black.iter(), acc.white.iter()) // Color::Black
        };

        let mut out = 0;
        for (&value, &weight) in us.zip(&MODEL.output_weights[..HIDDEN]) {
            out += squared_crelu(value) * weight as i32;
        }

        ((out / QA + MODEL.output_bias as i32) * SCALE / QAB ) as Eval
    }
}