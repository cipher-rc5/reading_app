// file: src/services/database_service.rs
// description: database operations service with graceful error handling

use crate::{
    config::AppConfig,
    database::{
        connection::DatabaseConnection,
        repositories::{ArticleRepository, ReadingHistoryRepository, SettingsRepository},
    },
    types::{AppResult, Article, UISettings},
};

use std::sync::Arc;
use tracing::{error, warn};

#[derive(Clone, Default)]
pub struct DatabaseService {
    article_repo: Option<Arc<ArticleRepository>>,
    settings_repo: Option<Arc<SettingsRepository>>,
    reading_history_repo: Option<Arc<ReadingHistoryRepository>>,
    is_available: bool,
}

impl DatabaseService {
    pub fn new(_config: &AppConfig) -> AppResult<Self> {
        // For backward compatibility, create a default disabled service
        Ok(Self::default())
    }

    pub async fn new_async(config: &AppConfig) -> AppResult<Self> {
        match DatabaseConnection::new(&config.database_path).await {
            Ok(connection) => {
                let conn = Arc::new(connection);

                let article_repo = Arc::new(ArticleRepository::new(conn.clone()));
                let settings_repo = Arc::new(SettingsRepository::new(conn.clone()));
                let reading_history_repo = Arc::new(ReadingHistoryRepository::new(conn.clone()));

                Ok(Self {
                    article_repo: Some(article_repo),
                    settings_repo: Some(settings_repo),
                    reading_history_repo: Some(reading_history_repo),
                    is_available: true,
                })
            }
            Err(e) => {
                error!("Failed to initialize database connection: {}", e);
                Ok(Self {
                    article_repo: None,
                    settings_repo: None,
                    reading_history_repo: None,
                    is_available: false,
                })
            }
        }
    }

    pub async fn save_article(&self, article: &Article) -> AppResult<String> {
        if !self.is_available {
            return Err(crate::types::AppError::Database(
                crate::types::DatabaseError::Connection("Database not available".to_string()),
            ));
        }

        if let Some(ref repo) = self.article_repo {
            repo.save(article).await
        } else {
            Err(crate::types::AppError::Database(
                crate::types::DatabaseError::Connection(
                    "Article repository not available".to_string(),
                ),
            ))
        }
    }

    pub async fn get_recent_articles(&self, limit: usize) -> AppResult<Vec<Article>> {
        if !self.is_available {
            warn!("Database not available, returning empty articles list");
            return Ok(Vec::new());
        }

        if let Some(ref repo) = self.article_repo {
            repo.get_recent(limit).await
        } else {
            Ok(Vec::new())
        }
    }

    pub async fn search_articles(&self, query: &str) -> AppResult<Vec<Article>> {
        if !self.is_available {
            warn!("Database not available, returning empty search results");
            return Ok(Vec::new());
        }

        if let Some(ref repo) = self.article_repo {
            repo.search(query).await
        } else {
            Ok(Vec::new())
        }
    }

    pub async fn delete_article(&self, title: &str) -> AppResult<bool> {
        if !self.is_available {
            return Err(crate::types::AppError::Database(
                crate::types::DatabaseError::Connection("Database not available".to_string()),
            ));
        }

        if let Some(ref repo) = self.article_repo {
            repo.delete_by_title(title).await
        } else {
            Ok(false)
        }
    }

    pub async fn get_ui_settings(&self) -> AppResult<UISettings> {
        if !self.is_available {
            warn!("Database not available, returning default UI settings");
            return Ok(UISettings::default());
        }

        if let Some(ref repo) = self.settings_repo {
            match repo.get_ui_settings().await {
                Ok(settings) => Ok(settings),
                Err(e) => {
                    error!("Failed to load UI settings from database: {}", e);
                    Ok(UISettings::default())
                }
            }
        } else {
            Ok(UISettings::default())
        }
    }

    pub async fn save_ui_settings(&self, settings: &UISettings) -> AppResult<()> {
        if !self.is_available {
            warn!("Database not available, cannot save UI settings");
            return Ok(()); // Silently ignore save attempts when DB is unavailable
        }

        if let Some(ref repo) = self.settings_repo {
            repo.save_ui_settings(settings).await
        } else {
            warn!("Settings repository not available");
            Ok(())
        }
    }

    pub async fn add_reading_session(&self, article_title: &str, duration: i32) -> AppResult<()> {
        if !self.is_available {
            return Ok(()); // Silently ignore when DB is unavailable
        }

        if let Some(ref repo) = self.reading_history_repo {
            repo.add_session(article_title, duration).await
        } else {
            Ok(())
        }
    }

    pub async fn get_reading_stats(&self) -> AppResult<(i32, i32, i64)> {
        if !self.is_available {
            return Ok((0, 0, 0));
        }

        if let Some(ref repo) = self.reading_history_repo {
            repo.get_stats().await
        } else {
            Ok((0, 0, 0))
        }
    }

    pub fn is_available(&self) -> bool {
        self.is_available
    }
}
