use egui::{Button, FontSelection, Rounding, Vec2};

use synth_solver::Color;

use crate::{
    sections::{GoalsInputSection, MaterialInputSection},
    util::synth_color_to_egui_color,
};

#[derive(Default)]
pub struct InputComponent {
    pub materials_input: MaterialInputSection,
    pub goals_input: GoalsInputSection,
}

impl InputComponent {
    pub fn render(&mut self, ui: &mut egui::Ui) {
        self.materials_input.render(ui);
        ui.add_space(16.);
        self.goals_input.render(ui);
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.materials_input.materials.is_empty() || self.goals_input.goals.is_empty() {
            return Err("No materials or goals specified".to_string());
        }

        if self.materials_input.materials.len() != self.goals_input.goals.len() {
            return Err("Number of material groups and goals must match".to_string());
        }

        // each goal must have ascending thresholds
        for (goal_idx, goal) in self.goals_input.goals.iter().enumerate() {
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
