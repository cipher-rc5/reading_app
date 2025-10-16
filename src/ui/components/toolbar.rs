// file: src/ui/components/toolbar.rs
// description: Enhanced toolbar with updated menu structure

use crate::ui::events::UIEvent;
use egui;

pub struct Toolbar {
    show_about: bool,
}

impl Toolbar {
    pub fn new() -> Self {
        Self { show_about: false }
    }
}

impl Default for Toolbar {
    fn default() -> Self {
        Self::new()
    }
}

impl Toolbar {
    pub fn draw(&mut self, ctx: &egui::Context) -> Vec<UIEvent> {
        let mut events = Vec::new();

        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Settings").clicked() {
                        events.push(UIEvent::OpenSettings);
                        ui.close();
                    }

                    ui.separator();

                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        ui.close();
                    }
                });

                ui.menu_button("View", |ui| {
                    if ui.button("Toggle Sidebar").clicked() {
                        events.push(UIEvent::ToggleSidebar);
                        ui.close();
                    }

                    ui.separator();

                    if ui.button("Show Bookmarks").clicked() {
                        events.push(UIEvent::ToggleBookmarks);
                        ui.close();
                    }
                });

                ui.menu_button("Tools", |ui| {
                    if ui.button("Search Articles").clicked() {
                        events.push(UIEvent::OpenSearch);
                        ui.close();
                    }

                    ui.separator();

                    if ui.button("Debug & Diagnostics").clicked() {
                        events.push(UIEvent::OpenDebug);
                        ui.close();
                    }
                });

                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        self.show_about = true;
                        ui.close();
                    }
                });
            });
        });

        if self.show_about {
            egui::Window::new("About Reading App")
                .open(&mut self.show_about)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label("Reading App helps you generate and organize articles locally.");
                    ui.label("Version:".to_owned() + env!("CARGO_PKG_VERSION"));
                });
        }

        events
    }
}
