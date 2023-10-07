use egui::RichText;

use synth_solver::{
    solver::{GoalResult, Move},
    Cauldron,
};

use crate::components::{CauldronComponent, InputComponent, SolverSettingsComponent};
use crate::util::synth_color_to_egui_color;

pub struct App {
    cauldron: CauldronComponent,
    input: InputComponent,
    settings: SolverSettingsComponent,
    results: Option<Vec<(GoalResult, synth_solver::tinyvec::ArrayVec<[Move; 20]>)>>,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_pixels_per_point(1.5);
        Self {
            cauldron: CauldronComponent::default(),
            input: Default::default(),
            settings: Default::default(),
            results: None,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("left panel").show(ctx, |ui| {
            ui.add_enabled_ui(self.results.is_none(), |ui| {
                self.input.render(ui);
                ui.add_space(16.);
                self.settings.render(ui);
                ui.add_space(16.);
            });

            if let Err(err) = self.input.validate() {
                ui.label("Input error");
                ui.label(err);
            } else {
                ui.add_enabled_ui(self.results.is_none(), |ui| {
                    if ui.button("Run solver").clicked() {
                        let found_routes = synth_solver::solver::find_optimal_routes(
                            &self.cauldron,
                            &self.input.materials,
                            &self.input.goals,
                            &self.settings.props,
                        );
                        self.results = Some(found_routes);
                    }
                });
                ui.add_enabled_ui(self.results.is_some(), |ui| {
                    if ui.button("Clear results").clicked() {
                        self.results = None;
                    }
                });
            }
        });

        egui::SidePanel::right("right panel").show(ctx, |ui| {
            ui.heading("Results");
            if let Some(routes) = &self.results {
                for (goal_result, route) in routes {
                    egui::CollapsingHeader::new(format!("Score: {:?}", goal_result.scores)).show(
                        ui,
                        |ui| {
                            // render move list
                            render_move_list(ui, &self.cauldron, route);

                            // calculate the playfield after these moves
                            let mut playfield = self.cauldron.clone();
                            let res = playfield.place_all(
                                &self.input.materials,
                                route,
                                self.settings.props.allow_overlaps,
                            );

                            match res {
                                Ok(scores) => scores,
                                Err(e) => {
                                    ui.label(format!("Error: {e:?}"));
                                    return;
                                }
                            };

                            // render playfield
                            render_playfield(ui, &playfield);
                        },
                    );
                }
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_enabled_ui(self.results.is_none(), |ui| {
                self.cauldron.render(ui);
            });
        });
    }
}

fn render_move_list(ui: &mut egui::Ui, cauldron: &Cauldron, route: &[Move]) {
    for move_ in route {
        let (x, y) = cauldron.get_position(move_.placement.index);
        if let Some(transformation) = move_.placement.transformation {
            ui.label(format!(
                "\t- Material {}-{} at {x},{y} with {:?}",
                move_.material_index.0, move_.material_index.1, transformation
            ));
        } else {
            ui.label(format!(
                "\t- Material {}-{} at {x},{y}",
                move_.material_index.0, move_.material_index.1,
            ));
        }
    }
}

fn render_playfield(ui: &mut egui::Ui, playfield: &Cauldron) {
    for row in 0..playfield.size {
        ui.horizontal(|ui| {
            for col in 0..playfield.size {
                let tile = playfield.get_tile((row, col));
                let Some(tile) = tile else {
                    ui.label(RichText::new(" ").monospace());
                    continue;
                };
                let Some(color) = tile.played_color else {
                    ui.label(RichText::new(".").monospace());
                    continue;
                };
                let color = synth_color_to_egui_color(color);
                let text = RichText::new("x").color(color).monospace();
                ui.label(text);
            }
        });
    }
}
