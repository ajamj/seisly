mod ai_client;
mod app;
mod interpretation;
mod project;
mod ui_styles;
mod widgets;
use app::StrataForgeApp;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "StrataForge",
        native_options,
        Box::new(|cc| Box::new(StrataForgeApp::new(cc))),
    )
}
