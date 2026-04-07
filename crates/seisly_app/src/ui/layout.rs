use eframe::egui;
use egui_dock::TabViewer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Tab {
    #[default]
    Viewport,
    ProjectExplorer,
    Properties,
    WellLogs,
    Plugins,
    CrossPlot,
    Velocity,
    Logs,
}

pub struct SeislyTabViewer<'a> {
    pub app: &'a mut crate::app::SeislyApp,
}

impl<'a> TabViewer for SeislyTabViewer<'a> {
    type Tab = Tab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        match tab {
            Tab::Viewport => "🌐 Viewport".into(),
            Tab::ProjectExplorer => "📂 Project Explorer".into(),
            Tab::Properties => "📊 Properties".into(),
            Tab::WellLogs => "📈 Well Logs".into(),
            Tab::Plugins => "🧩 Plugins".into(),
            Tab::CrossPlot => "📉 Crossplot".into(),
            Tab::Velocity => "📏 Velocity".into(),
            Tab::Logs => "📜 Logs".into(),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            Tab::Viewport => {
                self.app.render_viewport(ui);
            }
            Tab::ProjectExplorer => {
                self.app.render_project_explorer(ui);
            }
            Tab::Properties => {
                self.app.render_properties(ui);
            }
            Tab::WellLogs => {
                self.app.render_well_logs(ui);
            }
            Tab::Plugins => {
                self.app.render_plugins(ui);
            }
            Tab::CrossPlot => {
                self.app.render_crossplot(ui);
            }
            Tab::Velocity => {
                self.app.render_velocity(ui);
            }
            Tab::Logs => {
                self.app.render_logs(ui);
            }
        }
    }
}
