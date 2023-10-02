use crate::Color;

impl Color {
    pub fn get_index(self) -> usize {
        match self {
            Self::Red => 0,
            Self::Blue => 1,
            Self::Green => 2,
            Self::Yellow => 3,
            Self::White => 4,
        }
    }

    pub fn from_index(index: i32) -> Color {
        match index {
            0 => Color::Red,
            1 => Color::Blue,
            2 => Color::Green,
            3 => Color::Yellow,
            4 => Color::White,
            n => panic!("Invalid color index {n}"),
        }
    }
}
