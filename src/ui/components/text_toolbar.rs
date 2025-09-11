// file: src/ui/components/text_toolbar.rs
// description: Enhanced popup toolbar for text interactions with working dictionary and explanation

use crate::ui::events::UIEvent;
use egui;

#[derive(Default)]
pub struct TextToolbar {
    show: bool,
    position: egui::Pos2,
    selected_text: String,
    context: String,
}

impl TextToolbar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn show_at_position(&mut self, position: egui::Pos2, selected_text: String) {
        self.show = true;
        self.position = position;
        self.selected_text = selected_text.clone();
        self.context = selected_text; // For now, use selected text as context
    }

    pub fn hide(&mut self) {
        self.show = false;
        self.selected_text.clear();
        self.context.clear();
    }

    pub fn is_visible(&self) -> bool {
        self.show
    }

    pub fn draw(&mut self, ctx: &egui::Context) -> Vec<UIEvent> {
        let mut events = Vec::new();

        if !self.show {
            return events;
        }

        // Adjust position to stay within window bounds
        let window_rect = ctx.screen_rect();
        let mut adjusted_position = self.position;

        // Ensure toolbar doesn't go off screen
        if adjusted_position.x + 300.0 > window_rect.max.x {
            adjusted_position.x = window_rect.max.x - 300.0;
        }
        if adjusted_position.y - 80.0 < window_rect.min.y {
            adjusted_position.y = self.position.y + 40.0; // Show below instead of above
        }

        egui::Window::new("Text Tools")
            .id(egui::Id::new("text_toolbar"))
            .collapsible(false)
            .resizable(false)
            .title_bar(false)
            .fixed_pos(adjusted_position)
            .frame(egui::Frame::popup(&ctx.style()))
            .show(ctx, |ui| {
                ui.set_min_width(280.0);

                // Show selected text preview
                ui.horizontal(|ui| {
                    ui.label("Selected:");
                    ui.label(
                        egui::RichText::new(format!(
                            "\"{}\"",
                            if self.selected_text.len() > 30 {
                                format!("{}...", &self.selected_text[..27])
                            } else {
                                self.selected_text.clone()
                            }
                        ))
                        .italics()
                        .color(egui::Color32::LIGHT_GRAY),
                    );
                });

                ui.separator();

                // Action buttons
                ui.horizontal(|ui| {
                    if ui
                        .button("📖 Define")
                        .on_hover_text("Look up definition")
                        .clicked()
                    {
                        // Extract the first meaningful word for definition lookup
                        let word = self.extract_word_for_definition();
                        if !word.is_empty() {
                            events.push(UIEvent::LookupDefinition(word));
                        }
                        self.hide();
                    }

                    if ui
                        .button("💬 Explain")
                        .on_hover_text("Explain this text")
                        .clicked()
                    {
                        events.push(UIEvent::ExplainText {
                            text: self.selected_text.clone(),
                            context: self.context.clone(),
                        });
                        self.hide();
                    }

                    if ui
                        .button("📋 Copy")
                        .on_hover_text("Copy to clipboard")
                        .clicked()
                    {
                        ctx.copy_text(self.selected_text.clone());
                        self.hide();
                    }
                });

                ui.horizontal(|ui| {
                    if ui
                        .button("🔍 Search")
                        .on_hover_text("Search for this text")
                        .clicked()
                    {
                        events.push(UIEvent::SearchQuery(self.selected_text.clone()));
                        self.hide();
                    }

                    if ui.button("✕ Cancel").clicked() {
                        self.hide();
                    }
                });
            });

        // Auto-hide if user clicks elsewhere
        if ctx.input(|i| i.pointer.any_click()) {
            // Check if click was outside the toolbar window
            if let Some(pointer_pos) = ctx.pointer_latest_pos() {
                let toolbar_rect =
                    egui::Rect::from_min_size(adjusted_position, egui::Vec2::new(280.0, 80.0));

                if !toolbar_rect.contains(pointer_pos) {
                    self.hide();
                }
            }
        }

        events
    }

    fn extract_word_for_definition(&self) -> String {
        // Extract the first meaningful word from the selected text
        let words: Vec<&str> = self.selected_text.split_whitespace().collect();

        for word in words {
            let clean_word = word
                .trim_matches(|c: char| !c.is_alphabetic())
                .to_lowercase();

            // Only return words that are at least 2 characters and contain only letters
            if clean_word.len() >= 2 && clean_word.chars().all(|c| c.is_alphabetic()) {
                return clean_word;
            }
        }

        // Fallback: return the first word, cleaned
        if let Some(first_word) = self.selected_text.split_whitespace().next() {
            first_word
                .trim_matches(|c: char| !c.is_alphabetic())
                .to_lowercase()
        } else {
            String::new()
        }
    }

    // Helper method to set context separately if needed
    pub fn set_context(&mut self, context: String) {
        self.context = context;
    }

    // Method to check if the toolbar should be shown based on selection criteria
    pub fn should_show_for_text(text: &str) -> bool {
        let trimmed = text.trim();

        // Don't show for very short selections
        if trimmed.len() < 2 {
            return false;
        }

        // Don't show for selections that are just whitespace or punctuation
        if !trimmed.chars().any(|c| c.is_alphanumeric()) {
            return false;
        }

        // Don't show for very long selections (probably accidental)
        if trimmed.len() > 500 {
            return false;
        }

        true
    }
}
