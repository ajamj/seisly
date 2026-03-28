//! Fault Properties Panel Widget
//!
//! Provides UI for editing fault properties and managing fault layers.

use crate::interpretation::{Fault, InterpretationState};
use eframe::egui;
use uuid::Uuid;

/// Widget for editing fault properties
pub struct FaultPropertiesPanel {
    selected_fault_id: Option<Uuid>,
    name_buffer: String,
}

impl Default for FaultPropertiesPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl FaultPropertiesPanel {
    pub fn new() -> Self {
        Self {
            selected_fault_id: None,
            name_buffer: String::new(),
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, interpretation: &mut InterpretationState) {
        ui.heading("Fault Properties");
        ui.separator();

        // Fault layer list
        ui.collapsing("Fault Layers", |ui| {
            ui.vertical(|ui| {
                for fault in &mut interpretation.faults {
                    let is_selected = self.selected_fault_id == Some(fault.id);

                    ui.horizontal(|ui| {
                        // Visibility checkbox
                        if ui.checkbox(&mut fault.is_visible, "").changed() {
                            // Could trigger history command here
                        }

                        // Color indicator
                        let color = egui::Color32::from_rgba_unmultiplied(
                            (fault.color[0] * 255.0) as u8,
                            (fault.color[1] * 255.0) as u8,
                            (fault.color[2] * 255.0) as u8,
                            (fault.color[3] * 255.0) as u8,
                        );

                        let color_rect =
                            ui.allocate_response(egui::vec2(16.0, 16.0), egui::Sense::click());
                        ui.painter().rect_filled(color_rect.rect, 4.0, color);

                        // Fault name (selectable)
                        let response = ui.selectable_label(is_selected, &fault.name);
                        if response.clicked() {
                            self.selected_fault_id = Some(fault.id);
                            self.name_buffer = fault.name.clone();
                        }

                        // Pick count
                        ui.label(format!("({} sticks)", fault.sticks.len()));
                    });
                }

                // Add new fault button
                if ui.button("+ Add Fault").clicked() {
                    let name = format!("Fault {}", interpretation.faults.len() + 1);
                    let color = [1.0, 0.0, 0.0, 0.5];
                    let fault = Fault::new(name.clone(), color);
                    let fault_id = fault.id;
                    interpretation.add_fault(fault);
                    self.selected_fault_id = Some(fault_id);
                    self.name_buffer = name;
                }
            });
        });

        ui.separator();

        // Property editor (only if fault is selected)
        if let Some(fault_id) = self.selected_fault_id {
            if let Some(fault) = interpretation.faults.iter_mut().find(|f| f.id == fault_id) {
                ui.heading("Edit Properties");

                // Name editor
                ui.horizontal(|ui| {
                    ui.label("Name:");
                    if ui.text_edit_singleline(&mut self.name_buffer).lost_focus()
                        && ui.input(|i| i.key_pressed(egui::Key::Enter))
                    {
                        fault.set_name(self.name_buffer.clone());
                    }
                });

                // Color picker
                ui.horizontal(|ui| {
                    ui.label("Color:");

                    let mut color = egui::Color32::from_rgba_unmultiplied(
                        (fault.color[0] * 255.0) as u8,
                        (fault.color[1] * 255.0) as u8,
                        (fault.color[2] * 255.0) as u8,
                        (fault.color[3] * 255.0) as u8,
                    );

                    if ui.color_edit_button_srgba(&mut color).changed() {
                        fault.set_color([
                            color.r() as f32 / 255.0,
                            color.g() as f32 / 255.0,
                            color.b() as f32 / 255.0,
                            color.a() as f32 / 255.0,
                        ]);
                    }
                });

                // Alpha slider
                ui.horizontal(|ui| {
                    ui.label("Transparency:");
                    let mut alpha = fault.color[3];
                    if ui.add(egui::Slider::new(&mut alpha, 0.0..=1.0)).changed() {
                        fault.color[3] = alpha;
                    }
                });

                // Visibility toggle
                ui.checkbox(&mut fault.is_visible, "Visible");

                ui.separator();

                // Delete button
                let delete_response = ui.add_enabled(
                    interpretation.faults.len() > 0,
                    egui::Button::new("🗑 Delete Fault").fill(egui::Color32::from_rgb(180, 50, 50)),
                );

                if delete_response.clicked() {
                    interpretation.faults.retain(|f| f.id != fault_id);
                    if interpretation.active_fault_id == Some(fault_id) {
                        interpretation.active_fault_id =
                            interpretation.faults.first().map(|f| f.id);
                    }
                    self.selected_fault_id = interpretation.faults.first().map(|f| f.id);
                    if let Some(selected_id) = self.selected_fault_id {
                        if let Some(selected_fault) =
                            interpretation.faults.iter().find(|f| f.id == selected_id)
                        {
                            self.name_buffer = selected_fault.name.clone();
                        }
                    } else {
                        self.name_buffer = String::new();
                    }
                }
            }
        } else {
            ui.label("Select a fault to edit properties");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fault_properties_panel_creation() {
        let panel = FaultPropertiesPanel::new();
        assert!(panel.selected_fault_id.is_none());
        assert!(panel.name_buffer.is_empty());
    }
}
