use crate::{Cauldron, Color, ColorScoreSet, CoverageInfo, Material};

impl ColorScoreSet {
    pub fn get(&self, color: Color) -> u32 {
        self.scores[color.get_index()]
    }

    pub fn get_mut(&mut self, color: Color) -> &mut u32 {
        &mut self.scores[color.get_index()]
    }

    fn into_colors(self) -> [(Color, u32); 5] {
        [
            (Color::from_index(0), self.scores[0]),
            (Color::from_index(1), self.scores[1]),
            (Color::from_index(2), self.scores[2]),
            (Color::from_index(3), self.scores[3]),
            (Color::from_index(4), self.scores[4]),
        ]
    }

    pub fn calculate_score(
        &self,
        items: &[Material],
        coverage: &CoverageInfo,
        playfield: &Cauldron,
    ) -> u32 {
        self.into_iter()
            .map(|(color, color_score)| {
                let base = items
                    .iter()
                    .filter(|i| i.color == color)
                    .map(|i| i.effect_value)
                    .sum::<u32>();
                let ratio = coverage.get_color_ratio_conditional(color, playfield);
                (base + color_score) as f32 * (1. + ratio)
            })
            .map(|f| f as u32)
            .sum()
    }
}

impl IntoIterator for ColorScoreSet {
    type Item = (Color, u32);
    type IntoIter = std::array::IntoIter<Self::Item, 5>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_colors().into_iter()
    }
}
