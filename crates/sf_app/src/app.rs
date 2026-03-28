use eframe::egui;

use crate::widgets::viewport::ViewportWidget;

pub struct StrataForgeApp {
    name: String,
    viewport: ViewportWidget,
}

impl StrataForgeApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            name: "MyField".to_owned(),
            viewport: ViewportWidget::new(),
        }
    }
}

impl eframe::App for StrataForgeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("StrataForge");
                ui.separator();
                ui.label(format!("Project: {}", self.name));
            });
        });

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
            if ui.button("Run Fault Detection").clicked() {
                println!("Fault detection requested");
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.viewport.ui(ui);
        });
    }
}
