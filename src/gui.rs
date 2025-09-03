use eframe::egui;
use rfd::FileDialog;
use std::path::PathBuf;
use crate::tor_manager::TorManager;
use crate::process_launcher::launch_app;
use crate::divert::start_divert;

pub struct TorLauncherApp {
    app_path: Option<PathBuf>,
    tor_manager: TorManager,
}

impl Default for TorLauncherApp {
    fn default() -> Self {
        Self {
            app_path: None,
            tor_manager: TorManager::new(),
        }
    }
}

impl eframe::App for TorLauncherApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            
            ui.visuals_mut().override_text_color = Some(egui::Color32::WHITE);
            ui.visuals_mut().window_fill = egui::Color32::from_rgb(0, 0, 0);

            ui.heading(egui::RichText::new("Mefestat").color(egui::Color32::from_rgb(200, 33, 30)));

            if ui.add(egui::Button::new("Seleccionar aplicación")).clicked() {
                if let Some(file) = FileDialog::new().add_filter("Executable", &["exe"]).pick_file() {
                    self.app_path = Some(file);
                }
            }

            if let Some(app) = &self.app_path {
                ui.label(egui::RichText::new(format!("App seleccionada: {}", app.display())).color(egui::Color32::WHITE));

                if ui.add(egui::Button::new("Lanzar desde Tor")).clicked() {
                    match self.tor_manager.start_tor() {
                        Ok(_) => {
                            if let Ok(pid) = launch_app(app) {
                                if let Err(e) = start_divert(pid) {
                                    ui.label(egui::RichText::new(format!("Error WinDivert: {}", e))
                                        .color(egui::Color32::from_rgb(255, 0, 0)));
                                }
                            }
                        }
                        Err(e) => {
                            ui.label(egui::RichText::new(format!("Error arrancando Tor: {}", e))
                                .color(egui::Color32::from_rgb(255, 0, 0)));
                        }
                    }
                }
            }
        });
    }
}
