use eframe::egui;
use uuid::Uuid;

use crate::widgets::viewport::ViewportWidget;
use crate::widgets::crossplot::CrossPlotWidget;
use crate::interpretation::{InterpretationState, Horizon, Fault, PickingMode, VelocityState, HistoryManager};
use sf_compute::seismic::{SeismicVolume, InMemoryProvider};
use sf_storage::project::SeismicVolumeEntry;

pub struct VisualSettings {
    pub gain: f32,
    pub clip: f32,
    pub opacity: f32,
    pub colormap: String,
}

impl Default for VisualSettings {
    fn default() -> Self {
        Self {
            gain: 1.0,
            clip: 1.0,
            opacity: 0.5,
            colormap: "Seismic".to_string(),
        }
    }
}

pub struct StrataForgeApp {
    name: String,
    viewport: ViewportWidget,
    crossplot: CrossPlotWidget,
    interpretation: InterpretationState,
    history: HistoryManager,
    visuals: VisualSettings,
    volume: Option<SeismicVolume>,
    seismic_volumes: Vec<SeismicVolumeEntry>,
    velocity: VelocityState,
    volumetric_result: Option<f32>,
}

impl StrataForgeApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        let mut interpretation = InterpretationState::new();

        let target_format = cc.wgpu_render_state.as_ref().map(|rs| rs.target_format);
        // Add a default horizon for demo
        let h_id = Uuid::new_v4();
        let mut horizon = Horizon::new("Horizon A".to_string(), [0.0, 1.0, 0.0]);
        horizon.id = h_id;
        interpretation.add_horizon(horizon);
        interpretation.active_horizon_id = Some(h_id);

        // Add a default fault for demo
        let f_id = Uuid::new_v4();
        let mut fault = Fault::new("Fault A".to_string(), [1.0, 0.0, 0.0]);
        fault.id = f_id;
        interpretation.add_fault(fault);
        interpretation.active_fault_id = Some(f_id);

        // Create a dummy seismic volume for interaction demo
        let sample_count = 512;
        let inline_range = (0, 500);
        let crossline_range = (0, 500);
        let mut data = vec![0.0; 501 * 501 * sample_count];

        // Add a "reflector" at sample 250
        for i in 0..501 {
            for j in 0..501 {
                let idx = (i * 501 + j) * sample_count + 250;
                data[idx] = 1.0;
                if idx > 0 { data[idx-1] = 0.5; }
                if idx < data.len() - 1 { data[idx+1] = 0.5; }
            }
        }

        let provider = InMemoryProvider {
            data,
            inline_range,
            crossline_range,
            sample_count,
        };
        let volume = Some(SeismicVolume::new(Box::new(provider)));

        let mut viewport = ViewportWidget::new();
        viewport.target_format = target_format;

        let seismic_volumes = vec![
            SeismicVolumeEntry {
                id: Uuid::new_v4().to_string(),
                name: "Full Stack".to_string(),
                path: "seismic/full_stack.segy".to_string(),
                is_visible: true,
                channel_assignment: 0,
            },
        ];

        Self {
            name: "MyField".to_owned(),
            viewport,
            crossplot: CrossPlotWidget::new("Gamma Ray", "Depth"),
            interpretation,
            history: HistoryManager::new(),
            visuals: VisualSettings::default(),
            volume,
            seismic_volumes,
            velocity: VelocityState::new(),
            volumetric_result: None,
        }
    }

    fn calculate_volumetrics(&mut self) {
        if self.interpretation.selected_horizon_ids.len() < 2 {
            return;
        }

        let h1_id = self.interpretation.selected_horizon_ids[0];
        let h2_id = self.interpretation.selected_horizon_ids[1];

        let h1 = self.interpretation.horizons.iter().find(|h| h.id == h1_id);
        let h2 = self.interpretation.horizons.iter().find(|h| h.id == h2_id);

        if let (Some(upper), Some(lower)) = (h1, h2) {
            use sf_compute::interpolation::{RbfInterpolator, RbfType};
            use sf_compute::volumetrics::VolumetricEngine;

            let p1: Vec<[f32; 3]> = upper.picks.iter().map(|p| p.position).collect();
            let p2: Vec<[f32; 3]> = lower.picks.iter().map(|p| p.position).collect();

            if p1.len() >= 3 && p2.len() >= 3 {
                if let (Ok(interp1), Ok(interp2)) = (
                    RbfInterpolator::new(&p1, RbfType::ThinPlateSpline),
                    RbfInterpolator::new(&p2, RbfType::ThinPlateSpline)
                ) {
                    // Find common bounds
                    let mut min_x = f32::MAX;
                    let mut max_x = f32::MIN;
                    let mut min_y = f32::MAX;
                    let mut max_y = f32::MIN;

                    for p in p1.iter().chain(p2.iter()) {
                        min_x = min_x.min(p[0]);
                        max_x = max_x.max(p[0]);
                        min_y = min_y.min(p[1]);
                        max_y = max_y.max(p[1]);
                    }

                    let engine = VolumetricEngine::new();
                    let vol = engine.calculate_volume(
                        &interp1, &interp2,
                        min_x, max_x, min_y, max_y,
                        50, 50
                    );
                    self.volumetric_result = Some(vol);
                }
            }
        }
    }

    fn export_active_horizon(&self, format: &str) {
        use sf_io::export::{SurfaceExporter, json::JsonExporter, xyz::XyzExporter};
        
        if let Some(horizon) = self.interpretation.active_horizon() {
            if let Some(mesh) = horizon.meshes.first() {
                let path = format!("{}.{}", horizon.name, format);
                let result = match format {
                    "xyz" => XyzExporter.export_surface(mesh, std::path::Path::new(&path)),
                    "json" => JsonExporter.export_surface(mesh, std::path::Path::new(&path)),
                    _ => Ok(()),
                };

                if let Err(e) = result {
                    eprintln!("Export failed: {}", e);
                } else {
                    println!("Exported {} to {}", horizon.name, path);
                }
            }
        }
    }
}

impl eframe::App for StrataForgeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("StrataForge");
                ui.separator();
                
                // Contextual Toolbar based on active selection
                if self.interpretation.active_horizon_id.is_some() {
                    ui.label("Horizon:");
                    ui.add(egui::Image::new(egui::include_image!("../assets/icons/horizon.svg")).max_width(20.0));
                    if ui.button("Undo").clicked() { self.history.undo(&mut self.interpretation); }
                    if ui.button("Redo").clicked() { self.history.redo(&mut self.interpretation); }
                } else if self.interpretation.active_fault_id.is_some() {
                    ui.label("Fault:");
                    ui.add(egui::Image::new(egui::include_image!("../assets/icons/fault.svg")).max_width(20.0));
                    if ui.button("Undo").clicked() { self.history.undo(&mut self.interpretation); }
                }

                ui.separator();
                ui.label("Picking:");
                ui.selectable_value(&mut self.interpretation.picking_mode, PickingMode::None, "None");
                
                ui.horizontal(|ui| {
                    let seed_resp = ui.selectable_value(&mut self.interpretation.picking_mode, PickingMode::Seed, "Seed");
                    if seed_resp.hovered() { ui.label("Auto-Seed"); }
                    
                    ui.selectable_value(&mut self.interpretation.picking_mode, PickingMode::AutoTrack, "Auto-Track");
                    ui.selectable_value(&mut self.interpretation.picking_mode, PickingMode::Manual, "Manual");
                    
                    ui.separator();
                    ui.add(egui::Image::new(egui::include_image!("../assets/icons/seismic.svg")).max_width(20.0));
                    ui.selectable_value(&mut self.interpretation.picking_mode, PickingMode::SketchFault, "Sketch Fault");
                });

                ui.separator();
                ui.checkbox(&mut self.velocity.is_depth_mode, "Depth Mode");
                if self.velocity.is_depth_mode {
                    ui.label("V0:");
                    ui.add(egui::DragValue::new(&mut self.velocity.model.v0).speed(10.0));
                    ui.label("k:");
                    ui.add(egui::DragValue::new(&mut self.velocity.model.k).speed(0.01));
                }
            });
        });

        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            ui.heading("Project Data");
            ui.separator();

            ui.collapsing("Seismic Volumes", |ui| {
                for vol in &mut self.seismic_volumes {
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut vol.is_visible, "");
                        ui.label(&vol.name);

                        ui.separator();
                        ui.label("RGB:");
                        ui.radio_value(&mut vol.channel_assignment, 1, "R");
                        ui.radio_value(&mut vol.channel_assignment, 2, "G");
                        ui.radio_value(&mut vol.channel_assignment, 3, "B");
                        ui.radio_value(&mut vol.channel_assignment, 0, "None");
                    });
                }
            });
            ui.collapsing("Horizons", |ui| {
                if ui.button("Add Horizon").clicked() {
                    let name = format!("Horizon {}", self.interpretation.horizons.len() + 1);
                    self.interpretation.add_horizon(Horizon::new(name, [1.0, 1.0, 0.0]));
                }
                ui.separator();
                
                let modifier = ui.input(|i| i.modifiers.command || i.modifiers.shift);
                
                for horizon in &mut self.interpretation.horizons {
                    ui.horizontal(|ui| {
                        let is_active = self.interpretation.active_horizon_id == Some(horizon.id);
                        let is_selected = self.interpretation.selected_horizon_ids.contains(&horizon.id);
                        
                        let response = ui.selectable_label(is_active || is_selected, &horizon.name);
                        if response.clicked() {
                            if modifier {
                                if is_selected {
                                    self.interpretation.selected_horizon_ids.retain(|&id| id != horizon.id);
                                } else {
                                    self.interpretation.selected_horizon_ids.push(horizon.id);
                                }
                            } else {
                                self.interpretation.active_horizon_id = Some(horizon.id);
                                self.interpretation.selected_horizon_ids = vec![horizon.id];
                            }
                        }
                        ui.checkbox(&mut horizon.is_visible, "");
                        ui.label(format!("({} picks)", horizon.picks.len()));
                    });
                }
            });

            ui.collapsing("Faults", |ui| {
                if ui.button("Add Fault").clicked() {
                    let name = format!("Fault {}", self.interpretation.faults.len() + 1);
                    self.interpretation.add_fault(Fault::new(name, [1.0, 0.0, 0.0]));
                }
                ui.separator();

                let modifier = ui.input(|i| i.modifiers.command || i.modifiers.shift);

                for fault in &mut self.interpretation.faults {
                    ui.horizontal(|ui| {
                        let is_active = self.interpretation.active_fault_id == Some(fault.id);
                        let is_selected = self.interpretation.selected_fault_ids.contains(&fault.id);
                        
                        let response = ui.selectable_label(is_active || is_selected, &fault.name);
                        if response.clicked() {
                            if modifier {
                                if is_selected {
                                    self.interpretation.selected_fault_ids.retain(|&id| id != fault.id);
                                } else {
                                    self.interpretation.selected_fault_ids.push(fault.id);
                                }
                            } else {
                                self.interpretation.active_fault_id = Some(fault.id);
                                self.interpretation.selected_fault_ids = vec![fault.id];
                            }
                        }
                        ui.checkbox(&mut fault.is_visible, "");
                        ui.label(format!("({} sticks)", fault.sticks.len()));
                    });
                }
            });
            
            ui.collapsing("Wells", |ui| {
                ui.label("Well-1");
            });
        });

        egui::SidePanel::right("right_panel").show(ctx, |ui| {
            ui.heading("Analysis & Visuals");
            ui.separator();
            
            ui.collapsing("Visuals", |ui| {
                ui.add(egui::Slider::new(&mut self.visuals.gain, 0.1..=10.0).text("Gain"));
                ui.add(egui::Slider::new(&mut self.visuals.clip, 0.0..=1.0).text("Clip"));
                ui.add(egui::Slider::new(&mut self.visuals.opacity, 0.0..=1.0).text("Opacity"));
                egui::ComboBox::from_label("Colormap")
                    .selected_text(&self.visuals.colormap)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.visuals.colormap, "Seismic".to_string(), "Seismic");
                        ui.selectable_value(&mut self.visuals.colormap, "Viridis".to_string(), "Viridis");
                        ui.selectable_value(&mut self.visuals.colormap, "Magma".to_string(), "Magma");
                    });
            });

            ui.separator();
            ui.heading("Volumetrics");
            if self.interpretation.selected_horizon_ids.len() >= 2 {
                if ui.button("Calculate Volume").clicked() {
                    self.calculate_volumetrics();
                }
            } else {
                ui.label("Select 2 horizons to calculate volume");
            }

            if let Some(vol) = self.volumetric_result {
                ui.label(format!("Last Volume: {:.2} m³", vol));
            }

            ui.separator();
            ui.heading("Log Analysis");
            self.crossplot.ui(ui, &[[10.0, 500.0], [20.0, 600.0], [15.0, 550.0]]);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.viewport.ui(ui, &mut self.interpretation, &self.velocity, self.volume.as_ref());
        });
    }
}
