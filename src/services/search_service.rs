// file: src/services/search_service.rs
// description: Search functionality service

use crate::{
    services::DatabaseService,
    types::{AppResult, Article, InputValidator},
};

pub struct SearchService {
    database_service: DatabaseService,
}

impl SearchService {
    pub fn new(database_service: DatabaseService) -> Self {
        Self { database_service }
    }

    pub async fn search_articles(&self, query: &str) -> AppResult<Vec<Article>> {
        let sanitized_query = InputValidator::sanitize_search_query(query)?;
        self.database_service
            .search_articles(&sanitized_query)
            .await
    }

    pub async fn get_articles_by_timeframe(
        &self,
        timeframe: SearchTimeframe,
    ) -> AppResult<Vec<Article>> {
        match timeframe {
            SearchTimeframe::Today => {
                // Implementation would go here
                self.database_service.get_recent_articles(50).await
            }
            SearchTimeframe::LastWeek => {
                // Implementation would go here
                self.database_service.get_recent_articles(100).await
            }
            SearchTimeframe::LastMonth => {
                // Implementation would go here
                self.database_service.get_recent_articles(200).await
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum SearchTimeframe {
    Today,
    LastWeek,
    LastMonth,
}
