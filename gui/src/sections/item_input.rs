use egui::ComboBox;

pub struct TargetItemInputSection {
    pub target_item_tag: &'static str,
}

impl Default for TargetItemInputSection {
    fn default() -> Self {
        Self {
            target_item_tag: "ITEM_MIX_UNI_BAG",
        }
    }
}

impl TargetItemInputSection {
    pub fn render(&mut self, ui: &mut egui::Ui) {
        ui.heading("Item to craft");

        ComboBox::from_id_source("item")
            .selected_text(item_tag_to_name(self.target_item_tag).expect("get target item name"))
            .show_ui(ui, |ui| {
                for item in atelier_sophie_data::ITEMS
                    .iter()
                    .filter(|item| atelier_sophie_data::ITEM_BOARDS.contains_key(&item.tag))
                {
                    ui.selectable_value(
                        &mut self.target_item_tag,
                        item.tag.as_str(),
                        item.name.as_str(),
                    );
                }
            });
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.target_item_tag.is_empty() {
            return Err("Target item is empty".into());
        }

        Ok(())
    }
}

fn item_tag_to_name(tag: &str) -> Option<&'static str> {
    atelier_sophie_data::ITEMS
        .iter()
        .find(|item| item.tag == tag)
        .map(|item| item.name.as_str())
}
