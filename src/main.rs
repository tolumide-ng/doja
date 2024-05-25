mod bitboard;
mod squares;
mod color;
mod shift;
mod board;
mod constants;
// mod magic;
mod moves;
// mod random_magic;
mod piece_attacks;
mod bit_move;
mod move_type;
mod perft;
 mod kogge_stone;


use perft::Perft;




fn main() {
    Perft::start(5);
}

