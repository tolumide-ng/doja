use std::ops::Deref;

use crate::constants::SHIFT_DATA;


#[derive(Debug, Clone, Copy)]
pub struct ShiftData {
    /// The avoidWrap masks by som arbitrary dir8 enumeration
    pub mask: u64,
    /// The shift amount to move in this direction
    pub amount: i8
}

impl ShiftData {
    pub(crate) const fn new(mask: u64, amount: i8) -> Self {
        Self { mask, amount }
    }

    pub(crate) fn mask(&self) -> u64 {
        self.mask
    }

    pub(crate) fn amount(&self) -> i8 {
        self.amount
    }
}


#[derive(Debug, Clone, Copy)]
pub enum Shift {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Deref for Shift {
    type Target = ShiftData;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::North => &SHIFT_DATA[7],
            Self::NorthEast => &SHIFT_DATA[0],
            Self::East => &SHIFT_DATA[1],
            Self::SouthEast => &SHIFT_DATA[2],
            Self::South => &SHIFT_DATA[3],
            Self::SouthWest => &SHIFT_DATA[4],
            Self::West => &SHIFT_DATA[5],
            Self::NorthWest => &SHIFT_DATA[6],
        }
    }
}