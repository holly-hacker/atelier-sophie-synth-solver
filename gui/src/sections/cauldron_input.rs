pub struct CauldronInputSection {
    cauldron_item_tag: &'static str,
    pub size: usize,
    pub cauldron_effects: Vec<String>,
}

impl Default for CauldronInputSection {
    fn default() -> Self {
        Self {
            cauldron_item_tag: "ITEM_MIX_TIMEWORN_ALCHEMY_KETTLE",
            size: 4,
            cauldron_effects: vec![],
        }
    }
}

impl CauldronInputSection {
    pub fn render(&mut self, ui: &mut egui::Ui) {
        ui.heading("Cauldron");

        // Cauldron selection
        ui.add_enabled_ui(false, |ui| {
            egui::ComboBox::from_id_source("cauldron")
                .selected_text(item_tag_to_name(self.cauldron_item_tag).expect("get cauldron name"))
                .show_ui(ui, |ui| {
                    for item in atelier_sophie_data::ITEMS.iter() {
                        if item.use_type == "ITEM_USE_KETTLE" {
                            ui.selectable_value(
                                &mut self.cauldron_item_tag,
                                item.tag.as_str(),
                                item.name.as_str(),
                            );
                        }
                    }
                });
        });

        // Size up/down
        ui.horizontal(|ui| {
            ui.label("Size:");

            if ui
                .add_enabled_ui(self.size > 4, |ui| ui.button("<"))
                .inner
                .clicked()
            {
                self.size -= 1;
            }
            ui.label(format!("{}", self.size));
            if ui
                .add_enabled_ui(self.size < 6, |ui| ui.button(">"))
                .inner
                .clicked()
            {
                self.size += 1;
            }
        });

        // TODO: effects
    }
}

fn item_tag_to_name(tag: &str) -> Option<&'static str> {
    atelier_sophie_data::ITEMS
        .iter()
        .find(|item| item.tag == tag)
        .map(|item| item.name.as_str())
}
