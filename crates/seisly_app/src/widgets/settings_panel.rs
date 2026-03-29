//! Settings / Preferences Panel
//!
//! Native desktop app settings like display, performance, data paths, etc.

use eframe::egui;

/// Application settings - simplified for now
#[derive(Clone, Debug)]
pub struct AppSettings {
    pub theme: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: "Dark".to_string(),
        }
    }
}

/// Settings panel widget
pub struct SettingsPanel {
    settings: AppSettings,
}

impl Default for SettingsPanel {
    fn default() -> Self {
        Self {
            settings: AppSettings::default(),
        }
    }
}

impl SettingsPanel {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn ui(&mut self, ui: &mut egui::Ui) -> bool {
        ui.heading("⚙️ Settings");
        ui.separator();
        
        ui.label("Settings panel - coming soon!");
        ui.label("Theme:");
        ui.label(&self.settings.theme);
        
        false
    }
}
