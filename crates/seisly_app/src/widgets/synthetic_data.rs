//! Synthetic Data Generator Widget
//!
//! Provides UI for generating synthetic seismic, wells, and horizons.

use eframe::egui;
use sf_compute::synthetic::{SyntheticSeismic, SyntheticWellLog};
use sf_core::domain::well::{Well, WellLog};

/// Widget for synthetic data generation
pub struct SyntheticDataWidget {
    inline_count: i32,
    crossline_count: i32,
    sample_count: i32,
    frequency: f32,
    noise_level: f32,
    status_message: Option<String>,
}

impl Default for SyntheticDataWidget {
    fn default() -> Self {
        Self {
            inline_count: 100,
            crossline_count: 100,
            sample_count: 512,
            frequency: 35.0,
            noise_level: 0.1,
            status_message: None,
        }
    }
}

impl SyntheticDataWidget {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) -> Option<SyntheticDataResult> {
        ui.heading("🎲 Generate Synthetic Data");
        ui.separator();

        let mut result = None;

        egui::Grid::new("synthetic_grid")
            .num_columns(2)
            .spacing([10.0, 5.0])
            .show(ui, |ui| {
                
                // Seismic parameters
                ui.label("📊 Seismic Parameters:");
                ui.end_row();
                
                ui.label("  Inlines:");
                ui.add(egui::Slider::new(&mut self.inline_count, 10..=500));
                ui.end_row();
                
                ui.label("  Crosslines:");
                ui.add(egui::Slider::new(&mut self.crossline_count, 10..=500));
                ui.end_row();
                
                ui.label("  Samples:");
                ui.add(egui::Slider::new(&mut self.sample_count, 100..=2000));
                ui.end_row();
                
                ui.label("  Frequency (Hz):");
                ui.add(egui::DragValue::new(&mut self.frequency).speed(1.0));
                ui.end_row();
                
                ui.label("  Noise Level:");
                ui.add(egui::Slider::new(&mut self.noise_level, 0.0..=1.0));
                ui.end_row();
                
                ui.separator();
                ui.end_row();
                
                // Generate button
                ui.label("");
                if ui.button("🚀 Generate Synthetic Seismic").clicked() {
                    result = Some(self.generate_seismic());
                }
                ui.end_row();
                
                // Status message
                if let Some(ref msg) = self.status_message {
                    ui.label("");
                    ui.label(msg.clone());
                    ui.end_row();
                }
            });

        result
    }

    fn generate_seismic(&mut self) -> SyntheticDataResult {
        let seismic = SyntheticSeismic::new(
            self.inline_count as usize,
            self.crossline_count as usize,
            self.sample_count as usize,
        );

        let data = seismic.generate();
        
        self.status_message = Some(format!(
            "✅ Generated: {}x{}x{} = {} samples",
            self.inline_count,
            self.crossline_count,
            self.sample_count,
            data.len()
        ));

        SyntheticDataResult::Seismic(data)
    }
}

/// Result of synthetic data generation
pub enum SyntheticDataResult {
    Seismic(Vec<f32>),
    Well(Well),
}
