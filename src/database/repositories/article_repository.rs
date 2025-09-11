// file: src/database/repositories/article_repository.rs
// description: Article data access layer

// use crate::types::InputValidator;
use crate::{
    database::connection::DatabaseConnection,
    types::{AppResult, Article, ArticleSubject},
};
use libsql::Value;
use std::sync::Arc;
use uuid::Uuid;

pub struct ArticleRepository {
    conn: Arc<DatabaseConnection>,
}

impl ArticleRepository {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }

    pub async fn save(&self, article: &Article) -> AppResult<String> {
        let id = Uuid::new_v4().to_string();

        self.conn
            .execute(
                r#"
                INSERT INTO articles (id, title, content, subject, generated_at, word_count, estimated_read_time)
                VALUES (?, ?, ?, ?, ?, ?, ?)
                "#,
                libsql::params::Params::Positional(vec![
                    Value::from(id.clone()),
                    Value::from(article.title.clone()),
                    Value::from(article.content.clone()),
                    Value::from(article.subject.display_name()),
                    Value::from(article.generated_at.to_rfc3339()),
                    Value::from(article.word_count as i64),
                    Value::from(article.estimated_read_time as i64),
                ]),
            )
            .await?;

        Ok(id)
    }

    pub async fn get_recent(&self, limit: usize) -> AppResult<Vec<Article>> {
        let mut rows = self
            .conn
            .query(
                r#"
                SELECT title, content, subject, generated_at, word_count, estimated_read_time
                FROM articles
                ORDER BY generated_at DESC
                LIMIT ?
                "#,
                libsql::params::Params::Positional(vec![Value::from(limit as i64)]),
            )
            .await?;

        let mut articles = Vec::new();
        while let Some(row) = rows.next().await.map_err(|e| {
            crate::types::DatabaseError::Query(format!("Failed to fetch search row: {}", e))
        })? {
            if let Ok(article) = self.parse_article_row(&row) {
                articles.push(article);
            }
        }

        Ok(articles)
    }

    pub async fn search(&self, query: &str) -> AppResult<Vec<Article>> {
        let search_pattern = format!("%{}%", query);
        let mut rows = self
            .conn
            .query(
                r#"
                SELECT title, content, subject, generated_at, word_count, estimated_read_time
                FROM articles
                WHERE title LIKE ? OR content LIKE ?
                ORDER BY generated_at DESC
                "#,
                libsql::params::Params::Positional(vec![
                    Value::from(search_pattern.clone()),
                    Value::from(search_pattern),
                ]),
            )
            .await?;

        let mut articles = Vec::new();
        while let Some(row) = rows.next().await.map_err(|e| {
            crate::types::DatabaseError::Query(format!("Failed to fetch search row: {}", e))
        })? {
            if let Ok(article) = self.parse_article_row(&row) {
                articles.push(article);
            }
        }

        Ok(articles)
    }

    pub async fn delete_by_title(&self, title: &str) -> AppResult<bool> {
        let changes = self
            .conn
            .execute(
                "DELETE FROM articles WHERE title = ?",
                libsql::params::Params::Positional(vec![Value::from(title)]),
            )
            .await?;

        Ok(changes > 0)
    }

    fn parse_article_row(&self, row: &libsql::Row) -> AppResult<Article> {
        let subject_str: String = row.get(2).map_err(|e| {
            crate::types::DatabaseError::Query(format!("Failed to get subject: {}", e))
        })?;
        let subject = ArticleSubject::from_string(&subject_str);

        let generated_at_str: String = row.get(3).map_err(|e| {
            crate::types::DatabaseError::Query(format!("Failed to get generated_at: {}", e))
        })?;
        let generated_at = chrono::DateTime::parse_from_rfc3339(&generated_at_str)
            .map_err(|e| {
                crate::types::DatabaseError::Query(format!("Failed to parse generated_at: {}", e))
            })?
            .with_timezone(&chrono::Utc);

        Ok(Article {
            title: row.get(0).map_err(|e| {
                crate::types::DatabaseError::Query(format!("Failed to get title: {}", e))
            })?,
            content: row.get(1).map_err(|e| {
                crate::types::DatabaseError::Query(format!("Failed to get content: {}", e))
            })?,
            subject,
            generated_at,
            word_count: row.get::<i64>(4).map_err(|e| {
                crate::types::DatabaseError::Query(format!("Failed to get word_count: {}", e))
            })? as usize,
            estimated_read_time: row.get::<i64>(5).map_err(|e| {
                crate::types::DatabaseError::Query(format!(
                    "Failed to get estimated_read_time: {}",
                    e
                ))
            })? as u32,
        })
    }
}
