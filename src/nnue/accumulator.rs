const MIRROR: [u8; 8] = [3, 2, 1, 0, 0, 1, 2, 3];

pub(crate) const fn sq64_to_sq32(sq: u8) -> u8 {
    (sq >> 1) & !0x3 + MIRROR[(sq & 0x7) as usize]
}

