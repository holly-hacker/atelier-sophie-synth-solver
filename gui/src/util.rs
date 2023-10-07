pub fn synth_color_to_egui_color(color: synth_solver::Color) -> egui::Color32 {
    use synth_solver::Color;

    match color {
        Color::Red => egui::Color32::from_rgb(255, 0, 0),
        Color::Blue => egui::Color32::from_rgb(0, 0, 255),
        Color::Green => egui::Color32::from_rgb(0, 255, 0),
        Color::Yellow => egui::Color32::from_rgb(255, 255, 0),
        Color::White => egui::Color32::from_rgb(255, 255, 255),
    }
}
