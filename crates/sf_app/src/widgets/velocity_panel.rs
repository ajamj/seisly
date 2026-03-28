//! Velocity Model Panel Widget
//!
//! Provides UI for defining and editing velocity models for time-to-depth conversion.

use crate::interpretation::VelocityState;
use eframe::egui;

/// Velocity model type for UI selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VelocityModelType {
    Constant,
    Gradient,
}

impl Default for VelocityModelType {
    fn default() -> Self {
        Self::Gradient
    }
}

/// Widget for velocity model configuration
pub struct VelocityPanel {
    model_type: VelocityModelType,
    v0_buffer: String,
    k_buffer: String,
}

impl Default for VelocityPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl VelocityPanel {
    pub fn new() -> Self {
        Self {
            model_type: VelocityModelType::Gradient,
            v0_buffer: "2000".to_string(),
            k_buffer: "0.5".to_string(),
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, velocity: &mut VelocityState) {
        ui.heading("Velocity Model");
        ui.separator();

        // Model type selector
        ui.horizontal(|ui| {
            ui.label("Model Type:");
            ui.selectable_value(
                &mut self.model_type,
                VelocityModelType::Constant,
                "Constant",
            );
            ui.selectable_value(
                &mut self.model_type,
                VelocityModelType::Gradient,
                "Gradient",
            );
        });

        ui.separator();

        // Parameter inputs
        ui.label("Parameters:");

        // V0 input
        ui.horizontal(|ui| {
            ui.label("V0 (m/s):");
            if ui.text_edit_singleline(&mut self.v0_buffer).lost_focus()
                && ui.input(|i| i.key_pressed(egui::Key::Enter))
            {
                if let Ok(v0) = self.v0_buffer.parse::<f32>() {
                    velocity.model.v0 = v0;
                }
            }
        });

        // K input (only for Gradient mode)
        if self.model_type == VelocityModelType::Gradient {
            ui.horizontal(|ui| {
                ui.label("k (1/s):");
                if ui.text_edit_singleline(&mut self.k_buffer).lost_focus()
                    && ui.input(|i| i.key_pressed(egui::Key::Enter))
                {
                    if let Ok(k) = self.k_buffer.parse::<f32>() {
                        velocity.model.k = k;
                    }
                }
            });
        }

        ui.separator();

        // Preview table
        ui.label("Velocity Preview:");

        egui::Grid::new("velocity_preview")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("Depth (m)");
                ui.label("Velocity (m/s)");
                ui.end_row();

                let depths = [0.0, 500.0, 1000.0, 2000.0, 3000.0];
                for depth in depths {
                    let velocity = velocity.model.v0 + velocity.model.k * depth;
                    ui.label(format!("{:.0}", depth));
                    ui.label(format!("{:.0}", velocity));
                    ui.end_row();
                }
            });

        ui.separator();

        // Depth mode toggle
        ui.checkbox(&mut velocity.is_depth_mode, "Enable Depth Mode");

        ui.separator();

        // Info label
        if velocity.is_depth_mode {
            ui.colored_label(
                egui::Color32::from_rgb(0, 180, 0),
                "✓ Depth Mode Active - Data displayed in depth domain",
            );
        } else {
            ui.label("Time Mode - Data displayed in time domain (TWT)");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_velocity_panel_creation() {
        let panel = VelocityPanel::new();
        assert_eq!(panel.model_type, VelocityModelType::Gradient);
        assert_eq!(panel.v0_buffer, "2000");
        assert_eq!(panel.k_buffer, "0.5");
    }

    #[test]
    fn test_velocity_model_type_default() {
        let model_type = VelocityModelType::default();
        assert_eq!(model_type, VelocityModelType::Gradient);
    }
}
