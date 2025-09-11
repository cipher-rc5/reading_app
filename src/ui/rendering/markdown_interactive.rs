// file: src/ui/rendering/markdown_interactive.rs
// description: Interactive markdown renderer with text selection support

use crate::types::UISettings;
use crate::ui::components::text_toolbar::TextToolbar;
use crate::ui::events::UIEvent;
use egui;

pub struct InteractiveMarkdownRenderer {
    text_toolbar: TextToolbar,
    selection_start: Option<usize>,
    selection_end: Option<usize>,
}

impl InteractiveMarkdownRenderer {
    pub fn new() -> Self {
        Self {
            text_toolbar: TextToolbar::new(),
            selection_start: None,
            selection_end: None,
        }
    }

    pub fn render_with_settings(
        &mut self,
        ui: &mut egui::Ui,
        content: &str,
        settings: &UISettings,
    ) -> Vec<UIEvent> {
        let mut events = Vec::new();

        // Draw the text toolbar first (it renders as a popup)
        let toolbar_events = self.text_toolbar.draw(ui.ctx());
        events.extend(toolbar_events);

        // Render the content with text selection capabilities
        self.render_selectable_content(ui, content, settings, &mut events);

        events
    }

    fn render_selectable_content(
        &mut self,
        ui: &mut egui::Ui,
        content: &str,
        settings: &UISettings,
        _events: &mut Vec<UIEvent>,
    ) {
        // Split content into paragraphs for better selection handling
        let paragraphs: Vec<&str> = content.split("\n\n").collect();

        for (para_idx, paragraph) in paragraphs.iter().enumerate() {
            if para_idx > 0 {
                ui.add_space(settings.paragraph_spacing);
            }

            self.render_selectable_paragraph(ui, paragraph, settings, _events);
        }
    }

    fn render_selectable_paragraph(
        &mut self,
        ui: &mut egui::Ui,
        paragraph: &str,
        settings: &UISettings,
        _events: &mut Vec<UIEvent>,
    ) {
        // Handle different markdown elements
        if paragraph.starts_with("# ") {
            let heading = paragraph.trim_start_matches("# ").trim();
            self.render_selectable_text(ui, heading, settings, _events, true);
        } else if paragraph.starts_with("## ") {
            let heading = paragraph.trim_start_matches("## ").trim();
            self.render_selectable_text(ui, heading, settings, _events, true);
        } else if paragraph.starts_with("### ") {
            let heading = paragraph.trim_start_matches("### ").trim();
            self.render_selectable_text(ui, heading, settings, _events, true);
        } else if paragraph.starts_with("- ") {
            // Bullet point
            ui.horizontal(|ui| {
                let bullet = settings.apply_text_body_style(egui::RichText::new("• "));
                ui.label(bullet);
                let text = paragraph.trim_start_matches("- ").trim();
                self.render_selectable_text(ui, text, settings, _events, false);
            });
        } else {
            // Regular paragraph
            self.render_selectable_text(ui, paragraph, settings, _events, false);
        }
    }

    fn render_selectable_text(
        &mut self,
        ui: &mut egui::Ui,
        text: &str,
        settings: &UISettings,
        _events: &mut Vec<UIEvent>,
        is_heading: bool,
    ) {
        // Create a selectable label
        let rich_text = if is_heading {
            settings.apply_header_style(egui::RichText::new(text))
        } else {
            settings.apply_text_body_style(egui::RichText::new(text))
        };

        let response = ui.add(
            egui::Label::new(rich_text)
                .wrap()
                .sense(egui::Sense::click_and_drag()),
        );

        // Handle text selection
        if response.drag_started() {
            self.selection_start = Some(0); // Simplified - you'd want to calculate actual position
        }

        if response.dragged() {
            // Update selection end
            self.selection_end = Some(text.len()); // Simplified
        }

        if response.drag_stopped() {
            // Handle text selection completion
            if let Some(pointer_pos) = ui.ctx().pointer_latest_pos() {
                let selected_text = self.get_selected_text_from_response(&response, text);

                if !selected_text.is_empty() && TextToolbar::should_show_for_text(&selected_text) {
                    self.text_toolbar
                        .show_at_position(pointer_pos, selected_text);
                }
            }
        }

        // Handle double-click for word selection
        if response.double_clicked() {
            if let Some(pointer_pos) = ui.ctx().pointer_latest_pos() {
                if let Some(word) = self.get_word_at_position(&response, text) {
                    if TextToolbar::should_show_for_text(&word) {
                        self.text_toolbar.show_at_position(pointer_pos, word);
                    }
                }
            }
        }
    }

    fn get_selected_text_from_response(&self, response: &egui::Response, text: &str) -> String {
        // This is a simplified implementation
        // In a real implementation, you'd calculate the actual selected portion based on mouse positions

        if response.dragged() {
            // For now, return the first sentence or word depending on drag distance
            let drag_delta = response.drag_delta();
            if drag_delta.length() > 50.0 {
                // Long drag - try to select a sentence
                self.get_first_sentence(text)
            } else {
                // Short drag - select first word
                self.get_first_word(text)
            }
        } else {
            String::new()
        }
    }

    fn get_word_at_position(&self, _response: &egui::Response, text: &str) -> Option<String> {
        // Simplified: return the first word
        // In a real implementation, you'd calculate which word was clicked based on position
        let word = self.get_first_word(text);
        if word.is_empty() {
            None
        } else {
            Some(word)
        }
    }

    fn get_first_word(&self, text: &str) -> String {
        text.split_whitespace()
            .next()
            .unwrap_or("")
            .trim_matches(|c: char| !c.is_alphabetic())
            .to_string()
    }

    fn get_first_sentence(&self, text: &str) -> String {
        // Find the first sentence (up to first period, exclamation, or question mark)
        if let Some(pos) = text.find(&['.', '!', '?'][..]) {
            let sentence = &text[..=pos];
            sentence.trim().to_string()
        } else {
            // If no sentence ending found, return first few words
            text.split_whitespace()
                .take(10)
                .collect::<Vec<_>>()
                .join(" ")
        }
    }
}
