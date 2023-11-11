use egui::RichText;
use synth_solver::{Color, Material};

use crate::components::{color_button_group, input_shape};

pub struct MaterialInputSection {
    pub materials: Vec<Vec<Material>>,
}

impl MaterialInputSection {
    pub fn render(&mut self, ui: &mut egui::Ui) {
        ui.heading("Materials");

        let mut to_remove = vec![];
        for (group_idx, material_group) in self.materials.iter_mut().enumerate() {
            ui.push_id(group_idx, |ui| {
                ui.group(|ui| {
                    ui.label(RichText::new(format!("Material Group {}", group_idx + 1)));
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
            self.materials.push(vec![default_material()]);
        }

        for group_idx in to_remove {
            self.materials.remove(group_idx);
        }
    }
}

impl Default for MaterialInputSection {
    fn default() -> Self {
        Self {
            materials: vec![vec![default_material()]],
        }
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

fn default_material() -> Material {
    Material {
        color: Color::Red,
        effect_value: 10,
        shape: synth_solver::Shape::from_binary([0b110, 0b100, 0b000]),
    }
}
