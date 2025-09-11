// file: src/services/article_service.rs
// description: Enhanced article service supporting both articles and reading passages

use crate::{
    client::GroqClient,
    config::AppConfig,
    types::{
        reading_passage::{DifficultyLevel, ReadingPassage, SubjectCategory},
        AppResult, Article, ArticleSubject, ContentType,
    },
};
use std::sync::Arc;
use tracing::info;

#[derive(Clone)]
pub struct ArticleService {
    client: Arc<GroqClient>,
}

impl ArticleService {
    pub fn new(config: &AppConfig) -> AppResult<Self> {
        let client = GroqClient::new(config)?;
        Ok(Self {
            client: Arc::new(client),
        })
    }

    /// Generate a traditional article
    pub fn generate_article(
        &self,
        subject: ArticleSubject,
        custom_topic: Option<String>,
    ) -> AppResult<Article> {
        info!("Generating article for subject: {:?}", subject);
        self.client.generate_article(subject, custom_topic)
    }

    /// Generate a traditional article (async version)
    pub async fn generate_article_async(
        &self,
        subject: ArticleSubject,
        custom_topic: Option<String>,
    ) -> AppResult<Article> {
        info!("Generating article for subject: {:?}", subject);
        self.client
            .generate_article_async(subject, custom_topic)
            .await
    }

    /// Generate a structured reading passage with comprehension questions
    pub fn generate_reading_passage(
        &self,
        subject_category: Option<SubjectCategory>,
        difficulty_level: Option<DifficultyLevel>,
        custom_topic: Option<String>,
    ) -> AppResult<ReadingPassage> {
        info!(
            "Generating reading passage - Category: {:?}, Difficulty: {:?}",
            subject_category, difficulty_level
        );
        self.client
            .generate_reading_passage(subject_category, difficulty_level, custom_topic)
    }

    /// Generate content based on unified parameters
    pub fn generate_content(&self, request: ContentGenerationRequest) -> AppResult<ContentType> {
        match request {
            ContentGenerationRequest::Article {
                subject,
                custom_topic,
            } => {
                let article = self.generate_article(subject, custom_topic)?;
                Ok(ContentType::Article {
                    subject: article.subject.clone(),
                    content: article,
                })
            }
            ContentGenerationRequest::ReadingPassage {
                subject_category,
                difficulty_level,
                custom_topic,
            } => {
                let passage = self.generate_reading_passage(
                    subject_category,
                    difficulty_level,
                    custom_topic,
                )?;
                Ok(ContentType::ReadingPassage {
                    subject_category: passage.subject_category.clone(),
                    difficulty_level: passage.difficulty_level.clone(),
                    content: passage,
                })
            }
        }
    }

    /// Convert article subject to reading passage equivalent
    pub fn article_to_reading_passage(
        &self,
        article_subject: ArticleSubject,
        difficulty_level: Option<DifficultyLevel>,
        custom_topic: Option<String>,
    ) -> AppResult<ReadingPassage> {
        // Map ArticleSubject to SubjectCategory manually since the method doesn't exist
        let subject_category = match article_subject {
            ArticleSubject::Science => SubjectCategory::Science,
            ArticleSubject::History => SubjectCategory::History,
            ArticleSubject::Literature => SubjectCategory::Literature,
            ArticleSubject::Technology => SubjectCategory::Technology,
            ArticleSubject::Business => SubjectCategory::Entrepreneurship,
            ArticleSubject::Health => SubjectCategory::SocialSciences,
            ArticleSubject::Education => SubjectCategory::SocialSciences,
            // Map other subjects to appropriate categories or use a default
            _ => SubjectCategory::General,
        };

        self.generate_reading_passage(Some(subject_category), difficulty_level, custom_topic)
    }

    /// Get available difficulty levels
    pub fn get_available_difficulty_levels(&self) -> Vec<DifficultyLevel> {
        DifficultyLevel::all()
    }

    /// Get available subject categories for reading passages
    pub fn get_available_subject_categories(&self) -> Vec<SubjectCategory> {
        SubjectCategory::all()
    }

    /// Get recommended difficulty based on user performance (placeholder for future implementation)
    pub fn get_recommended_difficulty(&self, _user_performance: Option<f32>) -> DifficultyLevel {
        // For now, return intermediate as default
        // In the future, this could analyze user performance data
        DifficultyLevel::Intermediate
    }
}

#[derive(Debug, Clone)]
pub enum ContentGenerationRequest {
    Article {
        subject: ArticleSubject,
        custom_topic: Option<String>,
    },
    ReadingPassage {
        subject_category: Option<SubjectCategory>,
        difficulty_level: Option<DifficultyLevel>,
        custom_topic: Option<String>,
    },
}

impl ContentGenerationRequest {
    pub fn new_article(subject: ArticleSubject, custom_topic: Option<String>) -> Self {
        Self::Article {
            subject,
            custom_topic,
        }
    }

    pub fn new_reading_passage(
        subject_category: Option<SubjectCategory>,
        difficulty_level: Option<DifficultyLevel>,
        custom_topic: Option<String>,
    ) -> Self {
        Self::ReadingPassage {
            subject_category,
            difficulty_level,
            custom_topic,
        }
    }

    pub fn get_custom_topic(&self) -> &Option<String> {
        match self {
            Self::Article { custom_topic, .. } => custom_topic,
            Self::ReadingPassage { custom_topic, .. } => custom_topic,
        }
    }

    pub fn is_article_request(&self) -> bool {
        matches!(self, Self::Article { .. })
    }

    pub fn is_reading_passage_request(&self) -> bool {
        matches!(self, Self::ReadingPassage { .. })
    }
}

impl Default for ArticleService {
    fn default() -> Self {
        // Create a dummy service for fallback
        Self {
            client: Arc::new(GroqClient::default()),
        }
    }
}
