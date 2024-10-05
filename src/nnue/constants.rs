pub(crate) mod half_kp {
    /// HalfKP features (Half-King Piece Features)
    /// The Accumulator stores 40960 per perspective (white and black)
    /// 40960 * 2 = 81920 features
    pub(crate) const INPUT: usize = 64 * 64 * 2 * 5;

    /// Layer 1: Input to Hidden Transformation
    /// 256 * 2 = 512
    pub(crate) const L1_SIZE: usize = 256;

    pub(crate) const L2_SIZE: usize = 32;

    pub(crate) const L3_SIZE: usize = 32;

    pub(crate) const L4_SIZE: usize = 1;

    /// (L2_SIZE * L1_SIZE) * 2(perspectives) = 20_971_520 
    pub(crate) const L1_WEIGHTS: usize = (256 * 40960) * 2;

    /// L2_SIZE * 2
    pub(crate) const L1_BIAS: usize = 256 * 2;

    /// L2_SIZE * L3_SIZE
    pub(crate) const L2_WEIGHTS: usize = 512 * 32;

    pub(crate) const L2_BIAS: usize = 32;

    pub(crate) const L3_WEIGHTS: usize = 32 * 32;
    pub(crate) const L3_BIAS: usize = 32;

    pub(crate) const L4_WEIGHTS: usize = 1 * 32;
    pub(crate) const L4_BIAS: usize = 1;

    
}



pub(crate) mod half_ka {
    pub(crate) const SCALING_FACTOR: usize = 410;
    /// 2^(6)
    pub(crate) const LOG2_WEIGHT_SCALE: i32 = 64; 
    pub(crate) const INPUT: usize = 768;
    
    
    // TEST LATER WITH STOCKFISH: L1 size increased to 2560.
    // pub(crate) const L1_SIZE: usize = 2048 * 2;
    pub(crate) const L2_SIZE: usize = 16;
    pub(crate) const L3_SIZE: usize = 32;
    pub(crate) const L4_SIZE: usize = 1;
    
    pub(crate) const L1_WEIGHTS: usize = 16 * 4096;
    pub(crate) const L2_WEIGHTS: usize = 16 * 32;
    pub(crate) const L3_WEIGHTS: usize = 32 * 16;
    pub(crate) const L4_WEIGHTS: usize = 32 * 1;
    
    pub(crate) const L1_BIAS: usize = 4096;
    pub(crate) const L2_BIAS: usize = 16;
    pub(crate) const L3_BIAS: usize = 32;
    pub(crate) const L4_BIAS: usize = 1;
}


pub(crate) mod custom_kp {
    pub(crate) const INPUT: usize = 768;
    pub(crate) const L1_SIZE: usize = 1024;
    // pub(crate) const L2_SIZE: usize = 1024*2;
}