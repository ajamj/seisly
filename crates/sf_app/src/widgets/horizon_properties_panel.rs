//! Horizon Properties Panel Widget
//!
//! Provides UI for editing horizon properties and managing horizon layers.

use crate::interpretation::InterpretationState;
use eframe::egui;
use uuid::Uuid;

/// Widget for editing horizon properties
pub struct HorizonPropertiesPanel {
    selected_horizon_id: Option<Uuid>,
    name_buffer: String,
}

impl Default for HorizonPropertiesPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl HorizonPropertiesPanel {
    pub fn new() -> Self {
        Self {
            selected_horizon_id: None,
            name_buffer: String::new(),
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, interpretation: &mut InterpretationState) {
        ui.heading("Horizon Properties");
        ui.separator();

        // Horizon layer list
        ui.collapsing("Horizon Layers", |ui| {
            ui.vertical(|ui| {
                for horizon in &mut interpretation.horizons {
                    let is_selected = self.selected_horizon_id == Some(horizon.id);

                    ui.horizontal(|ui| {
                        // Visibility checkbox
                        if ui.checkbox(&mut horizon.is_visible, "").changed() {
                            // Could trigger history command here
                        }

                        // Color indicator
                        let color = egui::Color32::from_rgba_unmultiplied(
                            (horizon.color[0] * 255.0) as u8,
                            (horizon.color[1] * 255.0) as u8,
                            (horizon.color[2] * 255.0) as u8,
                            (horizon.color[3] * 255.0) as u8,
                        );

                        let color_rect =
                            ui.allocate_response(egui::vec2(16.0, 16.0), egui::Sense::click());
                        ui.painter().rect_filled(color_rect.rect, 4.0, color);

                        // Horizon name (selectable)
                        let response = ui.selectable_label(is_selected, &horizon.name);
                        if response.clicked() {
                            self.selected_horizon_id = Some(horizon.id);
                            self.name_buffer = horizon.name.clone();
                        }

                        // Pick count
                        ui.label(format!("({} picks)", horizon.picks.len()));
                    });
                }

                // Add new horizon button
                if ui.button("+ Add Horizon").clicked() {
                    let name = format!("Horizon {}", interpretation.horizons.len() + 1);
                    let color = [0.0, 1.0, 0.0, 0.7];
                    let horizon = crate::interpretation::Horizon::new(name.clone(), color);
                    let horizon_id = horizon.id;
                    interpretation.add_horizon(horizon);
                    self.selected_horizon_id = Some(horizon_id);
                    self.name_buffer = name;
                }
            });
        });

        ui.separator();

        // Property editor (only if horizon is selected)
        if let Some(horizon_id) = self.selected_horizon_id {
            if let Some(horizon) = interpretation
                .horizons
                .iter_mut()
                .find(|h| h.id == horizon_id)
            {
                ui.heading("Edit Properties");

                // Name editor
                ui.horizontal(|ui| {
                    ui.label("Name:");
                    if ui.text_edit_singleline(&mut self.name_buffer).lost_focus()
                        && ui.input(|i| i.key_pressed(egui::Key::Enter))
                    {
                        horizon.name = self.name_buffer.clone();
                    }
                });

                // Color picker
                ui.horizontal(|ui| {
                    ui.label("Color:");

                    let mut color = egui::Color32::from_rgba_unmultiplied(
                        (horizon.color[0] * 255.0) as u8,
                        (horizon.color[1] * 255.0) as u8,
                        (horizon.color[2] * 255.0) as u8,
                        (horizon.color[3] * 255.0) as u8,
                    );

                    if ui.color_edit_button_srgba(&mut color).changed() {
                        horizon.color = [
                            color.r() as f32 / 255.0,
                            color.g() as f32 / 255.0,
                            color.b() as f32 / 255.0,
                            color.a() as f32 / 255.0,
                        ];
                    }
                });

                // Alpha slider
                ui.horizontal(|ui| {
                    ui.label("Transparency:");
                    let mut alpha = horizon.color[3];
                    if ui.add(egui::Slider::new(&mut alpha, 0.0..=1.0)).changed() {
                        horizon.color[3] = alpha;
                    }
                });

                // Visibility toggle
                ui.checkbox(&mut horizon.is_visible, "Visible");

                ui.separator();

                // Export buttons
                ui.label("Export:");
                ui.horizontal(|ui| {
                    if ui.button("📤 Export XYZ").clicked() {
                        // Signal to app to export - for now just print
                        println!("Export horizon {} to XYZ", horizon.name);
                    }
                    if ui.button("📤 Export JSON").clicked() {
                        println!("Export horizon {} to JSON", horizon.name);
                    }
                });

                ui.separator();

                // Delete button
                let delete_response = ui.add_enabled(
                    interpretation.horizons.len() > 0,
                    egui::Button::new("🗑 Delete Horizon")
                        .fill(egui::Color32::from_rgb(180, 50, 50)),
                );

                if delete_response.clicked() {
                    interpretation.horizons.retain(|h| h.id != horizon_id);
                    if interpretation.active_horizon_id == Some(horizon_id) {
                        interpretation.active_horizon_id =
                            interpretation.horizons.first().map(|h| h.id);
                    }
                    self.selected_horizon_id = interpretation.horizons.first().map(|h| h.id);
                    if let Some(selected_id) = self.selected_horizon_id {
                        if let Some(selected_horizon) =
                            interpretation.horizons.iter().find(|h| h.id == selected_id)
                        {
                            self.name_buffer = selected_horizon.name.clone();
                        }
                    } else {
                        self.name_buffer = String::new();
                    }
                }
            }
        } else {
            ui.label("Select a horizon to edit properties");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_horizon_properties_panel_creation() {
        let panel = HorizonPropertiesPanel::new();
        assert!(panel.selected_horizon_id.is_none());
        assert!(panel.name_buffer.is_empty());
    }
}
