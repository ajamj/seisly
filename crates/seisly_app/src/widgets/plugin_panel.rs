use egui;
use seisly_plugin::PluginManager;

pub struct PluginPanel {
    pub is_open: bool,
}

impl PluginPanel {
    pub fn new() -> Self {
        Self { is_open: false }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, manager: &mut PluginManager, results: &mut Vec<serde_json::Value>) {
        ui.heading("Installed Plugins");
        ui.separator();

        let plugins = manager.list_plugins();
        if plugins.is_empty() {
            ui.label("No plugins discovered.");
        } else {
            egui::Grid::new("plugin_grid")
                .num_columns(3)
                .spacing([20.0, 8.0])
                .show(ui, |ui| {
                    ui.label("Name");
                    ui.label("Status");
                    ui.label("Action");
                    ui.end_row();

                    for name in plugins {
                        ui.label(name);
                        ui.label("Ready");
                        if ui.button("Run").clicked() {
                            // Trigger plugin execution
                            match manager.execute(name, "run", serde_json::Value::Null) {
                                Ok(res) => {
                                    println!("Executed plugin: {}", name);
                                    results.push(res);
                                },
                                Err(e) => eprintln!("Failed to execute plugin: {}", e),
                            }
                        }
                        ui.end_row();
                    }
                });
        }

        ui.separator();
        if ui.button("Refresh Plugins").clicked() {
            // In a real app, we'd know the path
            let _ = manager.discover(std::path::Path::new("plugins"));
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, manager: &mut PluginManager, results: &mut Vec<serde_json::Value>) {
        let mut is_open = self.is_open;
        egui::Window::new("Plugin Manager")
            .open(&mut is_open)
            .show(ctx, |ui| {
                self.ui(ui, manager, results);
            });
        self.is_open = is_open;
    }
}
