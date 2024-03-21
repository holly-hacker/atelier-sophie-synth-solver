#![warn(
    clippy::cloned_instead_of_copied,
    clippy::explicit_iter_loop,
    clippy::return_self_not_must_use,
    clippy::trivially_copy_pass_by_ref,
    clippy::uninlined_format_args,
    clippy::use_self
)]
#![allow(clippy::wildcard_imports)]

mod app;
mod components;
pub mod sections;
mod util;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_min_inner_size([300.0, 220.0])
            .with_inner_size([640.0 * 1.5, 480.0 * 1.5]),
        ..Default::default()
    };
    eframe::run_native(
        "Atelier Sophie: Synthesis Solver",
        native_options,
        Box::new(|cc| Box::new(app::App::new(cc))),
    )
}
