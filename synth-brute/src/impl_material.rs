use crate::*;

impl Material {
    pub fn new(color: Color, quality: usize, shape: Shape) -> Self {
        Self {
            color,
            effect_value: quality,
            shape,
        }
    }
}
