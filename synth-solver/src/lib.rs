#![warn(clippy::trivially_copy_pass_by_ref, clippy::use_self)]

pub use tinyvec;

pub mod errors;
mod impl_cauldron;
mod impl_color;
mod impl_color_score_set;
mod impl_coverage;
mod impl_material;
mod impl_placement;
mod impl_shape;
mod impl_shape_neighbours;
pub mod solver;
pub mod utils;

/// The maximum amount of item groups/goals that can be in a game.
pub const MAX_GOALS: usize = 4;

/// The max amount of items in a single item group.
pub const MAX_ITEMS_IN_GROUP: usize = 5;

pub const MAX_ITEMS: usize = MAX_GOALS * MAX_ITEMS_IN_GROUP;

// TODO: what is the actual practical amount of items that can be in a game?

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
    pub transformation: Option<Transformation>,
}

/// The playfield of the game.
#[derive(Clone)]
pub struct Cauldron {
    /// The panel size of a cauldron, eg. 4x4, 5x5 or 6x6.
    pub size: usize,
    /// The individual tiles of a cauldron, or None if the tile is a hole.
    pub tiles: tinyvec::ArrayVec<[Option<Tile>; 6 * 6]>, // TODO: maybe use const generic? size should be 4x4, 5x5 or 6x6
    /// The bonus scores for each level of a tile.
    // TODO: some cauldrons have percentages instead of fixed values
    pub bonus_scores: (u32, u32, u32),
    /// The color of the item being crafted. This is used when this cauldron has the [`CauldronProperties::SYNERGY`] property.
    pub color: Color,
    /// The properties of the cauldron.
    pub properties: CauldronProperties,
}

bitflags::bitflags! {
    /// Optional properties for the cauldron that change how score may be calculated.
    #[derive(Copy, Clone, Default)]
    pub struct CauldronProperties: u32 {
        /// Obtaining bonusses that are the same color as the liquid in the cauldron increases by 50%.
        const SYNERGY = 0b001;
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum TransformationType {
    #[default]
    None,
    FlipHorizontal,
    FlipVertical,
    Rotate,
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
    pub played_material_index: Option<(usize, usize)>,
}

/// An item that can be placed in the cauldron.
#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub color: Color,
    /// The base value of an item that gets added to the score, before applying the coverage
    /// multiplier.
    pub effect_value: u32,
    /// The shape of this item.
    pub shape: Shape,
}

/// A shape in a 3x3 grid.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Shape(u16);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ShapeNeighbours(u32);

#[derive(Debug, Copy, Clone)]
pub enum Transformation {
    FlipHorizontal,
    FlipVertical,
    Rotate90,
    Rotate180,
    Rotate270,
}

/// An item effect that can be reached by getting certain item effect levels.
#[derive(Debug, Clone)]
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
#[derive(Default, Debug)]
pub struct CoverageInfo {
    coverage: [u32; 5],
}

/// The effect value score for each color of an item group/a single goal, before item bonus and
/// coverage is applied.
#[derive(Default, Clone, Copy, Debug)]
pub struct ColorScoreSet {
    scores: [u32; 5],
}
