// file: src/ui/components/article_viewer.rs
// description: Enhanced article content display component with interactive text selection

use crate::{
    types::{Article, ContentType, RequestStatus, UISettings},
    ui::events::UIEvent,
    ui::rendering::markdown::MarkdownRenderer,
    ui::rendering::markdown_interactive::InteractiveMarkdownRenderer,
};
use egui;

pub struct ArticleViewer {
    markdown_renderer: MarkdownRenderer,
    interactive_renderer: InteractiveMarkdownRenderer,
    interactive_mode: bool,
}

impl ArticleViewer {
    pub fn new() -> Self {
        Self {
            markdown_renderer: MarkdownRenderer::new(),
            interactive_renderer: InteractiveMarkdownRenderer::new(),
            interactive_mode: true, // Default to interactive mode for text selection
        }
    }

    pub fn set_interactive_mode(&mut self, interactive: bool) {
        self.interactive_mode = interactive;
    }

    pub fn draw(&mut self, ui: &mut egui::Ui, status: &RequestStatus) -> Vec<UIEvent> {
        self.draw_with_settings(ui, status, &UISettings::default())
    }

    pub fn draw_with_settings(
        &mut self,
        ui: &mut egui::Ui,
        status: &RequestStatus,
        settings: &UISettings,
    ) -> Vec<UIEvent> {
        let mut events = Vec::new();

        match status {
            RequestStatus::Idle => {
                self.draw_welcome_screen(ui, settings);
            }
            RequestStatus::Loading => {
                self.draw_loading_screen(ui, settings);
            }
            RequestStatus::Success(content_type) => {
                let content_events = self.draw_content(ui, content_type, settings);
                events.extend(content_events);
            }
            RequestStatus::Error(error) => {
                self.draw_error_screen(ui, error, settings);
            }
        }

        events
    }

    fn draw_welcome_screen(&self, ui: &mut egui::Ui, settings: &UISettings) {
        ui.vertical_centered(|ui| {
            ui.add_space(50.0);
            let app_title = settings.apply_header_style(
                egui::RichText::new("Reading App")
                    .strong()
                    .size(settings.get_font_size() * 2.0),
            );
            ui.heading(app_title);
            ui.add_space(20.0);
            ui.separator();
            ui.add_space(20.0);

            let instruction_text =
                settings.apply_text_body_style(egui::RichText::new("Get started by:"));
            ui.label(instruction_text);

            let bullet_points = [
                "• Select a subject from the sidebar",
                "• Optionally specify a custom topic",
                "• Click 'Generate Article' to create content",
            ];

            for point in bullet_points {
                let point_text = settings.apply_text_body_style(egui::RichText::new(point));
                ui.label(point_text);
            }

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(10.0);
            let info_text = settings.apply_text_body_style(
                egui::RichText::new(
                    "Articles are generated using Groq AI and stored in your local database",
                )
                .size(settings.get_font_size() * 0.85),
            );
            ui.label(info_text);
        });
    }

    fn draw_loading_screen(&self, ui: &mut egui::Ui, settings: &UISettings) {
        ui.vertical_centered(|ui| {
            ui.add_space(100.0);
            let loading_title =
                settings.apply_header_style(egui::RichText::new("Generating Article...").strong());
            ui.heading(loading_title);
            ui.add_space(20.0);
            ui.add(egui::Spinner::new().size(40.0));
            ui.add_space(20.0);
            let wait_text = settings.apply_text_body_style(egui::RichText::new(
                "Please wait while we generate your article using Groq AI.",
            ));
            ui.label(wait_text);
            let duration_text = settings.apply_text_body_style(
                egui::RichText::new(
                    "This usually takes 10-30 seconds depending on article length.",
                )
                .size(settings.get_font_size() * 0.85),
            );
            ui.label(duration_text);

            ui.add_space(20.0);
            if ui.button("Cancel").clicked() {
                // TODO: Handle cancellation
            }
        });
    }

    fn draw_content(
        &mut self,
        ui: &mut egui::Ui,
        content_type: &ContentType,
        settings: &UISettings,
    ) -> Vec<UIEvent> {
        match content_type {
            ContentType::Article { content, .. } => {
                if self.interactive_mode {
                    self.draw_interactive_article(ui, content, settings)
                } else {
                    self.draw_article(ui, content, settings);
                    Vec::new()
                }
            }
            ContentType::ReadingPassage { content, .. } => {
                if self.interactive_mode {
                    self.draw_interactive_reading_passage(ui, content, settings)
                } else {
                    self.draw_reading_passage(ui, content, settings);
                    Vec::new()
                }
            }
        }
    }

    fn draw_article(&mut self, ui: &mut egui::Ui, article: &Article, settings: &UISettings) {
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                // Article header with enhanced styling
                let title_text = settings.apply_header_style(egui::RichText::new(&article.title));
                ui.heading(title_text);
                ui.add_space(settings.paragraph_spacing);

                // Article metadata
                ui.horizontal(|ui| {
                    let meta_style = |text: String| {
                        settings.apply_text_body_style(egui::RichText::new(text).weak())
                    };

                    ui.label(meta_style(format!(
                        "Subject: {}",
                        article.subject.display_name()
                    )));
                    ui.separator();

                    let formatted_time = article
                        .generated_at
                        .format("%Y-%m-%d %H:%M UTC")
                        .to_string();
                    ui.label(meta_style(format!("Generated: {}", formatted_time)));

                    if settings.show_article_stats {
                        ui.separator();
                        ui.label(meta_style(format!("Words: {}", article.word_count)));
                        ui.separator();
                        ui.label(meta_style(format!(
                            "Read time: {}m",
                            article.estimated_read_time
                        )));
                    }
                });

                ui.separator();
                ui.add_space(settings.paragraph_spacing * 2.0);

                // Article content with enhanced EasyMark formatting
                self.markdown_renderer
                    .render_with_settings(ui, &article.content, settings);
            });
    }

    fn draw_interactive_article(
        &mut self,
        ui: &mut egui::Ui,
        article: &Article,
        settings: &UISettings,
    ) -> Vec<UIEvent> {
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                // Article header (non-interactive)
                let title_text = settings.apply_header_style(egui::RichText::new(&article.title));
                ui.heading(title_text);
                ui.add_space(settings.paragraph_spacing);

                // Article metadata (non-interactive)
                ui.horizontal(|ui| {
                    let meta_style = |text: String| {
                        settings.apply_text_body_style(egui::RichText::new(text).weak())
                    };

                    ui.label(meta_style(format!(
                        "Subject: {}",
                        article.subject.display_name()
                    )));
                    ui.separator();

                    let formatted_time = article
                        .generated_at
                        .format("%Y-%m-%d %H:%M UTC")
                        .to_string();
                    ui.label(meta_style(format!("Generated: {}", formatted_time)));

                    if settings.show_article_stats {
                        ui.separator();
                        ui.label(meta_style(format!("Words: {}", article.word_count)));
                        ui.separator();
                        ui.label(meta_style(format!(
                            "Read time: {}m",
                            article.estimated_read_time
                        )));
                    }
                });

                ui.separator();
                ui.add_space(settings.paragraph_spacing * 2.0);

                // Mode toggle
                ui.horizontal(|ui| {
                    ui.label("Text Mode:");
                    if ui
                        .radio(self.interactive_mode, "Interactive (with text selection)")
                        .clicked()
                    {
                        self.interactive_mode = true;
                    }
                    if ui
                        .radio(!self.interactive_mode, "Static (read-only)")
                        .clicked()
                    {
                        self.interactive_mode = false;
                    }
                });

                ui.add_space(settings.paragraph_spacing);

                // Interactive article content
                self.interactive_renderer
                    .render_with_settings(ui, &article.content, settings)
            })
            .inner
    }

    fn draw_reading_passage(
        &mut self,
        ui: &mut egui::Ui,
        passage: &crate::types::reading_passage::ReadingPassage,
        settings: &UISettings,
    ) {
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                // Passage header with enhanced styling
                let title_text = settings.apply_header_style(egui::RichText::new(&passage.title));
                ui.heading(title_text);
                ui.add_space(settings.paragraph_spacing);

                // Passage metadata
                ui.horizontal(|ui| {
                    let meta_style = |text: String| {
                        settings.apply_text_body_style(egui::RichText::new(text).weak())
                    };

                    ui.label(meta_style(format!(
                        "Subject: {:?}",
                        passage.subject_category
                    )));
                    ui.separator();
                    ui.label(meta_style(format!(
                        "Difficulty: {:?}",
                        passage.difficulty_level
                    )));
                    ui.separator();

                    if settings.show_article_stats {
                        ui.label(meta_style(format!("Words: {}", passage.word_count)));
                        ui.separator();
                        ui.label(meta_style(format!(
                            "Questions: {}",
                            passage.questions.len()
                        )));
                    }
                });

                ui.separator();
                ui.add_space(settings.paragraph_spacing * 2.0);

                // Passage content with enhanced EasyMark formatting
                self.markdown_renderer
                    .render_with_settings(ui, &passage.content, settings);
            });
    }

    fn draw_interactive_reading_passage(
        &mut self,
        ui: &mut egui::Ui,
        passage: &crate::types::reading_passage::ReadingPassage,
        settings: &UISettings,
    ) -> Vec<UIEvent> {
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                // Passage header (non-interactive)
                let title_text = settings.apply_header_style(egui::RichText::new(&passage.title));
                ui.heading(title_text);
                ui.add_space(settings.paragraph_spacing);

                // Passage metadata (non-interactive)
                ui.horizontal(|ui| {
                    let meta_style = |text: String| {
                        settings.apply_text_body_style(egui::RichText::new(text).weak())
                    };

                    ui.label(meta_style(format!(
                        "Subject: {:?}",
                        passage.subject_category
                    )));
                    ui.separator();
                    ui.label(meta_style(format!(
                        "Difficulty: {:?}",
                        passage.difficulty_level
                    )));
                    ui.separator();

                    if settings.show_article_stats {
                        ui.label(meta_style(format!("Words: {}", passage.word_count)));
                        ui.separator();
                        ui.label(meta_style(format!(
                            "Questions: {}",
                            passage.questions.len()
                        )));
                    }
                });

                ui.separator();
                ui.add_space(settings.paragraph_spacing * 2.0);

                // Mode toggle
                ui.horizontal(|ui| {
                    ui.label("Text Mode:");
                    if ui
                        .radio(self.interactive_mode, "Interactive (with text selection)")
                        .clicked()
                    {
                        self.interactive_mode = true;
                    }
                    if ui
                        .radio(!self.interactive_mode, "Static (read-only)")
                        .clicked()
                    {
                        self.interactive_mode = false;
                    }
                });

                ui.add_space(settings.paragraph_spacing);

                // Interactive passage content
                self.interactive_renderer
                    .render_with_settings(ui, &passage.content, settings)
            })
            .inner
    }

    fn draw_error_screen(
        &self,
        ui: &mut egui::Ui,
        error: &crate::types::AppError,
        settings: &UISettings,
    ) {
        ui.vertical_centered(|ui| {
            ui.add_space(50.0);
            let error_title = settings.apply_header_style(egui::RichText::new("Error").strong());
            ui.heading(error_title);
            ui.add_space(20.0);
            ui.separator();
            ui.add_space(20.0);

            ui.colored_label(egui::Color32::RED, "Failed to generate article:");
            ui.add_space(10.0);

            // Better error display with wrapping
            ui.horizontal_wrapped(|ui| {
                let error_text =
                    settings.apply_text_body_style(egui::RichText::new(&error.to_string()));
                ui.label(error_text);
            });

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                if ui.button("Try Again").clicked() {
                    // TODO: Handle retry
                }

                if ui.button("Check Settings").clicked() {
                    // TODO: Handle settings
                }
            });
        });
    }

    // Helper method to get current interactive mode state
    pub fn is_interactive_mode(&self) -> bool {
        self.interactive_mode
    }

    // Method to toggle between interactive and static modes
    pub fn toggle_interactive_mode(&mut self) {
        self.interactive_mode = !self.interactive_mode;
    }
}

impl Default for ArticleViewer {
    fn default() -> Self {
        Self::new()
    }
}
