// file: src/client/groq_client.rs
// description: Groq API client with content cleaning for proper article display

use crate::{
    config::AppConfig,
    types::{
        AppError, AppResult, Article, ArticleSubject, InputValidator,
        reading_passage::{
            DifficultyLevel, ReadingPassage, ReadingPassageResponse, SubjectCategory,
        },
    },
};
use serde_json::{Value, json};
use std::time::Duration;

#[derive(Clone)]
pub struct GroqClient {
    client: reqwest::blocking::Client,
    api_key: String,
    base_url: String,
    max_retries: u32,
    base_delay: Duration,
}

impl GroqClient {
    pub fn new(config: &AppConfig) -> AppResult<Self> {
        let api_key = config.groq_api_key.clone();
        let base_url = config.groq_base_url.clone();

        if api_key.trim().is_empty() {
            return Err(AppError::Config("GROQ_API_KEY cannot be empty".to_string()));
        }

        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(180))
            .user_agent("reading-app/0.1.0")
            .connect_timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| AppError::Config(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self {
            client,
            api_key,
            base_url,
            max_retries: 3,
            base_delay: Duration::from_millis(1000),
        })
    }

    // Clean content for proper reading display
    fn clean_article_content(&self, content: &str) -> String {
        let mut cleaned = content.to_string();

        // Remove escape sequences and control characters
        cleaned = cleaned.replace("\\n", "\n");
        cleaned = cleaned.replace("\\t", " ");
        cleaned = cleaned.replace("\\r", "");

        // Remove HTML entities
        cleaned = cleaned.replace("&amp;", "&");
        cleaned = cleaned.replace("&lt;", "<");
        cleaned = cleaned.replace("&gt;", ">");
        cleaned = cleaned.replace("&quot;", "\"");
        cleaned = cleaned.replace("&#x27;", "'");
        cleaned = cleaned.replace("&#39;", "'");

        // Clean problematic unicode characters
        cleaned = cleaned
            .chars()
            .filter_map(|c| {
                match c {
                    // Keep standard printable ASCII
                    ' '..='~' => Some(c),

                    // Keep basic whitespace
                    '\n' | '\t' => Some(c),

                    // Replace problematic unicode with ASCII equivalents
                    '\u{2013}' | '\u{2014}' => Some('-'), // en-dash, em-dash
                    '\u{2018}' | '\u{2019}' => Some('\''), // smart quotes
                    '\u{201C}' | '\u{201D}' => Some('"'), // smart double quotes
                    '\u{2026}' => Some('.'),              // ellipsis

                    // Standard bullet point
                    '\u{2022}' | '\u{2023}' | '\u{25E6}' | '\u{2043}' => Some('•'),

                    // Remove control characters
                    c if c.is_control() && c != '\n' && c != '\t' => None,

                    // Replace other non-ASCII with space
                    c if c as u32 > 127 && !matches!(c, '•') => Some(' '),

                    _ => Some(c),
                }
            })
            .collect();

        // Normalize whitespace
        let lines: Vec<&str> = cleaned.lines().collect();
        let mut normalized_lines: Vec<String> = Vec::new();

        for line in lines {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                // Preserve paragraph breaks but limit consecutive empty lines
                if !normalized_lines.is_empty() && !normalized_lines.last().unwrap().is_empty() {
                    normalized_lines.push(String::new());
                }
            } else {
                // Clean up multiple spaces within lines
                let cleaned_line = trimmed.split_whitespace().collect::<Vec<_>>().join(" ");
                normalized_lines.push(cleaned_line);
            }
        }

        // Remove trailing empty lines
        while normalized_lines.last().is_some_and(|line| line.is_empty()) {
            normalized_lines.pop();
        }

        normalized_lines.join("\n")
    }

    // Existing article generation method with content cleaning
    pub fn generate_article(
        &self,
        subject: ArticleSubject,
        custom_topic: Option<String>,
    ) -> AppResult<Article> {
        let validated_topic = if let Some(topic) = custom_topic {
            let validated = InputValidator::validate_custom_topic(&topic)?;
            if validated.trim().is_empty() {
                None
            } else {
                Some(validated)
            }
        } else {
            None
        };

        let topic = validated_topic.unwrap_or_else(|| {
            format!(
                "an interesting and informative article about {}",
                subject.display_name().to_lowercase()
            )
        });

        let prompt = self.build_article_prompt(&subject, &topic);
        self.generate_article_with_retry(prompt, &subject)
    }

    // Async wrapper for article generation
    pub async fn generate_article_async(
        &self,
        subject: ArticleSubject,
        custom_topic: Option<String>,
    ) -> AppResult<Article> {
        let client_clone = self.clone();
        tokio::task::spawn_blocking(move || client_clone.generate_article(subject, custom_topic))
            .await
            .map_err(|e| AppError::Network(format!("Task join error: {}", e)))?
    }

    // Reading passage generation with content cleaning
    pub fn generate_reading_passage(
        &self,
        subject_category: Option<SubjectCategory>,
        difficulty_level: Option<DifficultyLevel>,
        custom_topic: Option<String>,
    ) -> AppResult<ReadingPassage> {
        let user_prompt =
            self.build_reading_passage_prompt(subject_category, difficulty_level, custom_topic);
        self.generate_reading_passage_with_retry(user_prompt)
    }

    fn generate_reading_passage_with_retry(
        &self,
        user_prompt: String,
    ) -> AppResult<ReadingPassage> {
        let mut last_error = None;

        for attempt in 0..=self.max_retries {
            match self.call_reading_passage_api(&user_prompt) {
                Ok(response_text) => {
                    return self.parse_reading_passage_response(&response_text);
                }
                Err(e) => {
                    last_error = Some(e);
                    if attempt < self.max_retries {
                        let delay = self.base_delay * 2_u32.pow(attempt);
                        std::thread::sleep(delay);
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| AppError::Network("Unknown error occurred".to_string())))
    }

    fn call_reading_passage_api(&self, user_prompt: &str) -> AppResult<String> {
        let system_prompt = include_str!("../../prompts/reading_passage_system_prompt.md");

        let request_body = json!({
            "messages": [
                {
                    "role": "system",
                    "content": system_prompt
                },
                {
                    "role": "user",
                    "content": user_prompt
                }
            ],
            "model": "openai/gpt-oss-20b",
            "temperature": 0.7,
            "max_completion_tokens": 40000,
            "top_p": 1,
            "stream": false,
            "reasoning_effort": "medium",
            "response_format": {"type": "json_object"},
            "stop": null
        });

        let response = self
            .client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .map_err(|e| AppError::Network(format!("Network error: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().unwrap_or_default();
            return Err(AppError::Network(format!(
                "API error {}: {}",
                status, error_text
            )));
        }

        let response_json: Value = response
            .json()
            .map_err(|e| AppError::Network(format!("Invalid JSON response: {}", e)))?;

        let content_str = response_json
            .get("choices")
            .and_then(|choices| choices.get(0))
            .and_then(|choice| choice.get("message"))
            .and_then(|message| message.get("content"))
            .and_then(|content| content.as_str())
            .ok_or_else(|| AppError::Network("Invalid response format".to_string()))?;

        Ok(content_str.to_string())
    }

    fn parse_reading_passage_response(&self, content_str: &str) -> AppResult<ReadingPassage> {
        let response: ReadingPassageResponse = serde_json::from_str(content_str).map_err(|e| {
            AppError::Network(format!("Failed to parse reading passage response: {}", e))
        })?;

        // Clean the content before creating the ReadingPassage
        let mut cleaned_response = response;
        cleaned_response.content = self.clean_article_content(&cleaned_response.content);
        cleaned_response.title = self.clean_article_content(&cleaned_response.title);

        Ok(ReadingPassage::from_response(cleaned_response))
    }

    fn build_reading_passage_prompt(
        &self,
        subject_category: Option<SubjectCategory>,
        difficulty_level: Option<DifficultyLevel>,
        custom_topic: Option<String>,
    ) -> String {
        let mut prompt_parts = Vec::new();

        // Add difficulty specification
        if let Some(level) = difficulty_level {
            prompt_parts.push(format!(
                "Generate a {} difficulty reading passage.",
                level.name()
            ));
        } else {
            prompt_parts.push("Generate an intermediate difficulty reading passage.".to_string());
        }

        // Add subject specification
        if let Some(category) = subject_category {
            prompt_parts.push(format!("Subject category: {}", category.name()));
        }

        // Add custom topic if provided
        if let Some(topic) = custom_topic.filter(|topic| !topic.trim().is_empty()) {
            prompt_parts.push(format!("Specific topic: {}", topic.trim()));
        }

        // Add content quality requirements
        prompt_parts.push("Create an engaging, educational passage with clear, readable text suitable for academic reading.".to_string());
        prompt_parts.push("Use standard English text without special characters, unicode symbols, or formatting artifacts.".to_string());
        prompt_parts
            .push("Follow standardized test format with multiple choice questions.".to_string());
        prompt_parts.push("Respond with valid JSON only, following the exact format specified in the system prompt.".to_string());

        prompt_parts.join(" ")
    }

    // Article generation methods with content cleaning
    fn generate_article_with_retry(
        &self,
        prompt: String,
        subject: &ArticleSubject,
    ) -> AppResult<Article> {
        let mut last_error = None;

        for attempt in 0..=self.max_retries {
            match self.call_article_api(&prompt) {
                Ok(response_text) => {
                    return self.parse_article_response(&response_text, subject);
                }
                Err(e) => {
                    last_error = Some(e);
                    if attempt < self.max_retries {
                        let delay = self.base_delay * 2_u32.pow(attempt);
                        std::thread::sleep(delay);
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| AppError::Network("Unknown error occurred".to_string())))
    }

    fn call_article_api(&self, prompt: &str) -> AppResult<String> {
        let enhanced_prompt = format!(
            "{} Create clear, readable content suitable for academic reading. Use standard English text without special characters, unicode symbols, or formatting artifacts that could interfere with reading comprehension.",
            prompt
        );

        let request_body = json!({
            "messages": [
                {
                    "role": "system",
                    "content": "You are an expert article writer who creates high-quality, informative content with clear, standard English text suitable for academic reading. Avoid special characters, unicode symbols, or formatting artifacts."
                },
                {
                    "role": "user",
                    "content": format!("{} Respond with JSON: {{\"title\": \"...\", \"content\": \"...\"}}", enhanced_prompt)
                }
            ],
            "model": "openai/gpt-oss-20b",
            "temperature": 0.7,
            "max_completion_tokens": 20000,
            "response_format": {"type": "json_object"}
        });

        let response = self
            .client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .map_err(|e| AppError::Network(format!("Network error: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::Network(format!(
                "API error: {}",
                response.status()
            )));
        }

        let response_json: Value = response
            .json()
            .map_err(|e| AppError::Network(format!("Invalid JSON response: {}", e)))?;

        let content_str = response_json
            .get("choices")
            .and_then(|choices| choices.get(0))
            .and_then(|choice| choice.get("message"))
            .and_then(|message| message.get("content"))
            .and_then(|content| content.as_str())
            .ok_or_else(|| AppError::Network("Invalid response format".to_string()))?;

        Ok(content_str.to_string())
    }

    fn parse_article_response(
        &self,
        content_str: &str,
        subject: &ArticleSubject,
    ) -> AppResult<Article> {
        let article_data: Value = serde_json::from_str(content_str)?;

        let title = article_data
            .get("title")
            .and_then(|t| t.as_str())
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| format!("{} Article", subject.display_name()));

        let content = article_data
            .get("content")
            .and_then(|c| c.as_str())
            .ok_or_else(|| AppError::Network("No content found in response".to_string()))?
            .trim()
            .to_string();

        // Clean the content before creating the Article
        let cleaned_title = self.clean_article_content(&title);
        let cleaned_content = self.clean_article_content(&content);

        Article::new(cleaned_title, cleaned_content, subject.clone())
    }

    fn build_article_prompt(&self, subject: &ArticleSubject, topic: &str) -> String {
        format!(
            "Write a comprehensive, well-researched article about {}.\n\
            Requirements:\n\
            - Clear, engaging title\n\
            - Well-structured content with headings\n\
            - 800-1200 words\n\
            - Use clean markdown formatting without special characters\n\
            - Write in clear, standard English suitable for academic reading\n\
            - Avoid unicode symbols, special characters, or formatting artifacts\n\
            - Focus on informative, educational content\n\
            Subject: {}\n\
            Return JSON with 'title' and 'content' fields.",
            topic,
            subject.description()
        )
    }
}

impl Default for GroqClient {
    fn default() -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
            api_key: String::new(),
            base_url: "https://api.groq.com/openai/v1".to_string(),
            max_retries: 3,
            base_delay: Duration::from_millis(1000),
        }
    }
}
