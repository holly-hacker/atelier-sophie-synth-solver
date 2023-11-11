use std::collections::HashMap;

// TODO: move everything to a build script, so we only include the data we actually need

const INGREDIENTS_JSON: &str =
    include_str!("../../atelier-data/sophie/manually_extracted/ingredients.json");
const ITEM_BOARDS_JSON: &str =
    include_str!("../../atelier-data/sophie/manually_extracted/item_boards.json");
const ITEM_EFFECT_THRESHOLDS_JSON: &str =
    include_str!("../../atelier-data/sophie/manually_extracted/item_effect_thresholds.json");
const ITEM_EFFECTS_JSON: &str = include_str!("../../atelier-data/sophie/item_effects.json");
const ITEMS_JSON: &str = include_str!("../../atelier-data/sophie/items.json");
const SHAPES_JSON: &str = include_str!("../../atelier-data/sophie/manually_extracted/shapes.json");

lazy_static::lazy_static! {
    pub static ref INGREDIENTS: HashMap<String, Vec<Ingredient>> = serde_json::from_str(INGREDIENTS_JSON).unwrap();
    pub static ref ITEM_BOARDS: HashMap<String, ItemBoard> = serde_json::from_str(ITEM_BOARDS_JSON).unwrap();
    pub static ref ITEM_EFFECT_THRESHOLDS: HashMap<String, Vec<Vec<ItemEffectThreshold>>> = serde_json::from_str(ITEM_EFFECT_THRESHOLDS_JSON).unwrap();
    pub static ref ITEM_EFFECTS: Vec<ItemEffect> = serde_json::from_str(ITEM_EFFECTS_JSON).unwrap();
    pub static ref ITEMS: Vec<Item> = serde_json::from_str(ITEMS_JSON).unwrap();
    pub static ref SHAPES: HashMap<String, Vec<usize>> = serde_json::from_str(SHAPES_JSON).unwrap();
}

#[derive(serde::Deserialize)]
pub struct Ingredient {
    pub ingredient: String,
    pub count: usize,
}

#[derive(serde::Deserialize)]
pub struct Item {
    pub name: String,
    pub tag: String,
    pub shape_type: String,
    pub use_type: String,
    pub color: String,
    pub categories: Vec<String>,
}

#[derive(serde::Deserialize)]
pub struct ItemBoard {
    pub colors: [String; 6],
    pub bonus_levels: [[String; 6]; 3],
}

#[derive(serde::Deserialize)]
pub struct ItemEffectThreshold {
    pub item_effect_tag: String,
    pub threshold: u32,
}

#[derive(serde::Deserialize)]
pub struct ItemEffect {
    pub name: String,
    pub tag: String,
    /// Hit sound effect
    pub hit_se: Option<String>,
    pub group_tag: String,
    pub actions: [EffectAction; 2],
}
#[derive(serde::Deserialize)]
pub struct EffectAction {
    /// The tag, or `ACT_NONE` for none.
    pub act_tag: String,
    /// The damage attribute, or `ATT_NONE` for none.
    pub attribute_tag: String,
    /// Minimum parameters for this effect.
    pub min: [Option<String>; 2],
    /// Maximum parameters for this effect.
    pub max: [Option<String>; 2],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ensure_json_deserializes_correctly() {
        let _ = &*INGREDIENTS;
        let _ = &*ITEM_BOARDS;
        let _ = &*ITEM_EFFECT_THRESHOLDS;
        let _ = &*ITEMS;
        let _ = &*ITEM_BOARDS;
        let _ = &*SHAPES;
    }
}
