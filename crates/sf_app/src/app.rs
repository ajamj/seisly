use eframe::egui;

pub struct StrataForgeApp {
    name: String,
}

impl StrataForgeApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            name: "MyField".to_owned(),
        }
    }
}

impl eframe::App for StrataForgeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("StrataForge");
            ui.label(format!("Project: {}", self.name));
        });
    }
}
