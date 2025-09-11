// file: src/types/article.rs
// description: Article-related types with proper import structure

use super::errors::AppResult;
use super::validation::InputValidator;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArticleSubject {
    Technology,
    Science,
    History,
    Literature,
    Health,
    Business,
    Sports,
    Travel,
    Cooking,
    Education,
}

impl ArticleSubject {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Technology,
            Self::Science,
            Self::History,
            Self::Literature,
            Self::Health,
            Self::Business,
            Self::Sports,
            Self::Travel,
            Self::Cooking,
            Self::Education,
        ]
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Technology => "Technology",
            Self::Science => "Science",
            Self::History => "History",
            Self::Literature => "Literature",
            Self::Health => "Health",
            Self::Business => "Business",
            Self::Sports => "Sports",
            Self::Travel => "Travel",
            Self::Cooking => "Cooking",
            Self::Education => "Education",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Technology => "Latest tech trends, innovations, and digital developments",
            Self::Science => "Scientific discoveries, research, and breakthroughs",
            Self::History => "Historical events, figures, and cultural heritage",
            Self::Literature => "Literary analysis, book reviews, and writing techniques",
            Self::Health => "Health tips, medical research, and wellness advice",
            Self::Business => "Business strategies, market analysis, and entrepreneurship",
            Self::Sports => "Sports news, athlete profiles, and game analysis",
            Self::Travel => "Travel guides, destinations, and cultural experiences",
            Self::Cooking => "Recipes, cooking techniques, and culinary traditions",
            Self::Education => "Learning methods, educational resources, and academic topics",
        }
    }

    pub fn from_string(s: &str) -> Self {
        match s {
            "Technology" => Self::Technology,
            "Science" => Self::Science,
            "History" => Self::History,
            "Literature" => Self::Literature,
            "Health" => Self::Health,
            "Business" => Self::Business,
            "Sports" => Self::Sports,
            "Travel" => Self::Travel,
            "Cooking" => Self::Cooking,
            "Education" => Self::Education,
            _ => Self::Technology,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub title: String,
    pub content: String,
    pub subject: ArticleSubject,
    pub generated_at: chrono::DateTime<chrono::Utc>,
    pub word_count: usize,
    pub estimated_read_time: u32,
}

impl Article {
    pub fn new(title: String, content: String, subject: ArticleSubject) -> AppResult<Self> {
        InputValidator::validate_title(&title)?;
        InputValidator::validate_content(&content)?;

        let word_count = content.split_whitespace().count();
        let estimated_read_time = (word_count / 200).max(1) as u32;

        Ok(Self {
            title,
            content,
            subject,
            generated_at: chrono::Utc::now(),
            word_count,
            estimated_read_time,
        })
    }
}

// Helper methods for ArticleSubject that don't require reading_passage imports
impl ArticleSubject {
    pub fn get_category_name(&self) -> &'static str {
        match self {
            Self::Technology | Self::Science => "Sciences",
            Self::Business => "Entrepreneurship",
            Self::History | Self::Literature => "Humanities",
            Self::Health | Self::Education => "Social Sciences",
            Self::Sports | Self::Travel | Self::Cooking => "Interdisciplinary",
        }
    }
}
