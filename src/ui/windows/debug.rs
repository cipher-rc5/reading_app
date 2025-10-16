// file: src/ui/windows/debug.rs

use crate::services::DatabaseService;
use egui;

pub struct DebugWindow {
    show: bool,
}

impl DebugWindow {
    pub fn new() -> Self {
        Self { show: false }
    }

    pub fn show(&mut self) {
        self.show = true;
    }

    pub fn draw(&mut self, ctx: &egui::Context, database_service: &DatabaseService) {
        if !self.show {
            return;
        }

        egui::Window::new("Debug & Diagnostics")
            .default_width(600.0)
            .resizable(true)
            .show(ctx, |ui| {
                ui.heading("System Information");
                ui.separator();

                ui.label(format!("App Version: {}", env!("CARGO_PKG_VERSION")));
                ui.label(format!("OS: {}", std::env::consts::OS));

                ui.add_space(10.0);
                ui.heading("Services Status");
                ui.separator();

                ui.label("Article Service: Connected");

                // Show actual database status
                let db_status = if database_service.is_available() {
                    "Connected"
                } else {
                    "NOT CONNECTED"
                };
                let color = if database_service.is_available() {
                    egui::Color32::GREEN
                } else {
                    egui::Color32::RED
                };
                ui.colored_label(color, format!("Database Service: {}", db_status));

                ui.add_space(10.0);
                if ui.button("Close").clicked() {
                    self.show = false;
                }
            });
    }
}

impl Default for DebugWindow {
    fn default() -> Self {
        Self::new()
    }
}
