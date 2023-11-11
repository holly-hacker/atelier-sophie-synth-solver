use crate::*;

impl CoverageInfo {
    #[must_use]
    pub fn add_color(mut self, color: Color) -> Self {
        self.coverage[color.get_index()] += 1;
        self
    }

    pub fn get_color(&self, color: Color) -> u32 {
        self.coverage[color.get_index()]
    }

    pub fn get_color_ratio(&self, color: Color, playfield: &Cauldron) -> f32 {
        let len = playfield.tiles.len();
        self.get_color(color) as f32 / len as f32
    }

    pub fn get_color_ratio_conditional(&self, color: Color, playfield: &Cauldron) -> f32 {
        let is_max = self.coverage.iter().max() == Some(&self.get_color(color));

        if is_max {
            self.get_color_ratio(color, playfield)
        } else {
            0.
        }
    }
}
