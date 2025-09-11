// file: src/types/mod.rs
pub mod article;
pub mod errors;
pub mod reading_passage;
pub mod settings;
pub mod time_utils;
pub mod validation;

pub use article::{Article, ArticleSubject};
pub use errors::{AppError, AppResult, DatabaseError, ValidationError};
pub use reading_passage::{
    ComprehensionQuestion, ContentType, DifficultyLevel, NextRecommendation, QuestionOptions,
    ReadingPassage, ReadingPassageInfo, ReadingPassageResponse, SkillPracticed, SubjectCategory,
};
pub use settings::UISettings;
pub use validation::InputValidator;

#[derive(Debug, Clone)]
pub enum RequestStatus {
    Idle,
    Loading,
    Success(ContentType), // Updated to support both articles and reading passages
    Error(AppError),
}

impl Default for RequestStatus {
    fn default() -> Self {
        Self::Idle
    }
}
