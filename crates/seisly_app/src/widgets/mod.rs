pub mod crossplot;
pub mod fault_properties_panel;
pub mod horizon_properties_panel;
pub mod plugin_panel;
pub mod qi_panel;
pub mod settings_panel;
pub mod synthetic_data;
pub mod time_lapse_panel;
pub mod velocity_panel;
pub mod viewport;
pub mod well_panel;

// Re-exports for app.rs - allowed unused as they are public API
#[allow(unused_imports)]
pub use crossplot::CrossPlotWidget;
#[allow(unused_imports)]
pub use fault_properties_panel::FaultPropertiesPanel;
#[allow(unused_imports)]
pub use horizon_properties_panel::HorizonPropertiesPanel;
#[allow(unused_imports)]
pub use plugin_panel::PluginPanel;
#[allow(unused_imports)]
pub use qi_panel::QiPanel;
#[allow(unused_imports)]
pub use settings_panel::SettingsPanel;
#[allow(unused_imports)]
pub use synthetic_data::SyntheticDataWidget;
#[allow(unused_imports)]
pub use time_lapse_panel::TimeLapsePanel;
#[allow(unused_imports)]
pub use velocity_panel::VelocityPanel;
#[allow(unused_imports)]
pub use viewport::ViewportWidget;
#[allow(unused_imports)]
pub use well_panel::WellPanel;
