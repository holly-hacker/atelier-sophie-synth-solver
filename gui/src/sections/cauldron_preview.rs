use eframe::epaint::Rounding;
use egui::{Button, Color32, FontSelection, RichText, Vec2};

use synth_solver::{Cauldron, Color, Tile};

use crate::util::synth_color_to_egui_color;

pub struct CauldronPreview {
    pub cauldron: Cauldron,
}

impl CauldronPreview {
    pub fn render(&mut self, ui: &mut egui::Ui) {
        // render playfield itself
        // rendering by row to prevent some layout issues
        ui.horizontal(|ui| {
            for x in 0..self.cauldron.size {
                ui.vertical(|ui| {
                    for y in 0..self.cauldron.size {
                        let tile = self.cauldron.get_tile_mut((x, y));
                        cauldron_tile(ui, tile);
                    }
                });
            }
        });
    }
}

fn cauldron_tile(ui: &mut egui::Ui, tile: &mut Option<Tile>) {
    let current_color = tile
        .as_ref()
        .map(|t| t.color)
        .map(synth_color_to_egui_color)
        .unwrap_or(egui::Color32::TRANSPARENT);
    // TODO: better dimming
    let current_color_dim = current_color.linear_multiply(0.3);
    egui::Frame::group(ui.style())
        .fill(current_color_dim)
        .show(ui, |ui| {
            let font_id = FontSelection::Default.resolve(ui.style());
            let font_height = ui.fonts(|fonts| fonts.row_height(&font_id));
            let item_spacing = ui.spacing().item_spacing.max_elem();

            ui.allocate_ui(Vec2::splat(font_height * 3. + item_spacing * 2.), |ui| {
                ui.vertical_centered(|ui| {
                    // top line: tile level
                    ui.horizontal_top(|ui| {
                        if let Some(tile) = tile {
                            ui.add_enabled_ui(tile.level > 0, |ui| {
                                if ui.button("<").clicked() {
                                    tile.level -= 1;
                                }
                            });
                            ui.label(
                                RichText::new(format!("{}", tile.level))
                                    .strong()
                                    .color(Color32::BLACK),
                            );
                            ui.add_enabled_ui(tile.level < 3, |ui| {
                                if ui.button(">").clicked() {
                                    tile.level += 1;
                                }
                            });
                        } else {
                            // HACK: we just need to reserve some space
                            ui.label(" ");
                        }
                    });

                    // bottom 2 lines:
                    ui.vertical_centered(|ui| {
                        for color_group in [
                            [Some(Color::Red), Some(Color::Blue), Some(Color::Green)].as_slice(),
                            [Some(Color::Yellow), Some(Color::White), None].as_slice(),
                        ] {
                            ui.horizontal_centered(|ui| {
                                for input_color in color_group {
                                    // some hackery to get a round button without manual padding
                                    let font_id = FontSelection::Default.resolve(ui.style());
                                    let size =
                                        Vec2::splat(ui.fonts(|fonts| fonts.row_height(&font_id)))
                                            + ui.spacing().button_padding;
                                    let text = input_color.map_or("x", |_| "");
                                    let button = Button::new(text)
                                        .fill(
                                            input_color
                                                .map(synth_color_to_egui_color)
                                                .unwrap_or(egui::Color32::TRANSPARENT),
                                        )
                                        .min_size(size);

                                    let button = if tile.as_ref().map(|t| t.color) == *input_color {
                                        button.selected(true).rounding(Rounding::ZERO)
                                    } else {
                                        button.selected(false).rounding(Rounding::same(999.))
                                    };

                                    if ui.add(button).clicked() {
                                        if let Some(color) = input_color {
                                            if let Some(tile) = tile {
                                                tile.color = *color;
                                            } else {
                                                *tile = Some(Tile {
                                                    color: *color,
                                                    level: 0,
                                                    played_material_index: None,
                                                });
                                            }
                                        } else {
                                            *tile = None;
                                        }
                                    }
                                }
                            });
                        }
                    });
                })
            });
        });
}
