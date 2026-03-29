use eframe::egui;
use uuid::Uuid;

use crate::interpretation::{
    Fault, HistoryManager, Horizon, InterpretationState, PickingMode, VelocityState, WellState,
};
use crate::ui_styles::{self, ThemeManager};
use crate::widgets::crossplot::CrossPlotWidget;
use crate::widgets::fault_properties_panel::FaultPropertiesPanel;
use crate::widgets::horizon_properties_panel::HorizonPropertiesPanel;
use crate::widgets::velocity_panel::VelocityPanel;
use crate::widgets::viewport::ViewportWidget;
use crate::widgets::well_panel::WellPanel;
use sf_compute::seismic::{InMemoryProvider, SeismicVolume};
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
    #[allow(dead_code)]
    name: String,
    viewport: ViewportWidget,
    #[allow(dead_code)]
    crossplot: CrossPlotWidget,
    #[allow(dead_code)]
    fault_properties: FaultPropertiesPanel,
    #[allow(dead_code)]
    horizon_properties: HorizonPropertiesPanel,
    #[allow(dead_code)]
    velocity_panel: VelocityPanel,
    #[allow(dead_code)]
    well_panel: WellPanel,
    interpretation: InterpretationState,
    history: HistoryManager,
    visuals: VisualSettings,
    volume: Option<SeismicVolume>,
    seismic_volumes: Vec<SeismicVolumeEntry>,
    velocity: VelocityState,
    volumetric_result: Option<f32>,
    wells: WellState,
    #[allow(dead_code)]
    theme_manager: ThemeManager,
    #[allow(dead_code)]
    current_project_path: Option<std::path::PathBuf>,
    #[allow(dead_code)]
    recent_projects: Vec<std::path::PathBuf>,
    settings: crate::widgets::settings_panel::SettingsPanel,
    show_settings: bool,
}

impl StrataForgeApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let theme_manager = ThemeManager::new();
        
        // Setup egui with proper icon support
        egui_extras::install_image_loaders(&cc.egui_ctx);

        // Apply theme
        ui_styles::apply_theme(&cc.egui_ctx, theme_manager.current_theme);
        
        let mut interpretation = InterpretationState::new();

        let target_format = cc.wgpu_render_state.as_ref().map(|rs| rs.target_format);
        // Add a default horizon for demo
        let h_id = Uuid::new_v4();
        let mut horizon = Horizon::new("Horizon A".to_string(), [0.0, 1.0, 0.0, 0.7]);
        horizon.id = h_id;
        interpretation.add_horizon(horizon);
        interpretation.active_horizon_id = Some(h_id);

        // Add a default fault for demo
        let f_id = Uuid::new_v4();
        let mut fault = Fault::new("Fault A".to_string(), [1.0, 0.0, 0.0, 0.5]);
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
                if idx > 0 {
                    data[idx - 1] = 0.5;
                }
                if idx < data.len() - 1 {
                    data[idx + 1] = 0.5;
                }
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

        let seismic_volumes = vec![SeismicVolumeEntry {
            id: Uuid::new_v4().to_string(),
            name: "Full Stack".to_string(),
            path: "seismic/full_stack.segy".to_string(),
            is_visible: true,
            channel_assignment: 0,
        }];

        Self {
            name: "MyField".to_owned(),
            viewport,
            crossplot: CrossPlotWidget::new("Gamma Ray", "Depth"),
            fault_properties: FaultPropertiesPanel::new(),
            horizon_properties: HorizonPropertiesPanel::new(),
            velocity_panel: VelocityPanel::new(),
            well_panel: WellPanel::new(),
            wells: WellState::new(),
            interpretation,
            history: HistoryManager::new(),
            visuals: VisualSettings::default(),
            volume,
            seismic_volumes,
            velocity: VelocityState::new(),
            volumetric_result: None,
            theme_manager,
            current_project_path: None,
            recent_projects: Vec::new(),
            settings: crate::widgets::settings_panel::SettingsPanel::new(),
            show_settings: false,
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
                    RbfInterpolator::new(&p2, RbfType::ThinPlateSpline),
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
                    let vol = engine
                        .calculate_volume(&interp1, &interp2, min_x, max_x, min_y, max_y, 50, 50);
                    self.volumetric_result = Some(vol);
                }
            }
        }
    }

    #[allow(dead_code)] // Export feature - used via UI buttons
    fn export_active_horizon(&self, format: &str) {
        use sf_io::export::{xyz::XyzExporter, SurfaceExporter};
        use std::fs::File;
        use std::io::Write;

        if let Some(horizon) = self.interpretation.active_horizon() {
            // Export picks as XYZ or JSON
            let result = match format {
                "xyz" => {
                    let filename = format!("{}_picks.xyz", horizon.name.replace(" ", "_"));
                    let mut file = File::create(&filename);
                    if let Ok(ref mut f) = file {
                        for pick in &horizon.picks {
                            let _ = writeln!(
                                f,
                                "{:.2} {:.2} {:.2}",
                                pick.position[0], pick.position[1], pick.position[2]
                            );
                        }
                        Ok(())
                    } else {
                        Err(anyhow::anyhow!("Failed to create file: {}", filename))
                    }
                }
                "json" => {
                    let filename = format!("{}_picks.json", horizon.name.replace(" ", "_"));
                    // Simple JSON export without serde_json dependency
                    let mut file = File::create(&filename);
                    if let Ok(ref mut f) = file {
                        let _ = writeln!(f, "[");
                        for (i, pick) in horizon.picks.iter().enumerate() {
                            let comma = if i < horizon.picks.len() - 1 { "," } else { "" };
                            let _ = writeln!(
                                f,
                                "  {{\"id\": \"{}\", \"position\": [{:.2}, {:.2}, {:.2}]}}{}",
                                pick.id,
                                pick.position[0],
                                pick.position[1],
                                pick.position[2],
                                comma
                            );
                        }
                        let _ = writeln!(f, "]");
                        Ok(())
                    } else {
                        Err(anyhow::anyhow!("Failed to create file: {}", filename))
                    }
                }
                "mesh_xyz" => {
                    if let Some(mesh) = horizon.meshes.first() {
                        XyzExporter
                            .export_surface(
                                mesh,
                                std::path::Path::new(&format!(
                                    "{}_mesh.xyz",
                                    horizon.name.replace(" ", "_")
                                )),
                            )
                            .map_err(|e| anyhow::anyhow!("Mesh export failed: {}", e))
                    } else {
                        Err(anyhow::anyhow!("No mesh to export"))
                    }
                }
                _ => Ok(()),
            };

            if let Err(e) = result {
                eprintln!("Export failed: {}", e);
            } else {
                println!("Exported {} picks to {}", horizon.name, format);
            }
        }
    }
}

impl eframe::App for StrataForgeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Menu Bar - Native desktop app style
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // File menu
                ui.menu_button("📁 File", |ui| {
                    if ui.button("New Project\tCtrl+N").clicked() {
                        // New project
                    }
                    if ui.button("Open Project\tCtrl+O").clicked() {
                        // Open project
                    }
                    if ui.button("Save Project\tCtrl+S").clicked() {
                        // Save project
                    }
                    ui.separator();
                    if ui.button("Import Seismic\tCtrl+I").clicked() {
                        // Import seismic
                    }
                    if ui.button("Import Well\tCtrl+W").clicked() {
                        // Import well
                    }
                    ui.separator();
                    if ui.button("Exit\tAlt+F4").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                // Tools menu
                ui.menu_button("🔧 Tools", |ui| {
                    if ui.button("Generate Synthetic Data\tCtrl+G").clicked() {
                        // Show synthetic data generator
                    }
                    if ui.button("Velocity Modeling\tV").clicked() {
                        // Show velocity panel
                    }
                    if ui.button("Well-Seismic Tie\tT").clicked() {
                        // Show well tie
                    }
                    ui.separator();
                    if ui.button("⚙️ Settings...").clicked() {
                        self.show_settings = true;
                    }
                });

                // View menu
                ui.menu_button("👁 View", |ui| {
                    if ui.button("Reset View\tR").clicked() {
                        // Reset viewport
                    }
                    if ui.button("Toggle Depth Mode\tD").clicked() {
                        // Toggle depth mode
                    }
                    ui.separator();
                    if ui.button("Fullscreen\tF11").clicked() {
                        // Toggle fullscreen
                    }
                });

                // Help menu
                ui.menu_button("❓ Help", |ui| {
                    if ui.button("Documentation\tF1").clicked() {
                        // Open docs
                    }
                    if ui.button("Check for Updates").clicked() {
                        // Check updates
                    }
                    ui.separator();
                    if ui.button("About StrataForge").clicked() {
                        // Show about dialog
                    }
                });
            });
        });

        // Settings Panel (modal dialog)
        if self.show_settings {
            egui::Window::new("⚙️ Settings")
                .collapsible(false)
                .resizable()
                .default_size([600.0, 500.0])
                .show(ctx, |ui| {
                    if self.settings.ui(ui) {
                        // Settings changed
                    }
                    
                    if ui.button("Close").clicked() {
                        self.show_settings = false;
                    }
                });
        }

        // Top Ribbon - Modern toolbar
        egui::TopBottomPanel::top("top_ribbon").show(ctx, |ui| {
            ui.add_space(4.0);

            // First row - App title and menu
            ui.horizontal(|ui| {
                ui.heading("🛢 StrataForge");
                ui.separator();

                // Quick access toolbar
                if ui.button("💾").clicked() { /* Save */ }
                if ui.button("↶").clicked() {
                    self.history.undo(&mut self.interpretation);
                }
                if ui.button("↷").clicked() {
                    self.history.redo(&mut self.interpretation);
                }

                ui.separator();

                // Context-aware tools
                if self.interpretation.active_horizon_id.is_some() {
                    ui.label("🌈 Horizon");
                } else if self.interpretation.active_fault_id.is_some() {
                    ui.label("⚡ Fault");
                } else {
                    ui.label("📊 Seismic");
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("❓").clicked() { /* Help */ }
                });
            });

            ui.add_space(4.0);
            ui.separator();
            ui.add_space(4.0);

            // Second row - Tools
            ui.horizontal(|ui| {
                // File operations
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        if ui.button("📂 New").clicked() { /* New project */ }
                        if ui.button("📁 Open").clicked() { /* Open */ }
                        if ui.button("💾 Save").clicked() { /* Save */ }
                    });
                });

                ui.separator();

                // Interpretation tools
                ui.group(|ui| {
                    ui.label("Picking:");
                    ui.selectable_value(
                        &mut self.interpretation.picking_mode,
                        PickingMode::None,
                        "⊘",
                    );
                    ui.selectable_value(
                        &mut self.interpretation.picking_mode,
                        PickingMode::Seed,
                        "🌱",
                    );
                    ui.selectable_value(
                        &mut self.interpretation.picking_mode,
                        PickingMode::Manual,
                        "✏️",
                    );
                    ui.selectable_value(
                        &mut self.interpretation.picking_mode,
                        PickingMode::AutoTrack,
                        "🔄",
                    );
                    ui.selectable_value(
                        &mut self.interpretation.picking_mode,
                        PickingMode::SketchFault,
                        "⚡",
                    );
                });

                ui.separator();

                // View controls
                ui.group(|ui| {
                    ui.checkbox(&mut self.velocity.is_depth_mode, "📏 Depth");
                    if self.velocity.is_depth_mode {
                        ui.horizontal(|ui| {
                            ui.label("V0:");
                            ui.add(
                                egui::DragValue::new(&mut self.velocity.model.v0)
                                    .speed(100.0)
                                    .prefix("m/s"),
                            );
                            ui.label("k:");
                            ui.add(
                                egui::DragValue::new(&mut self.velocity.model.k)
                                    .speed(0.01)
                                    .prefix("1/s"),
                            );
                        });
                    }
                });
            });

            ui.add_space(4.0);
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
                    self.interpretation
                        .add_horizon(Horizon::new(name, [1.0, 1.0, 0.0, 0.7]));
                }
                ui.separator();

                let modifier = ui.input(|i| i.modifiers.command || i.modifiers.shift);

                for horizon in &mut self.interpretation.horizons {
                    ui.horizontal(|ui| {
                        let is_active = self.interpretation.active_horizon_id == Some(horizon.id);
                        let is_selected = self
                            .interpretation
                            .selected_horizon_ids
                            .contains(&horizon.id);

                        let response = ui.selectable_label(is_active || is_selected, &horizon.name);
                        if response.clicked() {
                            if modifier {
                                if is_selected {
                                    self.interpretation
                                        .selected_horizon_ids
                                        .retain(|&id| id != horizon.id);
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
                    self.interpretation
                        .add_fault(Fault::new(name, [1.0, 0.0, 0.0, 0.5]));
                }
                ui.separator();

                let modifier = ui.input(|i| i.modifiers.command || i.modifiers.shift);

                for fault in &mut self.interpretation.faults {
                    ui.horizontal(|ui| {
                        let is_active = self.interpretation.active_fault_id == Some(fault.id);
                        let is_selected =
                            self.interpretation.selected_fault_ids.contains(&fault.id);

                        let response = ui.selectable_label(is_active || is_selected, &fault.name);
                        if response.clicked() {
                            if modifier {
                                if is_selected {
                                    self.interpretation
                                        .selected_fault_ids
                                        .retain(|&id| id != fault.id);
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

            // Wells section - integrated with WellPanel
            self.well_panel.ui(ui, &mut self.wells);
        });

        // Right Panel - Properties & Analysis
        egui::SidePanel::right("right_panel")
            .min_width(250.0)
            .max_width(400.0)
            .resizable(true)
            .show(ctx, |ui| {
                ui.add_space(8.0);
                ui.heading("📊 Properties");
                ui.add_space(8.0);
                ui.separator();
                ui.add_space(8.0);

                // Context-aware properties
                if let Some(horizon) = self.interpretation.active_horizon() {
                    ui.collapsing("🌈 Horizon Properties", |ui| {
                        ui.label(format!("Name: {}", horizon.name));
                        ui.horizontal(|ui| {
                            ui.label("Color:");
                            let color = egui::Color32::from_rgba_unmultiplied(
                                (horizon.color[0] * 255.0) as u8,
                                (horizon.color[1] * 255.0) as u8,
                                (horizon.color[2] * 255.0) as u8,
                                255,
                            );
                            let color_rect =
                                ui.allocate_response(egui::vec2(24.0, 24.0), egui::Sense::click());
                            ui.painter().rect_filled(color_rect.rect, 4.0, color);
                        });
                        ui.label(format!("Picks: {}", horizon.picks.len()));
                        ui.label(format!(
                            "Visible: {}",
                            if horizon.is_visible { "Yes" } else { "No" }
                        ));

                        ui.separator();
                        ui.horizontal(|ui| {
                            if ui.button("📤 Export XYZ").clicked() {
                                self.export_active_horizon("xyz");
                            }
                            if ui.button("📤 JSON").clicked() {
                                self.export_active_horizon("json");
                            }
                        });
                    });
                } else if let Some(fault) = self.interpretation.active_fault() {
                    ui.collapsing("⚡ Fault Properties", |ui| {
                        ui.label(format!("Name: {}", fault.name));
                        ui.horizontal(|ui| {
                            ui.label("Color:");
                            let color = egui::Color32::from_rgba_unmultiplied(
                                (fault.color[0] * 255.0) as u8,
                                (fault.color[1] * 255.0) as u8,
                                (fault.color[2] * 255.0) as u8,
                                255,
                            );
                            let color_rect =
                                ui.allocate_response(egui::vec2(24.0, 24.0), egui::Sense::click());
                            ui.painter().rect_filled(color_rect.rect, 4.0, color);
                        });
                        ui.label(format!("Sticks: {}", fault.sticks.len()));
                        ui.label(format!(
                            "Visible: {}",
                            if fault.is_visible { "Yes" } else { "No" }
                        ));
                    });
                } else {
                    ui.label("Select a horizon or fault to view properties");
                }

                ui.add_space(16.0);
                ui.separator();
                ui.add_space(8.0);

                // Velocity Model
                ui.collapsing("📏 Velocity Model", |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Mode:");
                        ui.checkbox(&mut self.velocity.is_depth_mode, "Depth");
                    });
                    if self.velocity.is_depth_mode {
                        ui.add_space(8.0);
                        ui.horizontal(|ui| {
                            ui.label("V0:");
                            ui.add(
                                egui::DragValue::new(&mut self.velocity.model.v0)
                                    .speed(100.0)
                                    .prefix("m/s"),
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label("k:");
                            ui.add(
                                egui::DragValue::new(&mut self.velocity.model.k)
                                    .speed(0.01)
                                    .prefix("1/s"),
                            );
                        });

                        ui.add_space(8.0);
                        ui.separator();
                        ui.label("Velocity Preview:");

                        egui::Grid::new("velocity_preview")
                            .num_columns(2)
                            .spacing([40.0, 4.0])
                            .striped(true)
                            .show(ui, |ui| {
                                ui.label("Depth (m)");
                                ui.label("Velocity (m/s)");
                                ui.end_row();

                                let depths = [0.0, 500.0, 1000.0, 2000.0];
                                for depth in depths {
                                    let velocity =
                                        self.velocity.model.v0 + self.velocity.model.k * depth;
                                    ui.label(format!("{:.0}", depth));
                                    ui.label(format!("{:.0}", velocity));
                                    ui.end_row();
                                }
                            });
                    }
                });

                ui.add_space(16.0);
                ui.separator();
                ui.add_space(8.0);

                // Visual Settings
                ui.collapsing("🎨 Visual Settings", |ui| {
                    ui.add(egui::Slider::new(&mut self.visuals.gain, 0.1..=10.0).text("Gain"));
                    ui.add(egui::Slider::new(&mut self.visuals.clip, 0.0..=1.0).text("Clip"));
                    ui.add(egui::Slider::new(&mut self.visuals.opacity, 0.0..=1.0).text("Opacity"));

                    ui.separator();
                    ui.label("Colormap:");
                    egui::ComboBox::from_label("")
                        .selected_text(&self.visuals.colormap)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.visuals.colormap,
                                "Seismic".to_string(),
                                "🌈 Seismic",
                            );
                            ui.selectable_value(
                                &mut self.visuals.colormap,
                                "Viridis".to_string(),
                                "🟢 Viridis",
                            );
                            ui.selectable_value(
                                &mut self.visuals.colormap,
                                "Magma".to_string(),
                                "🔴 Magma",
                            );
                        });
                });

                ui.add_space(16.0);
                ui.separator();
                ui.add_space(8.0);

                // Volumetrics
                ui.collapsing("📐 Volumetrics", |ui| {
                    if self.interpretation.selected_horizon_ids.len() >= 2 {
                        if ui.button("Calculate Volume").clicked() {
                            self.calculate_volumetrics();
                        }
                        if let Some(vol) = self.volumetric_result {
                            ui.label(format!("Volume: {:.2} m³", vol));
                        }
                    } else {
                        ui.label("Select 2 horizons");
                    }
                });

                ui.add_space(16.0);
                ui.separator();
                ui.add_space(8.0);

                // Wells (quick access)
                ui.collapsing("🛢 Wells", |ui| {
                    for well in &self.wells.wells {
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut well.is_visible.clone(), "");
                            ui.label(&well.name);
                        });
                    }
                    if ui.button("+ Add Well").clicked() {
                        // Demo well creation
                        use sf_core::domain::well::Well;
                        let mut well = Well::new(
                            "New Well".to_string(),
                            "NW-1".to_string(),
                            300000.0,
                            600000.0,
                            75.0,
                        );
                        well.add_top(
                            "Top Sand".to_string(),
                            1200.0,
                            "TOP".to_string(),
                            [1.0, 0.8, 0.0, 1.0],
                        );
                        self.wells.add_well(well);
                    }
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.viewport.ui(
                ui,
                &mut self.interpretation,
                &self.velocity,
                self.volume.as_ref(),
            );
        });

        // Bottom Panel - Well Logs (collapsible)
        egui::TopBottomPanel::bottom("bottom_panel")
            .min_height(100.0)
            .max_height(300.0)
            .resizable(true)
            .show(ctx, |ui| {
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.heading("📊 Well Logs & Crossplots");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("▼").clicked() {
                            // Could toggle collapse here
                        }
                    });
                });
                ui.add_space(4.0);
                ui.separator();
                ui.add_space(8.0);

                if !self.wells.wells.is_empty() {
                    ui.horizontal(|ui| {
                        ui.label("Well:");
                        // Well selector dropdown could go here
                        for well in &self.wells.wells {
                            if ui.selectable_label(false, &well.name).clicked() {
                                // Select well
                            }
                        }

                        ui.separator();
                        ui.label("Log:");
                        let _ = ui.selectable_label(true, "GR");
                        let _ = ui.selectable_label(false, "DT");
                        let _ = ui.selectable_label(false, "RHOB");
                    });

                    ui.add_space(8.0);

                    // Placeholder for log plot
                    ui.label("📈 Log plot area - to be implemented");
                    ui.label("Select a well and log to display");
                } else {
                    ui.label("No wells loaded. Click '+ Add Well' in the left panel.");
                }
            });

        // Status Bar
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Coordinates
                ui.label("📍 Position:");
                ui.monospace("X: 250.5  Y: 312.8  Z: 1523m");

                ui.separator();

                // TWT
                ui.label("⏱ TWT:");
                ui.monospace("1.250s");

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Auto-tracking progress
                    if let Some(progress) = self.viewport.tracking_progress {
                        ui.label("🔄 Auto-Tracking:");
                        ui.add(
                            egui::ProgressBar::new(progress)
                                .show_percentage()
                                .desired_width(100.0),
                        );
                    } else {
                        ui.label("✅ Ready");
                    }
                });
            });
        });
    }
}
