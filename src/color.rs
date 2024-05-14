use std::ops::{Index, IndexMut, Not};

// sides to move
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White=0, Black=1, Both=2
}


impl<T> Index<Color> for [T] {
    type Output = T;
    fn index(&self, index: Color) -> &Self::Output {
        match index {
            Color::White => &self[0], Color::Black=> &self[1], Color::Both => &self[2]
        }
    }
}

impl<T> IndexMut<Color> for [T] {
    fn index_mut(&mut self, index: Color) -> &mut Self::Output {
        match index {
            Color::White => &mut self[0], Color::Black=> &mut self[1], Color::Both => &mut self[2]
        }
    }
}


impl From<&str> for Color {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "w" => Self::White,
            "b" => Self::Black,
            _ => panic!("Unrecognized color provided: {value}")
        }
    }
}



impl Not for Color {
    type Output = Self;
    
    fn not(self) -> Self::Output {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black,
            Self::Both => Self::White
        }
    }
}