use crate::*;

impl Material {
    pub fn new(color: Color, quality: u32, shape: Shape) -> Self {
        Self {
            color,
            effect_value: quality,
            shape,
        }
    }
}
