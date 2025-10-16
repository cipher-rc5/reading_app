// file: src/database/repositories/settings_repository.rs
// description: enhanced settings data access layer with graceful column handling

use crate::{
    database::connection::DatabaseConnection,
    types::{AppResult, UISettings},
};
use libsql::Value;
use std::sync::Arc;
use tracing::{error, warn};

pub struct SettingsRepository {
    conn: Arc<DatabaseConnection>,
}

impl SettingsRepository {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }

    pub async fn get_ui_settings(&self) -> AppResult<UISettings> {
        // First try the full query with all columns
        match self.try_get_full_settings().await {
            Ok(settings) => Ok(settings),
            Err(_) => {
                warn!("Failed to get full settings, trying basic settings");
                // Fall back to basic settings and provide defaults for missing columns
                self.get_basic_settings().await
            }
        }
    }

    async fn try_get_full_settings(&self) -> AppResult<UISettings> {
        let mut rows = self
            .conn
            .query(
                r#"
                    SELECT font_size, zoom_level, background_color, text_color, font_family,
                           theme_mode, show_article_stats, sidebar_width, text_body_font_size,
                           header_font_size, text_body_font, header_font, line_height,
                           paragraph_spacing, header_color, link_color, accent_color, corner_style
                    FROM user_preferences
                    ORDER BY id DESC
                    LIMIT 1
                    "#,
                libsql::params::Params::Positional(vec![]),
            )
            .await?;

        if let Some(row) = rows.next().await.map_err(|e| {
            crate::types::DatabaseError::Query(format!("Failed to fetch settings row: {}", e))
        })? {
            let settings = UISettings {
                // Basic settings
                font_size: row.get::<f64>(0).unwrap_or(14.0) as f32,
                zoom_level: row.get::<f64>(1).unwrap_or(1.0) as f32,
                background_color: row
                    .get::<String>(2)
                    .unwrap_or_else(|_| "#2b2b2b".to_string()),
                text_color: row
                    .get::<String>(3)
                    .unwrap_or_else(|_| "#ffffff".to_string()),
                font_family: row
                    .get::<String>(4)
                    .unwrap_or_else(|_| "default".to_string()),
                theme_mode: row.get::<String>(5).unwrap_or_else(|_| "dark".to_string()),
                show_article_stats: row.get::<i64>(6).unwrap_or(1) != 0,
                sidebar_width: row.get::<f64>(7).unwrap_or(300.0) as f32,

                // Enhanced settings from bibliotheca
                text_body_font_size: row.get::<f64>(8).unwrap_or(14.0) as f32,
                header_font_size: row.get::<f64>(9).unwrap_or(20.0) as f32,
                text_body_font: row
                    .get::<String>(10)
                    .unwrap_or_else(|_| "default".to_string()),
                header_font: row
                    .get::<String>(11)
                    .unwrap_or_else(|_| "default".to_string()),
                line_height: row.get::<f64>(12).unwrap_or(1.5) as f32,
                paragraph_spacing: row.get::<f64>(13).unwrap_or(8.0) as f32,
                header_color: row
                    .get::<String>(14)
                    .unwrap_or_else(|_| "#ffffff".to_string()),
                link_color: row
                    .get::<String>(15)
                    .unwrap_or_else(|_| "#4a9eff".to_string()),
                accent_color: row
                    .get::<String>(16)
                    .unwrap_or_else(|_| "#ff6b6b".to_string()),
                corner_style: row
                    .get::<String>(17)
                    .unwrap_or_else(|_| "rounded".to_string()),
            };

            settings.validate()?;
            Ok(settings)
        } else {
            // Insert default settings if none exist
            self.insert_default_settings().await?;
            Ok(UISettings::default())
        }
    }

    async fn get_basic_settings(&self) -> AppResult<UISettings> {
        // Try to get basic settings without corner_style column
        let mut rows = self
            .conn
            .query(
                r#"
                    SELECT font_size, zoom_level, background_color, text_color, font_family,
                           theme_mode, show_article_stats, sidebar_width, text_body_font_size,
                           header_font_size, text_body_font, header_font, line_height,
                           paragraph_spacing, header_color, link_color, accent_color
                    FROM user_preferences
                    ORDER BY id DESC
                    LIMIT 1
                    "#,
                libsql::params::Params::Positional(vec![]),
            )
            .await?;

        if let Some(row) = rows.next().await.map_err(|e| {
            crate::types::DatabaseError::Query(format!("Failed to fetch basic settings row: {}", e))
        })? {
            let settings = UISettings {
                // Basic settings
                font_size: row.get::<f64>(0).unwrap_or(14.0) as f32,
                zoom_level: row.get::<f64>(1).unwrap_or(1.0) as f32,
                background_color: row
                    .get::<String>(2)
                    .unwrap_or_else(|_| "#2b2b2b".to_string()),
                text_color: row
                    .get::<String>(3)
                    .unwrap_or_else(|_| "#ffffff".to_string()),
                font_family: row
                    .get::<String>(4)
                    .unwrap_or_else(|_| "default".to_string()),
                theme_mode: row.get::<String>(5).unwrap_or_else(|_| "dark".to_string()),
                show_article_stats: row.get::<i64>(6).unwrap_or(1) != 0,
                sidebar_width: row.get::<f64>(7).unwrap_or(300.0) as f32,

                // Enhanced settings from bibliotheca
                text_body_font_size: row.get::<f64>(8).unwrap_or(14.0) as f32,
                header_font_size: row.get::<f64>(9).unwrap_or(20.0) as f32,
                text_body_font: row
                    .get::<String>(10)
                    .unwrap_or_else(|_| "default".to_string()),
                header_font: row
                    .get::<String>(11)
                    .unwrap_or_else(|_| "default".to_string()),
                line_height: row.get::<f64>(12).unwrap_or(1.5) as f32,
                paragraph_spacing: row.get::<f64>(13).unwrap_or(8.0) as f32,
                header_color: row
                    .get::<String>(14)
                    .unwrap_or_else(|_| "#ffffff".to_string()),
                link_color: row
                    .get::<String>(15)
                    .unwrap_or_else(|_| "#4a9eff".to_string()),
                accent_color: row
                    .get::<String>(16)
                    .unwrap_or_else(|_| "#ff6b6b".to_string()),
                // Use default for missing corner_style
                corner_style: "rounded".to_string(),
            };

            // Try to update the database with the missing column
            match self.add_missing_corner_style_column().await {
                Ok(_) => warn!("Added missing corner_style column"),
                Err(e) => error!("Failed to add corner_style column: {}", e),
            }

            settings.validate()?;
            Ok(settings)
        } else {
            // Insert default settings if none exist
            self.insert_default_settings().await?;
            Ok(UISettings::default())
        }
    }

    async fn add_missing_corner_style_column(&self) -> AppResult<()> {
        self.conn
            .execute(
                "ALTER TABLE user_preferences ADD COLUMN corner_style TEXT DEFAULT 'rounded'",
                libsql::params::Params::Positional(vec![]),
            )
            .await?;
        Ok(())
    }

    pub async fn save_ui_settings(&self, settings: &UISettings) -> AppResult<()> {
        settings.validate()?;

        // Check if settings exist
        let count: i64 = {
            let mut rows = self
                .conn
                .query(
                    "SELECT COUNT(*) as count FROM user_preferences",
                    libsql::params::Params::Positional(vec![]),
                )
                .await?;

            if let Some(row) = rows.next().await.map_err(|e| {
                crate::types::DatabaseError::Query(format!(
                    "Failed to get preferences count: {}",
                    e
                ))
            })? {
                row.get::<i64>(0).unwrap_or(0)
            } else {
                0
            }
        };

        if count == 0 {
            self.insert_settings(settings).await
        } else {
            self.update_settings(settings).await
        }
    }

    async fn insert_default_settings(&self) -> AppResult<()> {
        let default_settings = UISettings::default();
        self.insert_settings(&default_settings).await
    }

    async fn insert_settings(&self, settings: &UISettings) -> AppResult<()> {
        self.conn
            .execute(
                r#"
                INSERT INTO user_preferences
                (font_size, zoom_level, background_color, text_color, font_family, theme_mode,
                 show_article_stats, sidebar_width, text_body_font_size, header_font_size,
                 text_body_font, header_font, line_height, paragraph_spacing, header_color,
                 link_color, accent_color, corner_style)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#,
                libsql::params::Params::Positional(vec![
                    Value::from(settings.font_size as f64),
                    Value::from(settings.zoom_level as f64),
                    Value::from(settings.background_color.clone()),
                    Value::from(settings.text_color.clone()),
                    Value::from(settings.font_family.clone()),
                    Value::from(settings.theme_mode.clone()),
                    Value::from(if settings.show_article_stats { 1 } else { 0 }),
                    Value::from(settings.sidebar_width as f64),
                    Value::from(settings.text_body_font_size as f64),
                    Value::from(settings.header_font_size as f64),
                    Value::from(settings.text_body_font.clone()),
                    Value::from(settings.header_font.clone()),
                    Value::from(settings.line_height as f64),
                    Value::from(settings.paragraph_spacing as f64),
                    Value::from(settings.header_color.clone()),
                    Value::from(settings.link_color.clone()),
                    Value::from(settings.accent_color.clone()),
                    Value::from(settings.corner_style.clone()),
                ]),
            )
            .await?;

        Ok(())
    }

    async fn update_settings(&self, settings: &UISettings) -> AppResult<()> {
        // Try the full update first
        match self.try_full_update(settings).await {
            Ok(_) => Ok(()),
            Err(_) => {
                // Fall back to update without corner_style if column doesn't exist
                warn!("Full update failed, trying basic update");
                self.try_basic_update(settings).await
            }
        }
    }

    async fn try_full_update(&self, settings: &UISettings) -> AppResult<()> {
        self.conn
            .execute(
                r#"
                UPDATE user_preferences SET
                    font_size = ?,
                    zoom_level = ?,
                    background_color = ?,
                    text_color = ?,
                    font_family = ?,
                    theme_mode = ?,
                    show_article_stats = ?,
                    sidebar_width = ?,
                    text_body_font_size = ?,
                    header_font_size = ?,
                    text_body_font = ?,
                    header_font = ?,
                    line_height = ?,
                    paragraph_spacing = ?,
                    header_color = ?,
                    link_color = ?,
                    accent_color = ?,
                    corner_style = ?,
                    updated_at = CURRENT_TIMESTAMP
                WHERE id = (SELECT id FROM user_preferences ORDER BY id DESC LIMIT 1)
                "#,
                libsql::params::Params::Positional(vec![
                    Value::from(settings.font_size as f64),
                    Value::from(settings.zoom_level as f64),
                    Value::from(settings.background_color.clone()),
                    Value::from(settings.text_color.clone()),
                    Value::from(settings.font_family.clone()),
                    Value::from(settings.theme_mode.clone()),
                    Value::from(if settings.show_article_stats { 1 } else { 0 }),
                    Value::from(settings.sidebar_width as f64),
                    Value::from(settings.text_body_font_size as f64),
                    Value::from(settings.header_font_size as f64),
                    Value::from(settings.text_body_font.clone()),
                    Value::from(settings.header_font.clone()),
                    Value::from(settings.line_height as f64),
                    Value::from(settings.paragraph_spacing as f64),
                    Value::from(settings.header_color.clone()),
                    Value::from(settings.link_color.clone()),
                    Value::from(settings.accent_color.clone()),
                    Value::from(settings.corner_style.clone()),
                ]),
            )
            .await?;

        Ok(())
    }

    async fn try_basic_update(&self, settings: &UISettings) -> AppResult<()> {
        self.conn
            .execute(
                r#"
                UPDATE user_preferences SET
                    font_size = ?,
                    zoom_level = ?,
                    background_color = ?,
                    text_color = ?,
                    font_family = ?,
                    theme_mode = ?,
                    show_article_stats = ?,
                    sidebar_width = ?,
                    text_body_font_size = ?,
                    header_font_size = ?,
                    text_body_font = ?,
                    header_font = ?,
                    line_height = ?,
                    paragraph_spacing = ?,
                    header_color = ?,
                    link_color = ?,
                    accent_color = ?,
                    updated_at = CURRENT_TIMESTAMP
                WHERE id = (SELECT id FROM user_preferences ORDER BY id DESC LIMIT 1)
                "#,
                libsql::params::Params::Positional(vec![
                    Value::from(settings.font_size as f64),
                    Value::from(settings.zoom_level as f64),
                    Value::from(settings.background_color.clone()),
                    Value::from(settings.text_color.clone()),
                    Value::from(settings.font_family.clone()),
                    Value::from(settings.theme_mode.clone()),
                    Value::from(if settings.show_article_stats { 1 } else { 0 }),
                    Value::from(settings.sidebar_width as f64),
                    Value::from(settings.text_body_font_size as f64),
                    Value::from(settings.header_font_size as f64),
                    Value::from(settings.text_body_font.clone()),
                    Value::from(settings.header_font.clone()),
                    Value::from(settings.line_height as f64),
                    Value::from(settings.paragraph_spacing as f64),
                    Value::from(settings.header_color.clone()),
                    Value::from(settings.link_color.clone()),
                    Value::from(settings.accent_color.clone()),
                ]),
            )
            .await?;

        // Try to add the missing column after basic update
        let _ = self.add_missing_corner_style_column().await;

        Ok(())
    }
}
