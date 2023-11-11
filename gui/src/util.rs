use synth_solver::tinyvec::ArrayVec;

use crate::sections::{CauldronInputSection, MaterialsInputSection, TargetItemInputSection};

pub fn synth_color_to_egui_color(color: synth_solver::Color) -> egui::Color32 {
    use synth_solver::Color;

    match color {
        Color::Red => egui::Color32::from_rgb(255, 0, 0),
        Color::Blue => egui::Color32::from_rgb(0, 0, 255),
        Color::Green => egui::Color32::from_rgb(0, 255, 0),
        Color::Yellow => egui::Color32::from_rgb(255, 255, 0),
        Color::White => egui::Color32::from_rgb(255, 255, 255),
    }
}

pub fn create_synth_cauldron(
    cauldron_input: &CauldronInputSection,
    item_input: &TargetItemInputSection,
) -> synth_solver::Cauldron {
    // TODO: calculate properties from cauldron input
    let bonus_level = 0;
    let properties = synth_solver::CauldronProperties::default();
    let bonus_scores = (3, 5, 7);

    synth_solver::Cauldron {
        size: cauldron_input.size,
        tiles: create_tiles(item_input.target_item_tag, cauldron_input.size, bonus_level),
        bonus_scores,
        color: synth_solver::Color::from_mat_color_tag(
            &atelier_sophie_data::ITEMS
                .iter()
                .find(|item| item.tag == item_input.target_item_tag)
                .unwrap()
                .color,
        ),
        properties,
    }
}

pub fn create_materials(
    materials_input: &MaterialsInputSection,
) -> Vec<Vec<synth_solver::Material>> {
    materials_input
        .item_groups
        .iter()
        .map(|group| {
            group
                .materials
                .iter()
                .map(|material| {
                    let item = atelier_sophie_data::ITEMS
                        .iter()
                        .find(|item| item.tag == material.item_tag.unwrap())
                        .unwrap();

                    synth_solver::Material {
                        color: material.color_override.unwrap_or_else(|| {
                            synth_solver::Color::from_mat_color_tag(&item.color)
                        }),
                        effect_value: material.effect_value,
                        shape: create_shape(&item.shape_type, material.shape_size),
                    }
                })
                .collect()
        })
        .collect()
}

pub fn create_goals(item_input: &TargetItemInputSection) -> Vec<synth_solver::Goal> {
    atelier_sophie_data::ITEM_EFFECT_THRESHOLDS
        .get(item_input.target_item_tag)
        .unwrap()
        .iter()
        .map(|thresholds_group| synth_solver::Goal {
            effect_value_thresholds: thresholds_group
                .iter()
                .map(|threshold| threshold.threshold)
                .collect(),
        })
        .collect()
}

pub fn create_solver_settings(
    _cauldron_input: &CauldronInputSection,
    allow_overlaps: bool,
) -> synth_solver::solver::SolverSettings {
    let transformations = synth_solver::TransformationType::Rotate; // TODO: derive from traits

    synth_solver::solver::SolverSettings {
        transformations,
        allow_overlaps,
    }
}

fn create_tiles(
    item: &str,
    size: usize,
    bonus_level: usize,
) -> ArrayVec<[Option<synth_solver::Tile>; 6 * 6]> {
    let board = atelier_sophie_data::ITEM_BOARDS.get(item).unwrap();

    // clippy lint for false positive, see https://github.com/rust-lang/rust-clippy/issues/11761
    #[allow(clippy::iter_skip_zero)]
    board
        .colors
        .iter()
        .zip(board.bonus_levels[bonus_level].iter())
        .enumerate()
        .flat_map(|(y, (row_col, row_bonus))| match (size, y) {
            // with size 4, take the middle 4x4 slots
            (4, 1..=4) => Iterator::zip(
                row_col.chars().skip(1).take(4),
                row_bonus.chars().skip(1).take(4),
            ),
            // with size 5, take the 5x5 slots to the top left
            (5, 0..=4) => Iterator::zip(
                row_col.chars().skip(0).take(5),
                row_bonus.chars().skip(0).take(5),
            ),
            // with size 6, take everything
            (6, _) => Iterator::zip(
                row_col.chars().skip(0).take(6),
                row_bonus.chars().skip(0).take(6),
            ),
            // cases for size 4 or 5 where no items are taken
            (4, 0 | 5) | (5, 5) => Iterator::zip(
                row_col.chars().skip(0).take(0),
                row_bonus.chars().skip(0).take(0),
            ),
            (..=3 | 6..=usize::MAX, _) => panic!("Invalid size {} for item {}", size, item),
            (_, _) => panic!("Invalid y {} for item {}", y, item),
        })
        .map(|(color, bonus)| {
            if color != ' ' {
                Some(synth_solver::Tile {
                    color: match color {
                        'R' => synth_solver::Color::Red,
                        'B' => synth_solver::Color::Blue,
                        'G' => synth_solver::Color::Green,
                        'Y' => synth_solver::Color::Yellow,
                        'W' => synth_solver::Color::White,
                        ' ' => unreachable!(),
                        c => panic!("Invalid char {c}"),
                    },
                    level: match bonus {
                        ' ' => 0,
                        '1' => 1,
                        '2' => 2,
                        '3' => 3,
                        c => panic!("Invalid char {c}"),
                    },
                    played_material_index: None,
                })
            } else {
                None
            }
        })
        .collect()
}

fn create_shape(shape_tag: &str, size: usize) -> synth_solver::Shape {
    let shape = atelier_sophie_data::SHAPES.get(shape_tag).unwrap();
    synth_solver::Shape::from_indices(shape.iter().take(size).cloned())
}
