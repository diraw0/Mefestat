mod gui;
mod tor_manager;
mod process_launcher;
mod divert;
mod config;
mod logger;
mod errors;

use eframe::NativeOptions;

fn main() -> eframe::Result<()> {
    logger::init_logging();

    let options = NativeOptions {
        initial_window_size: Some(egui::vec2(500.0, 350.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Tor App Launcher",
        options,
        Box::new(|_cc| Box::new(gui::TorLauncherApp::default())),
    )
}
