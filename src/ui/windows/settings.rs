// file: src/ui/windows/settings.rs
// description: Enhanced settings window with corner style option and improved layout

use crate::services::SettingsService;
use crate::types::UISettings;
use crate::ui::events::UIEvent;
use crate::utils::fonts::FontRegistry;
use egui;
use tracing::{error, info};

pub struct SettingsWindow {
    show: bool,
}

impl SettingsWindow {
    pub fn new() -> Self {
        Self { show: false }
    }
}

impl Default for SettingsWindow {
    fn default() -> Self {
        Self::new()
    }
}

impl SettingsWindow {
    pub fn show(&mut self) {
        self.show = true;
    }

    pub fn draw(
        &mut self,
        ctx: &egui::Context,
        settings_service: &mut SettingsService,
        fonts: &FontRegistry,
    ) -> Vec<UIEvent> {
        let mut events = Vec::new();

        if !self.show {
            return events;
        }

        egui::Window::new("Settings")
            .default_width(750.0)
            .default_height(650.0)
            .resizable(true)
            .show(ctx, |ui| {
                let mut ui_settings = settings_service.get_ui_settings();
                let mut changed = false;

                egui::ScrollArea::vertical().show(ui, |ui| {
                    let heading_text = ui_settings
                        .apply_font_style(fonts, egui::RichText::new("UI Customization").strong());
                    ui.heading(heading_text);
                    ui.separator();

                    // Interface Style
                    ui.collapsing("Interface Style", |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Corner Style:");
                            egui::ComboBox::from_id_salt("corner_style")
                                .selected_text(&ui_settings.corner_style)
                                .show_ui(ui, |ui| {
                                    let styles = [
                                        ("rounded", "Rounded Corners"),
                                        ("square", "Square Corners"),
                                    ];
                                    for (value, display) in styles {
                                        if ui
                                            .selectable_value(
                                                &mut ui_settings.corner_style,
                                                value.to_string(),
                                                display,
                                            )
                                            .changed()
                                        {
                                            changed = true;
                                        }
                                    }
                                });
                        });

                        ui.horizontal(|ui| {
                            ui.label("Theme Mode:");
                            egui::ComboBox::from_id_salt("theme_mode")
                                .selected_text(&ui_settings.theme_mode)
                                .show_ui(ui, |ui| {
                                    let modes = [
                                        ("dark", "Dark"),
                                        ("light", "Light"),
                                        ("sepia", "Sepia"),
                                        ("high_contrast", "High Contrast"),
                                    ];
                                    for (value, display) in modes {
                                        if ui
                                            .selectable_value(
                                                &mut ui_settings.theme_mode,
                                                value.to_string(),
                                                display,
                                            )
                                            .changed()
                                        {
                                            changed = true;
                                        }
                                    }
                                });
                        });
                    });

                    // Font Settings
                    ui.collapsing("Font Settings", |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Text Body Font Size:");
                            if ui
                                .add(
                                    egui::Slider::new(
                                        &mut ui_settings.text_body_font_size,
                                        8.0..=32.0,
                                    )
                                    .suffix("pt"),
                                )
                                .changed()
                            {
                                changed = true;
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("Header Font Size:");
                            if ui
                                .add(
                                    egui::Slider::new(
                                        &mut ui_settings.header_font_size,
                                        12.0..=48.0,
                                    )
                                    .suffix("pt"),
                                )
                                .changed()
                            {
                                changed = true;
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("Text Body Font:");
                            egui::ComboBox::from_id_salt("text_body_font")
                                .selected_text(&ui_settings.text_body_font)
                                .show_ui(ui, |ui| {
                                    // Use the font manager to get available fonts
                                    let available_fonts = UISettings::get_available_fonts(fonts);
                                    for (value, display) in available_fonts {
                                        if ui
                                            .selectable_value(
                                                &mut ui_settings.text_body_font,
                                                value.to_string(),
                                                display,
                                            )
                                            .changed()
                                        {
                                            changed = true;
                                        }
                                    }
                                });
                        });

                        ui.horizontal(|ui| {
                            ui.label("Header Font:");
                            egui::ComboBox::from_id_salt("header_font")
                                .selected_text(&ui_settings.header_font)
                                .show_ui(ui, |ui| {
                                    // Use the font manager to get available fonts
                                    let available_fonts = UISettings::get_available_fonts(fonts);
                                    for (value, display) in available_fonts {
                                        if ui
                                            .selectable_value(
                                                &mut ui_settings.header_font,
                                                value.to_string(),
                                                display,
                                            )
                                            .changed()
                                        {
                                            changed = true;
                                        }
                                    }
                                });
                        });

                        ui.horizontal(|ui| {
                            ui.label("General Font Family:");
                            egui::ComboBox::from_id_salt("font_family")
                                .selected_text(&ui_settings.font_family)
                                .show_ui(ui, |ui| {
                                    // Use the font manager to get available fonts
                                    let available_fonts = UISettings::get_available_fonts(fonts);
                                    for (value, display) in available_fonts {
                                        if ui
                                            .selectable_value(
                                                &mut ui_settings.font_family,
                                                value.to_string(),
                                                display,
                                            )
                                            .changed()
                                        {
                                            changed = true;
                                        }
                                    }
                                });
                        });

                        ui.horizontal(|ui| {
                            ui.label("General Font Size:");
                            if ui
                                .add(
                                    egui::Slider::new(&mut ui_settings.font_size, 8.0..=32.0)
                                        .suffix("pt"),
                                )
                                .changed()
                            {
                                changed = true;
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("Line Height:");
                            if ui
                                .add(
                                    egui::Slider::new(&mut ui_settings.line_height, 1.0..=3.0)
                                        .suffix("x"),
                                )
                                .changed()
                            {
                                changed = true;
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("Paragraph Spacing:");
                            if ui
                                .add(
                                    egui::Slider::new(
                                        &mut ui_settings.paragraph_spacing,
                                        0.0..=50.0,
                                    )
                                    .suffix("px"),
                                )
                                .changed()
                            {
                                changed = true;
                            }
                        });

                        // Font size presets
                        ui.separator();
                        ui.label("Font Size Presets:");
                        ui.horizontal_wrapped(|ui| {
                            if ui.button("Small").clicked() {
                                ui_settings.apply_small_font_preset();
                                changed = true;
                            }
                            if ui.button("Medium").clicked() {
                                ui_settings.apply_medium_font_preset();
                                changed = true;
                            }
                            if ui.button("Large").clicked() {
                                ui_settings.apply_large_font_preset();
                                changed = true;
                            }
                            if ui.button("Extra Large").clicked() {
                                ui_settings.apply_extra_large_font_preset();
                                changed = true;
                            }
                        });
                    });

                    // Layout Settings
                    ui.collapsing("Layout Settings", |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Zoom Level:");
                            if ui
                                .add(
                                    egui::Slider::new(&mut ui_settings.zoom_level, 0.5..=3.0)
                                        .suffix("x"),
                                )
                                .changed()
                            {
                                changed = true;
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("Sidebar Width:");
                            if ui
                                .add(
                                    egui::Slider::new(
                                        &mut ui_settings.sidebar_width,
                                        200.0..=800.0,
                                    )
                                    .suffix("px"),
                                )
                                .changed()
                            {
                                changed = true;
                            }
                        });
                    });

                    // Color Settings
                    ui.collapsing("Color Settings", |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Background:");
                            let mut bg_color = ui_settings.get_background_color();
                            if ui.color_edit_button_srgba(&mut bg_color).changed() {
                                ui_settings.background_color = format!(
                                    "#{:02X}{:02X}{:02X}",
                                    bg_color.r(),
                                    bg_color.g(),
                                    bg_color.b()
                                );
                                changed = true;
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("Text Color:");
                            let mut text_color = ui_settings.get_text_color();
                            if ui.color_edit_button_srgba(&mut text_color).changed() {
                                ui_settings.text_color = format!(
                                    "#{:02X}{:02X}{:02X}",
                                    text_color.r(),
                                    text_color.g(),
                                    text_color.b()
                                );
                                changed = true;
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("Header Color:");
                            let mut header_color = ui_settings.get_header_color();
                            if ui.color_edit_button_srgba(&mut header_color).changed() {
                                ui_settings.header_color = format!(
                                    "#{:02X}{:02X}{:02X}",
                                    header_color.r(),
                                    header_color.g(),
                                    header_color.b()
                                );
                                changed = true;
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("Link Color:");
                            let mut link_color = ui_settings.get_link_color();
                            if ui.color_edit_button_srgba(&mut link_color).changed() {
                                ui_settings.link_color = format!(
                                    "#{:02X}{:02X}{:02X}",
                                    link_color.r(),
                                    link_color.g(),
                                    link_color.b()
                                );
                                changed = true;
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("Accent Color:");
                            let mut accent_color = ui_settings.get_accent_color();
                            if ui.color_edit_button_srgba(&mut accent_color).changed() {
                                ui_settings.accent_color = format!(
                                    "#{:02X}{:02X}{:02X}",
                                    accent_color.r(),
                                    accent_color.g(),
                                    accent_color.b()
                                );
                                changed = true;
                            }
                        });

                        ui.separator();
                        ui.label("Theme Presets:");
                        ui.horizontal_wrapped(|ui| {
                            if ui.button("Dark Theme").clicked() {
                                ui_settings.apply_dark_theme();
                                changed = true;
                            }
                            if ui.button("Light Theme").clicked() {
                                ui_settings.apply_light_theme();
                                changed = true;
                            }
                            if ui.button("Sepia").clicked() {
                                ui_settings.apply_sepia_theme();
                                changed = true;
                            }
                            if ui.button("High Contrast").clicked() {
                                ui_settings.apply_high_contrast_theme();
                                changed = true;
                            }
                        });
                    });

                    // Display Options
                    ui.collapsing("Display Options", |ui| {
                        if ui
                            .checkbox(
                                &mut ui_settings.show_article_stats,
                                "Show Article Statistics",
                            )
                            .changed()
                        {
                            changed = true;
                        }
                    });

                    ui.separator();

                    // Action buttons
                    ui.horizontal(|ui| {
                        if ui.button("Reset to Defaults").clicked() {
                            ui_settings = UISettings::default();
                            changed = true;
                        }

                        if ui.button("Close Settings").clicked() {
                            self.show = false;
                        }
                    });

                    // Save settings if changed
                    if changed {
                        if let Err(e) = ui_settings.validate() {
                            error!("Settings validation failed: {}", e);
                            ui.colored_label(
                                egui::Color32::RED,
                                format!("Validation Error: {}", e),
                            );
                        } else {
                            match settings_service.save_ui_settings(&ui_settings) {
                                Ok(_) => {
                                    info!("Settings saved successfully");
                                    events.push(UIEvent::SettingsChanged);
                                }
                                Err(e) => {
                                    error!("Failed to save settings: {}", e);
                                    ui.colored_label(
                                        egui::Color32::RED,
                                        format!("Save Error: {}", e),
                                    );
                                }
                            }
                        }
                    }
                });
            });

        events
    }
}
