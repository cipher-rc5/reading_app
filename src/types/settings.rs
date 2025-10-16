// file: src/types/settings.rs
// description: enhanced UI settings with corner style and improved validation

use super::errors::{AppResult, ValidationError};
use crate::utils::fonts::FontRegistry;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UISettings {
    // Basic settings (existing)
    pub font_size: f32,
    pub zoom_level: f32,
    pub background_color: String,
    pub text_color: String,
    pub font_family: String,
    pub theme_mode: String,
    pub show_article_stats: bool,
    pub sidebar_width: f32,

    // enhanced font settings (from bibliotheca)
    pub text_body_font_size: f32,
    pub header_font_size: f32,
    pub text_body_font: String,
    pub header_font: String,
    pub line_height: f32,
    pub paragraph_spacing: f32,
    pub header_color: String,
    pub link_color: String,
    pub accent_color: String,

    // new corner style option
    pub corner_style: String, // "rounded" or "square"
}

impl Default for UISettings {
    fn default() -> Self {
        Self {
            font_size: 14.0,
            zoom_level: 1.0,
            background_color: "#2b2b2b".to_string(),
            text_color: "#ffffff".to_string(),
            font_family: "default".to_string(),
            theme_mode: "dark".to_string(),
            show_article_stats: true,
            sidebar_width: 300.0,
            text_body_font_size: 14.0,
            header_font_size: 20.0,
            text_body_font: "default".to_string(),
            header_font: "default".to_string(),
            line_height: 1.5,
            paragraph_spacing: 8.0,
            header_color: "#ffffff".to_string(),
            link_color: "#4a9eff".to_string(),
            accent_color: "#ff6b6b".to_string(),
            corner_style: "rounded".to_string(),
        }
    }
}

impl UISettings {
    pub fn validate(&self) -> AppResult<()> {
        // enhanced validation from bibliotheca
        if self.font_size < 8.0 || self.font_size > 32.0 {
            return Err(ValidationError::InvalidFormat(
                "Font size must be between 8.0 and 32.0".to_string(),
            )
            .into());
        }

        if self.text_body_font_size < 8.0 || self.text_body_font_size > 32.0 {
            return Err(ValidationError::InvalidFormat(
                "Text body font size must be between 8.0 and 32.0".to_string(),
            )
            .into());
        }

        if self.header_font_size < 12.0 || self.header_font_size > 48.0 {
            return Err(ValidationError::InvalidFormat(
                "Header font size must be between 12.0 and 48.0".to_string(),
            )
            .into());
        }

        if self.zoom_level < 0.5 || self.zoom_level > 3.0 {
            return Err(ValidationError::InvalidFormat(
                "Zoom level must be between 0.5 and 3.0".to_string(),
            )
            .into());
        }

        if self.line_height < 1.0 || self.line_height > 3.0 {
            return Err(ValidationError::InvalidFormat(
                "Line height must be between 1.0 and 3.0".to_string(),
            )
            .into());
        }

        if self.paragraph_spacing < 0.0 || self.paragraph_spacing > 50.0 {
            return Err(ValidationError::InvalidFormat(
                "Paragraph spacing must be between 0.0 and 50.0".to_string(),
            )
            .into());
        }

        if self.sidebar_width < 200.0 || self.sidebar_width > 800.0 {
            return Err(ValidationError::InvalidFormat(
                "Sidebar width must be between 200.0 and 800.0".to_string(),
            )
            .into());
        }

        // Validate corner style
        if !matches!(self.corner_style.as_str(), "rounded" | "square") {
            return Err(ValidationError::InvalidFormat(
                "Corner style must be 'rounded' or 'square'".to_string(),
            )
            .into());
        }

        // Validate color formats
        let colors = [
            (&self.background_color, "Background color"),
            (&self.text_color, "Text color"),
            (&self.header_color, "Header color"),
            (&self.link_color, "Link color"),
            (&self.accent_color, "Accent color"),
        ];

        for (color, name) in &colors {
            if parse_hex_color(color).is_none() {
                return Err(ValidationError::InvalidFormat(format!(
                    "{} must be in #RRGGBB format",
                    name
                ))
                .into());
            }
        }

        Ok(())
    }

    // Color getters
    pub fn get_background_color(&self) -> egui::Color32 {
        parse_hex_color(&self.background_color).unwrap_or(egui::Color32::from_rgb(43, 43, 43))
    }

    pub fn get_text_color(&self) -> egui::Color32 {
        parse_hex_color(&self.text_color).unwrap_or(egui::Color32::WHITE)
    }

    pub fn get_header_color(&self) -> egui::Color32 {
        parse_hex_color(&self.header_color).unwrap_or(egui::Color32::WHITE)
    }

    pub fn get_link_color(&self) -> egui::Color32 {
        parse_hex_color(&self.link_color).unwrap_or(egui::Color32::from_rgb(74, 158, 255))
    }

    pub fn get_accent_color(&self) -> egui::Color32 {
        parse_hex_color(&self.accent_color).unwrap_or(egui::Color32::from_rgb(255, 107, 107))
    }

    // Font size getters with zoom
    pub fn get_font_size(&self) -> f32 {
        (self.font_size * self.zoom_level).clamp(8.0, 48.0)
    }

    pub fn get_text_body_font_size(&self) -> f32 {
        (self.text_body_font_size * self.zoom_level).clamp(8.0, 48.0)
    }

    pub fn get_header_font_size(&self) -> f32 {
        (self.header_font_size * self.zoom_level).clamp(12.0, 64.0)
    }

    // Corner style helpers
    pub fn get_rounding(&self) -> egui::CornerRadius {
        match self.corner_style.as_str() {
            "rounded" => egui::CornerRadius::same(8),
            "square" => egui::CornerRadius::ZERO,
            _ => egui::CornerRadius::same(8), // default to rounded
        }
    }

    pub fn is_rounded(&self) -> bool {
        self.corner_style == "rounded"
    }

    // Font configuration - updated to use the font manager
    pub fn get_available_fonts(fonts: &FontRegistry) -> Vec<(String, String)> {
        fonts.available_fonts()
    }

    pub fn get_font_family(&self, font_name: &str, fonts: &FontRegistry) -> egui::FontFamily {
        fonts.font_family_for(font_name)
    }

    // **EXISTING TEXT STYLING METHODS** - These are retained from the original code
    pub fn apply_text_body_style(
        &self,
        fonts: &FontRegistry,
        mut text: egui::RichText,
    ) -> egui::RichText {
        text = text
            .size(self.get_text_body_font_size())
            .color(self.get_text_color())
            .family(self.get_font_family(&self.text_body_font, fonts));
        text
    }

    pub fn apply_header_style(
        &self,
        fonts: &FontRegistry,
        mut text: egui::RichText,
    ) -> egui::RichText {
        text = text
            .size(self.get_header_font_size())
            .color(self.get_header_color())
            .family(self.get_font_family(&self.header_font, fonts))
            .strong();
        text
    }

    pub fn apply_font_style(
        &self,
        fonts: &FontRegistry,
        mut text: egui::RichText,
    ) -> egui::RichText {
        text = text.size(self.get_font_size()).color(self.get_text_color());

        match self.font_family.as_str() {
            "terminus_nerd_mono" | "monospace" => {
                text.family(self.get_font_family("monospace", fonts))
            }
            "serif" => text.family(self.get_font_family("serif", fonts)),
            "sans-serif" => text.family(self.get_font_family("sans-serif", fonts)),
            _ => text.family(self.get_font_family("default", fonts)),
        }
    }

    // **THEME PRESETS** - Existing methods from bibliotheca
    pub fn apply_dark_theme(&mut self) {
        self.background_color = "#2b2b2b".to_string();
        self.text_color = "#ffffff".to_string();
        self.header_color = "#ffffff".to_string();
        self.link_color = "#4a9eff".to_string();
        self.accent_color = "#ff6b6b".to_string();
        self.theme_mode = "dark".to_string();
    }

    pub fn apply_light_theme(&mut self) {
        self.background_color = "#f5f5f5".to_string();
        self.text_color = "#333333".to_string();
        self.header_color = "#1a1a1a".to_string();
        self.link_color = "#0066cc".to_string();
        self.accent_color = "#cc4444".to_string();
        self.theme_mode = "light".to_string();
    }

    pub fn apply_sepia_theme(&mut self) {
        self.background_color = "#f4f1e8".to_string();
        self.text_color = "#5c4b37".to_string();
        self.header_color = "#3d2f23".to_string();
        self.link_color = "#8b4513".to_string();
        self.accent_color = "#cd853f".to_string();
        self.theme_mode = "sepia".to_string();
    }

    pub fn apply_high_contrast_theme(&mut self) {
        self.background_color = "#000000".to_string();
        self.text_color = "#ffffff".to_string();
        self.header_color = "#ffffff".to_string();
        self.link_color = "#00ffff".to_string();
        self.accent_color = "#ffff00".to_string();
        self.theme_mode = "high_contrast".to_string();
    }

    // **FONT SIZE PRESETS** - Existing methods
    pub fn apply_small_font_preset(&mut self) {
        self.text_body_font_size = 12.0;
        self.header_font_size = 16.0;
        self.font_size = 12.0;
    }

    pub fn apply_medium_font_preset(&mut self) {
        self.text_body_font_size = 14.0;
        self.header_font_size = 20.0;
        self.font_size = 14.0;
    }

    pub fn apply_large_font_preset(&mut self) {
        self.text_body_font_size = 18.0;
        self.header_font_size = 26.0;
        self.font_size = 18.0;
    }

    pub fn apply_extra_large_font_preset(&mut self) {
        self.text_body_font_size = 22.0;
        self.header_font_size = 32.0;
        self.font_size = 22.0;
    }

    // Clean up legacy font settings that might cause warnings
    pub fn sanitize_font_settings(&mut self, fonts: &FontRegistry) {
        let available_fonts = Self::get_available_fonts(fonts);
        let available_font_keys: Vec<String> =
            available_fonts.iter().map(|(k, _)| k.clone()).collect();

        // Reset font settings to "default" if they reference unavailable fonts
        if !available_font_keys.contains(&self.font_family) {
            self.font_family = "default".to_string();
        }
        if !available_font_keys.contains(&self.text_body_font) {
            self.text_body_font = "default".to_string();
        }
        if !available_font_keys.contains(&self.header_font) {
            self.header_font = "default".to_string();
        }
    }
}

fn parse_hex_color(hex: &str) -> Option<egui::Color32> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }

    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;

    Some(egui::Color32::from_rgb(r, g, b))
}
