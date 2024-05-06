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