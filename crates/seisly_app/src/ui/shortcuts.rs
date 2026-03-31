use eframe::egui;
use crate::app::SeislyApp;

pub fn handle_shortcuts(ctx: &egui::Context, app: &mut SeislyApp) {
    ctx.input_mut(|i| {
        // History
        if i.consume_shortcut(&egui::KeyboardShortcut::new(egui::Modifiers::COMMAND, egui::Key::Z)) {
            app.history.undo(&mut app.interpretation);
        }
        if i.consume_shortcut(&egui::KeyboardShortcut::new(egui::Modifiers::COMMAND, egui::Key::Y)) {
            app.history.redo(&mut app.interpretation);
        }

        // File
        if i.consume_shortcut(&egui::KeyboardShortcut::new(egui::Modifiers::COMMAND, egui::Key::S)) {
            println!("Quick Save triggered");
        }
        if i.consume_shortcut(&egui::KeyboardShortcut::new(egui::Modifiers::COMMAND, egui::Key::N)) {
            println!("New Project triggered");
        }

        // View
        if i.consume_shortcut(&egui::KeyboardShortcut::new(egui::Modifiers::NONE, egui::Key::D)) {
            app.velocity.is_depth_mode = !app.velocity.is_depth_mode;
        }
        if i.consume_shortcut(&egui::KeyboardShortcut::new(egui::Modifiers::NONE, egui::Key::R)) {
            // Reset view logic could go here
        }
    });
}
