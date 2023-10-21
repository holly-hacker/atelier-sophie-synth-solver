use egui::{Button, FontSelection, RichText, Rounding, Vec2};

use synth_solver::{Color, Goal, Material};

use crate::util::synth_color_to_egui_color;

use super::input_shape;

pub struct InputComponent {
    pub materials: Vec<Vec<Material>>,
    pub goals: Vec<Goal>,
}

impl InputComponent {
    pub fn render(&mut self, ui: &mut egui::Ui) {
        ui.heading("Materials");
        materials_list_input(ui, &mut self.materials);
        ui.add_space(16.);

        ui.heading("Goals");
        goals_input(ui, &mut self.goals);
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.materials.is_empty() || self.goals.is_empty() {
            return Err("No materials or goals specified".to_string());
        }

        if self.materials.len() != self.goals.len() {
            return Err("Number of material groups and goals must match".to_string());
        }

        // each goal must have ascending thresholds
        for (goal_idx, goal) in self.goals.iter().enumerate() {
            let mut last_threshold = None;
            for threshold in goal.effect_value_thresholds.iter() {
                if let Some(last_threshold) = last_threshold.as_mut() {
                    if *threshold <= *last_threshold {
                        return Err(format!(
                            "Goal thresholds for goal {goal_idx} must be ascending"
                        ));
                    }
                }
                last_threshold = Some(*threshold);
            }
        }

        Ok(())
    }
}

impl Default for InputComponent {
    fn default() -> Self {
        Self {
            materials: vec![vec![default_material()]],
            goals: vec![default_goal()],
        }
    }
}

fn materials_list_input(ui: &mut egui::Ui, materials: &mut Vec<Vec<Material>>) {
    let mut to_remove = vec![];
    for (group_idx, material_group) in materials.iter_mut().enumerate() {
        ui.push_id(group_idx, |ui| {
            ui.group(|ui| {
                ui.label(RichText::new(format!("Material Group {}", group_idx + 1)).raised());
                materials_group_input(ui, material_group);
                ui.horizontal(|ui| {
                    if ui.button("Add Material").clicked() {
                        material_group.push(default_material());
                    }
                    if ui.button("Remove Group").clicked() {
                        to_remove.push(group_idx);
                    }
                });
            })
        });
    }
    if ui.button("Add Group").clicked() {
        materials.push(vec![default_material()]);
    }

    for group_idx in to_remove {
        materials.remove(group_idx);
    }
}

fn materials_group_input(ui: &mut egui::Ui, materials: &mut Vec<Material>) {
    let mut to_remove = vec![];
    for (mat_idx, material) in materials.iter_mut().enumerate() {
        ui.push_id(mat_idx, |ui| {
            ui.horizontal(|ui| {
                color_button_group(ui, &mut material.color);
                ui.add(
                    egui::DragValue::new(&mut material.effect_value)
                        .speed(1.0)
                        .clamp_range(0..=999),
                );

                if ui.button("X").clicked() {
                    to_remove.push(mat_idx);
                }
            });
            input_shape(ui, &mut material.shape);
        });
    }

    for mat_idx in to_remove {
        materials.remove(mat_idx);
    }
}

fn goals_input(ui: &mut egui::Ui, goals: &mut Vec<Goal>) {
    let mut to_remove = vec![];
    for (goal_idx, goal) in goals.iter_mut().enumerate() {
        ui.push_id(goal_idx, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new(format!("\t- Goal {}:", goal_idx + 1)));

                for (threshold_idx, threshold) in
                    goal.effect_value_thresholds.iter_mut().enumerate()
                {
                    ui.push_id(threshold_idx, |ui| {
                        ui.add(
                            egui::DragValue::new(threshold)
                                .speed(1.0)
                                .clamp_range(0..=999),
                        );
                    });
                }

                if ui.button("+").clicked() {
                    let new_val = goal
                        .effect_value_thresholds
                        .iter()
                        .max()
                        .copied()
                        .unwrap_or(0)
                        + 1;
                    goal.effect_value_thresholds.push(new_val);
                }

                if ui.button("-").clicked() {
                    goal.effect_value_thresholds.pop();

                    if goal.effect_value_thresholds.is_empty() {
                        to_remove.push(goal_idx);
                    }
                }
            });
        });
    }

    if ui.button("Add Goal").clicked() {
        goals.push(default_goal());
    }

    debug_assert!(to_remove.len() <= 1);
    for goal_idx in to_remove {
        goals.remove(goal_idx);
    }
}

pub fn color_button_group(ui: &mut egui::Ui, input_color: &mut Color) {
    ui.horizontal(|ui| {
        for color in [
            Color::Red,
            Color::Blue,
            Color::Green,
            Color::Yellow,
            Color::White,
        ] {
            // some hackery to get a round button without manual padding
            let font_id = FontSelection::Default.resolve(ui.style());
            let size = Vec2::splat(ui.fonts(|fonts| fonts.row_height(&font_id)))
                + ui.spacing().button_padding;
            let button = Button::new("")
                .fill(synth_color_to_egui_color(color))
                .min_size(size);

            let button = if color == *input_color {
                button.selected(true).rounding(Rounding::ZERO)
            } else {
                button.selected(false).rounding(Rounding::same(999.))
            };

            if ui.add(button).clicked() {
                *input_color = color;
            }
        }
    });
}

fn default_material() -> Material {
    Material {
        color: Color::Red,
        effect_value: 10,
        shape: synth_solver::Shape::from_binary([0b110, 0b100, 0b000]),
    }
}

fn default_goal() -> Goal {
    Goal {
        effect_value_thresholds: vec![30],
    }
}
