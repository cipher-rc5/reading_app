// file: src/ui/events.rs
// description: Enhanced UI event handling with reading passage support

use crate::services::article_service::ContentGenerationRequest;
use crate::types::{
    reading_passage::{DifficultyLevel, ReadingPassage, SubjectCategory},
    AppError, Article, ArticleSubject, ContentType,
};

#[derive(Debug, Clone)]
pub enum UIEvent {
    // Existing article generation
    GenerateArticle {
        subject: ArticleSubject,
        custom_topic: Option<String>,
    },

    // New reading passage generation
    GenerateReadingPassage {
        subject_category: Option<SubjectCategory>,
        difficulty_level: Option<DifficultyLevel>,
        custom_topic: Option<String>,
    },

    // Unified content generation
    GenerateContent(ContentGenerationRequest),

    // Content loading and management
    ArticleGenerated(Article),
    ReadingPassageGenerated(ReadingPassage),
    ContentGenerated(ContentType),
    LoadContent(ContentType),
    LoadArticle(Article),
    LoadReadingPassage(ReadingPassage),

    // Error handling
    Error(AppError),

    // Content management
    DeleteArticle(String),
    DeleteReadingPassage(String),
    DeleteContent(String, String), // (title, content_type)

    // Search and discovery
    SearchQuery(String),
    SearchResults(Vec<Article>),
    SearchArticles(String),
    SearchReadingPassages(String),

    // Window management
    OpenSettings,
    OpenSearch,
    OpenDebug,
    SettingsChanged,

    // UI state management
    ToggleSidebar,
    ToggleBookmarks,

    // Content actions
    CopyArticle(Article),
    CopyReadingPassage(ReadingPassage),
    CopyContent(ContentType),
    DownloadArticle(Article),
    DownloadReadingPassage(ReadingPassage),
    DownloadContent(ContentType),
    BookmarkArticle(Article),
    BookmarkReadingPassage(ReadingPassage),
    BookmarkContent(ContentType),
    UnbookmarkArticle(Article),
    UnbookmarkReadingPassage(ReadingPassage),
    UnbookmarkContent(ContentType),

    // Reading passage specific events
    StartReadingPassage(ReadingPassage),
    AnswerQuestion {
        passage_id: String,
        question_number: u32,
        selected_answer: String,
    },
    SubmitAllAnswers {
        passage_id: String,
        answers: Vec<(u32, String)>, // (question_number, selected_answer)
    },
    ShowQuestionExplanation {
        passage_id: String,
        question_number: u32,
    },
    ViewPassageProgress(String), // passage_id
    RestartPassage(String),      // passage_id

    // Interactive text features
    LookupDefinition(String),
    ExplainText {
        text: String,
        context: String,
    },

    // Reading progress and analytics
    UpdateReadingProgress {
        content_id: String,
        content_type: String,
        time_spent: u32,
        progress_percentage: f32,
    },
    ShowReadingStats,

    // Content filtering and preferences
    FilterByDifficulty(Option<DifficultyLevel>),
    FilterBySubjectCategory(Option<SubjectCategory>),
    FilterByArticleSubject(Option<ArticleSubject>),
    ShowContentType(ContentTypeFilter),

    // Adaptive learning features
    RequestAdaptiveDifficulty,
    UpdateUserPerformance {
        passage_id: String,
        score: f32,
        time_taken: u32,
    },
}

#[derive(Debug, Clone)]
pub enum ContentTypeFilter {
    All,
    ArticlesOnly,
    ReadingPassagesOnly,
    ByDifficulty(DifficultyLevel),
    BySubjectCategory(SubjectCategory),
    ByArticleSubject(ArticleSubject),
}

impl UIEvent {
    // Helper constructors for common events
    pub fn generate_article(subject: ArticleSubject, custom_topic: Option<String>) -> Self {
        Self::GenerateArticle {
            subject,
            custom_topic,
        }
    }

    pub fn generate_reading_passage(
        subject_category: Option<SubjectCategory>,
        difficulty_level: Option<DifficultyLevel>,
        custom_topic: Option<String>,
    ) -> Self {
        Self::GenerateReadingPassage {
            subject_category,
            difficulty_level,
            custom_topic,
        }
    }

    pub fn answer_question(
        passage_id: String,
        question_number: u32,
        selected_answer: String,
    ) -> Self {
        Self::AnswerQuestion {
            passage_id,
            question_number,
            selected_answer,
        }
    }

    // Helper methods for event categorization
    pub fn is_content_generation(&self) -> bool {
        matches!(
            self,
            Self::GenerateArticle { .. }
                | Self::GenerateReadingPassage { .. }
                | Self::GenerateContent(_)
        )
    }

    pub fn is_content_action(&self) -> bool {
        matches!(
            self,
            Self::CopyArticle(_)
                | Self::CopyReadingPassage(_)
                | Self::CopyContent(_)
                | Self::DownloadArticle(_)
                | Self::DownloadReadingPassage(_)
                | Self::DownloadContent(_)
                | Self::BookmarkArticle(_)
                | Self::BookmarkReadingPassage(_)
                | Self::BookmarkContent(_)
                | Self::UnbookmarkArticle(_)
                | Self::UnbookmarkReadingPassage(_)
                | Self::UnbookmarkContent(_)
        )
    }

    pub fn is_reading_passage_interaction(&self) -> bool {
        matches!(
            self,
            Self::StartReadingPassage(_)
                | Self::AnswerQuestion { .. }
                | Self::SubmitAllAnswers { .. }
                | Self::ShowQuestionExplanation { .. }
                | Self::ViewPassageProgress(_)
                | Self::RestartPassage(_)
        )
    }

    pub fn is_window_management(&self) -> bool {
        matches!(
            self,
            Self::OpenSettings
                | Self::OpenSearch
                | Self::OpenDebug
                | Self::ToggleSidebar
                | Self::ToggleBookmarks
        )
    }

    pub fn is_filter_event(&self) -> bool {
        matches!(
            self,
            Self::FilterByDifficulty(_)
                | Self::FilterBySubjectCategory(_)
                | Self::FilterByArticleSubject(_)
                | Self::ShowContentType(_)
        )
    }

    // Extract content from events for processing
    pub fn extract_content(&self) -> Option<&ContentType> {
        match self {
            Self::ContentGenerated(content) | Self::LoadContent(content) => Some(content),
            _ => None,
        }
    }

    pub fn extract_article(&self) -> Option<&Article> {
        match self {
            Self::ArticleGenerated(article)
            | Self::LoadArticle(article)
            | Self::CopyArticle(article)
            | Self::DownloadArticle(article)
            | Self::BookmarkArticle(article)
            | Self::UnbookmarkArticle(article) => Some(article),
            _ => None,
        }
    }

    pub fn extract_reading_passage(&self) -> Option<&ReadingPassage> {
        match self {
            Self::ReadingPassageGenerated(passage)
            | Self::LoadReadingPassage(passage)
            | Self::StartReadingPassage(passage)
            | Self::CopyReadingPassage(passage)
            | Self::DownloadReadingPassage(passage)
            | Self::BookmarkReadingPassage(passage)
            | Self::UnbookmarkReadingPassage(passage) => Some(passage),
            _ => None,
        }
    }
}
