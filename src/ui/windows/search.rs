// file: src/ui/windows/search.rs
// description: Search window

use crate::services::DatabaseService;
use egui;

pub struct SearchWindow {
    show: bool,
    query: String,
}

impl SearchWindow {
    pub fn new() -> Self {
        Self {
            show: false,
            query: String::new(),
        }
    }
}

impl Default for SearchWindow {
    fn default() -> Self {
        Self::new()
    }
}

impl SearchWindow {
    pub fn show(&mut self) {
        self.show = true;
    }

    pub fn draw(&mut self, ctx: &egui::Context, _database_service: &DatabaseService) {
        if !self.show {
            return;
        }

        egui::Window::new("Search Articles")
            .default_width(500.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Search:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.query)
                            .hint_text("Enter search terms..."),
                    );

                    if ui.button("Search").clicked() {
                        // Perform search
                    }
                });

                ui.separator();

                // TODO: Display search results

                ui.add_space(10.0);
                if ui.button("Close").clicked() {
                    self.show = false;
                }
            });
    }
}
