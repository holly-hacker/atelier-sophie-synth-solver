use egui::{Button, FontSelection, Rounding, Vec2};

use synth_solver::Color;

use crate::util::synth_color_to_egui_color;

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
