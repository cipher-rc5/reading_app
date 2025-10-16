// file: src/types/reading_passage.rs
// description: generalized reading passage types that extend existing article system

use super::errors::AppResult;
use super::validation::InputValidator;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DifficultyLevel {
    Foundation,   // Level 1: 9-10th grade
    Intermediate, // Level 2: 10-11th grade
    Advanced,     // Level 3: 11-12th grade
    Elite,        // Level 4: College-level
}

impl DifficultyLevel {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Foundation,
            Self::Intermediate,
            Self::Advanced,
            Self::Elite,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Foundation => "Foundation",
            Self::Intermediate => "Intermediate",
            Self::Advanced => "Advanced",
            Self::Elite => "Elite",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Foundation => "Level 1 (9-10th grade reading level)",
            Self::Intermediate => "Level 2 (10-11th grade reading level)",
            Self::Advanced => "Level 3 (11-12th grade reading level)",
            Self::Elite => "Level 4 (College-level reading)",
        }
    }

    pub fn lexile_range(&self) -> &'static str {
        match self {
            Self::Foundation => "1050-1200L",
            Self::Intermediate => "1200-1350L",
            Self::Advanced => "1350-1450L",
            Self::Elite => "1450L+",
        }
    }

    pub fn from_string(s: &str) -> Self {
        match s {
            "Foundation" => Self::Foundation,
            "Intermediate" => Self::Intermediate,
            "Advanced" => Self::Advanced,
            "Elite" => Self::Elite,
            _ => Self::Foundation, // Default fallback
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SubjectCategory {
    Science,
    Sciences,
    SocialSciences,
    Humanities,
    History,
    Literature,
    Technology,
    Arts,
    Philosophy,
    Entrepreneurship,
    Interdisciplinary,
    General,
}

impl SubjectCategory {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Science,
            Self::Sciences,
            Self::SocialSciences,
            Self::Humanities,
            Self::History,
            Self::Literature,
            Self::Technology,
            Self::Arts,
            Self::Philosophy,
            Self::Entrepreneurship,
            Self::Interdisciplinary,
            Self::General,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Science => "Science",
            Self::Sciences => "Sciences",
            Self::SocialSciences => "Social Sciences",
            Self::Humanities => "Humanities",
            Self::History => "History",
            Self::Literature => "Literature",
            Self::Technology => "Technology",
            Self::Arts => "Arts",
            Self::Philosophy => "Philosophy",
            Self::Entrepreneurship => "Entrepreneurship",
            Self::Interdisciplinary => "Interdisciplinary",
            Self::General => "General",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Science => "Scientific discoveries, research, and breakthroughs",
            Self::Sciences => "Biology, Physics, Chemistry, Technology, Medicine",
            Self::SocialSciences => "Psychology, Sociology, Economics, Political Science",
            Self::Humanities => "History, Philosophy, Literature, Art, Linguistics",
            Self::History => "Historical events, figures, and cultural heritage",
            Self::Literature => "Literary analysis, book reviews, and writing techniques",
            Self::Technology => "Latest tech trends, innovations, and digital developments",
            Self::Arts => "Visual arts, performing arts, and creative expression",
            Self::Philosophy => "Philosophical thought, ethics, and reasoning",
            Self::Entrepreneurship => "Business, Innovation, Leadership, Ventures",
            Self::Interdisciplinary => "Environmental Studies, Digital Humanities, Bioethics",
            Self::General => "General topics across various subjects",
        }
    }

    pub fn from_string(s: &str) -> Self {
        match s {
            "Science" => Self::Science,
            "Sciences" => Self::Sciences,
            "Social Sciences" => Self::SocialSciences,
            "Humanities" => Self::Humanities,
            "History" => Self::History,
            "Literature" => Self::Literature,
            "Technology" => Self::Technology,
            "Arts" => Self::Arts,
            "Philosophy" => Self::Philosophy,
            "Entrepreneurship" => Self::Entrepreneurship,
            "Interdisciplinary" => Self::Interdisciplinary,
            "General" => Self::General,
            _ => Self::General, // Default fallback
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadingPassageInfo {
    pub number: u32,
    pub subject: String,
    pub difficulty: String,
    pub lexile_range: String,
    pub estimated_time: String,
    pub learning_objectives: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionOptions {
    #[serde(rename = "A")]
    pub a: String,
    #[serde(rename = "B")]
    pub b: String,
    #[serde(rename = "C")]
    pub c: String,
    #[serde(rename = "D")]
    pub d: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComprehensionQuestion {
    pub number: u32,
    #[serde(rename = "type")]
    pub question_type: String,
    pub question: String,
    pub options: QuestionOptions,
    pub correct_answer: String,
    pub explanation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillPracticed {
    pub skill: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NextRecommendation {
    pub topic: String,
    pub level: String,
    pub reasoning: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadingPassageResponse {
    pub passage_info: ReadingPassageInfo,
    pub title: String,
    pub content: String,
    pub questions: Vec<ComprehensionQuestion>,
    pub skills_practiced: Vec<SkillPracticed>,
    pub next_recommendation: NextRecommendation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadingPassage {
    pub title: String,
    pub content: String,
    pub subject_category: SubjectCategory,
    pub difficulty_level: DifficultyLevel,
    pub lexile_range: String,
    pub estimated_time: String,
    pub learning_objectives: Vec<String>,
    pub questions: Vec<ComprehensionQuestion>,
    pub skills_practiced: Vec<SkillPracticed>,
    pub next_recommendation: Option<NextRecommendation>,
    pub generated_at: chrono::DateTime<chrono::Utc>,
    pub word_count: usize,
}

impl ReadingPassage {
    pub fn new(
        title: String,
        content: String,
        subject_category: SubjectCategory,
        difficulty_level: DifficultyLevel,
        questions: Vec<ComprehensionQuestion>,
    ) -> AppResult<Self> {
        InputValidator::validate_title(&title)?;
        InputValidator::validate_content(&content)?;

        let word_count = content.split_whitespace().count();
        let estimated_time = Self::calculate_estimated_time(word_count, questions.len());

        Ok(Self {
            title,
            content,
            subject_category,
            difficulty_level: difficulty_level.clone(),
            lexile_range: difficulty_level.lexile_range().to_string(),
            estimated_time,
            learning_objectives: Vec::new(),
            questions,
            skills_practiced: Vec::new(),
            next_recommendation: None,
            generated_at: chrono::Utc::now(),
            word_count,
        })
    }

    pub fn from_response(response: ReadingPassageResponse) -> Self {
        let word_count = response.content.split_whitespace().count();

        // Parse difficulty from response
        let difficulty_level = if response.passage_info.difficulty.contains("Level 1") {
            DifficultyLevel::Foundation
        } else if response.passage_info.difficulty.contains("Level 2") {
            DifficultyLevel::Intermediate
        } else if response.passage_info.difficulty.contains("Level 3") {
            DifficultyLevel::Advanced
        } else {
            DifficultyLevel::Elite
        };

        // Parse subject category from response
        let subject_category = SubjectCategory::from_string(&response.passage_info.subject);

        Self {
            title: response.title,
            content: response.content,
            subject_category,
            difficulty_level,
            lexile_range: response.passage_info.lexile_range,
            estimated_time: response.passage_info.estimated_time,
            learning_objectives: response.passage_info.learning_objectives,
            questions: response.questions,
            skills_practiced: response.skills_practiced,
            next_recommendation: Some(response.next_recommendation),
            generated_at: chrono::Utc::now(),
            word_count,
        }
    }

    fn calculate_estimated_time(word_count: usize, question_count: usize) -> String {
        // Reading time: ~200 words per minute
        // Question time: ~1 minute per question
        let reading_minutes = (word_count as f32 / 200.0).ceil() as u32;
        let question_minutes = question_count as u32;
        let total_minutes = reading_minutes + question_minutes;

        format!("{}-{} minutes", total_minutes, total_minutes + 2)
    }

    pub fn get_question_by_number(&self, number: u32) -> Option<&ComprehensionQuestion> {
        self.questions.iter().find(|q| q.number == number)
    }

    pub fn get_correct_answers(&self) -> Vec<(u32, String)> {
        self.questions
            .iter()
            .map(|q| (q.number, q.correct_answer.clone()))
            .collect()
    }

    pub fn calculate_score(&self, user_answers: &[(u32, String)]) -> f32 {
        if self.questions.is_empty() {
            return 0.0;
        }

        let correct_count = user_answers
            .iter()
            .filter_map(|(number, answer)| {
                self.get_question_by_number(*number)
                    .map(|q| q.correct_answer == *answer)
            })
            .filter(|&correct| correct)
            .count();

        (correct_count as f32 / self.questions.len() as f32) * 100.0
    }
}

// Extend existing ContentType enum in article.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentType {
    Article {
        subject: crate::types::ArticleSubject,
        content: crate::types::Article,
    },
    ReadingPassage {
        subject_category: SubjectCategory,
        difficulty_level: DifficultyLevel,
        content: ReadingPassage,
    },
}

impl ContentType {
    pub fn get_title(&self) -> &str {
        match self {
            Self::Article { content, .. } => &content.title,
            Self::ReadingPassage { content, .. } => &content.title,
        }
    }

    pub fn get_content(&self) -> &str {
        match self {
            Self::Article { content, .. } => &content.content,
            Self::ReadingPassage { content, .. } => &content.content,
        }
    }

    pub fn get_word_count(&self) -> usize {
        match self {
            Self::Article { content, .. } => content.word_count,
            Self::ReadingPassage { content, .. } => content.word_count,
        }
    }

    pub fn get_generated_at(&self) -> chrono::DateTime<chrono::Utc> {
        match self {
            Self::Article { content, .. } => content.generated_at,
            Self::ReadingPassage { content, .. } => content.generated_at,
        }
    }

    pub fn get_estimated_time(&self) -> String {
        match self {
            Self::Article { content, .. } => format!("{}m", content.estimated_read_time),
            Self::ReadingPassage { content, .. } => content.estimated_time.clone(),
        }
    }

    pub fn get_subject_display(&self) -> String {
        match self {
            Self::Article { subject, .. } => subject.display_name().to_string(),
            Self::ReadingPassage {
                subject_category, ..
            } => subject_category.name().to_string(),
        }
    }

    pub fn is_article(&self) -> bool {
        matches!(self, Self::Article { .. })
    }

    pub fn is_reading_passage(&self) -> bool {
        matches!(self, Self::ReadingPassage { .. })
    }

    pub fn as_article(&self) -> Option<&crate::types::Article> {
        match self {
            Self::Article { content, .. } => Some(content),
            _ => None,
        }
    }

    pub fn as_reading_passage(&self) -> Option<&ReadingPassage> {
        match self {
            Self::ReadingPassage { content, .. } => Some(content),
            _ => None,
        }
    }

    pub fn content_type_string(&self) -> &'static str {
        match self {
            Self::Article { .. } => "article",
            Self::ReadingPassage { .. } => "reading_passage",
        }
    }
}
