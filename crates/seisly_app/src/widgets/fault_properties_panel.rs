use crate::interpretation::InterpretationState;
use eframe::egui;
use uuid::Uuid;

pub struct FaultPropertiesPanel {
    pub selected_fault_id: Option<Uuid>,
    pub name_buffer: String,
}

impl FaultPropertiesPanel {
    pub fn new() -> Self {
        Self {
            selected_fault_id: None,
            name_buffer: String::new(),
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, interpretation: &mut InterpretationState) {
        if let Some(fault_id) = interpretation.active_fault_id {
            if self.selected_fault_id != Some(fault_id) {
                self.selected_fault_id = Some(fault_id);
                if let Some(fault) = interpretation.active_fault() {
                    self.name_buffer = fault.name.clone();
                }
            }

            if let Some(fault) = interpretation.active_fault_mut() {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Name:");
                        if ui.text_edit_singleline(&mut self.name_buffer).changed() {
                            fault.name = self.name_buffer.clone();
                        }
                    });
                    ui.label(format!("Sticks: {}", fault.sticks.len()));
                    ui.checkbox(&mut fault.is_visible, "Visible");
                });
            }
        } else {
            ui.label("No fault selected");
        }
    }
}
