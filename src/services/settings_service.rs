// file: src/services/settings_service.rs
// description: settings management service

use crate::{
    app::runtime::AppRuntime,
    services::DatabaseService,
    types::{AppResult, UISettings},
    utils::fonts::FontRegistry,
};
use std::sync::RwLock;
use tracing::{error, info};

pub struct SettingsService {
    database_service: DatabaseService,
    cached_settings: RwLock<Option<UISettings>>,
    runtime: AppRuntime,
    font_registry: FontRegistry,
}

impl SettingsService {
    pub fn new(
        database_service: DatabaseService,
        runtime: AppRuntime,
        font_registry: FontRegistry,
    ) -> Self {
        Self {
            database_service,
            cached_settings: RwLock::new(None),
            runtime,
            font_registry,
        }
    }

    pub fn get_ui_settings(&self) -> UISettings {
        // Check cache first
        if let Some(settings) = self
            .cached_settings
            .read()
            .ok()
            .and_then(|guard| guard.clone())
        {
            return settings;
        }

        // Load from database using spawn_blocking for UI thread
        let database_service = self.database_service.clone();
        let result = self
            .runtime
            .block_on(async { database_service.get_ui_settings().await });

        match result {
            Ok(mut settings) => {
                settings.sanitize_font_settings(&self.font_registry);

                // Update cache
                if let Ok(mut cached) = self.cached_settings.write() {
                    *cached = Some(settings.clone());
                }
                settings
            }
            Err(e) => {
                error!("Failed to load UI settings: {}", e);
                UISettings::default()
            }
        }
    }

    pub fn save_ui_settings(&self, settings: &UISettings) -> AppResult<()> {
        settings.validate()?;

        let mut sanitized_settings = settings.clone();
        sanitized_settings.sanitize_font_settings(&self.font_registry);

        // Use the runtime handle to execute async operation
        let database_service = self.database_service.clone();
        let settings_clone = sanitized_settings.clone();

        self.runtime
            .block_on(async { database_service.save_ui_settings(&settings_clone).await })?;

        // Update cache
        if let Ok(mut cached) = self.cached_settings.write() {
            *cached = Some(sanitized_settings);
        }

        info!("UI settings saved successfully");
        Ok(())
    }

    pub async fn save_ui_settings_async(&self, settings: &UISettings) -> AppResult<()> {
        // Validate before saving
        settings.validate()?;

        let mut sanitized_settings = settings.clone();
        sanitized_settings.sanitize_font_settings(&self.font_registry);

        // Save to database
        self.database_service
            .save_ui_settings(&sanitized_settings)
            .await?;

        // Update cache
        if let Ok(mut cached) = self.cached_settings.write() {
            *cached = Some(sanitized_settings);
        }

        info!("UI settings saved successfully");
        Ok(())
    }

    pub fn apply_theme_preset(&self, theme: ThemePreset) -> AppResult<()> {
        let mut settings = self.get_ui_settings();

        match theme {
            ThemePreset::Dark => settings.apply_dark_theme(),
            ThemePreset::Light => settings.apply_light_theme(),
        }

        self.save_ui_settings(&settings)
    }
}

#[derive(Debug, Clone)]
pub enum ThemePreset {
    Dark,
    Light,
}
