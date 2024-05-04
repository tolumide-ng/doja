use std::ops::Deref;

pub struct Magic(u32);

impl Magic {
    pub fn new() -> Self {
        Self(1804289383)
    }

    pub fn random_u32(&mut self) -> u32 {
        // XOR shift algorithm
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 17;
        self.0 ^= self.0 << 5;

        self.0
    }


    pub fn random_u64(&mut self) -> u64 {
        let u1 = (self.random_u32() as u64) & 0xFFFF;
        let u2 = (self.random_u32() as u64) & 0xFFFF;
        let u3 = (self.random_u32() as u64) & 0xFFFF;
        let u4 = (self.random_u32() as u64) & 0xFFFF;

        u1 | u2 << 16 | u3 << 32 | u4 << 48
    }

    /// Generate magic number
    pub fn random_u64_fewbits(&mut self) -> u64 {
        self.random_u64() & self.random_u64() & self.random_u64()
    }
}


impl Deref for Magic {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}