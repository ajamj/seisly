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
        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            ui.heading("Project Data");
            ui.separator();
            
            ui.collapsing("Seismic Volumes", |ui| {
                ui.label("None loaded");
            });
            
            ui.collapsing("Wells", |ui| {
                ui.label("Well-1");
            });
        });

        egui::SidePanel::right("right_panel").show(ctx, |ui| {
            ui.heading("AI Analysis");
            ui.separator();
            ui.button("Run Fault Detection");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("3D Viewport");
            ui.label("Viewport goes here");
        });
    }
}
