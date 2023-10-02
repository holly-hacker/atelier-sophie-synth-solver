use crate::{Color, ColorScoreSet};

impl ColorScoreSet {
    pub fn get(&self, color: Color) -> usize {
        self.scores[color.get_index()]
    }

    pub fn get_mut(&mut self, color: Color) -> &mut usize {
        &mut self.scores[color.get_index()]
    }

    fn into_colors(self) -> [(Color, usize); 5] {
        [
            (Color::from_index(0), self.scores[0]),
            (Color::from_index(1), self.scores[1]),
            (Color::from_index(2), self.scores[2]),
            (Color::from_index(3), self.scores[3]),
            (Color::from_index(4), self.scores[4]),
        ]
    }
}

impl IntoIterator for ColorScoreSet {
    type Item = (Color, usize);
    type IntoIter = std::array::IntoIter<Self::Item, 5>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_colors().into_iter()
    }
}
