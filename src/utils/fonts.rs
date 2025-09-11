// file: src/utils/fonts.rs
// description: Working font configuration with safe customization

use egui::{FontData, FontDefinitions, FontFamily};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tracing::{error, info, warn};

pub struct FontManager {
    available_fonts: HashMap<String, String>, // name -> path
    loaded_font_keys: Vec<String>,            // Successfully loaded font keys
}

impl FontManager {
    pub fn new() -> Self {
        let mut manager = Self {
            available_fonts: HashMap::new(),
            loaded_font_keys: Vec::new(),
        };
        manager.scan_system_fonts();
        manager
    }

    fn scan_system_fonts(&mut self) {
        let font_paths: Vec<String> = if cfg!(target_os = "macos") {
            let home = std::env::var("HOME").unwrap_or_default();
            vec![
                "/System/Library/Fonts".to_string(),
                "/Library/Fonts".to_string(),
                format!("{}/.fonts", home),
                format!("{}/Library/Fonts", home),
            ]
        } else if cfg!(target_os = "windows") {
            let user_profile = std::env::var("USERPROFILE").unwrap_or_default();
            vec![
                "C:/Windows/Fonts".to_string(),
                format!("{}/.fonts", user_profile),
            ]
        } else {
            let home = std::env::var("HOME").unwrap_or_default();
            vec![
                "/usr/share/fonts".to_string(),
                "/usr/local/share/fonts".to_string(),
                format!("{}/.fonts", home),
                format!("{}/.local/share/fonts", home),
            ]
        };

        for font_dir in &font_paths {
            self.scan_directory(font_dir);
        }

        info!("Found {} system fonts", self.available_fonts.len());
    }

    fn scan_directory(&mut self, dir_path: &str) {
        let path = Path::new(dir_path);
        if !path.exists() || !path.is_dir() {
            return;
        }

        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(extension) = path.extension() {
                        let ext = extension.to_string_lossy().to_lowercase();
                        if matches!(ext.as_str(), "ttf" | "otf") {
                            if let Some(name) = path.file_stem() {
                                let font_name = name.to_string_lossy().to_string();
                                let font_path = path.to_string_lossy().to_string();

                                // Special handling for TerminessTTF Nerd Font
                                if font_name.contains("TerminessTTF")
                                    || font_name.contains("Terminess")
                                {
                                    self.available_fonts
                                        .insert("terminus_nerd_mono".to_string(), font_path);
                                }
                                // Look for common system fonts by name patterns
                                else if font_name.to_lowercase().contains("times") {
                                    self.available_fonts
                                        .insert("times_font".to_string(), font_path);
                                } else if font_name.to_lowercase().contains("arial") {
                                    self.available_fonts
                                        .insert("arial_font".to_string(), font_path);
                                } else if font_name.to_lowercase().contains("helvetica") {
                                    self.available_fonts
                                        .insert("helvetica_font".to_string(), font_path);
                                }
                                // Store other fonts with sanitized names
                                else {
                                    let sanitized_name = font_name
                                        .replace(" ", "_")
                                        .replace("(", "")
                                        .replace(")", "")
                                        .to_lowercase();
                                    self.available_fonts.insert(sanitized_name, font_path);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn get_available_fonts(&self) -> Vec<(String, String)> {
        let mut fonts = vec![
            ("default".to_string(), "Default (Proportional)".to_string()),
            ("monospace".to_string(), "System Monospace".to_string()),
        ];

        // Add successfully loaded fonts
        for font_key in &self.loaded_font_keys {
            let display_name = match font_key.as_str() {
                "terminus_nerd_mono" => "TerminessTTF Nerd Font Mono".to_string(),
                "times_font" => "Times".to_string(),
                "arial_font" => "Arial".to_string(),
                "helvetica_font" => "Helvetica".to_string(),
                _ => font_key.replace("_", " ").to_string(),
            };
            fonts.push((font_key.clone(), display_name));
        }

        fonts.sort_by(|a, b| a.1.cmp(&b.1));
        fonts
    }

    pub fn load_font_data(&self, font_name: &str) -> Option<FontData> {
        if let Some(font_path) = self.available_fonts.get(font_name) {
            match fs::read(font_path) {
                Ok(data) => {
                    info!("Loaded font: {} from {}", font_name, font_path);
                    Some(FontData::from_owned(data))
                }
                Err(e) => {
                    error!("Failed to read font file {}: {}", font_path, e);
                    None
                }
            }
        } else {
            None
        }
    }

    pub fn has_font(&self, font_name: &str) -> bool {
        self.available_fonts.contains_key(font_name)
    }

    pub fn register_loaded_font(&mut self, font_key: String) {
        if !self.loaded_font_keys.contains(&font_key) {
            self.loaded_font_keys.push(font_key);
        }
    }
}

// Global font manager instance
pub fn get_font_manager() -> &'static std::sync::Mutex<FontManager> {
    use std::sync::{Mutex, OnceLock};

    static FONT_MANAGER_CELL: OnceLock<Mutex<FontManager>> = OnceLock::new();
    FONT_MANAGER_CELL.get_or_init(|| Mutex::new(FontManager::new()))
}

pub fn configure(cc: &eframe::CreationContext<'_>) {
    let ctx = &cc.egui_ctx;
    let font_manager_mutex = get_font_manager();
    let mut font_manager = font_manager_mutex.lock().unwrap();

    let mut fonts = FontDefinitions::default();

    // Load system fonts that were found and CAN BE LOADED
    let font_keys_to_try: Vec<String> = font_manager.available_fonts.keys().cloned().collect();

    for font_key in font_keys_to_try {
        if let Some(font_data) = font_manager.load_font_data(&font_key) {
            // Insert the font data with a unique key
            fonts.font_data.insert(font_key.clone(), font_data.into());

            // Create a font family for this font
            fonts.families.insert(
                FontFamily::Name(font_key.clone().into()),
                vec![font_key.clone()],
            );

            // Track that this font was successfully loaded
            font_manager.register_loaded_font(font_key);
        }
    }
    ctx.set_fonts(fonts);
}

// Helper functions for UISettings
pub fn get_available_fonts() -> Vec<(String, String)> {
    let font_manager_mutex = get_font_manager();
    let font_manager = font_manager_mutex.lock().unwrap();
    font_manager.get_available_fonts()
}

pub fn get_font_family_for_name(font_name: &str) -> egui::FontFamily {
    let font_manager_mutex = get_font_manager();
    let font_manager = font_manager_mutex.lock().unwrap();

    match font_name {
        "default" => egui::FontFamily::Proportional,
        "monospace" => egui::FontFamily::Monospace,
        _ => {
            // Check if it's a system font we actually loaded
            if font_manager
                .loaded_font_keys
                .contains(&font_name.to_string())
            {
                egui::FontFamily::Name(font_name.into())
            } else {
                // Safe fallback - use built-in fonts only
                warn!("Font '{}' not loaded, falling back to default", font_name);
                egui::FontFamily::Proportional
            }
        }
    }
}
