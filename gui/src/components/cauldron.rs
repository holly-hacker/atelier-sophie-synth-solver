use std::ops::Deref;

use egui::RichText;

use crate::util::synth_color_to_egui_color;
use synth_solver::Cauldron;

pub struct CauldronComponent {
    cauldron: Cauldron,
}

impl Default for CauldronComponent {
    fn default() -> Self {
        Self {
            cauldron: synth_solver::utils::test_data::cauldron::uni_bag_5x5_bonus1(),
        }
    }
}

impl Deref for CauldronComponent {
    type Target = Cauldron;

    fn deref(&self) -> &Self::Target {
        &self.cauldron
    }
}

impl CauldronComponent {
    pub fn render(&mut self, ui: &mut egui::Ui) {
        ui.heading("Cauldron");

        // TODO: edit controls

        // render playfield itself
        ui.horizontal(|ui| {
            for row in 0..self.size {
                ui.vertical(|ui| {
                    for col in 0..self.size {
                        let tile = self.get_tile((row, col));
                        let Some(tile) = tile else {
                            ui.label(RichText::new(" ").monospace());
                            continue;
                        };
                        let color = synth_color_to_egui_color(tile.color);
                        let text = RichText::new(format!("{}", tile.level))
                            .color(color)
                            .monospace();

                        ui.label(text);
                    }
                });
            }
        });
    }
}
