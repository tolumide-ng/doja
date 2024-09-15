use std::ops::{Index, IndexMut, Not};

// sides to move
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    White=0, Black=1, Both=2
}


impl From<Color> for usize {
    fn from(value: Color) -> Self {
        match value {
            Color::White => 0, Color::Black=> 1, Color::Both=>2,
        }
    }
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


#[cfg(test)]
mod  color_tests {
    use super::*;


    #[test]
    fn should_return_the_respective_u8_value_for_each_color() {
        let white = Color::White;
        let black = Color::Black;
        let both = Color::Both;

        assert_eq!(white as u8, 0);
        assert_eq!(black as u8, 1);
        assert_eq!(both as u8, 2);
    }


    #[test]
    fn can_index_slice_with_color() {
        let mut some_arr = [1, 2, 3, 6];
        assert_eq!(some_arr[Color::Black], some_arr[1]); 
        assert_eq!(some_arr[Color::White], some_arr[0]); 
        assert_eq!(some_arr[Color::Both], some_arr[2]);

        some_arr[Color::Black] = 16;
        assert_eq!(some_arr[Color::Black], 16); 
    }

    #[test]
    fn has_str_implementation() {
        assert_eq!(Color::from("w"), Color::White);
        assert_eq!(Color::from("W"), Color::White);
        assert_eq!(Color::from("B"), Color::Black);
        assert_eq!(Color::from("b"), Color::Black);
    }

    #[test]
    #[should_panic(expected = "Unrecognized color provided: x")]
    fn should_fail_to_convert_an_invalid_str() {
        let _ = Color::from("x");
    }

    #[test]
    fn has_not_implementation() {
        assert_eq!(!Color::White, Color::Black);
        assert_eq!(!Color::Black, Color::White);
        assert_eq!(!Color::Both, Color::White);
    }
}