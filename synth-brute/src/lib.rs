mod impl_color;
mod impl_color_score_set;
mod impl_coverage;
mod impl_playfield;
mod impl_shape;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Color {
    /// Fire
    Red,
    /// Ice
    Blue,
    /// Leaf
    Green,
    /// Bolt
    Yellow,
    /// Holy
    White,
}

#[derive(Copy, Clone)]
pub struct Placement {
    /// The index in the playfield where the item is placed.
    pub index: usize,
    pub transformations: (), // TODO: flipping h/v, rotating
}

#[derive(Clone)]
pub struct Playfield {
    pub width: usize,
    pub data: Vec<Option<Tile>>, // TODO: maybe use const generic? size should be 4x4, 5x5 or 6x6
}

#[derive(Copy, Clone)]
pub struct Tile {
    pub color: Color,
    /// The level of the tile. Value between 0 and 3 inclusive.
    pub level: usize,
    /// The color that was played here
    pub played_color: Option<Color>,
}

#[derive(Copy, Clone)]
pub struct Item {
    pub color: Color,
    pub quality: usize,
    pub shape: Shape,
}

#[derive(Copy, Clone)]
pub struct Shape(pub [[bool; Self::WIDTH]; Self::HEIGHT]);

pub struct Goal {
    pub color: Color,
    /// Thresholds where the goal is considered met.
    pub quality_levels: Vec<usize>,
}

/// The coverage of a playfield.
///
/// This indicates how much of the playfield is covered by an item of a specific color. Overlapped
/// tiles are removed so the removed tiles don't count towards coverage.
///
/// Coverage is later used to multiply the progress towards the goals.
#[derive(Default)]
pub struct CoverageInfo {
    coverage: [usize; 5],
}

#[derive(Default, Clone, Copy, Debug)]
pub struct ColorScoreSet {
    scores: [usize; 5],
}
