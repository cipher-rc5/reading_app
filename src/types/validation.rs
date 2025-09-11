// file: src/types/validation.rs
// description: Input validation utilities

use super::errors::{AppResult, ValidationError};

pub struct InputValidator;

impl InputValidator {
    pub fn validate_title(title: &str) -> AppResult<()> {
        let trimmed = title.trim();
        if trimmed.is_empty() {
            return Err(ValidationError::EmptyInput.into());
        }
        if trimmed.len() > 200 {
            return Err(ValidationError::InputTooLong {
                current: trimmed.len(),
                max: 200,
            }
            .into());
        }
        Ok(())
    }

    pub fn validate_content(content: &str) -> AppResult<()> {
        let trimmed = content.trim();
        if trimmed.is_empty() {
            return Err(ValidationError::EmptyInput.into());
        }
        if trimmed.len() > 50_000 {
            return Err(ValidationError::InputTooLong {
                current: trimmed.len(),
                max: 50_000,
            }
            .into());
        }
        Ok(())
    }

    pub fn sanitize_search_query(query: &str) -> AppResult<String> {
        let trimmed = query.trim();

        if trimmed.is_empty() {
            return Err(ValidationError::EmptyInput.into());
        }

        if trimmed.len() > 500 {
            return Err(ValidationError::InputTooLong {
                current: trimmed.len(),
                max: 500,
            }
            .into());
        }

        let sanitized: String = trimmed
            .chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace() || ".,!?-'\"".contains(*c))
            .collect();

        if sanitized.is_empty() {
            return Err(ValidationError::InvalidCharacters.into());
        }

        Ok(sanitized)
    }

    pub fn validate_custom_topic(topic: &str) -> AppResult<String> {
        let trimmed = topic.trim();

        if trimmed.len() > 1000 {
            return Err(ValidationError::InputTooLong {
                current: trimmed.len(),
                max: 1000,
            }
            .into());
        }

        Ok(trimmed.to_string())
    }
}
