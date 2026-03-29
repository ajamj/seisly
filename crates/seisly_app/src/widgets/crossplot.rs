use eframe::egui;

pub struct CrossPlotWidget {
    x_attr: String,
    y_attr: String,
}

impl CrossPlotWidget {
    pub fn new(x_attr: &str, y_attr: &str) -> Self {
        Self {
            x_attr: x_attr.to_string(),
            y_attr: y_attr.to_string(),
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, _data: &[[f32; 2]]) {
        ui.label(format!("Cross-plot: {} vs {}", self.x_attr, self.y_attr));
    }
}
