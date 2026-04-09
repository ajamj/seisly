mod ai_client;
mod app;
mod diagnostics;
mod interpretation;
mod project;
mod ui;
mod widgets;
use app::SeislyApp;

fn main() -> eframe::Result<()> {
    // Initialize custom logger
    let _ = diagnostics::init();

    // Sentry initialization (uses SENTRY_DSN env var if present)
    let _guard = sentry::init(sentry::ClientOptions {
        release: sentry::release_name!(),
        ..Default::default()
    });

    // Custom panic hook for user-friendly error reporting
    std::panic::set_hook(Box::new(|panic_info| {
        let message = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            "Unknown panic".to_string()
        };

        let location = panic_info
            .location()
            .map(|loc| format!(" at {}:{}", loc.file(), loc.line()))
            .unwrap_or_default();

        let error_msg = format!("Seisly encountered a fatal error and must close.\n\nError: {}{}\n\nA report has been prepared for our engineering team.", message, location);

        // Report to Sentry
        sentry::integrations::panic::panic_handler(panic_info);

        // Show native message dialog
        rfd::MessageDialog::new()
            .set_title("Seisly Crash Reporter")
            .set_description(&error_msg)
            .set_level(rfd::MessageLevel::Error)
            .show();
    }));

    // Handle test panic argument
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--test-panic".to_string()) {
        panic!("Manual test panic triggered via --test-panic flag.");
    }

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Seisly",
        native_options,
        Box::new(|cc| Ok(Box::new(SeislyApp::new(cc)))),
    )
}
