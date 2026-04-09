use eframe::egui;
use seisly_plugin::PluginManager;

pub struct PluginPanel {
    pub is_open: bool,
}

impl PluginPanel {
    pub fn new() -> Self {
        Self { is_open: false }
    }

    pub fn ui(
        &mut self,
        ui: &mut egui::Ui,
        manager: &mut PluginManager,
        _results: &mut Vec<serde_json::Value>,
    ) {
        ui.heading("🧩 Plugins");
        ui.separator();

        for plugin_name in manager.list_plugins() {
            ui.horizontal(|ui| {
                ui.label(plugin_name);
                if ui.button("Run").clicked() {
                    // Plugin execution logic
                }
            });
        }
    }

    #[allow(dead_code)]
    pub fn show(
        &mut self,
        ctx: &egui::Context,
        manager: &mut PluginManager,
        results: &mut Vec<serde_json::Value>,
    ) {
        if self.is_open {
            let mut is_open = self.is_open;
            egui::Window::new("Plugin Manager")
                .open(&mut is_open)
                .show(ctx, |ui| {
                    self.ui(ui, manager, results);
                });
            self.is_open = is_open;
        }
    }
}
