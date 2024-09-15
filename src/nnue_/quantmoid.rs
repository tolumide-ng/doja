// Sigmoid--->>>> y = 1/(1+e^(-kx))
// where k is the paramter that determines how stretched the shaped is. However Sigmoid is too expensive.
// The reason for the choice of the upper range bneing defined as 126 is that this is the largest even 8-bit integer.


/// https://disservin.github.io/stockfish-docs/nnue-pytorch-wiki/docs/nnue.html#quantmoid4
fn quantmoid4(x: i32) -> i32{
    let sign = (x > 0) as i32; // x > 0 ? 1 : 0 

    let abs_x = i32::min(x.abs(), 127) - 127;
    let abs_sq = (abs_x * abs_x)/256;

    (sign * abs_sq) + ((1-sign) * (126-abs_sq))
}