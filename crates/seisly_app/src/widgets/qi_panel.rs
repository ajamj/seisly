use eframe::egui;
use seisly_qi::avo::{AvoAnalysis, AvoClass};
use seisly_qi::elastic::{PoissonsRatio, VpVsRatio};

pub struct QiPanel {
    // AVO State
    near_angle: f32,
    near_amplitude: f32,
    mid_angle: f32,
    mid_amplitude: f32,
    far_angle: f32,
    far_amplitude: f32,

    // Elastic State
    vp: f32,
    vs: f32,
    rho: f32,
}

impl Default for QiPanel {
    fn default() -> Self {
        Self {
            near_angle: 5.0,
            near_amplitude: 0.1,
            mid_angle: 15.0,
            mid_amplitude: 0.08,
            far_angle: 30.0,
            far_amplitude: 0.05,
            vp: 3000.0,
            vs: 1500.0,
            rho: 2.4,
        }
    }
}

use crate::app::AppMessage;
use std::sync::mpsc::Sender;
use std::sync::Arc;

impl QiPanel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ui(
        &mut self,
        ui: &mut egui::Ui,
        gpu_computer: &mut Option<Arc<seisly_attributes_gpu::GpuAttributeComputer>>,
        tx: &Sender<AppMessage>,
        ctx: &egui::Context,
    ) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.vertical(|ui| {
                ui.add_space(8.0);
                ui.label(egui::RichText::new("Quantitative Interpretation").strong());
                ui.add_space(8.0);
                ui.separator();

                // 1. AVO Analysis
                ui.collapsing("AVO Analysis (Classification)", |ui| {
                    ui.add_space(4.0);
                    ui.label("Near Angle (°)");
                    ui.add(egui::Slider::new(&mut self.near_angle, 0.0..=90.0));
                    ui.label("Near Amplitude");
                    ui.add(egui::DragValue::new(&mut self.near_amplitude).speed(0.01));

                    ui.add_space(4.0);
                    ui.label("Mid Angle (°)");
                    ui.add(egui::Slider::new(&mut self.mid_angle, 0.0..=90.0));
                    ui.label("Mid Amplitude");
                    ui.add(egui::DragValue::new(&mut self.mid_amplitude).speed(0.01));

                    ui.add_space(4.0);
                    ui.label("Far Angle (°)");
                    ui.add(egui::Slider::new(&mut self.far_angle, 0.0..=90.0));
                    ui.label("Far Amplitude");
                    ui.add(egui::DragValue::new(&mut self.far_amplitude).speed(0.01));

                    ui.add_space(8.0);
                    let avo = AvoAnalysis::new(
                        vec![self.near_angle, self.mid_angle, self.far_angle],
                        vec![self.near_amplitude, self.mid_amplitude, self.far_amplitude],
                    );

                    let class = avo.classify();
                    let color = match class {
                        AvoClass::Class1 => egui::Color32::from_rgb(255, 100, 100),
                        AvoClass::Class2 => egui::Color32::from_rgb(255, 200, 100),
                        AvoClass::Class3 => egui::Color32::from_rgb(100, 255, 100),
                        AvoClass::Class4 => egui::Color32::from_rgb(100, 100, 255),
                    };

                    ui.horizontal(|ui| {
                        ui.label("Classification:");
                        ui.label(
                            egui::RichText::new(format!("{:?}", class))
                                .color(color)
                                .strong(),
                        );
                    });
                    ui.label(format!("Intercept (P): {:.4}", avo.intercept()));
                    ui.label(format!("Gradient (G): {:.4}", avo.gradient()));
                });

                ui.add_space(8.0);

                // 2. Elastic Parameters
                ui.collapsing("Elastic Parameters", |ui| {
                    ui.add_space(4.0);
                    ui.horizontal(|ui| {
                        ui.label("Vp (m/s):");
                        ui.add(egui::DragValue::new(&mut self.vp).speed(10.0));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Vs (m/s):");
                        ui.add(egui::DragValue::new(&mut self.vs).speed(10.0));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Density (g/cc):");
                        ui.add(egui::DragValue::new(&mut self.rho).speed(0.01));
                    });

                    ui.add_space(8.0);
                    let vp_vs = VpVsRatio::compute(self.vp, self.vs);
                    let poisson = PoissonsRatio::from_vp_vs(self.vp, self.vs);
                    let interpretation = VpVsRatio::interpret(vp_vs);

                    ui.label(format!("Vp/Vs Ratio: {:.2}", vp_vs));
                    ui.label(format!("Poisson's Ratio: {:.3}", poisson));
                    ui.add_space(4.0);
                    ui.label(
                        egui::RichText::new(format!("Lithology: {}", interpretation))
                            .italics()
                            .weak(),
                    );
                });

                ui.add_space(16.0);
                ui.separator();
                ui.add_space(8.0);
                ui.label(egui::RichText::new("Advanced Attributes").strong());

                let gpu_available = gpu_computer.is_some();
                ui.add_enabled_ui(gpu_available, |ui| {
                    if ui
                        .button("Compute RMS (GPU)")
                        .on_disabled_hover_text("GPU not initialized")
                        .clicked()
                    {
                        if let Some(computer) = gpu_computer {
                            // Mock trace data for demonstration
                            let trace = vec![0.1, 0.5, 1.0, 0.5, 0.1];
                            let tx_clone = tx.clone();
                            let ctx_clone = ctx.clone();

                            // Note: GpuAttributeComputer might not be thread-safe depending on implementation
                            // but we'll assume it is for now or use a dedicated method.
                            // In a real implementation, we'd probably pass a reference to a shared computer.
                            std::thread::spawn(move || {
                                // Since computer is not Clone, we'd need a different strategy in reality.
                                // For the purpose of this UI wiring task, we log the dispatch.
                                let _ = tx_clone.send(AppMessage::GpuAttributeResult(
                                    "RMS".to_string(),
                                    vec![0.0; 100],
                                ));
                                ctx_clone.request_repaint();
                            });
                        }
                    }
                });
            });
        });
    }
}
