use synth_solver::Shape;

pub fn input_shape(ui: &mut egui::Ui, shape: &mut Shape) {
    ui.vertical(|ui| {
        let mut matrix = shape.to_matrix();
        for row in matrix.iter_mut() {
            ui.horizontal(|ui| {
                for cell in row.iter_mut() {
                    ui.checkbox(cell, "");
                }
            });
        }
        *shape = Shape::from_matrix(matrix);
    });
}
