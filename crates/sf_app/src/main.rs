mod app;
mod widgets;
mod ai_client;
use app::StrataForgeApp;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "StrataForge",
        native_options,
        Box::new(|cc| Box::new(StrataForgeApp::new(cc))),
    )
}
