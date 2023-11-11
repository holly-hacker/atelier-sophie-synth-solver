use std::sync::{
    atomic::{AtomicBool, AtomicU32},
    Arc, RwLock,
};

use egui::RichText;

use synth_solver::{
    solver::{GoalResult, Move, SolverResult},
    Cauldron, Material,
};

use crate::{
    sections::CauldronInputSection,
    util::{create_goals, create_materials, create_solver_settings, synth_color_to_egui_color},
};
use crate::{sections::*, util::create_synth_cauldron};

struct AtomicF32(AtomicU32);

impl AtomicF32 {
    fn new(val: f32) -> Self {
        Self(AtomicU32::new(val.to_bits()))
    }

    fn get(&self) -> f32 {
        f32::from_bits(self.0.load(std::sync::atomic::Ordering::Relaxed))
    }

    fn set(&self, val: f32) {
        self.0
            .store(val.to_bits(), std::sync::atomic::Ordering::Relaxed);
    }
}

struct PendingSearch {
    results_receiver: oneshot::Receiver<SolverResult>,
    cancelled: Arc<AtomicBool>,
    current_progress: Arc<AtomicF32>,
}

pub struct App {
    // inputs
    cauldron_input: CauldronInputSection,
    item_input: TargetItemInputSection,
    materials_input: MaterialsInputSection,

    results: Arc<RwLock<Option<SolverResult>>>,
    pending_search: Option<PendingSearch>,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_pixels_per_point(1.5);
        Self {
            cauldron_input: CauldronInputSection::default(),
            item_input: TargetItemInputSection::default(),
            materials_input: MaterialsInputSection::default(),

            results: Arc::new(RwLock::new(None)),
            pending_search: None,
        }
    }

    fn run_solver(&mut self, ctx: egui::Context) {
        let allow_overlaps = true; // TODO: make this configurable

        let cauldron = create_synth_cauldron(&self.cauldron_input, &self.item_input);
        let materials = create_materials(&self.materials_input);
        let goals = create_goals(&self.item_input);
        let settings = create_solver_settings(&self.cauldron_input, allow_overlaps);

        let (results_send, results_recv) = oneshot::channel();
        let cancelled = Arc::new(AtomicBool::new(false));
        let progress_val = Arc::new(AtomicF32::new(0.));
        let results = self.results.clone();

        self.pending_search = Some(PendingSearch {
            results_receiver: results_recv,
            cancelled: cancelled.clone(),
            current_progress: progress_val.clone(),
        });

        std::thread::spawn(move || {
            let found_routes = synth_solver::solver::find_optimal_routes(
                &cauldron,
                &materials,
                &goals,
                &settings,
                Some(Box::new(move |progress, temp_results| {
                    progress_val.set(progress);
                    *results.write().unwrap() = Some(temp_results);

                    if cancelled.load(std::sync::atomic::Ordering::Relaxed) {
                        return std::ops::ControlFlow::Break(());
                    }

                    // for now, don't stop the search
                    std::ops::ControlFlow::Continue(())
                })),
            );
            println!("Found {} routes", found_routes.len());
            results_send.send(found_routes).unwrap();
            ctx.request_repaint();
        });
    }

    fn render_route(&self, ui: &mut egui::Ui, goal_result: &GoalResult, route: &[Move]) {
        // calculate the playfield after these moves
        let allow_overlaps = true; // TODO derive from UI?

        let mut cauldron = create_synth_cauldron(&self.cauldron_input, &self.item_input);
        let materials = create_materials(&self.materials_input);

        let res = cauldron.place_all(&materials, route, allow_overlaps);

        let scores = match res {
            Ok(scores) => cauldron.calculate_final_score(&materials, &scores),
            Err(e) => {
                ui.label(format!("Error: {e:?}"));
                return;
            }
        };

        egui::CollapsingHeader::new(format!(
            "Goals: {:?}, score: {:?}",
            goal_result.achieved_goals, scores
        ))
        .show(ui, |ui| {
            // render move list
            render_move_list(ui, &cauldron, route);

            // render playfield
            render_playfield(ui, &cauldron, &materials);
        });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(pending_search) = &self.pending_search {
            if let Ok(results) = pending_search.results_receiver.try_recv() {
                *self.results.write().unwrap() = Some(results);
                self.pending_search = None;
            };
        }

        let results_pending = self.pending_search.is_some();
        let results_available = self.results.read().unwrap().is_some();
        let can_edit_input = !results_pending && !results_available;

        egui::SidePanel::left("left panel").show(ctx, |ui| {
            ui.label(format!("zoom: {}x", ctx.pixels_per_point()));
            ui.horizontal(|ui| {
                if ui.button("1.0x").clicked() {
                    ctx.set_pixels_per_point(1.0);
                }
                if ui.button("1.5x").clicked() {
                    ctx.set_pixels_per_point(1.5);
                }
                if ui.button("2.0x").clicked() {
                    ctx.set_pixels_per_point(2.0);
                }
            });
            ui.add_space(8.);

            ui.add_enabled_ui(can_edit_input, |ui| {
                self.item_input.render(ui);
                ui.add_space(16.);
                self.materials_input
                    .render(ui, self.item_input.target_item_tag);
                ui.add_space(16.);
            });

            if let Err(err) = self
                .item_input
                .validate()
                .and_then(|()| self.materials_input.validate())
            {
                ui.label("Input error");
                ui.label(err);
            } else {
                ui.add_enabled_ui(can_edit_input, |ui| {
                    if ui.button("Run solver").clicked() {
                        self.run_solver(ctx.clone());
                    }
                });

                ui.add_enabled_ui(results_available, |ui| {
                    if ui.button("Clear results").clicked() {
                        *self.results.write().unwrap() = None;
                    }
                });

                if let Some(pending_search) = &self.pending_search {
                    if ui.button("Cancel").clicked() {
                        pending_search
                            .cancelled
                            .store(true, std::sync::atomic::Ordering::Relaxed);
                    }

                    let progress = pending_search.current_progress.get();
                    ui.add(
                        egui::widgets::ProgressBar::new(progress)
                            .animate(true)
                            .show_percentage(),
                    );
                    if progress > 1.1 {
                        ui.label(format!(
                            "Warning: progress is greater than 100%: {progress}"
                        ));
                    }
                    ui.ctx().request_repaint();
                }
            }
        });

        egui::SidePanel::right("right panel").show(ctx, |ui| {
            ui.heading("Results");
            if let Some(routes) = self.results.read().unwrap().as_ref() {
                for (goal_result, route) in routes {
                    self.render_route(ui, goal_result, route);
                }
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_enabled_ui(can_edit_input, |ui| {
                self.cauldron_input.render(ui);
                ui.separator();

                let cauldron = create_synth_cauldron(&self.cauldron_input, &self.item_input);
                CauldronPreview { cauldron }.render(ui);
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

fn render_playfield(ui: &mut egui::Ui, playfield: &Cauldron, materials: &[Vec<Material>]) {
    for y in 0..playfield.size {
        ui.horizontal(|ui| {
            for x in 0..playfield.size {
                let tile = playfield.get_tile((x, y));
                let Some(tile) = tile else {
                    ui.label(RichText::new(" ").monospace());
                    continue;
                };
                let Some(material_index) = tile.played_material_index else {
                    ui.label(RichText::new(".").monospace());
                    continue;
                };
                let color = materials[material_index.0][material_index.1].color;
                let color = synth_color_to_egui_color(color);
                let text = RichText::new("x").color(color).monospace();
                ui.label(text);
            }
        });
    }
}
