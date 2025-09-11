// file: src/ui/rendering/markdown.rs
// description: Enhanced markdown parser with proper text flow and clean output

use crate::types::UISettings;
use egui;

// Enhanced EasyMark parser based on the working version
mod easy_mark {
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub enum Item<'a> {
        Newline,
        Text(Style, &'a str),
        Heading(&'a str),
        BulletPoint(&'a str),
        NumberedPoint(&'a str, &'a str),
        CodeBlock(&'a str),
        InlineCode(&'a str),
        Separator,
    }

    #[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
    pub struct Style {
        pub strong: bool,
        pub italics: bool,
        pub code: bool,
    }

    pub struct Parser<'a> {
        s: &'a str,
        start_of_line: bool,
        style: Style,
    }

    impl<'a> Parser<'a> {
        pub fn new(s: &'a str) -> Self {
            Self {
                s: s.trim(),
                start_of_line: true,
                style: Style::default(),
            }
        }
    }

    impl<'a> Iterator for Parser<'a> {
        type Item = Item<'a>;

        fn next(&mut self) -> Option<Self::Item> {
            loop {
                if self.s.is_empty() {
                    return None;
                }

                // Skip leading whitespace on lines that aren't start of line
                if !self.start_of_line && self.s.starts_with(' ') {
                    let trimmed = self.s.trim_start();
                    if trimmed.is_empty() {
                        self.s = "";
                        continue;
                    }
                    let to_consume = self.s.len() - trimmed.len();
                    if to_consume > 0 && to_consume <= 4 {
                        self.s = trimmed;
                        continue;
                    }
                }

                // Handle newlines - double newlines create paragraph breaks
                if self.s.starts_with("\n\n") {
                    self.s = &self.s[2..].trim_start();
                    self.start_of_line = true;
                    self.style = Style::default();
                    return Some(Item::Newline);
                } else if self.s.starts_with('\n') {
                    // Single newline - just move to next line within paragraph
                    self.s = &self.s[1..];
                    // Convert single newline to space for continuous text flow
                    if !self.s.is_empty() && !self.s.starts_with('\n') {
                        return Some(Item::Text(self.style, " "));
                    }
                    self.start_of_line = true;
                    continue;
                }

                if self.start_of_line {
                    // Handle different heading levels
                    if let Some(after) = self.s.strip_prefix("### ") {
                        let end = after.find('\n').unwrap_or(after.len());
                        let heading = &after[..end];
                        self.s = &after[end..];
                        self.start_of_line = false;
                        return Some(Item::Heading(heading));
                    } else if let Some(after) = self.s.strip_prefix("## ") {
                        let end = after.find('\n').unwrap_or(after.len());
                        let heading = &after[..end];
                        self.s = &after[end..];
                        self.start_of_line = false;
                        return Some(Item::Heading(heading));
                    } else if let Some(after) = self.s.strip_prefix("# ") {
                        let end = after.find('\n').unwrap_or(after.len());
                        let heading = &after[..end];
                        self.s = &after[end..];
                        self.start_of_line = false;
                        return Some(Item::Heading(heading));
                    }

                    // Bullet point
                    if let Some(after) = self.s.strip_prefix("- ") {
                        let end = after.find('\n').unwrap_or(after.len());
                        let text = &after[..end];
                        self.s = &after[end..];
                        self.start_of_line = false;
                        return Some(Item::BulletPoint(text));
                    }

                    // Numbered list
                    let n_digits = self.s.chars().take_while(|c| c.is_ascii_digit()).count();
                    if n_digits > 0 {
                        let remaining = &self.s[n_digits..];
                        if let Some(after) = remaining.strip_prefix(". ") {
                            let number = &self.s[..n_digits];
                            let end = after.find('\n').unwrap_or(after.len());
                            let text = &after[..end];
                            self.s = &after[end..];
                            self.start_of_line = false;
                            return Some(Item::NumberedPoint(number, text));
                        }
                    }

                    // Separator
                    if self.s.starts_with("---") {
                        let after = self.s.trim_start_matches('-');
                        self.s = after.strip_prefix('\n').unwrap_or(after);
                        self.start_of_line = false;
                        return Some(Item::Separator);
                    }

                    // Code block
                    if self.s.starts_with("```") {
                        let after = &self.s[3..];
                        if let Some(end_pos) = after.find("\n```") {
                            let code = &after[..end_pos];
                            self.s = &after[end_pos + 4..];
                            self.start_of_line = false;
                            return Some(Item::CodeBlock(code));
                        } else if let Some(end_pos) = after.find("```") {
                            let code = &after[..end_pos];
                            self.s = &after[end_pos + 3..];
                            self.start_of_line = false;
                            return Some(Item::CodeBlock(code));
                        }
                    }
                }

                // Inline code
                if self.s.starts_with('`') {
                    let after = &self.s[1..];
                    if let Some(end) = after.find('`') {
                        let code = &after[..end];
                        self.s = &after[end + 1..];
                        self.start_of_line = false;
                        return Some(Item::InlineCode(code));
                    }
                }

                // Bold text
                if self.s.starts_with("**") {
                    self.s = &self.s[2..];
                    self.start_of_line = false;
                    self.style.strong = !self.style.strong;
                    continue;
                }

                // Italic text
                if self.s.starts_with('*') && !self.s.starts_with("**") {
                    self.s = &self.s[1..];
                    self.start_of_line = false;
                    self.style.italics = !self.style.italics;
                    continue;
                }

                // Regular text - find the end of this text segment
                let end = self.s.find(&['*', '`', '\n'][..]).unwrap_or(self.s.len());

                if end == 0 {
                    // Handle special characters at the start
                    let text = &self.s[..1];
                    self.s = &self.s[1..];
                    self.start_of_line = false;
                    return Some(Item::Text(self.style, text));
                }

                let text = &self.s[..end];
                self.s = &self.s[end..];
                self.start_of_line = false;

                if !text.trim().is_empty() {
                    return Some(Item::Text(self.style, text));
                }
            }
        }
    }
}

pub struct MarkdownRenderer {
    // Store settings reference for consistent styling
}

impl MarkdownRenderer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&mut self, ui: &mut egui::Ui, content: &str) {
        self.render_with_settings(ui, content, &UISettings::default())
    }

    // Clean content for proper display
    fn clean_content(&self, content: &str) -> String {
        content
            .chars()
            .filter_map(|c| {
                match c {
                    // Keep standard printable ASCII characters
                    ' '..='~' => Some(c),

                    // Keep common whitespace
                    '\n' | '\t' => Some(c),

                    // Replace common problematic unicode with ASCII equivalents
                    '\u{2013}' | '\u{2014}' => Some('-'), // en-dash, em-dash
                    '\u{2018}' | '\u{2019}' => Some('\''), // smart quotes
                    '\u{201C}' | '\u{201D}' => Some('"'), // smart double quotes
                    '\u{2026}' => Some('.'),              // ellipsis -> single dot

                    // Replace bullets with simple ones
                    '\u{2022}' | '\u{2023}' | '\u{25E6}' | '\u{2043}' => Some('•'),

                    // Remove other control characters and problematic unicode
                    c if c.is_control() && c != '\n' && c != '\t' => None,
                    c if c as u32 > 127 && !matches!(c, '•') => {
                        // Replace other non-ASCII with space if it's not our allowed bullet
                        Some(' ')
                    }

                    _ => Some(c),
                }
            })
            .collect::<String>()
            // Clean up multiple spaces and normalize whitespace
            .lines()
            .map(|line| line.split_whitespace().collect::<Vec<_>>().join(" "))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn render_with_settings(
        &mut self,
        ui: &mut egui::Ui,
        content: &str,
        settings: &UISettings,
    ) {
        use easy_mark::{Item, Parser};

        // Clean the content first
        let cleaned_content = self.clean_content(content);

        let mut is_first_item = true;
        let mut current_paragraph_parts = Vec::new();

        for item in Parser::new(&cleaned_content) {
            match item {
                Item::Newline => {
                    // Render accumulated paragraph parts if any
                    if !current_paragraph_parts.is_empty() {
                        if !is_first_item {
                            ui.add_space(settings.paragraph_spacing);
                        }
                        self.render_paragraph_parts(ui, &current_paragraph_parts, settings);
                        current_paragraph_parts.clear();
                        is_first_item = false;
                    }
                }
                Item::Text(style, text) => {
                    if !text.trim().is_empty() {
                        current_paragraph_parts.push((style, text));
                    }
                }
                Item::Heading(text) => {
                    // Render any pending paragraph first
                    if !current_paragraph_parts.is_empty() {
                        if !is_first_item {
                            ui.add_space(settings.paragraph_spacing);
                        }
                        self.render_paragraph_parts(ui, &current_paragraph_parts, settings);
                        current_paragraph_parts.clear();
                        is_first_item = false;
                    }

                    if !is_first_item {
                        ui.add_space(settings.paragraph_spacing * 1.5);
                    }
                    let heading_text = settings.apply_header_style(egui::RichText::new(text));
                    ui.heading(heading_text);
                    ui.add_space(settings.paragraph_spacing);
                    is_first_item = false;
                }
                Item::BulletPoint(text) => {
                    // Render any pending paragraph first
                    if !current_paragraph_parts.is_empty() {
                        if !is_first_item {
                            ui.add_space(settings.paragraph_spacing);
                        }
                        self.render_paragraph_parts(ui, &current_paragraph_parts, settings);
                        current_paragraph_parts.clear();
                        is_first_item = false;
                    }

                    if !is_first_item {
                        ui.add_space(settings.paragraph_spacing / 2.0);
                    }
                    ui.horizontal(|ui| {
                        let bullet = settings.apply_text_body_style(egui::RichText::new("• "));
                        ui.label(bullet);

                        let bullet_text = settings.apply_text_body_style(egui::RichText::new(text));
                        ui.add(egui::Label::new(bullet_text).wrap());
                    });
                    is_first_item = false;
                }
                Item::NumberedPoint(number, text) => {
                    // Render any pending paragraph first
                    if !current_paragraph_parts.is_empty() {
                        if !is_first_item {
                            ui.add_space(settings.paragraph_spacing);
                        }
                        self.render_paragraph_parts(ui, &current_paragraph_parts, settings);
                        current_paragraph_parts.clear();
                        is_first_item = false;
                    }

                    if !is_first_item {
                        ui.add_space(settings.paragraph_spacing / 2.0);
                    }
                    ui.horizontal(|ui| {
                        let number_text = settings
                            .apply_text_body_style(egui::RichText::new(format!("{}. ", number)));
                        ui.label(number_text);

                        let point_text = settings.apply_text_body_style(egui::RichText::new(text));
                        ui.add(egui::Label::new(point_text).wrap());
                    });
                    is_first_item = false;
                }
                Item::CodeBlock(code) => {
                    // Render any pending paragraph first
                    if !current_paragraph_parts.is_empty() {
                        if !is_first_item {
                            ui.add_space(settings.paragraph_spacing);
                        }
                        self.render_paragraph_parts(ui, &current_paragraph_parts, settings);
                        current_paragraph_parts.clear();
                        is_first_item = false;
                    }

                    if !is_first_item {
                        ui.add_space(settings.paragraph_spacing);
                    }
                    ui.group(|ui| {
                        let code_text = egui::RichText::new(code)
                            .monospace()
                            .size(settings.get_text_body_font_size() * 0.9)
                            .color(egui::Color32::from_rgb(200, 200, 200))
                            .family(egui::FontFamily::Monospace);

                        ui.add(egui::Label::new(code_text).wrap());
                    });
                    ui.add_space(settings.paragraph_spacing);
                    is_first_item = false;
                }
                Item::InlineCode(code) => {
                    // Inline code is part of the current paragraph
                    current_paragraph_parts.push((
                        easy_mark::Style {
                            code: true,
                            ..Default::default()
                        },
                        code,
                    ));
                }
                Item::Separator => {
                    // Render any pending paragraph first
                    if !current_paragraph_parts.is_empty() {
                        if !is_first_item {
                            ui.add_space(settings.paragraph_spacing);
                        }
                        self.render_paragraph_parts(ui, &current_paragraph_parts, settings);
                        current_paragraph_parts.clear();
                        is_first_item = false;
                    }

                    if !is_first_item {
                        ui.add_space(settings.paragraph_spacing);
                    }
                    ui.separator();
                    ui.add_space(settings.paragraph_spacing);
                    is_first_item = false;
                }
            }
        }

        // Render any remaining paragraph parts
        if !current_paragraph_parts.is_empty() {
            if !is_first_item {
                ui.add_space(settings.paragraph_spacing);
            }
            self.render_paragraph_parts(ui, &current_paragraph_parts, settings);
        }
    }

    fn render_paragraph_parts(
        &self,
        ui: &mut egui::Ui,
        parts: &[(easy_mark::Style, &str)],
        settings: &UISettings,
    ) {
        ui.horizontal_wrapped(|ui| {
            for (style, text) in parts {
                if text.trim().is_empty() {
                    continue;
                }

                let mut rich_text = settings.apply_text_body_style(egui::RichText::new(*text));

                if style.strong {
                    rich_text = rich_text.strong();
                }
                if style.italics {
                    rich_text = rich_text.italics();
                }
                if style.code {
                    rich_text = rich_text
                        .monospace()
                        .background_color(egui::Color32::from_rgb(40, 40, 40))
                        .color(egui::Color32::from_rgb(255, 255, 255))
                        .family(egui::FontFamily::Monospace);
                }

                ui.add(egui::Label::new(rich_text).wrap());
            }
        });
    }
}
