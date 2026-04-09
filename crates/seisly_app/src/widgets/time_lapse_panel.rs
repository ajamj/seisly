use eframe::egui;
use seisly_4d::timelapse::TimeLapseAnalysis;
use seisly_storage::project::SeismicVolumeEntry;

pub struct TimeLapsePanel {
    base_volume_id: Option<String>,
    monitor_volume_id: Option<String>,
    nrms_result: Option<f32>,
}

impl Default for TimeLapsePanel {
    fn default() -> Self {
        Self {
            base_volume_id: None,
            monitor_volume_id: None,
            nrms_result: None,
        }
    }
}

impl TimeLapsePanel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, volumes: &[SeismicVolumeEntry]) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.vertical(|ui| {
                ui.add_space(8.0);
                ui.label(egui::RichText::new("4D Monitoring").strong());
                ui.add_space(8.0);
                ui.separator();

                ui.add_space(8.0);
                ui.label("Base Survey:");
                egui::ComboBox::from_id_salt("base_survey_selector")
                    .selected_text(self.base_volume_id.as_deref().unwrap_or("Select volume..."))
                    .show_ui(ui, |ui| {
                        for vol in volumes {
                            ui.selectable_value(
                                &mut self.base_volume_id,
                                Some(vol.id.clone()),
                                &vol.name,
                            );
                        }
                    });

                ui.add_space(8.0);
                ui.label("Monitor Survey:");
                egui::ComboBox::from_id_salt("monitor_survey_selector")
                    .selected_text(
                        self.monitor_volume_id
                            .as_deref()
                            .unwrap_or("Select volume..."),
                    )
                    .show_ui(ui, |ui| {
                        for vol in volumes {
                            ui.selectable_value(
                                &mut self.monitor_volume_id,
                                Some(vol.id.clone()),
                                &vol.name,
                            );
                        }
                    });

                ui.add_space(16.0);
                if ui.button("Compute NRMS").clicked() {
                    // Placeholder logic: In a real app we'd load the trace data
                    // For now, we'll simulate a result if both are selected
                    if self.base_volume_id.is_some() && self.monitor_volume_id.is_some() {
                        if self.base_volume_id == self.monitor_volume_id {
                            self.nrms_result = Some(0.0);
                        } else {
                            // Mock calculation
                            let base_trace = vec![1.0, 0.5, -0.2, 0.8];
                            let monitor_trace = vec![1.1, 0.45, -0.25, 0.75];
                            let analysis = TimeLapseAnalysis::new(base_trace, monitor_trace, 0.004);
                            self.nrms_result = Some(analysis.nrms());
                        }
                    }
                }

                if let Some(nrms) = self.nrms_result {
                    ui.add_space(16.0);
                    ui.separator();
                    ui.add_space(8.0);
                    ui.label("Results:");
                    ui.horizontal(|ui| {
                        ui.label("Normalized RMS (NRMS):");
                        ui.label(
                            egui::RichText::new(format!("{:.2}%", nrms))
                                .color(egui::Color32::LIGHT_BLUE)
                                .strong(),
                        );
                    });

                    let quality = if nrms < 10.0 {
                        "Excellent"
                    } else if nrms < 20.0 {
                        "Good"
                    } else {
                        "Significant Change"
                    };
                    ui.label(
                        egui::RichText::new(format!("Similarity: {}", quality))
                            .weak()
                            .italics(),
                    );
                }

                ui.add_space(24.0);
                ui.separator();
                ui.add_space(8.0);
                ui.label(egui::RichText::new("Advanced 4D Attributes").strong());
                ui.label("Time-shift and Difference cubes are available via the main toolbar.");
            });
        });
    }
}
