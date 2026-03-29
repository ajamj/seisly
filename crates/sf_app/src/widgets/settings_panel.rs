//! Settings / Preferences Panel
//!
//! Native desktop app settings like display, performance, data paths, etc.

use eframe::egui;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Application settings
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppSettings {
    // Display
    pub theme: String,
    pub font_size: f32,
    pub vsync: bool,
    pub fullscreen: bool,
    
    // Performance
    pub gpu_acceleration: bool,
    pub max_texture_size: usize,
    pub lod_distance: f32,
    
    // Data paths
    pub default_data_directory: Option<PathBuf>,
    pub recent_projects: Vec<PathBuf>,
    pub auto_save: bool,
    pub auto_save_interval_minutes: u32,
    
    // Seismic defaults
    pub default_colormap: String,
    pub default_gain: f32,
    pub default_clip: f32,
    
    // Well defaults
    pub default_well_color: [f32; 4],
    pub well_track_width: f32,
    
    // Keyboard shortcuts
    pub shortcuts: Shortcuts,
}

/// Keyboard shortcuts configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Shortcuts {
    pub open_project: String,
    pub save_project: String,
    pub import_seismic: String,
    pub import_well: String,
    pub generate_synthetic: String,
    pub pick_horizon: String,
    pub sketch_fault: String,
    pub toggle_depth_mode: String,
    pub reset_view: String,
    pub zoom_in: String,
    pub zoom_out: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            // Display
            theme: "Dark".to_string(),
            font_size: 14.0,
            vsync: true,
            fullscreen: false,
            
            // Performance
            gpu_acceleration: true,
            max_texture_size: 4096,
            lod_distance: 1000.0,
            
            // Data paths
            default_data_directory: None,
            recent_projects: Vec::new(),
            auto_save: true,
            auto_save_interval_minutes: 5,
            
            // Seismic defaults
            default_colormap: "Seismic".to_string(),
            default_gain: 1.0,
            default_clip: 1.0,
            
            // Well defaults
            default_well_color: [1.0, 1.0, 0.0, 1.0], // Yellow
            well_track_width: 2.0,
            
            // Keyboard shortcuts
            shortcuts: Shortcuts::default(),
        }
    }
}

impl Default for Shortcuts {
    fn default() -> Self {
        Self {
            open_project: "Ctrl+O".to_string(),
            save_project: "Ctrl+S".to_string(),
            import_seismic: "Ctrl+I".to_string(),
            import_well: "Ctrl+W".to_string(),
            generate_synthetic: "Ctrl+G".to_string(),
            pick_horizon: "H".to_string(),
            sketch_fault: "F".to_string(),
            toggle_depth_mode: "D".to_string(),
            reset_view: "R".to_string(),
            zoom_in: "+".to_string(),
            zoom_out: "-".to_string(),
        }
    }
}

/// Settings panel widget
pub struct SettingsPanel {
    settings: AppSettings,
    active_tab: SettingsTab,
    unsaved_changes: bool,
}

#[derive(PartialEq, Clone, Copy)]
enum SettingsTab {
    Display,
    Performance,
    Data,
    Defaults,
    Shortcuts,
}

impl Default for SettingsPanel {
    fn default() -> Self {
        Self {
            settings: AppSettings::default(),
            active_tab: SettingsTab::Display,
            unsaved_changes: false,
        }
    }
}

impl SettingsPanel {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn load() -> Self {
        // Load from config file if exists
        // For now, use defaults
        Self::default()
    }
    
    pub fn save(&self) -> Result<(), String> {
        // Save to config file
        // ~/.config/stratforge/settings.json
        Ok(())
    }
    
    pub fn ui(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;
        
        ui.heading("⚙️ Settings");
        ui.separator();
        
        // Tab bar
        ui.horizontal(|ui| {
            if ui.selectable_value(&mut self.active_tab, SettingsTab::Display, "🖥️ Display").changed() {
                changed = true;
            }
            if ui.selectable_value(&mut self.active_tab, SettingsTab::Performance, "⚡ Performance").changed() {
                changed = true;
            }
            if ui.selectable_value(&mut self.active_tab, SettingsTab::Data, "📁 Data").changed() {
                changed = true;
            }
            if ui.selectable_value(&mut self.active_tab, SettingsTab::Defaults, "📊 Defaults").changed() {
                changed = true;
            }
            if ui.selectable_value(&mut self.active_tab, SettingsTab::Shortcuts, "⌨️ Shortcuts").changed() {
                changed = true;
            }
        });
        
        ui.separator();
        
        // Tab content
        egui::ScrollArea::vertical()
            .max_height(400.0)
            .show(ui, |ui| {
                match self.active_tab {
                    SettingsTab::Display => self.display_tab(ui),
                    SettingsTab::Performance => self.performance_tab(ui),
                    SettingsTab::Data => self.data_tab(ui),
                    SettingsTab::Defaults => self.defaults_tab(ui),
                    SettingsTab::Shortcuts => self.shortcuts_tab(ui),
                }
            });
        
        // Save/Cancel buttons
        ui.separator();
        ui.horizontal(|ui| {
            if ui.button("💾 Save Settings").clicked() {
                if let Err(e) = self.save() {
                    eprintln!("Failed to save settings: {}", e);
                }
            }
            
            if ui.button("❌ Cancel").clicked() {
                // Reload settings
                *self = Self::load();
            }
            
            if self.unsaved_changes {
                ui.label("🔴 Unsaved changes");
            }
        });
        
        changed
    }
    
    fn display_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("Display Settings");
        
        ui.horizontal(|ui| {
            ui.label("Theme:");
            if ui.selectable_value(&mut self.settings.theme, "Dark".to_string(), "🌙 Dark").changed() {
                self.unsaved_changes = true;
            }
            if ui.selectable_value(&mut self.settings.theme, "Light".to_string(), "☀️ Light").changed() {
                self.unsaved_changes = true;
            }
        });
        
        ui.horizontal(|ui| {
            ui.label("Font Size:");
            if ui.add(egui::DragValue::new(&mut self.settings.font_size)
                .range(10.0..=24.0)
                .speed(1.0))
                .changed()
            {
                self.unsaved_changes = true;
            }
        });
        
        ui.checkbox(&mut self.settings.vsync, "Enable VSync");
        ui.checkbox(&mut self.settings.fullscreen, "Fullscreen Mode");
    }
    
    fn performance_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("Performance Settings");
        
        ui.checkbox(&mut self.settings.gpu_acceleration, "GPU Acceleration");
        
        ui.horizontal(|ui| {
            ui.label("Max Texture Size:");
            if ui.add(egui::DragValue::new(&mut self.settings.max_texture_size)
                .range(1024..=16384)
                .suffix(" px"))
                .changed()
            {
                self.unsaved_changes = true;
            }
        });
        
        ui.horizontal(|ui| {
            ui.label("LOD Distance:");
            if ui.add(egui::DragValue::new(&mut self.settings.lod_distance)
                .range(100.0..=5000.0)
                .suffix(" m"))
                .changed()
            {
                self.unsaved_changes = true;
            }
        });
    }
    
    fn data_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("Data Settings");
        
        ui.horizontal(|ui| {
            ui.label("Default Data Directory:");
            if let Some(ref path) = self.settings.default_data_directory {
                ui.label(path.display().to_string());
            } else {
                ui.label("Not set");
            }
            if ui.button("Browse...").clicked() {
                // Open file dialog
                // self.settings.default_data_directory = Some(...);
                self.unsaved_changes = true;
            }
        });
        
        ui.checkbox(&mut self.settings.auto_save, "Auto-save");
        
        if self.settings.auto_save {
            ui.horizontal(|ui| {
                ui.label("Auto-save interval:");
                if ui.add(egui::DragValue::new(&mut self.settings.auto_save_interval_minutes)
                    .range(1..=60)
                    .suffix(" min"))
                    .changed()
                {
                    self.unsaved_changes = true;
                }
            });
        }
        
        ui.label("Recent Projects:");
        for path in &self.settings.recent_projects {
            ui.label(format!("  📁 {}", path.display()));
        }
    }
    
    fn defaults_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("Default Values");
        
        ui.horizontal(|ui| {
            ui.label("Default Colormap:");
            egui::ComboBox::from_id_salt("colormap")
                .selected_text(&self.settings.default_colormap)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.settings.default_colormap, "Seismic".to_string(), "Seismic");
                    ui.selectable_value(&mut self.settings.default_colormap, "Viridis".to_string(), "Viridis");
                    ui.selectable_value(&mut self.settings.default_colormap, "Gray".to_string(), "Gray");
                    ui.selectable_value(&mut self.settings.default_colormap, "Rainbow".to_string(), "Rainbow");
                });
        });
        
        ui.horizontal(|ui| {
            ui.label("Default Gain:");
            if ui.add(egui::DragValue::new(&mut self.settings.default_gain)
                .range(0.1..=5.0)
                .speed(0.1))
                .changed()
            {
                self.unsaved_changes = true;
            }
        });
        
        ui.horizontal(|ui| {
            ui.label("Default Clip:");
            if ui.add(egui::DragValue::new(&mut self.settings.default_clip)
                .range(0.1..=5.0)
                .speed(0.1))
                .changed()
            {
                self.unsaved_changes = true;
            }
        });
        
        ui.horizontal(|ui| {
            ui.label("Well Track Width:");
            if ui.add(egui::DragValue::new(&mut self.settings.well_track_width)
                .range(1.0..=10.0)
                .suffix(" px"))
                .changed()
            {
                self.unsaved_changes = true;
            }
        });
    }
    
    fn shortcuts_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("Keyboard Shortcuts");
        
        ui.horizontal(|ui| {
            ui.label("Open Project:");
            if ui.add(egui::TextEdit::singleline(&mut self.settings.shortcuts.open_project).desired_width(100.0)).changed() {
                self.unsaved_changes = true;
            }
        });
        
        ui.horizontal(|ui| {
            ui.label("Save Project:");
            if ui.add(egui::TextEdit::singleline(&mut self.settings.shortcuts.save_project).desired_width(100.0)).changed() {
                self.unsaved_changes = true;
            }
        });
        
        ui.horizontal(|ui| {
            ui.label("Generate Synthetic:");
            if ui.add(egui::TextEdit::singleline(&mut self.settings.shortcuts.generate_synthetic).desired_width(100.0)).changed() {
                self.unsaved_changes = true;
            }
        });
        
        ui.horizontal(|ui| {
            ui.label("Pick Horizon:");
            if ui.add(egui::TextEdit::singleline(&mut self.settings.shortcuts.pick_horizon).desired_width(100.0)).changed() {
                self.unsaved_changes = true;
            }
        });
        
        ui.horizontal(|ui| {
            ui.label("Sketch Fault:");
            if ui.add(egui::TextEdit::singleline(&mut self.settings.shortcuts.sketch_fault).desired_width(100.0)).changed() {
                self.unsaved_changes = true;
            }
        });
    }
}
