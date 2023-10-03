use crate::*;

impl Item {
    pub fn new(color: Color, quality: usize, shape: Shape) -> Self {
        Self {
            color,
            quality,
            shape,
        }
    }
}
