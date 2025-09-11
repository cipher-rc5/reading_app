// file: src/ui/components/status_bar.rs
// description: Status bar component

use crate::types::RequestStatus;
use egui;

pub struct StatusBar;

impl StatusBar {
    pub fn new() -> Self {
        Self
    }
}

impl Default for StatusBar {
    fn default() -> Self {
        Self::new()
    }
}

impl StatusBar {
    pub fn draw(&self, ui: &mut egui::Ui, status: &RequestStatus) {
        ui.horizontal(|ui| {
            let (status_text, color) = match status {
                RequestStatus::Idle => ("Ready", egui::Color32::GREEN),
                RequestStatus::Loading => ("Generating...", egui::Color32::YELLOW),
                RequestStatus::Success(_) => ("Article loaded", egui::Color32::GREEN),
                RequestStatus::Error(_) => ("Error", egui::Color32::RED),
            };

            ui.colored_label(color, status_text);
        });
    }
}
