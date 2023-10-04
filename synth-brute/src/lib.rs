pub mod errors;
pub mod find_optimal;
mod impl_cauldron;
mod impl_color;
mod impl_color_score_set;
mod impl_coverage;
mod impl_material;
mod impl_placement;
mod impl_shape;

pub use tinyvec;

/// The maximum amount of item groups/goals that can be in a game.
pub const MAX_GOALS: usize = 4;

/// The max amount of items in a single item group.
pub const MAX_ITEMS_IN_GROUP: usize = 5;

pub const MAX_ITEMS: usize = MAX_GOALS * MAX_ITEMS_IN_GROUP;

// TODO: what is the actual practical amount of items that can be in a game?

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

/// The placement of a material on the playfield.
#[derive(Debug, Copy, Clone, Default)]
pub struct Placement {
    /// The index in the playfield where the item is placed.
    pub index: usize,
    pub transformations: (), // TODO: flipping v/h, rotating
}

/// The playfield of the game.
#[derive(Clone)]
pub struct Cauldron {
    /// The panel size of a cauldron, eg. 4x4, 5x5 or 6x6.
    pub size: usize,
    /// The individual tiles of a cauldron, or None if the tile is a hole.
    pub tiles: tinyvec::ArrayVec<[Option<Tile>; 6 * 6]>, // TODO: maybe use const generic? size should be 4x4, 5x5 or 6x6
}

/// A tile in the cauldron's playfield.
#[derive(Copy, Clone)]
pub struct Tile {
    pub color: Color,
    /// The bonus level of the tile. Value between 0 and 3 inclusive.
    ///
    /// The effect of this tile depends on the cauldron itself.
    pub level: u32,
    /// The color that was played here
    pub played_color: Option<Color>,
}

/// An item that can be placed in the cauldron.
#[derive(Copy, Clone)]
pub struct Material {
    pub color: Color,
    /// The base value of an item that gets added to the score, before applying the coverage
    /// multiplier.
    pub effect_value: u32,
    /// The shape of this item.
    pub shape: Shape,
}

/// A shape in a 3x3 grid.
#[derive(Copy, Clone)]
pub struct Shape(pub [[bool; Self::WIDTH]; Self::HEIGHT]);

/// An item effect that can be reached by getting certain item effect levels.
pub struct Goal {
    /// Thresholds where the goal is considered met.
    pub effect_value_thresholds: Vec<u32>,
}

/// The coverage of a playfield.
///
/// This indicates how much of the playfield is covered by a material of a specific color.
/// Overlapped tiles are removed so the removed tiles don't count towards coverage.
///
/// Coverage is later used to multiply the progress towards the goals.
#[derive(Default)]
pub struct CoverageInfo {
    coverage: [u32; 5],
}

/// The effect value score for each color of an item group/a single goal.
#[derive(Default, Clone, Copy, Debug)]
pub struct ColorScoreSet {
    scores: [u32; 5],
}
