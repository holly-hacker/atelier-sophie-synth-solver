use synth_brute::find_optimal::SearchProperties;

#[derive(Default)]
pub struct SearchSettings {
    pub props: SearchProperties,
}

impl SearchSettings {
    pub fn render(&mut self, ui: &mut egui::Ui) {
        ui.heading("Settings");
        ui.group(|ui| {
            transformations_combobox(ui, &mut self.props.transformations);

            ui.add_enabled_ui(false, |ui| {
                ui.checkbox(&mut self.props.allow_overlaps, "Allow overlaps");
            });
        });
    }
}

fn transformations_combobox(
    ui: &mut egui::Ui,
    transformation: &mut synth_brute::TransformationType,
) {
    egui::ComboBox::from_label("Transformation")
        .selected_text(format!("{:?}", transformation))
        .show_ui(ui, |ui| {
            ui.selectable_value(
                transformation,
                synth_brute::TransformationType::None,
                "None",
            );
            ui.selectable_value(
                transformation,
                synth_brute::TransformationType::FlipHorizontal,
                "Flip H",
            );
            ui.selectable_value(
                transformation,
                synth_brute::TransformationType::FlipVertical,
                "Flip V",
            );
            ui.selectable_value(
                transformation,
                synth_brute::TransformationType::Rotate,
                "Rotate",
            );
        });
}
