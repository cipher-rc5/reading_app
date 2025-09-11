// file: src/database/schema.rs
// description: Fixed database schema with proper column handling

use super::connection::DatabaseConnection;
use crate::types::AppResult;
use tracing::info;

pub async fn initialize(conn: &DatabaseConnection) -> AppResult<()> {
    info!("Initializing enhanced database schema with reading passage support");

    // Existing articles table
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS articles (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            subject TEXT NOT NULL,
            generated_at TEXT NOT NULL,
            word_count INTEGER NOT NULL,
            estimated_read_time INTEGER NOT NULL,
            is_favorited INTEGER DEFAULT 0,
            user_rating INTEGER DEFAULT NULL,
            tags TEXT DEFAULT '[]',
            created_at TEXT DEFAULT CURRENT_TIMESTAMP
        )
        "#,
        libsql::params::Params::Positional(vec![]),
    )
    .await?;

    // Reading history table - FIXED to check existing structure first
    // First, check if the table exists and what columns it has
    let mut existing_columns = Vec::new();

    // Get table info
    let mut rows = conn
        .query(
            "PRAGMA table_info(reading_history)",
            libsql::params::Params::Positional(vec![]),
        )
        .await?;

    while let Some(row) = rows.next().await.map_err(|e| {
        crate::types::DatabaseError::Query(format!("Failed to check table info: {}", e))
    })? {
        if let Ok(column_name) = row.get::<String>(1) {
            existing_columns.push(column_name);
        }
    }

    // If table doesn't exist or is missing columns, recreate it
    if existing_columns.is_empty() || !existing_columns.contains(&"passage_id".to_string()) {
        info!("Recreating reading_history table with proper schema");

        // Drop the old table if it exists
        conn.execute(
            "DROP TABLE IF EXISTS reading_history",
            libsql::params::Params::Positional(vec![]),
        )
        .await?;

        // Create the new table with all required columns
        conn.execute(
            r#"
            CREATE TABLE reading_history (
                id TEXT PRIMARY KEY,
                article_id TEXT,
                passage_id TEXT,
                content_type TEXT NOT NULL DEFAULT 'article',
                opened_at TEXT NOT NULL,
                reading_time_seconds INTEGER DEFAULT 0,
                completed INTEGER DEFAULT 0,
                last_position INTEGER DEFAULT 0,
                FOREIGN KEY (article_id) REFERENCES articles(id) ON DELETE CASCADE,
                FOREIGN KEY (passage_id) REFERENCES reading_passages(id) ON DELETE CASCADE
            )
            "#,
            libsql::params::Params::Positional(vec![]),
        )
        .await?;
    }

    // New reading passages table
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS reading_passages (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            subject_category TEXT NOT NULL,
            difficulty_level TEXT NOT NULL,
            lexile_range TEXT NOT NULL,
            estimated_time TEXT NOT NULL,
            learning_objectives TEXT NOT NULL, -- JSON array
            skills_practiced TEXT NOT NULL, -- JSON array
            next_recommendation TEXT, -- JSON object
            generated_at TEXT NOT NULL,
            word_count INTEGER NOT NULL,
            is_favorited INTEGER DEFAULT 0,
            user_rating INTEGER DEFAULT NULL,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP
        )
        "#,
        libsql::params::Params::Positional(vec![]),
    )
    .await?;

    // Reading passage questions table
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS reading_passage_questions (
            id TEXT PRIMARY KEY,
            passage_id TEXT NOT NULL,
            question_number INTEGER NOT NULL,
            question_type TEXT NOT NULL,
            question_text TEXT NOT NULL,
            option_a TEXT NOT NULL,
            option_b TEXT NOT NULL,
            option_c TEXT NOT NULL,
            option_d TEXT NOT NULL,
            correct_answer TEXT NOT NULL,
            explanation TEXT NOT NULL,
            FOREIGN KEY (passage_id) REFERENCES reading_passages(id) ON DELETE CASCADE
        )
        "#,
        libsql::params::Params::Positional(vec![]),
    )
    .await?;

    // User progress tracking for reading passages
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS reading_passage_progress (
            id TEXT PRIMARY KEY,
            passage_id TEXT NOT NULL,
            questions_answered INTEGER DEFAULT 0,
            questions_correct INTEGER DEFAULT 0,
            score_percentage REAL DEFAULT 0.0,
            time_spent_seconds INTEGER DEFAULT 0,
            completed_at TEXT,
            user_answers TEXT, -- JSON array of {question_number, selected_answer}
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (passage_id) REFERENCES reading_passages(id) ON DELETE CASCADE
        )
        "#,
        libsql::params::Params::Positional(vec![]),
    )
    .await?;

    // Enhanced user preferences table
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS user_preferences (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            -- Basic settings
            font_size REAL DEFAULT 14.0,
            zoom_level REAL DEFAULT 1.0,
            background_color TEXT DEFAULT '#2b2b2b',
            text_color TEXT DEFAULT '#ffffff',
            font_family TEXT DEFAULT 'default',
            theme_mode TEXT DEFAULT 'dark',
            show_article_stats INTEGER DEFAULT 1,
            sidebar_width REAL DEFAULT 300.0,

            -- Enhanced font settings from bibliotheca
            text_body_font_size REAL DEFAULT 14.0,
            header_font_size REAL DEFAULT 20.0,
            text_body_font TEXT DEFAULT 'default',
            header_font TEXT DEFAULT 'default',
            line_height REAL DEFAULT 1.5,
            paragraph_spacing REAL DEFAULT 8.0,
            header_color TEXT DEFAULT '#ffffff',
            link_color TEXT DEFAULT '#4a9eff',
            accent_color TEXT DEFAULT '#ff6b6b',
            corner_style TEXT DEFAULT 'rounded',

            -- Reading passage preferences
            show_passage_progress INTEGER DEFAULT 1,
            auto_advance_questions INTEGER DEFAULT 0,
            show_explanations_immediately INTEGER DEFAULT 0,
            preferred_difficulty TEXT DEFAULT 'Intermediate',

            updated_at TEXT DEFAULT CURRENT_TIMESTAMP
        )
        "#,
        libsql::params::Params::Positional(vec![]),
    )
    .await?;

    // Create indexes for better performance
    let indexes = [
        "CREATE INDEX IF NOT EXISTS idx_articles_generated_at ON articles(generated_at DESC)",
        "CREATE INDEX IF NOT EXISTS idx_articles_subject ON articles(subject)",
        "CREATE INDEX IF NOT EXISTS idx_reading_history_article_id ON reading_history(article_id)",
        "CREATE INDEX IF NOT EXISTS idx_reading_history_passage_id ON reading_history(passage_id)",
        "CREATE INDEX IF NOT EXISTS idx_reading_history_opened_at ON reading_history(opened_at DESC)",
        "CREATE INDEX IF NOT EXISTS idx_reading_history_content_type ON reading_history(content_type)",
    ];

    for index_sql in &indexes {
        conn.execute(index_sql, libsql::params::Params::Positional(vec![]))
            .await?;
    }

    // Insert default user preferences if none exist
    let count: i64 = {
        let mut rows = conn
            .query(
                "SELECT COUNT(*) as count FROM user_preferences",
                libsql::params::Params::Positional(vec![]),
            )
            .await?;

        if let Some(row) = rows.next().await.map_err(|e| {
            crate::types::DatabaseError::Query(format!("Failed to get preferences count: {}", e))
        })? {
            row.get::<i64>(0).unwrap_or(0)
        } else {
            0
        }
    };

    if count == 0 {
        conn.execute(
            r#"
            INSERT INTO user_preferences
            (font_size, zoom_level, background_color, text_color, font_family, theme_mode,
             show_article_stats, sidebar_width, text_body_font_size, header_font_size,
             text_body_font, header_font, line_height, paragraph_spacing, header_color,
             link_color, accent_color, corner_style, show_passage_progress,
             auto_advance_questions, show_explanations_immediately, preferred_difficulty)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            libsql::params::Params::Positional(vec![
                libsql::Value::from(14.0),
                libsql::Value::from(1.0),
                libsql::Value::from("#2b2b2b"),
                libsql::Value::from("#ffffff"),
                libsql::Value::from("default"),
                libsql::Value::from("dark"),
                libsql::Value::from(1),
                libsql::Value::from(300.0),
                libsql::Value::from(14.0),
                libsql::Value::from(20.0),
                libsql::Value::from("default"),
                libsql::Value::from("default"),
                libsql::Value::from(1.5),
                libsql::Value::from(8.0),
                libsql::Value::from("#ffffff"),
                libsql::Value::from("#4a9eff"),
                libsql::Value::from("#ff6b6b"),
                libsql::Value::from("rounded"),
                libsql::Value::from(1),
                libsql::Value::from(0),
                libsql::Value::from(0),
                libsql::Value::from("Intermediate"),
            ]),
        )
        .await?;
        info!("Inserted default enhanced user preferences with reading passage settings");
    }

    info!("Enhanced database schema with reading passage support initialized successfully");
    Ok(())
}
