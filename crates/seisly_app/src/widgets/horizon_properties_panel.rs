use crate::interpretation::InterpretationState;
use eframe::egui;
use uuid::Uuid;

pub struct HorizonPropertiesPanel {
    pub selected_horizon_id: Option<Uuid>,
    pub name_buffer: String,
}

impl HorizonPropertiesPanel {
    pub fn new() -> Self {
        Self {
            selected_horizon_id: None,
            name_buffer: String::new(),
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, interpretation: &mut InterpretationState) {
        if let Some(horizon_id) = interpretation.active_horizon_id {
            if self.selected_horizon_id != Some(horizon_id) {
                self.selected_horizon_id = Some(horizon_id);
                if let Some(horizon) = interpretation.active_horizon() {
                    self.name_buffer = horizon.name.clone();
                }
            }

            if let Some(horizon) = interpretation.active_horizon_mut() {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Name:");
                        if ui.text_edit_singleline(&mut self.name_buffer).changed() {
                            horizon.name = self.name_buffer.clone();
                        }
                    });
                    ui.label(format!("Picks: {}", horizon.picks.len()));
                    ui.checkbox(&mut horizon.is_visible, "Visible");
                });
            }
        } else {
            ui.label("No horizon selected");
        }
    }
}
