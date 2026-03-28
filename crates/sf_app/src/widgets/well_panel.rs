//! Well Panel Widget
//!
//! Provides UI for well management and visualization.

use crate::interpretation::WellState;
use eframe::egui;

/// Widget for well management
pub struct WellPanel {
    #[allow(dead_code)] // Reserved for future LAS file import dialog
    import_path: String,
}

impl Default for WellPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl WellPanel {
    pub fn new() -> Self {
        Self {
            import_path: String::new(),
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, well_state: &mut WellState) {
        ui.heading("Wells");
        ui.separator();

        // Well list
        ui.collapsing("Well List", |ui| {
            ui.vertical(|ui| {
                for well in &mut well_state.wells {
                    let is_active = well_state.active_well_id == Some(well.id);

                    ui.horizontal(|ui| {
                        // Visibility checkbox
                        if ui.checkbox(&mut well.is_visible, "").changed() {
                            // Visibility toggled
                        }

                        // Well symbol as colored box
                        let color = if is_active {
                            egui::Color32::YELLOW
                        } else {
                            egui::Color32::GRAY
                        };

                        let symbol_rect =
                            ui.allocate_response(egui::vec2(20.0, 20.0), egui::Sense::click());
                        ui.painter().rect_filled(symbol_rect.rect, 2.0, color);

                        // Well name (selectable)
                        let response = ui.selectable_label(is_active, &well.name);
                        if response.clicked() {
                            well_state.active_well_id = Some(well.id);
                        }

                        // Well symbol
                        ui.label(format!("[{}]", well.symbol));

                        // Tops count
                        ui.label(format!("({} tops)", well.tops.len()));
                    });
                }

                // Import well button
                ui.separator();

                ui.horizontal(|ui| {
                    if ui.button("📂 Import Well (LAS)").clicked() {
                        // For now, create a demo well
                        self.create_demo_well(well_state);
                    }
                });
            });
        });

        ui.separator();

        // Well details (if well is selected)
        if let Some(well) = well_state.active_well() {
            ui.heading("Well Details");

            ui.label(format!("Name: {}", well.name));
            ui.label(format!("Symbol: {}", well.symbol));
            ui.label(format!(
                "Location: ({:.1}, {:.1})",
                well.location.x, well.location.y
            ));
            ui.label(format!(
                "Datum: {} @ {:.1}m",
                well.datum.name, well.datum.elevation
            ));

            ui.separator();

            // Well tops
            ui.collapsing("Formation Tops", |ui| {
                for top in &well.tops {
                    ui.horizontal(|ui| {
                        // Color indicator
                        let color = egui::Color32::from_rgba_unmultiplied(
                            (top.color[0] * 255.0) as u8,
                            (top.color[1] * 255.0) as u8,
                            (top.color[2] * 255.0) as u8,
                            (top.color[3] * 255.0) as u8,
                        );

                        let color_rect =
                            ui.allocate_response(egui::vec2(12.0, 12.0), egui::Sense::hover());
                        ui.painter().rect_filled(color_rect.rect, 2.0, color);

                        ui.label(format!("{}: {:.1}m ({})", top.name, top.depth, top.type_));
                    });
                }
            });

            // Well logs
            ui.collapsing("Log Curves", |ui| {
                for log in &well.logs {
                    ui.horizontal(|ui| {
                        ui.label(format!(
                            "{}: {} ({})",
                            log.mnemonic, log.description, log.units
                        ));
                        ui.label(format!(
                            "Depth: {:.1}m - {:.1}m",
                            log.min_depth, log.max_depth
                        ));
                    });
                }
            });
        } else {
            ui.label("Select a well to view details");
        }
    }

    fn create_demo_well(&mut self, well_state: &mut WellState) {
        use sf_core::domain::well::Well;

        // Create a demo well
        let mut well = Well::new(
            "Demo Well".to_string(),
            "DW-1".to_string(),
            250000.0,
            500000.0,
            50.0,
        );

        // Add some demo formation tops
        well.add_top(
            "Top Sand A".to_string(),
            1500.0,
            "TOP".to_string(),
            [1.0, 0.8, 0.0, 1.0],
        );
        well.add_top(
            "Base Sand A".to_string(),
            1600.0,
            "BASE".to_string(),
            [0.8, 0.6, 0.0, 1.0],
        );
        well.add_top(
            "Top Shale B".to_string(),
            1800.0,
            "TOP".to_string(),
            [0.5, 0.5, 0.8, 1.0],
        );
        well.add_top(
            "Top Sand C".to_string(),
            2000.0,
            "TOP".to_string(),
            [1.0, 0.5, 0.0, 1.0],
        );

        // Add a demo GR log
        let depths: Vec<f32> = (0..100).map(|i| i as f32 * 10.0).collect();
        let gr_values: Vec<f32> = depths
            .iter()
            .map(|d| 50.0 + (d / 100.0).sin() * 30.0)
            .collect();
        well.add_log(
            "GR".to_string(),
            "GAPI".to_string(),
            gr_values,
            depths.clone(),
        );

        // Add a demo DT log
        let dt_values: Vec<f32> = depths
            .iter()
            .map(|d| 200.0 + (d / 50.0).cos() * 50.0)
            .collect();
        well.add_log("DT".to_string(), "us/m".to_string(), dt_values, depths);

        well_state.add_well(well);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_well_panel_creation() {
        let panel = WellPanel::new();
        assert!(panel.import_path.is_empty());
    }
}
