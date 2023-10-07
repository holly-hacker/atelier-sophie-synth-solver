mod app;
mod components;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        initial_window_size: Some([640.0 * 1.5, 480.0 * 1.5].into()),
        min_window_size: Some([300.0, 220.0].into()),
        ..Default::default()
    };
    eframe::run_native(
        "Atelier Sophie: Synthesis Solver",
        native_options,
        Box::new(|cc| Box::new(app::App::new(cc))),
    )
}
