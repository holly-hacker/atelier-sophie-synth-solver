use egui::RichText;
use synth_solver::Goal;

pub struct GoalsInputSection {
    pub goals: Vec<Goal>,
}

impl GoalsInputSection {
    pub fn render(&mut self, ui: &mut egui::Ui) {
        ui.heading("Goals");
        goals_input(ui, &mut self.goals);
    }
}

impl Default for GoalsInputSection {
    fn default() -> Self {
        Self {
            goals: vec![default_goal()],
        }
    }
}

fn goals_input(ui: &mut egui::Ui, goals: &mut Vec<Goal>) {
    let mut to_remove = vec![];
    for (goal_idx, goal) in goals.iter_mut().enumerate() {
        ui.push_id(goal_idx, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new(format!("\t- Goal {}:", goal_idx + 1)));

                for (threshold_idx, threshold) in
                    goal.effect_value_thresholds.iter_mut().enumerate()
                {
                    ui.push_id(threshold_idx, |ui| {
                        ui.add(
                            egui::DragValue::new(threshold)
                                .speed(1.0)
                                .clamp_range(0..=999),
                        );
                    });
                }

                if ui.button("+").clicked() {
                    let new_val = goal
                        .effect_value_thresholds
                        .iter()
                        .max()
                        .copied()
                        .unwrap_or(0)
                        + 1;
                    goal.effect_value_thresholds.push(new_val);
                }

                if ui.button("-").clicked() {
                    goal.effect_value_thresholds.pop();

                    if goal.effect_value_thresholds.is_empty() {
                        to_remove.push(goal_idx);
                    }
                }
            });
        });
    }

    if ui.button("Add Goal").clicked() {
        goals.push(default_goal());
    }

    debug_assert!(to_remove.len() <= 1);
    for goal_idx in to_remove {
        goals.remove(goal_idx);
    }
}

fn default_goal() -> Goal {
    Goal {
        effect_value_thresholds: vec![30],
    }
}
