use synth_solver::solver::SolverSettings;

#[derive(Default)]
pub struct SolverSettingsComponent {
    pub props: SolverSettings,
}

impl SolverSettingsComponent {
    pub fn render(&mut self, ui: &mut egui::Ui) {
        ui.heading("Settings");
        ui.group(|ui| {
            transformations_combobox(ui, &mut self.props.transformations);

            ui.checkbox(&mut self.props.allow_overlaps, "Allow overlaps");
        });
    }
}

fn transformations_combobox(
    ui: &mut egui::Ui,
    transformation: &mut synth_solver::TransformationType,
) {
    egui::ComboBox::from_label("Transformation")
        .selected_text(format!("{transformation:?}"))
        .show_ui(ui, |ui| {
            ui.selectable_value(
                transformation,
                synth_solver::TransformationType::None,
                "None",
            );
            ui.selectable_value(
                transformation,
                synth_solver::TransformationType::FlipHorizontal,
                "Flip H",
            );
            ui.selectable_value(
                transformation,
                synth_solver::TransformationType::FlipVertical,
                "Flip V",
            );
            ui.selectable_value(
                transformation,
                synth_solver::TransformationType::Rotate,
                "Rotate",
            );
        });
}
