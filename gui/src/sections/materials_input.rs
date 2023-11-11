use egui::RichText;

use crate::components::color_button_group;

#[derive(Default)]
pub struct MaterialsInputSection {
    pub target_item: &'static str,
    pub item_groups: Vec<IngredientGroup>,
}

impl MaterialsInputSection {
    pub fn render(&mut self, ui: &mut egui::Ui, target_item: &'static str) {
        if target_item.is_empty() {
            eprintln!("target item in materials input is empty!");
            return;
        }
        if self.target_item != target_item {
            self.target_item = target_item;

            // reset item groups
            self.item_groups = atelier_sophie_data::INGREDIENTS
                .get(target_item)
                .expect("find target item")
                .iter()
                .map(|ingredient| IngredientGroup {
                    ingredient_tag: ingredient.ingredient.as_str(),
                    materials: vec![IndividualMaterialState::default(); ingredient.count],
                })
                .collect();
        }
        debug_assert_ne!(self.target_item, "");

        ui.heading("Materials");
        for (group_idx, group) in self.item_groups.iter_mut().enumerate() {
            ui.push_id(group_idx, |ui| {
                ui.group(|ui| {
                    ui.label(RichText::new(group.ingredient_tag).monospace());

                    for (material_idx, material) in group.materials.iter_mut().enumerate() {
                        if material_idx > 0 {
                            ui.separator();
                        }

                        ui.push_id(material_idx, |ui| {
                            // item selection. pre-fill with item if it's not a category
                            if !group.ingredient_tag.starts_with("ITEM_CAT") {
                                material.item_tag = Some(group.ingredient_tag);
                            }
                            egui::ComboBox::from_id_source("item")
                                .selected_text(
                                    material
                                        .item_tag
                                        .and_then(item_tag_to_name)
                                        .unwrap_or("<None>"),
                                )
                                .show_ui(ui, |ui| {
                                    for item in atelier_sophie_data::ITEMS.iter().filter(|item| {
                                        item.tag == group.ingredient_tag
                                            || item
                                                .categories
                                                .iter()
                                                .any(|c| c == group.ingredient_tag)
                                    }) {
                                        ui.selectable_value(
                                            &mut material.item_tag,
                                            Some(item.tag.as_str()),
                                            item.name.as_str(),
                                        );
                                    }
                                });

                            // effect value
                            ui.horizontal(|ui| {
                                ui.label("Effect value:");
                                ui.add(
                                    egui::DragValue::new(&mut material.effect_value)
                                        .clamp_range(0..=150),
                                )
                            });

                            if let Some(item) = atelier_sophie_data::ITEMS
                                .iter()
                                .find(|item| Some(item.tag.as_str()) == material.item_tag)
                            {
                                // color override
                                ui.horizontal(|ui| {
                                    ui.label("Color override:");
                                    let mut enabled = material.color_override.is_some();
                                    if ui.checkbox(&mut enabled, "").changed() {
                                        if enabled {
                                            material.color_override =
                                                Some(synth_solver::Color::Red);
                                        } else {
                                            material.color_override = None;
                                        }
                                    }
                                });
                                if let Some(color) = &mut material.color_override {
                                    color_button_group(ui, color);
                                }

                                // shape
                                render_input_shape_index(
                                    ui,
                                    &item.shape_type,
                                    &mut material.shape_size,
                                );
                            }
                        });
                    }
                })
            });
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        for group in self.item_groups.iter() {
            for material in group.materials.iter() {
                if material.item_tag.is_none() {
                    return Err(format!(
                        "Material group {} has an empty item",
                        group.ingredient_tag
                    ));
                }
            }
        }

        Ok(())
    }
}

fn render_input_shape_index(ui: &mut egui::Ui, shape_tag: &str, size: &mut usize) {
    // Size up/down
    ui.horizontal(|ui| {
        ui.label("Size:");

        if ui
            .add_enabled_ui(*size > 1, |ui| ui.button("<"))
            .inner
            .clicked()
        {
            *size -= 1;
        }
        ui.label(format!("{}", *size));
        if ui
            .add_enabled_ui(*size < 9, |ui| ui.button(">"))
            .inner
            .clicked()
        {
            *size += 1;
        }
    });

    let shape = atelier_sophie_data::SHAPES
        .get(shape_tag)
        .expect("find shape");
    let shape = synth_solver::Shape::from_indices(shape.iter().take(*size).cloned()).normalize();
    let shape = shape.to_matrix();

    // Shape view
    ui.horizontal(|ui| {
        ui.label("Shape");

        let old_spacing = ui.style().spacing.item_spacing;
        ui.style_mut().spacing.item_spacing = egui::Vec2::ZERO;
        ui.vertical(|ui| {
            for row in shape.iter() {
                ui.horizontal(|ui| {
                    for cell in row.iter() {
                        ui.add_enabled(false, egui::Checkbox::new(&mut (cell.clone()), ""));
                    }
                });
            }
        });
        ui.style_mut().spacing.item_spacing = old_spacing;
    });
}

pub struct IngredientGroup {
    /// Either a category tag or an item tag
    pub ingredient_tag: &'static str,
    pub materials: Vec<IndividualMaterialState>,
}

#[derive(Clone)]
pub struct IndividualMaterialState {
    pub item_tag: Option<&'static str>,
    pub shape_size: usize,
    pub color_override: Option<synth_solver::Color>,
    pub effect_value: u32,
}

impl Default for IndividualMaterialState {
    fn default() -> Self {
        Self {
            item_tag: None,
            shape_size: 3,
            color_override: None,
            effect_value: 10,
        }
    }
}

fn item_tag_to_name(tag: &str) -> Option<&'static str> {
    atelier_sophie_data::ITEMS
        .iter()
        .find(|item| item.tag == tag)
        .map(|item| item.name.as_str())
}
