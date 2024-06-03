use std::ops::Deref;

pub(crate) struct Rand(u32);

impl Deref for Rand {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Rand {
    pub(crate) fn new(seed: u32) -> Self {Self(seed)}

    fn get_random_u32(&mut self) -> u32 {
        let mut number = **self ;
        number ^= number.wrapping_shl(13);
        number ^= number.wrapping_shr(17);
        number ^= number.wrapping_shl(5);

        self.0 = number;
        number
    }

    pub(crate) fn get_random_u64(&mut self) -> u64 {
        let n1 = (self.get_random_u32() as u64) & 0xFFFF;
        let n2 = (self.get_random_u32() as u64) & 0xFFFF;
        let n3 = (self.get_random_u32() as u64) & 0xFFFF;
        let n4 = (self.get_random_u32() as u64) & 0xFFFF;

        n1 | n2 << 16 | n3 << 32 | n4 << 48
    }


    pub(crate) fn generate_magic(&mut self) -> u64 {
        self.get_random_u64() & self.get_random_u64() & self.get_random_u64() 
    }
}