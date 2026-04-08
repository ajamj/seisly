use eframe::egui;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

use crate::interpretation::{
    Fault,
    HistoryManager,
    Horizon,
    InterpretationState,
    VelocityState,
    WellState,
};
use crate::ui::style::{self, ThemeManager};
use crate::ui::layout::{Tab, SeislyTabViewer};
use crate::widgets::crossplot::CrossPlotWidget;
use crate::widgets::fault_properties_panel::FaultPropertiesPanel;
use crate::widgets::horizon_properties_panel::HorizonPropertiesPanel;
use crate::widgets::velocity_panel::VelocityPanel;
use crate::widgets::viewport::ViewportWidget;
use crate::widgets::well_panel::WellPanel;
use seisly_compute::seismic::{InMemoryProvider, SeismicVolume};
use seisly_plugin::PluginManager;
use seisly_storage::project::SeismicVolumeEntry;

use seisly_attributes_gpu::GpuAttributeComputer;

use crate::widgets::qi_panel::QiPanel;
use crate::widgets::time_lapse_panel::TimeLapsePanel;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SidebarTab {
    Explorer,
    Interpretation,
    QI,
    TimeLapse,
    Search,
    Diagnostics,
    Extensions,
}

#[derive(Clone, Default)]
pub enum ImportState {
    #[default]
    Idle,
    Scanning,
    Scanned(std::path::PathBuf, seisly_io::segy::parser::SegyMetadata),
}

pub struct VisualSettings {
    #[allow(dead_code)]
    pub gain: f32,
    #[allow(dead_code)]
    pub clip: f32,
    #[allow(dead_code)]
    pub opacity: f32,
    #[allow(dead_code)]
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

pub struct SeislyApp {
    #[allow(dead_code)]
    name: String,
    pub(crate) viewport: ViewportWidget,
    pub(crate) crossplot: CrossPlotWidget,
    pub(crate) fault_properties: FaultPropertiesPanel,
    pub(crate) horizon_properties: HorizonPropertiesPanel,
    pub(crate) velocity_panel: VelocityPanel,
    pub(crate) well_panel: WellPanel,
    pub(crate) qi_panel: QiPanel,
    pub(crate) time_lapse_panel: TimeLapsePanel,
    pub(crate) interpretation: InterpretationState,
    pub(crate) history: HistoryManager,
    #[allow(dead_code)]
    pub(crate) visuals: VisualSettings,
    pub(crate) volume: Option<SeismicVolume>,
    pub(crate) seismic_volumes: Vec<SeismicVolumeEntry>,
    pub(crate) velocity: VelocityState,
    #[allow(dead_code)]
    pub(crate) volumetric_result: Option<f32>,
    pub(crate) wells: WellState,
    pub(crate) theme_manager: ThemeManager,
    #[allow(dead_code)]
    pub(crate) current_project_path: Option<std::path::PathBuf>,
    #[allow(dead_code)]
    pub(crate) recent_projects: Vec<std::path::PathBuf>,
    #[allow(dead_code)]
    pub(crate) settings: crate::widgets::settings_panel::SettingsPanel,
    pub(crate) show_settings: bool,
    pub(crate) plugin_manager: PluginManager,
    pub(crate) plugin_panel: crate::widgets::plugin_panel::PluginPanel,
    pub(crate) plugin_results: Vec<serde_json::Value>,
    pub(crate) tree: egui_dock::DockState<Tab>,
    #[allow(dead_code)]
    pub(crate) show_help: bool,
    #[allow(dead_code)]
    pub(crate) show_synthetic_data: bool,
    pub(crate) is_busy: bool,
    pub(crate) busy_message: String,
    pub(crate) import_state: ImportState,

    // IDE Layout state
    pub(crate) show_activity_bar: bool,
    pub(crate) show_sidebar: bool,
    pub(crate) show_bottom_panel: bool,
    pub(crate) active_sidebar_tab: SidebarTab,

    // Phase 2
    pub(crate) gpu_computer: Option<std::sync::Arc<GpuAttributeComputer>>,

    // Performance & Async
    pub(crate) last_theme_name: String,
    pub(crate) tx: std::sync::mpsc::Sender<AppMessage>,
    pub(crate) rx: std::sync::mpsc::Receiver<AppMessage>,
}

pub enum AppMessage {
    ScanComplete(std::path::PathBuf, seisly_io::segy::parser::SegyMetadata),
    ScanFailed(String),
    GpuInitialized(Result<std::sync::Arc<GpuAttributeComputer>, String>),
    GpuAttributeResult(String, Vec<f32>),
}

impl SeislyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut theme_manager = ThemeManager::new();
        if let Some(theme_name) = cc.storage
            .and_then(|s| s.get_string("seisly_theme")) {
            theme_manager.set_theme(&theme_name);
        }

        egui_extras::install_image_loaders(&cc.egui_ctx);
        style::apply_theme(&cc.egui_ctx, &theme_manager.current_theme);
        
        let mut interpretation = InterpretationState::new();
        let target_format = cc.wgpu_render_state.as_ref().map(|rs| rs.target_format);
        
        // Default data
        let h_id = Uuid::new_v4();
        let mut horizon = Horizon::new("Horizon A".to_string(), [0.0, 1.0, 0.0, 0.7]);
        horizon.id = h_id;
        interpretation.add_horizon(horizon);
        interpretation.active_horizon_id = Some(h_id);

        let f_id = Uuid::new_v4();
        let mut fault = Fault::new("Fault A".to_string(), [1.0, 0.0, 0.0, 0.5]);
        fault.id = f_id;
        interpretation.add_fault(fault);
        interpretation.active_fault_id = Some(f_id);

        let sample_count = 512;
        let inline_range = (0, 500);
        let crossline_range = (0, 500);
        let mut data = vec![0.0; 501 * 501 * sample_count];
        for i in 0..501 {
            for j in 0..501 {
                let idx = (i * 501 + j) * sample_count + 250;
                data[idx] = 1.0;
            }
        }

        let provider = InMemoryProvider { data, inline_range, crossline_range, sample_count };
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

        let mut plugin_manager = PluginManager::new();
        let _ = plugin_manager.discover(std::path::Path::new("plugins"));

        let mut tree = cc.storage
            .and_then(|s| s.get_string("seisly_dock_tree"))
            .and_then(|json| serde_json::from_str(&json).ok())
            .unwrap_or_else(Self::default_tree);

        // Ensure Logs tab is available if it was missing from saved state
        let has_logs = tree.iter_all_tabs().any(|(_, t)| matches!(t, Tab::Logs));
        if !has_logs {
            tree.main_surface_mut().push_to_focused_leaf(Tab::Logs);
        }

        let (show_activity_bar, show_sidebar, show_bottom_panel, active_sidebar_tab) = cc.storage
            .and_then(|s| s.get_string("seisly_ui_state"))
            .and_then(|json| serde_json::from_str::<(bool, bool, bool, SidebarTab)>(&json).ok())
            .unwrap_or((true, true, true, SidebarTab::Explorer));

        let (tx, rx) = std::sync::mpsc::channel();

        // Phase 2: Async GPU Initialization
        let tx_gpu = tx.clone();
        let egui_ctx = cc.egui_ctx.clone();
        std::thread::spawn(move || {
            log::info!("GPU Initialization started...");
            let result = pollster::block_on(GpuAttributeComputer::new());
            match result {
                Ok(computer) => {
                    let _ = tx_gpu.send(AppMessage::GpuInitialized(Ok(std::sync::Arc::new(computer))));
                }
                Err(e) => {
                    let _ = tx_gpu.send(AppMessage::GpuInitialized(Err::<std::sync::Arc<GpuAttributeComputer>, String>(e.to_string())));
                }
            }
            egui_ctx.request_repaint();
        });

        let mut settings = crate::widgets::settings_panel::SettingsPanel::new();
        settings.settings.theme = theme_manager.current_theme.name.clone();

        Self {
            name: "MyField".to_owned(),
            viewport,
            crossplot: CrossPlotWidget::new("Gamma Ray", "Depth"),
            fault_properties: FaultPropertiesPanel::new(),
            horizon_properties: HorizonPropertiesPanel::new(),
            velocity_panel: VelocityPanel::new(),
            well_panel: WellPanel::new(),
            qi_panel: QiPanel::new(),
            time_lapse_panel: TimeLapsePanel::new(),
            wells: WellState::new(),
            interpretation,
            history: HistoryManager::new(100),
            visuals: VisualSettings::default(),
            volume,
            seismic_volumes,
            velocity: VelocityState::new(),
            volumetric_result: None,
            theme_manager,
            current_project_path: None,
            recent_projects: Vec::new(),
            settings,
            show_settings: false,
            show_help: false,
            show_synthetic_data: false,
            is_busy: false,
            busy_message: String::new(),
            import_state: ImportState::Idle,
            plugin_manager,
            plugin_panel: crate::widgets::plugin_panel::PluginPanel::new(),
            plugin_results: Vec::new(),
            tree,
            show_activity_bar,
            show_sidebar,
            show_bottom_panel,
            active_sidebar_tab,
            gpu_computer: None,
            last_theme_name: String::new(),
            tx,
            rx,
        }
    }

    fn default_tree() -> egui_dock::DockState<Tab> {
        let tree = egui_dock::DockState::new(vec![Tab::Viewport]);
        tree
    }

    fn show_loading_overlay(&self, ctx: &egui::Context) {
        if self.is_busy {
            egui::Area::new("loading_overlay".into())
                .order(egui::Order::Foreground)
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .show(ctx, |ui| {
                    let theme = &self.theme_manager.current_theme;
                    egui::Frame::window(ui.style())
                        .fill(theme.panel_bg.gamma_multiply(0.8))
                        .rounding(8.0)
                        .show(ui, |ui| {
                            ui.vertical_centered(|ui| {
                                ui.add_space(16.0);
                                ui.add(egui::Spinner::new().size(40.0));
                                ui.add_space(8.0);
                                ui.heading(&self.busy_message);
                                ui.add_space(16.0);
                            });
                        });
                });
        }
    }

    fn render_activity_bar(&mut self, ctx: &egui::Context) {
        if !self.show_activity_bar { return; }
        let theme_bg = self.theme_manager.current_theme.activity_bar_bg;
        let active_icon_color = self.theme_manager.current_theme.activity_bar_active_icon;
        let inactive_icon_color = self.theme_manager.current_theme.activity_bar_inactive_icon;

        egui::SidePanel::left("activity_bar")
            .exact_width(crate::ui::style::spacing::ACTIVITY_BAR_WIDTH)
            .frame(egui::Frame::none().fill(theme_bg))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(8.0);
                    self.activity_button(ui, SidebarTab::Explorer, egui::include_image!("../assets/icons/files.svg"), "Explorer");
                    self.activity_button(ui, SidebarTab::Interpretation, egui::include_image!("../assets/icons/horizon.svg"), "Interpretation");
                    self.activity_button(ui, SidebarTab::QI, egui::include_image!("../assets/icons/qi.svg"), "Quantitative Interpretation");
                    self.activity_button(ui, SidebarTab::TimeLapse, egui::include_image!("../assets/icons/time_lapse.svg"), "4D Monitoring");
                    self.activity_button(ui, SidebarTab::Search, egui::include_image!("../assets/icons/search.svg"), "Search");
                    self.activity_button(ui, SidebarTab::Diagnostics, egui::include_image!("../assets/icons/terminal.svg"), "Diagnostics");
                    
                    ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                        ui.add_space(8.0);
                        let settings_icon = egui::include_image!("../assets/icons/settings.svg");
                        let tint = if self.show_settings { active_icon_color } else { inactive_icon_color };
                        if ui.add(egui::ImageButton::new(egui::Image::new(settings_icon).tint(tint)).frame(false))
                            .on_hover_text("Settings")
                            .clicked() { 
                            self.show_settings = !self.show_settings; 
                        }
                        self.activity_button(ui, SidebarTab::Extensions, egui::include_image!("../assets/icons/fault.svg"), "Plugins");
                    });
                });
            });
    }

    fn activity_button(&mut self, ui: &mut egui::Ui, tab: SidebarTab, icon: egui::ImageSource<'_>, tooltip: &str) {
        let is_active = (self.show_sidebar && self.active_sidebar_tab == tab) || (tab == SidebarTab::Diagnostics && self.show_bottom_panel);
        let theme = &self.theme_manager.current_theme;
        let tint = if is_active { theme.activity_bar_active_icon } else { theme.activity_bar_inactive_icon };
        let button = egui::ImageButton::new(egui::Image::new(icon).tint(tint)).frame(false);
        let response = ui.add(button);
        if response.clicked() {
            if tab == SidebarTab::Diagnostics {
                self.show_bottom_panel = !self.show_bottom_panel;
            } else {
                if is_active { self.show_sidebar = false; }
                else { self.show_sidebar = true; self.active_sidebar_tab = tab; }
            }
        }
        response.clone().on_hover_text(tooltip);
        if is_active {
            let rect = response.rect;
            ui.painter().rect_filled(
                egui::Rect::from_min_max(egui::pos2(rect.min.x, rect.min.y + 4.0), egui::pos2(rect.min.x + 2.0, rect.max.y - 4.0)),
                0.0, theme.accent
            );
        }
    }

    fn render_sidebar(&mut self, ctx: &egui::Context) {
        if !self.show_sidebar { return; }
        let theme = self.theme_manager.current_theme.clone();
        let active_tab = self.active_sidebar_tab;
        egui::SidePanel::left("sidebar")
            .default_width(crate::ui::style::spacing::SIDEBAR_DEFAULT_WIDTH)
            .resizable(true)
            .frame(egui::Frame::none().fill(theme.side_bar_bg).inner_margin(8.0))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(match active_tab {
                        SidebarTab::Explorer => "EXPLORER",
                        SidebarTab::Interpretation => "INTERPRETATION",
                        SidebarTab::QI => "QUANTITATIVE INTERPRETATION",
                        SidebarTab::TimeLapse => "4D MONITORING",
                        SidebarTab::Search => "SEARCH",
                        SidebarTab::Diagnostics => "DIAGNOSTICS",
                        SidebarTab::Extensions => "PLUGINS",
                    }).strong().color(theme.side_bar_header_fg));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("X").clicked() { self.show_sidebar = false; }
                    });
                });
                ui.separator();
                match active_tab {
                    SidebarTab::Explorer => self.render_project_explorer(ui),
                    SidebarTab::Interpretation => self.render_interpretation_panel(ui),
                    SidebarTab::QI => self.qi_panel.ui(ui, &mut self.gpu_computer, &self.tx, ctx),
                    SidebarTab::TimeLapse => self.time_lapse_panel.ui(ui, &self.seismic_volumes),
                    SidebarTab::Search => { ui.label("Search implementation coming soon..."); },
                    SidebarTab::Diagnostics => { ui.label("Diagnostics (Logs) are shown in the bottom panel."); },
                    SidebarTab::Extensions => self.render_plugins(ui),
                }
            });
    }

    fn render_interpretation_panel(&mut self, ui: &mut egui::Ui) {
        ui.collapsing("Horizons", |ui| {
            if ui.button("Add Horizon").clicked() {
                let name = format!("Horizon {}", self.interpretation.horizons.len() + 1);
                self.interpretation.add_horizon(Horizon::new(name, [1.0, 1.0, 0.0, 0.7]));
            }
            let mut to_remove = None;
            for horizon in &mut self.interpretation.horizons {
                ui.horizontal(|ui| {
                    let is_active = self.interpretation.active_horizon_id == Some(horizon.id);
                    if ui.selectable_label(is_active, &horizon.name).clicked() {
                        self.interpretation.active_horizon_id = Some(horizon.id);
                    }
                    ui.checkbox(&mut horizon.is_visible, "");
                    if ui.button("Delete").clicked() { to_remove = Some(horizon.id); }
                });
            }
            if let Some(id) = to_remove { self.interpretation.horizons.retain(|h| h.id != id); }
        });
        ui.collapsing("Faults", |ui| {
            if ui.button("Add Fault").clicked() {
                let name = format!("Fault {}", self.interpretation.faults.len() + 1);
                self.interpretation.add_fault(Fault::new(name, [1.0, 0.0, 0.0, 0.5]));
            }
            let mut to_remove = None;
            for fault in &mut self.interpretation.faults {
                ui.horizontal(|ui| {
                    let is_active = self.interpretation.active_fault_id == Some(fault.id);
                    if ui.selectable_label(is_active, &fault.name).clicked() {
                        self.interpretation.active_fault_id = Some(fault.id);
                    }
                    ui.checkbox(&mut fault.is_visible, "");
                    if ui.button("Delete").clicked() { to_remove = Some(fault.id); }
                });
            }
            if let Some(id) = to_remove { self.interpretation.faults.retain(|f| f.id != id); }
        });
    }

    pub fn render_viewport(&mut self, ui: &mut egui::Ui) {
        self.viewport.ui(ui, &mut self.interpretation, &mut self.history, &self.velocity, self.volume.as_ref());
    }

    pub fn render_project_explorer(&mut self, ui: &mut egui::Ui) {
        ui.collapsing("Seismic Volumes", |ui| {
            for vol in &mut self.seismic_volumes {
                ui.horizontal(|ui| {
                    ui.checkbox(&mut vol.is_visible, "");
                    ui.label(&vol.name);
                });
            }
        });
        self.well_panel.ui(ui, &mut self.wells);
    }

    pub fn render_properties(&mut self, ui: &mut egui::Ui) {
        ui.heading("Properties");
        ui.separator();
        if self.interpretation.active_horizon_id.is_some() {
            self.horizon_properties.ui(ui, &mut self.interpretation);
        } else if self.interpretation.active_fault_id.is_some() {
            self.fault_properties.ui(ui, &mut self.interpretation);
        } else {
            ui.label("Select an entity to view properties");
        }
    }

    pub fn render_well_logs(&mut self, ui: &mut egui::Ui) {
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.heading("Well Logs & Crossplots");
        });
        ui.add_space(4.0);
        ui.separator();
        ui.add_space(8.0);

        if !self.wells.wells.is_empty() {
            ui.horizontal(|ui| {
                ui.label("Well:");
                for well in &self.wells.wells {
                    let is_selected = self.wells.active_well_id == Some(well.id);
                    if ui.selectable_label(is_selected, &well.name).clicked() {
                        self.wells.active_well_id = Some(well.id);
                        self.well_panel.selected_curve_id = None; // Reset curve selection
                    }
                }
            });
            ui.separator();

            if let Some(well) = self.wells.active_well() {
                ui.horizontal(|ui| {
                    ui.label("Log:");
                    for log in &well.logs {
                        let is_selected = self.well_panel.selected_curve_id == Some(log.id);
                        if ui.selectable_label(is_selected, &log.mnemonic).clicked() {
                            self.well_panel.selected_curve_id = Some(log.id);
                        }
                    }
                });

                ui.add_space(8.0);

                if let Some(curve_id) = self.well_panel.selected_curve_id {
                    if let Some(log) = well.logs.iter().find(|l| l.id == curve_id) {
                        let points: egui_plot::PlotPoints = log.data.iter().zip(log.depths.iter())
                            .filter(|(&val, _)| val != -999.25 && !val.is_nan()) // Filter null values
                            .map(|(&val, &depth)| [val as f64, -depth as f64]) // Negative depth so 0 is at top
                            .collect();
                            
                        let line = egui_plot::Line::new(points).name(&log.mnemonic);

                        egui_plot::Plot::new("well_log_plot")
                            .view_aspect(0.3)
                            .y_axis_formatter(|mark, _range| format!("{:.0} m", -mark.value))
                            .label_formatter(|name, value| {
                                format!("{}: {:.2}\nDepth: {:.2} m", name, value.x, -value.y)
                            })
                            .show(ui, |plot_ui| {
                                plot_ui.line(line);
                            });
                    }
                } else {
                    ui.label("Select a log curve to visualize.");
                }
            }
        } else {
            ui.label("No wells loaded. Click 'Import Well (LAS)' in the Explorer.");
        }
    }

    pub fn render_plugins(&mut self, ui: &mut egui::Ui) {
        self.plugin_panel.ui(ui, &mut self.plugin_manager, &mut self.plugin_results);
    }

    pub fn render_crossplot(&mut self, ui: &mut egui::Ui) {
        self.crossplot.ui(ui, &[]);
    }

    pub fn render_velocity(&mut self, ui: &mut egui::Ui) {
        self.velocity_panel.ui(ui, &mut self.velocity);
    }

    pub fn render_logs(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                if ui.button("Clear").clicked() {
                    if let Ok(mut buffer) = crate::diagnostics::GLOBAL_LOGS.lock() {
                        buffer.clear();
                    }
                }
            });
            ui.separator();
            egui::ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui| {
                if let Ok(buffer) = crate::diagnostics::GLOBAL_LOGS.lock() {
                    for entry in buffer.entries() {
                        let color = match entry.level {
                            log::Level::Error => egui::Color32::RED,
                            log::Level::Warn => egui::Color32::YELLOW,
                            _ => ui.visuals().text_color(),
                        };
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new(format!("[{}]", entry.timestamp.format("%H:%M:%S"))).weak());
                            ui.label(egui::RichText::new(format!("{:?}", entry.level)).color(color).strong());
                            ui.label(&entry.message);
                        });
                    }
                }
            });
        });
    }

    #[allow(dead_code)]
    fn calculate_volumetrics(&mut self) {
        if self.interpretation.selected_horizon_ids.len() < 2 { return; }
    }

    #[allow(dead_code)]
    fn export_active_horizon(&self, format: &str) {
        if let Some(horizon) = self.interpretation.active_horizon() {
            println!("Exporting {} to {}", horizon.name, format);
        }
    }

    #[allow(dead_code)]
    fn handle_plugin_result(&mut self, result: serde_json::Value) {
        println!("Plugin result received: {:?}", result);
    }

    fn new_project(&mut self) {
        use rfd::FileDialog;
        if let Some(path) = FileDialog::new().set_title("New Project").save_file() {
            self.current_project_path = Some(path);
        }
    }

    fn open_project(&mut self) {
        use rfd::FileDialog;
        if let Some(path) = FileDialog::new().set_title("Open Project").pick_file() {
            self.current_project_path = Some(path);
        }
    }

    fn save_project(&mut self) {
        println!("Project saved.");
    }

    fn import_seismic(&mut self) {
        use rfd::FileDialog;
        if let Some(path) = FileDialog::new()
            .set_title("Import Seismic Data")
            .add_filter("SEG-Y File", &["segy", "sgy"])
            .pick_file()
        {
            self.import_state = ImportState::Scanning;
            self.is_busy = true;
            self.busy_message = format!("Scanning: {}", path.file_name().unwrap().to_string_lossy());
            
            let path_clone = path.clone();
            let tx = self.tx.clone();
            
            std::thread::spawn(move || {
                match seisly_io::segy::parser::parse_metadata(&path_clone) {
                    Ok(metadata) => {
                        let _ = tx.send(AppMessage::ScanComplete(path_clone, metadata));
                    }
                    Err(e) => {
                        let _ = tx.send(AppMessage::ScanFailed(e.to_string()));
                    }
                }
            });
        }
    }

    #[allow(dead_code)]
    fn import_well(&mut self) {
        use rfd::FileDialog;
        if let Some(path) = FileDialog::new()
            .set_title("Import Well Data")
            .add_filter("LAS File", &["las"])
            .pick_file()
        {
            match seisly_io::las::parser::LasParser::read(&path) {
                Ok(mut well) => {
                    if well.name.is_empty() {
                        well.name = path.file_stem().unwrap_or_default().to_string_lossy().to_string();
                    }
                    self.wells.active_well_id = Some(well.id);
                    self.wells.wells.push(well);
                    log::info!("Successfully imported well: {:?}", path);
                }
                Err(e) => {
                    log::error!("Failed to import well: {}", e);
                }
            }
        }
    }
}

impl eframe::App for SeislyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle background messages
        while let Ok(msg) = self.rx.try_recv() {
            match msg {
                AppMessage::ScanComplete(path, metadata) => {
                    self.import_state = ImportState::Scanned(path, metadata);
                    self.is_busy = false;
                }
                AppMessage::ScanFailed(err) => {
                    log::error!("Scan failed: {}", err);
                    self.is_busy = false;
                    self.import_state = ImportState::Idle;
                }
                AppMessage::GpuInitialized(result) => {
                    match result {
                        Ok(computer) => {
                            log::info!("GPU Accelerator initialized successfully.");
                            self.gpu_computer = Some(computer);
                        }
                        Err(e) => {
                            log::warn!("GPU Initialization failed: {}. Falling back to CPU.", e);
                        }
                    }
                }
                AppMessage::GpuAttributeResult(name, data) => {
                    log::info!("GPU computation '{}' finished with {} samples.", name, data.len());
                    self.is_busy = false;
                }
            }
        }

        crate::ui::shortcuts::handle_shortcuts(ctx, self);
        
        let theme = self.theme_manager.current_theme.clone();
        if theme.name != self.last_theme_name {
            style::apply_theme(ctx, &theme);
            self.last_theme_name = theme.name.clone();
        }

        // 1. Menu Bar
        egui::TopBottomPanel::top("menu_bar")
            .frame(egui::Frame::none().fill(theme.side_bar_bg).inner_margin(4.0))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // Quick access tools
                    let icon_tint = theme.activity_bar_inactive_icon;
                    
                    if ui.add(egui::Button::image(egui::Image::new(egui::include_image!("../assets/icons/undo.svg")).tint(icon_tint)).frame(false))
                        .on_hover_text("Undo (Ctrl+Z)")
                        .clicked() {
                        self.history.undo(&mut self.interpretation);
                    }
                    
                    if ui.add(egui::Button::image(egui::Image::new(egui::include_image!("../assets/icons/redo.svg")).tint(icon_tint)).frame(false))
                        .on_hover_text("Redo (Ctrl+Y)")
                        .clicked() {
                        self.history.redo(&mut self.interpretation);
                    }
                    
                    ui.separator();

                    ui.menu_button("File", |ui| {
                        if ui.button("New Project").clicked() { self.new_project(); }
                        if ui.button("Open Project").clicked() { self.open_project(); }
                        if ui.button("Save Project").clicked() { self.save_project(); }
                        ui.separator();
                        if ui.button("Import Seismic").clicked() { self.import_seismic(); }
                        if ui.button("Exit").clicked() { ctx.send_viewport_cmd(egui::ViewportCommand::Close); }
                    });
                    ui.menu_button("View", |ui| {
                        ui.checkbox(&mut self.show_activity_bar, "Activity Bar");
                        ui.checkbox(&mut self.show_sidebar, "Side Bar");
                        ui.checkbox(&mut self.show_bottom_panel, "Bottom Panel");
                        ui.separator();
                        if ui.button("Reset Layout").clicked() { self.tree = Self::default_tree(); }
                        if ui.button(self.theme_manager.icon()).clicked() {
                            self.theme_manager.toggle();
                        }
                    });
                });
            });

        // 2. Activity Bar (Fixed Left)
        self.render_activity_bar(ctx);

        // 3. Sidebar (Dynamic Left)
        self.render_sidebar(ctx);

        // 4. Status Bar (Fixed Bottom)
        egui::TopBottomPanel::bottom("status_bar")
            .exact_height(crate::ui::style::spacing::STATUS_BAR_HEIGHT)
            .frame(egui::Frame::none().fill(theme.status_bar_bg))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Ready").color(theme.status_bar_fg).size(10.0));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(egui::RichText::new("X: 250.5  Y: 312.8  Z: 1523m ").color(theme.status_bar_fg).size(10.0));
                    });
                });
            });

        // 5. Bottom Panel (Toggleable)
        if self.show_bottom_panel {
            egui::TopBottomPanel::bottom("bottom_panel")
                .resizable(true)
                .default_height(crate::ui::style::spacing::BOTTOM_PANEL_DEFAULT_HEIGHT)
                .frame(egui::Frame::none().fill(theme.panel_bg).inner_margin(8.0))
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("LOGS").strong());
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("X").clicked() { self.show_bottom_panel = false; }
                        });
                    });
                    ui.separator();
                    self.render_logs(ui);
                });
        }

        // 6. Central Editor Area
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(theme.editor_bg))
            .show(ctx, |ui| {
                let mut tree = std::mem::replace(&mut self.tree, egui_dock::DockState::new(vec![]));
                let mut viewer = SeislyTabViewer { app: self };
                egui_dock::DockArea::new(&mut tree)
                    .style(egui_dock::Style::from_egui(ui.style()))
                    .show_inside(ui, &mut viewer);
                self.tree = tree;
            });

        // Import Wizard Popup
        if let ImportState::Scanned(path, metadata) = self.import_state.clone() {
            egui::Window::new("SEG-Y Import Wizard")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.heading("File Metadata");
                    ui.label(format!("Path: {:?}", path));
                    ui.separator();
                    egui::Grid::new("metadata_grid").show(ui, |ui| {
                        ui.label("Samples per Trace:");
                        ui.label(metadata.sample_count.to_string());
                        ui.end_row();
                        ui.label("Sample Interval:");
                        ui.label(format!("{} µs", metadata.sample_interval));
                        ui.end_row();
                        ui.label("Data Format:");
                        ui.label(match metadata.format {
                            1 => "IBM Float",
                            5 => "IEEE Float",
                            _ => "Unknown",
                        });
                        ui.end_row();
                    });
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() { self.import_state = ImportState::Idle; }
                        if ui.button("Confirm Import").clicked() {
                            let name = path.file_stem().unwrap().to_string_lossy().to_string();
                            self.seismic_volumes.push(SeismicVolumeEntry {
                                id: Uuid::new_v4().to_string(),
                                name,
                                path: path.to_string_lossy().to_string(),
                                is_visible: true,
                                channel_assignment: 0,
                            });
                            self.import_state = ImportState::Idle;
                        }
                    });
                });
        }

        self.show_loading_overlay(ctx);

        if self.show_settings {
            egui::Window::new("Settings")
                .open(&mut self.show_settings)
                .show(ctx, |ui| {
                    if self.settings.ui(ui) {
                        self.theme_manager.set_theme(&self.settings.settings.theme);
                    }
                });
        }
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        if let Ok(json) = serde_json::to_string(&self.tree) {
            storage.set_string("seisly_dock_tree", json);
        }
        if let Ok(json) = serde_json::to_string(&(self.show_activity_bar, self.show_sidebar, self.show_bottom_panel, self.active_sidebar_tab)) {
            storage.set_string("seisly_ui_state", json);
        }
        storage.set_string("seisly_theme", self.theme_manager.current_theme.name.clone());
    }
}
