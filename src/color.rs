// sides to move
#[derive(Debug, Clone, Copy)]
pub enum Color {
    White=0, Black=1, Both=2
}

impl From<Color> for usize {
    fn from(value: Color) -> Self {
        match value {
            Color::White => 0,
            Color::Black => 1,
            Color::Both => 2,
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

// impl From<Color> for usize {
//     fn from(value: Color) -> Self {
//         match value {
//             Color::White => 0,
//             Color::Black => 1,
//             Color::Both => 2,
//         }
//     }
// }