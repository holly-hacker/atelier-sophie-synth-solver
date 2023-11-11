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

    pub fn from_index(index: i32) -> Self {
        match index {
            0 => Self::Red,
            1 => Self::Blue,
            2 => Self::Green,
            3 => Self::Yellow,
            4 => Self::White,
            n => panic!("Invalid color index {n}"),
        }
    }

    pub fn from_mat_color_tag(tag: &str) -> Self {
        match tag {
            "ITEM_MAT_COLOR_RED" => Self::Red,
            "ITEM_MAT_COLOR_BLUE" => Self::Blue,
            "ITEM_MAT_COLOR_GREEN" => Self::Green,
            "ITEM_MAT_COLOR_YELLOW" => Self::Yellow,
            "ITEM_MAT_COLOR_WHITE" => Self::White,
            _ => panic!("Invalid color tag {tag}"),
        }
    }
}
