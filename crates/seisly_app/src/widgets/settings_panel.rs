use eframe::egui;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AppSettings {
    pub theme: String,
}

pub struct SettingsPanel {
    #[allow(dead_code)]
    pub settings: AppSettings,
}

impl SettingsPanel {
    pub fn new() -> Self {
        Self {
            settings: AppSettings::default(),
        }
    }

    #[allow(dead_code)]
    pub fn ui(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;
        ui.vertical(|ui| {
            ui.heading("General Settings");
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.label("Theme:");
                egui::ComboBox::from_id_source("theme_selector")
                    .selected_text(&self.settings.theme)
                    .show_ui(ui, |ui| {
                        if ui
                            .selectable_value(
                                &mut self.settings.theme,
                                "Seisly Dark".to_string(),
                                "Dark",
                            )
                            .clicked()
                        {
                            changed = true;
                        }
                        if ui
                            .selectable_value(
                                &mut self.settings.theme,
                                "Seisly Light".to_string(),
                                "Light",
                            )
                            .clicked()
                        {
                            changed = true;
                        }
                    });
            });
        });
        changed
    }
}
