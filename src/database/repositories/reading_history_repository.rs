// file: src/database/repositories/reading_history_repository.rs
// description: Fixed reading history repository with proper column handling

use crate::{database::connection::DatabaseConnection, types::AppResult};
use libsql::Value;
use std::sync::Arc;
use uuid::Uuid;

pub struct ReadingHistoryRepository {
    conn: Arc<DatabaseConnection>,
}

impl ReadingHistoryRepository {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }

    pub async fn add_session(&self, article_title: &str, duration: i32) -> AppResult<()> {
        // First find the article ID
        let mut rows = self
            .conn
            .query(
                "SELECT id FROM articles WHERE title = ? LIMIT 1",
                libsql::params::Params::Positional(vec![Value::from(article_title)]),
            )
            .await?;

        if let Some(row) = rows.next().await.map_err(|e| {
            crate::types::DatabaseError::Query(format!("Failed to fetch article row: {}", e))
        })? {
            let article_id: String = row.get(0).map_err(|e| {
                crate::types::DatabaseError::Query(format!("Failed to get article_id: {}", e))
            })?;

            let session_id = Uuid::new_v4().to_string();
            let opened_at = chrono::Utc::now().to_rfc3339();

            // Use the new schema with proper column names
            self.conn
                .execute(
                    r#"
                    INSERT INTO reading_history (id, article_id, content_type, opened_at, reading_time_seconds)
                    VALUES (?, ?, ?, ?, ?)
                    "#,
                    libsql::params::Params::Positional(vec![
                        Value::from(session_id),
                        Value::from(article_id),
                        Value::from("article"), // content_type
                        Value::from(opened_at),
                        Value::from(duration as i64),
                    ]),
                )
                .await?;
        }

        Ok(())
    }

    pub async fn add_passage_session(&self, passage_title: &str, duration: i32) -> AppResult<()> {
        // Find the passage ID
        let mut rows = self
            .conn
            .query(
                "SELECT id FROM reading_passages WHERE title = ? LIMIT 1",
                libsql::params::Params::Positional(vec![Value::from(passage_title)]),
            )
            .await?;

        if let Some(row) = rows.next().await.map_err(|e| {
            crate::types::DatabaseError::Query(format!("Failed to fetch passage row: {}", e))
        })? {
            let passage_id: String = row.get(0).map_err(|e| {
                crate::types::DatabaseError::Query(format!("Failed to get passage_id: {}", e))
            })?;

            let session_id = Uuid::new_v4().to_string();
            let opened_at = chrono::Utc::now().to_rfc3339();

            self.conn
                .execute(
                    r#"
                    INSERT INTO reading_history (id, passage_id, content_type, opened_at, reading_time_seconds)
                    VALUES (?, ?, ?, ?, ?)
                    "#,
                    libsql::params::Params::Positional(vec![
                        Value::from(session_id),
                        Value::from(passage_id),
                        Value::from("reading_passage"), // content_type
                        Value::from(opened_at),
                        Value::from(duration as i64),
                    ]),
                )
                .await?;
        }

        Ok(())
    }

    pub async fn get_stats(&self) -> AppResult<(i32, i32, i64)> {
        let total_articles: i32 = {
            let mut rows = self
                .conn
                .query(
                    "SELECT COUNT(*) as count FROM articles",
                    libsql::params::Params::Positional(vec![]),
                )
                .await?;

            if let Some(row) = rows.next().await.map_err(|e| {
                crate::types::DatabaseError::Query(format!("Failed to fetch articles count: {}", e))
            })? {
                row.get::<i64>(0).unwrap_or(0) as i32
            } else {
                0
            }
        };

        let total_sessions: i32 = {
            let mut rows = self
                .conn
                .query(
                    "SELECT COUNT(*) as count FROM reading_history",
                    libsql::params::Params::Positional(vec![]),
                )
                .await?;

            if let Some(row) = rows.next().await.map_err(|e| {
                crate::types::DatabaseError::Query(format!("Failed to fetch sessions count: {}", e))
            })? {
                row.get::<i64>(0).unwrap_or(0) as i32
            } else {
                0
            }
        };

        let total_time: i64 = {
            let mut rows = self
                .conn
                .query(
                    "SELECT COALESCE(SUM(reading_time_seconds), 0) as total FROM reading_history",
                    libsql::params::Params::Positional(vec![]),
                )
                .await?;

            if let Some(row) = rows.next().await.map_err(|e| {
                crate::types::DatabaseError::Query(format!("Failed to fetch reading time: {}", e))
            })? {
                row.get::<i64>(0).unwrap_or(0)
            } else {
                0
            }
        };

        Ok((total_articles, total_sessions, total_time))
    }

    pub async fn get_recent_sessions(&self, limit: usize) -> AppResult<Vec<ReadingSession>> {
        let mut rows = self
            .conn
            .query(
                r#"
                SELECT rh.id, rh.content_type, rh.opened_at, rh.reading_time_seconds,
                       COALESCE(a.title, rp.title) as title
                FROM reading_history rh
                LEFT JOIN articles a ON rh.article_id = a.id AND rh.content_type = 'article'
                LEFT JOIN reading_passages rp ON rh.passage_id = rp.id AND rh.content_type = 'reading_passage'
                ORDER BY rh.opened_at DESC
                LIMIT ?
                "#,
                libsql::params::Params::Positional(vec![Value::from(limit as i64)]),
            )
            .await?;

        let mut sessions = Vec::new();
        while let Some(row) = rows.next().await.map_err(|e| {
            crate::types::DatabaseError::Query(format!("Failed to fetch session row: {}", e))
        })? {
            let session = ReadingSession {
                id: row.get(0).unwrap_or_default(),
                content_type: row.get(1).unwrap_or_default(),
                title: row.get(4).unwrap_or_default(),
                opened_at: row.get(2).unwrap_or_default(),
                reading_time_seconds: row.get::<i64>(3).unwrap_or(0) as i32,
            };
            sessions.push(session);
        }

        Ok(sessions)
    }
}

#[derive(Debug, Clone)]
pub struct ReadingSession {
    pub id: String,
    pub content_type: String,
    pub title: String,
    pub opened_at: String,
    pub reading_time_seconds: i32,
}
