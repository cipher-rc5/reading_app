# Directory Structure
```
src/
  app/
    app.rs
    mod.rs
    runtime.rs
  client/
    groq_client.rs
    mod.rs
  config/
    app_config.rs
    environment.rs
    mod.rs
  database/
    repositories/
      article_repository.rs
      mod.rs
      reading_history_repository.rs
      settings_repository.rs
    connection.rs
    mod.rs
    schema.rs
  services/
    article_service.rs
    database_service.rs
    mod.rs
    search_service.rs
    settings_service.rs
  types/
    article.rs
    errors.rs
    mod.rs
    reading_passage.rs
    settings.rs
    time_utils.rs
    validation.rs
  ui/
    components/
      article_viewer.rs
      mod.rs
      sidebar.rs
      status_bar.rs
      text_toolbar.rs
      toolbar.rs
    rendering/
      markdown_interactive.rs
      markdown.rs
      mod.rs
      themes.rs
    windows/
      debug.rs
      definition.rs
      explanation.rs
      mod.rs
      search.rs
      settings.rs
    events.rs
    mod.rs
  utils/
    fonts.rs
    logging.rs
    mod.rs
  lib.rs
  main.rs
Cargo.toml
```

# Files

## File: src/app/app.rs
````rust
// file: src/app/app.rs
// description: Updated app with proper database initialization and window integration

use crate::{
    app::runtime,
    config::AppConfig,
    services::{ArticleService, DatabaseService, SettingsService},
    types::{Article, ContentType, RequestStatus, UISettings},
    ui::{components::*, events::UIEvent, windows::*},
};
use eframe::egui;
use tracing::{error, info};

pub struct App {
    // Services
    article_service: ArticleService,
    database_service: DatabaseService,
    settings_service: SettingsService,

    // UI Components
    sidebar: Sidebar,
    article_viewer: ArticleViewer,
    toolbar: Toolbar,
    status_bar: StatusBar,

    // Windows
    settings_window: SettingsWindow,
    search_window: SearchWindow,
    debug_window: DebugWindow,
    definition_window: DefinitionWindow,
    explanation_window: ExplanationWindow,

    // Application state
    current_status: RequestStatus,
    message_receiver: Option<tokio::sync::mpsc::UnboundedReceiver<UIEvent>>,
    message_sender: Option<tokio::sync::mpsc::UnboundedSender<UIEvent>>,

    // Runtime handle for async operations
    runtime_handle: tokio::runtime::Handle,

    // Enhanced UI state
    ui_settings: UISettings,
    settings_changed: bool,

    // New UI state
    sidebar_collapsed: bool,
    recent_articles: Vec<Article>,
    selected_article: Option<Article>,
    bookmarked_articles: Vec<Article>,
    show_bookmarks: bool,
}

impl App {
    pub fn new(config: AppConfig) -> Self {
        info!("Initializing enhanced application with bibliotheca features");

        // Get the global runtime handle
        let runtime_handle = runtime::get_runtime_handle();

        // Initialize database service synchronously using the runtime
        let database_service = runtime_handle.block_on(async {
            match DatabaseService::new_async(&config).await {
                Ok(service) => {
                    info!("Database service initialized successfully");
                    service
                }
                Err(e) => {
                    error!("Failed to initialize database service: {}", e);
                    DatabaseService::default()
                }
            }
        });

        let article_service = ArticleService::new(&config).unwrap_or_else(|e| {
            error!("Failed to initialize article service: {}", e);
            ArticleService::default()
        });

        let settings_service = SettingsService::new(database_service.clone());

        // Load UI settings
        let ui_settings = settings_service.get_ui_settings();

        // Create async message channel
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

        // Initialize UI components
        let sidebar = Sidebar::new();
        let article_viewer = ArticleViewer::new();
        let toolbar = Toolbar::new();
        let status_bar = StatusBar::new();

        // Initialize windows
        let settings_window = SettingsWindow::new();
        let search_window = SearchWindow::new();
        let debug_window = DebugWindow::new();
        let definition_window = DefinitionWindow::new();
        let explanation_window = ExplanationWindow::new();

        // Pre-load recent articles synchronously
        let recent_articles = runtime_handle.block_on(async {
            match database_service.get_recent_articles(20).await {
                Ok(articles) => {
                    info!("Loaded {} recent articles", articles.len());
                    articles
                }
                Err(e) => {
                    error!("Failed to load recent articles: {}", e);
                    Vec::new()
                }
            }
        });

        Self {
            article_service,
            database_service,
            settings_service,
            sidebar,
            article_viewer,
            toolbar,
            status_bar,
            settings_window,
            search_window,
            debug_window,
            definition_window,
            explanation_window,
            current_status: RequestStatus::Idle,
            message_receiver: Some(rx),
            message_sender: Some(tx),
            runtime_handle,
            ui_settings,
            settings_changed: false,
            sidebar_collapsed: false,
            recent_articles,
            selected_article: None,
            bookmarked_articles: Vec::new(),
            show_bookmarks: false,
        }
    }

    // Add async initialization method
    pub async fn new_async(config: AppConfig) -> crate::types::AppResult<Self> {
        info!("Initializing enhanced application with bibliotheca features");

        // Initialize services asynchronously
        let database_service = DatabaseService::new_async(&config)
            .await
            .unwrap_or_else(|e| {
                error!("Failed to initialize database service: {}", e);
                DatabaseService::default()
            });

        let article_service = ArticleService::new(&config).unwrap_or_else(|e| {
            error!("Failed to initialize article service: {}", e);
            ArticleService::default()
        });

        let settings_service = SettingsService::new(database_service.clone());

        // Load UI settings asynchronously
        let ui_settings = settings_service.get_ui_settings();

        // Get runtime handle
        let runtime_handle = tokio::runtime::Handle::current();
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

        // Initialize UI components
        let sidebar = Sidebar::new();
        let article_viewer = ArticleViewer::new();
        let toolbar = Toolbar::new();
        let status_bar = StatusBar::new();

        // Initialize windows
        let settings_window = SettingsWindow::new();
        let search_window = SearchWindow::new();
        let debug_window = DebugWindow::new();
        let definition_window = DefinitionWindow::new();
        let explanation_window = ExplanationWindow::new();

        let mut app = Self {
            article_service,
            database_service,
            settings_service,
            sidebar,
            article_viewer,
            toolbar,
            status_bar,
            settings_window,
            search_window,
            debug_window,
            definition_window,
            explanation_window,
            current_status: RequestStatus::Idle,
            message_receiver: Some(rx),
            message_sender: Some(tx),
            runtime_handle,
            ui_settings,
            settings_changed: false,
            sidebar_collapsed: false,
            recent_articles: Vec::new(),
            selected_article: None,
            bookmarked_articles: Vec::new(),
            show_bookmarks: false,
        };

        // Load recent articles asynchronously
        app.load_recent_articles().await;

        Ok(app)
    }

    async fn load_recent_articles(&mut self) {
        match self.database_service.get_recent_articles(20).await {
            Ok(articles) => {
                self.recent_articles = articles;
                info!("Loaded {} recent articles", self.recent_articles.len());
            }
            Err(e) => {
                error!("Failed to load recent articles: {}", e);
            }
        }
    }

    fn handle_ui_event(&mut self, event: UIEvent, ctx: &egui::Context) {
        match event {
            UIEvent::GenerateArticle {
                subject,
                custom_topic,
            } => {
                self.start_article_generation(subject, custom_topic);
            }
            UIEvent::OpenSettings => {
                self.settings_window.show();
            }
            UIEvent::OpenSearch => {
                self.search_window.show();
            }
            UIEvent::OpenDebug => {
                self.debug_window.show();
            }
            UIEvent::ToggleBookmarks => {
                self.show_bookmarks = !self.show_bookmarks;
            }
            UIEvent::ArticleGenerated(article) => {
                let content_type = ContentType::Article {
                    subject: article.subject.clone(),
                    content: article.clone(),
                };
                self.current_status = RequestStatus::Success(content_type);
                self.selected_article = Some(article.clone());
                // Add to recent articles at the beginning
                self.recent_articles.insert(0, article);
                // Keep only the last 20 articles
                if self.recent_articles.len() > 20 {
                    self.recent_articles.truncate(20);
                }
            }
            UIEvent::Error(error) => {
                self.current_status = RequestStatus::Error(error);
            }
            UIEvent::SettingsChanged => {
                // Reload UI settings when they change
                self.ui_settings = self.settings_service.get_ui_settings();
                self.settings_changed = true;
            }
            UIEvent::LoadArticle(article) => {
                self.selected_article = Some(article.clone());
                let content_type = ContentType::Article {
                    subject: article.subject.clone(),
                    content: article,
                };
                self.current_status = RequestStatus::Success(content_type);
            }
            UIEvent::DeleteArticle(title) => {
                // Remove from recent articles
                self.recent_articles.retain(|a| a.title != title);
                // Remove from bookmarks
                self.bookmarked_articles.retain(|a| a.title != title);
                // If the deleted article was selected, clear selection
                if let Some(ref selected) = self.selected_article {
                    if selected.title == title {
                        self.selected_article = None;
                        self.current_status = RequestStatus::Idle;
                    }
                }
            }
            UIEvent::SearchQuery(query) => {
                let database_service = self.database_service.clone();
                let tx = self.message_sender.as_ref().unwrap().clone();

                self.runtime_handle.spawn(async move {
                    match database_service.search_articles(&query).await {
                        Ok(articles) => {
                            // Send results back through message system
                            let _ = tx.send(UIEvent::SearchResults(articles));
                        }
                        Err(e) => {
                            let _ = tx.send(UIEvent::Error(e));
                        }
                    }
                });
            }
            UIEvent::SearchResults(articles) => {
                self.recent_articles = articles;
                if !self.recent_articles.is_empty() {
                    self.selected_article = Some(self.recent_articles[0].clone());
                    let article = self.recent_articles[0].clone();
                    let content_type = ContentType::Article {
                        subject: article.subject.clone(),
                        content: article,
                    };
                    self.current_status = RequestStatus::Success(content_type);
                }
            }
            UIEvent::ToggleSidebar => {
                self.sidebar_collapsed = !self.sidebar_collapsed;
            }
            UIEvent::CopyArticle(article) => {
                // Copy article content as markdown to clipboard
                let markdown = format!("# {}\n\n{}", article.title, article.content);
                ctx.copy_text(markdown);
            }
            UIEvent::DownloadArticle(article) => {
                // Implement file download
                let markdown = format!("# {}\n\n{}", article.title, article.content);
                if let Some(mut download_dir) = dirs::download_dir() {
                    download_dir.push(format!("{}.md", article.title.replace(" ", "_")));
                    match std::fs::write(&download_dir, markdown) {
                        Ok(_) => {
                            info!("Article saved to: {:?}", download_dir);
                        }
                        Err(e) => {
                            error!("Failed to save article: {}", e);
                            self.current_status =
                                RequestStatus::Error(crate::types::AppError::Io(e.to_string()));
                        }
                    }
                } else {
                    error!("Could not find download directory");
                    self.current_status = RequestStatus::Error(crate::types::AppError::Io(
                        "Could not find download directory".to_string(),
                    ));
                }
            }
            UIEvent::BookmarkArticle(article) => {
                let database_service = self.database_service.clone();
                let tx = self.message_sender.as_ref().unwrap().clone();
                let article_clone = article.clone();

                self.runtime_handle.spawn(async move {
                    match database_service.save_article(&article_clone).await {
                        Ok(_) => {
                            info!("Article saved to database: {}", article_clone.title);
                        }
                        Err(e) => {
                            error!("Failed to save article to database: {}", e);
                            let _ = tx.send(UIEvent::Error(e));
                        }
                    }
                });

                // Update local state immediately for responsive UI
                if !self
                    .bookmarked_articles
                    .iter()
                    .any(|a| a.title == article.title)
                {
                    info!("Article bookmarked: {}", article.title);
                    self.bookmarked_articles.push(article);
                }
            }
            UIEvent::UnbookmarkArticle(article) => {
                // Remove from bookmarks
                self.bookmarked_articles
                    .retain(|a| a.title != article.title);
                info!("Article unbookmarked: {}", article.title);
            }
            UIEvent::LookupDefinition(word) => {
                info!("Word lookup requested: {}", word);
                self.definition_window.lookup_word(word);
            }
            UIEvent::ExplainText { text, context } => {
                info!("Text explanation requested: {}", text);
                self.explanation_window.explain_text(text, context);
            }
            // Handle all other events with todo!() for now
            _ => {
                // All other events are handled with placeholder implementations
            }
        }
    }

    fn start_article_generation(
        &mut self,
        subject: crate::types::ArticleSubject,
        custom_topic: Option<String>,
    ) {
        self.current_status = RequestStatus::Loading;

        if let Some(tx) = &self.message_sender {
            let tx_clone = tx.clone();
            let article_service = self.article_service.clone();

            // Spawn on the runtime instead of std::thread
            self.runtime_handle.spawn(async move {
                match article_service
                    .generate_article_async(subject, custom_topic)
                    .await
                {
                    Ok(article) => {
                        let _ = tx_clone.send(UIEvent::ArticleGenerated(article));
                    }
                    Err(error) => {
                        let _ = tx_clone.send(UIEvent::Error(error));
                    }
                }
            });
        }
    }

    fn check_messages(&mut self, ctx: &egui::Context) {
        if let Some(ref mut receiver) = self.message_receiver {
            let mut events = Vec::new();

            // Use try_recv to avoid blocking
            while let Ok(event) = receiver.try_recv() {
                events.push(event);
            }

            for event in events {
                self.handle_ui_event(event, ctx);
            }
        }
    }

    fn apply_ui_settings(&self, ctx: &egui::Context) {
        // Apply enhanced theme from bibliotheca
        crate::ui::rendering::themes::apply_theme(ctx, &self.ui_settings);

        // Apply additional styling if settings changed
        if self.settings_changed {
            ctx.request_repaint();
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.check_messages(ctx);
        self.apply_ui_settings(ctx);

        // Draw toolbar with menu bar (includes settings in File menu)
        let toolbar_events = self.toolbar.draw(ctx);
        for event in toolbar_events {
            self.handle_ui_event(event, ctx);
        }

        // Draw collapsible sidebar
        if !self.sidebar_collapsed {
            let sidebar_width = self.ui_settings.sidebar_width;

            let mut sidebar_events = Vec::new();
            egui::SidePanel::left("sidebar")
                .min_width(sidebar_width)
                .max_width(sidebar_width + 50.0)
                .show(ctx, |ui| {
                    sidebar_events = self.sidebar.draw_with_articles(
                        ui,
                        &self.recent_articles,
                        &self.bookmarked_articles,
                    );
                });

            for event in sidebar_events {
                self.handle_ui_event(event, ctx);
            }
        }

        let mut article_events = Vec::new();
        // Main content panel with padding
        egui::CentralPanel::default().show(ctx, |ui| {
            // Add left padding when sidebar is visible/collapsed
            let left_padding = 20.0;

            ui.add_space(10.0); // Top padding

            egui::Frame::new()
                .inner_margin(egui::Margin {
                    left: left_padding as i8,
                    right: 20,
                    top: 0,
                    bottom: 0,
                })
                .show(ui, |ui| {
                    // Collapse/expand sidebar button
                    ui.horizontal(|ui| {
                        let button_text = if self.sidebar_collapsed {
                            "Show Sidebar"
                        } else {
                            "Hide Sidebar"
                        };
                        if ui.button(button_text).clicked() {
                            self.handle_ui_event(UIEvent::ToggleSidebar, ctx);
                        }
                    });

                    ui.add_space(10.0);

                    // Article content with action buttons
                    if let Some(article) = &self.selected_article {
                        article_events = self.draw_article_with_actions(ui, article);
                    } else {
                        self.article_viewer.draw_with_settings(
                            ui,
                            &self.current_status,
                            &self.ui_settings,
                        );
                    }
                });
        });

        for event in article_events {
            self.handle_ui_event(event, ctx);
        }

        // Status bar
        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            self.status_bar.draw(ui, &self.current_status);
        });

        // Draw windows
        let settings_events = self.settings_window.draw(ctx, &mut self.settings_service);
        for event in settings_events {
            self.handle_ui_event(event, ctx);
        }

        self.search_window.draw(ctx, &self.database_service);
        self.debug_window.draw(ctx, &self.database_service);
        self.definition_window.draw(ctx);
        self.explanation_window.draw(ctx);

        self.settings_changed = false;
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        if let Ok(serialized) = serde_json::to_string(&self.ui_settings) {
            storage.set_string("ui_settings", serialized);
        }
        storage.set_string("sidebar_collapsed", self.sidebar_collapsed.to_string());
    }
}

impl App {
    fn draw_article_with_actions(&self, ui: &mut egui::Ui, article: &Article) -> Vec<UIEvent> {
        let mut events = Vec::new();

        // Article header
        ui.horizontal(|ui| {
            let title_text = self
                .ui_settings
                .apply_header_style(egui::RichText::new(&article.title));
            ui.heading(title_text);
        });

        ui.add_space(self.ui_settings.paragraph_spacing);

        // Action buttons below and to the right of title
        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
            let is_bookmarked = self
                .bookmarked_articles
                .iter()
                .any(|a| a.title == article.title);

            let bookmark_text = if is_bookmarked {
                "Unbookmark"
            } else {
                "Bookmark"
            };

            if ui.button(bookmark_text).clicked() {
                if is_bookmarked {
                    events.push(UIEvent::UnbookmarkArticle(article.clone()));
                } else {
                    events.push(UIEvent::BookmarkArticle(article.clone()));
                }
            }

            if ui.button("Download").clicked() {
                events.push(UIEvent::DownloadArticle(article.clone()));
            }

            if ui.button("Copy").clicked() {
                events.push(UIEvent::CopyArticle(article.clone()));
            }
        });

        ui.add_space(self.ui_settings.paragraph_spacing);

        // Article metadata
        ui.horizontal(|ui| {
            let meta_style = |text: String| {
                self.ui_settings
                    .apply_text_body_style(egui::RichText::new(text).weak())
            };

            ui.label(meta_style(format!(
                "Subject: {}",
                article.subject.display_name()
            )));
            ui.separator();

            let formatted_time = article
                .generated_at
                .format("%Y-%m-%d %H:%M UTC")
                .to_string();
            ui.label(meta_style(format!("Generated: {}", formatted_time)));

            if self.ui_settings.show_article_stats {
                ui.separator();
                ui.label(meta_style(format!("Words: {}", article.word_count)));
                ui.separator();
                ui.label(meta_style(format!(
                    "Read time: {}m",
                    article.estimated_read_time
                )));
            }
        });

        ui.separator();
        ui.add_space(self.ui_settings.paragraph_spacing * 2.0);

        // Article content - USE THE REGULAR MARKDOWN RENDERER, NOT INTERACTIVE
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                // Set proper layout to prevent justification
                ui.set_max_width(ui.available_width() - 20.0);

                // Use the regular markdown renderer for clean display
                let mut markdown_renderer = crate::ui::rendering::markdown::MarkdownRenderer::new();
                markdown_renderer.render_with_settings(ui, &article.content, &self.ui_settings);

                // Bottom padding
                ui.add_space(40.0);
            });

        events
    }
}
````

## File: src/app/mod.rs
````rust
// file: src/app/mod.rs

pub mod app;
pub mod runtime;

pub use app::App;
pub use runtime::RuntimeManager;
````

## File: src/app/runtime.rs
````rust
// file: src/app/runtime.rs
// description: Global runtime management for async operations

use std::sync::OnceLock;
use tokio::runtime::{Handle, Runtime};
use tracing::info;

static RUNTIME: OnceLock<Runtime> = OnceLock::new();

/// Get or create the global tokio runtime
pub fn get_or_create_runtime() -> &'static Runtime {
    RUNTIME.get_or_init(|| {
        info!("Creating global tokio runtime");
        Runtime::new().expect("Failed to create tokio runtime")
    })
}

/// Get the handle to the global runtime
pub fn get_runtime_handle() -> Handle {
    get_or_create_runtime().handle().clone()
}

/// Initialize the global runtime (called early in main)
pub fn init_runtime() {
    let _ = get_or_create_runtime();
    info!("Global runtime initialized");
}

// Legacy RuntimeManager for backward compatibility
pub struct RuntimeManager {
    runtime: Runtime,
}

impl RuntimeManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        info!("Creating async runtime");

        let runtime =
            Runtime::new().map_err(|e| format!("Failed to create Tokio runtime: {}", e))?;

        Ok(Self { runtime })
    }

    pub fn handle(&self) -> Handle {
        self.runtime.handle().clone()
    }

    pub fn block_on<F: std::future::Future>(&self, future: F) -> F::Output {
        self.runtime.block_on(future)
    }
}

impl Default for RuntimeManager {
    fn default() -> Self {
        Self::new().expect("Failed to create default runtime")
    }
}
````

## File: src/client/groq_client.rs
````rust
// file: src/client/groq_client.rs
// description: Updated Groq API client with content cleaning for proper article display

use crate::{
    config::AppConfig,
    types::{
        reading_passage::{
            DifficultyLevel, ReadingPassage, ReadingPassageResponse, SubjectCategory,
        },
        AppError, AppResult, Article, ArticleSubject, InputValidator,
    },
};
use serde_json::{json, Value};
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
        while normalized_lines
            .last()
            .map_or(false, |line| line.is_empty())
        {
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
        if let Some(topic) = custom_topic {
            if !topic.trim().is_empty() {
                prompt_parts.push(format!("Specific topic: {}", topic.trim()));
            }
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
````

## File: src/client/mod.rs
````rust
// file: src/client/mod.rs

pub mod groq_client;
pub use groq_client::GroqClient;
````

## File: src/config/app_config.rs
````rust
// file: src/config/app_config.rs
// description: Application configuration management

use crate::types::AppResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub groq_api_key: String,
    pub groq_base_url: String,
    pub database_path: String,
    pub log_level: String,
}

impl AppConfig {
    pub fn load() -> AppResult<Self> {
        super::environment::load_env_file()?;

        let groq_api_key = std::env::var("GROQ_API_KEY").map_err(|_| {
            crate::types::AppError::Config("GROQ_API_KEY environment variable not set".to_string())
        })?;

        let groq_base_url = std::env::var("GROQ_BASE_URL")
            .unwrap_or_else(|_| "https://api.groq.com/openai/v1".to_string());

        let database_path =
            std::env::var("DATABASE_PATH").unwrap_or_else(|_| "reading_app.db".to_string());

        let log_level =
            std::env::var("RUST_LOG").unwrap_or_else(|_| "reading_app=info".to_string());

        Ok(Self {
            groq_api_key,
            groq_base_url,
            database_path,
            log_level,
        })
    }
}
````

## File: src/config/environment.rs
````rust
// file: src/config/environment.rs
// description: Environment variable handling

use crate::types::{AppError, AppResult};
use std::path::Path;
use tracing::{info, warn};

pub fn load_env_file() -> AppResult<()> {
    let env_file_exists = Path::new(".env").exists();

    if env_file_exists {
        info!("Loading environment variables from .env file");
        dotenvy::dotenv()
            .map_err(|e| AppError::Config(format!("Failed to load .env file: {}", e)))?;
    } else {
        warn!("No .env file found, using system environment variables");
    }

    Ok(())
}
````

## File: src/config/mod.rs
````rust
// file: src/config/mod.rs
pub mod app_config;
pub mod environment;

pub use app_config::AppConfig;
````

## File: src/database/repositories/article_repository.rs
````rust
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
````

## File: src/database/repositories/mod.rs
````rust
// file: src/database/repositories/mod.rs
pub mod article_repository;
pub mod reading_history_repository;
pub mod settings_repository;

pub use article_repository::ArticleRepository;
pub use reading_history_repository::ReadingHistoryRepository;
pub use settings_repository::SettingsRepository;
````

## File: src/database/repositories/reading_history_repository.rs
````rust
// file: src/database/repositories/reading_history_repository.rs
// description: Fixed reading history repository with proper column handling

use crate::{database::connection::DatabaseConnection, types::AppResult};
use libsql::Value;
use std::sync::Arc;
use uuid::Uuid;

pub struct ReadingHistoryRepository {
    conn: Arc<DatabaseConnection>,
}

impl ReadingHistoryRepository {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }

    pub async fn add_session(&self, article_title: &str, duration: i32) -> AppResult<()> {
        // First find the article ID
        let mut rows = self
            .conn
            .query(
                "SELECT id FROM articles WHERE title = ? LIMIT 1",
                libsql::params::Params::Positional(vec![Value::from(article_title)]),
            )
            .await?;

        if let Some(row) = rows.next().await.map_err(|e| {
            crate::types::DatabaseError::Query(format!("Failed to fetch article row: {}", e))
        })? {
            let article_id: String = row.get(0).map_err(|e| {
                crate::types::DatabaseError::Query(format!("Failed to get article_id: {}", e))
            })?;

            let session_id = Uuid::new_v4().to_string();
            let opened_at = chrono::Utc::now().to_rfc3339();

            // Use the new schema with proper column names
            self.conn
                .execute(
                    r#"
                    INSERT INTO reading_history (id, article_id, content_type, opened_at, reading_time_seconds)
                    VALUES (?, ?, ?, ?, ?)
                    "#,
                    libsql::params::Params::Positional(vec![
                        Value::from(session_id),
                        Value::from(article_id),
                        Value::from("article"), // content_type
                        Value::from(opened_at),
                        Value::from(duration as i64),
                    ]),
                )
                .await?;
        }

        Ok(())
    }

    pub async fn add_passage_session(&self, passage_title: &str, duration: i32) -> AppResult<()> {
        // Find the passage ID
        let mut rows = self
            .conn
            .query(
                "SELECT id FROM reading_passages WHERE title = ? LIMIT 1",
                libsql::params::Params::Positional(vec![Value::from(passage_title)]),
            )
            .await?;

        if let Some(row) = rows.next().await.map_err(|e| {
            crate::types::DatabaseError::Query(format!("Failed to fetch passage row: {}", e))
        })? {
            let passage_id: String = row.get(0).map_err(|e| {
                crate::types::DatabaseError::Query(format!("Failed to get passage_id: {}", e))
            })?;

            let session_id = Uuid::new_v4().to_string();
            let opened_at = chrono::Utc::now().to_rfc3339();

            self.conn
                .execute(
                    r#"
                    INSERT INTO reading_history (id, passage_id, content_type, opened_at, reading_time_seconds)
                    VALUES (?, ?, ?, ?, ?)
                    "#,
                    libsql::params::Params::Positional(vec![
                        Value::from(session_id),
                        Value::from(passage_id),
                        Value::from("reading_passage"), // content_type
                        Value::from(opened_at),
                        Value::from(duration as i64),
                    ]),
                )
                .await?;
        }

        Ok(())
    }

    pub async fn get_stats(&self) -> AppResult<(i32, i32, i64)> {
        let total_articles: i32 = {
            let mut rows = self
                .conn
                .query(
                    "SELECT COUNT(*) as count FROM articles",
                    libsql::params::Params::Positional(vec![]),
                )
                .await?;

            if let Some(row) = rows.next().await.map_err(|e| {
                crate::types::DatabaseError::Query(format!("Failed to fetch articles count: {}", e))
            })? {
                row.get::<i64>(0).unwrap_or(0) as i32
            } else {
                0
            }
        };

        let total_sessions: i32 = {
            let mut rows = self
                .conn
                .query(
                    "SELECT COUNT(*) as count FROM reading_history",
                    libsql::params::Params::Positional(vec![]),
                )
                .await?;

            if let Some(row) = rows.next().await.map_err(|e| {
                crate::types::DatabaseError::Query(format!("Failed to fetch sessions count: {}", e))
            })? {
                row.get::<i64>(0).unwrap_or(0) as i32
            } else {
                0
            }
        };

        let total_time: i64 = {
            let mut rows = self
                .conn
                .query(
                    "SELECT COALESCE(SUM(reading_time_seconds), 0) as total FROM reading_history",
                    libsql::params::Params::Positional(vec![]),
                )
                .await?;

            if let Some(row) = rows.next().await.map_err(|e| {
                crate::types::DatabaseError::Query(format!("Failed to fetch reading time: {}", e))
            })? {
                row.get::<i64>(0).unwrap_or(0)
            } else {
                0
            }
        };

        Ok((total_articles, total_sessions, total_time))
    }

    pub async fn get_recent_sessions(&self, limit: usize) -> AppResult<Vec<ReadingSession>> {
        let mut rows = self
            .conn
            .query(
                r#"
                SELECT rh.id, rh.content_type, rh.opened_at, rh.reading_time_seconds,
                       COALESCE(a.title, rp.title) as title
                FROM reading_history rh
                LEFT JOIN articles a ON rh.article_id = a.id AND rh.content_type = 'article'
                LEFT JOIN reading_passages rp ON rh.passage_id = rp.id AND rh.content_type = 'reading_passage'
                ORDER BY rh.opened_at DESC
                LIMIT ?
                "#,
                libsql::params::Params::Positional(vec![Value::from(limit as i64)]),
            )
            .await?;

        let mut sessions = Vec::new();
        while let Some(row) = rows.next().await.map_err(|e| {
            crate::types::DatabaseError::Query(format!("Failed to fetch session row: {}", e))
        })? {
            let session = ReadingSession {
                id: row.get(0).unwrap_or_default(),
                content_type: row.get(1).unwrap_or_default(),
                title: row.get(4).unwrap_or_default(),
                opened_at: row.get(2).unwrap_or_default(),
                reading_time_seconds: row.get::<i64>(3).unwrap_or(0) as i32,
            };
            sessions.push(session);
        }

        Ok(sessions)
    }
}

#[derive(Debug, Clone)]
pub struct ReadingSession {
    pub id: String,
    pub content_type: String,
    pub title: String,
    pub opened_at: String,
    pub reading_time_seconds: i32,
}
````

## File: src/database/repositories/settings_repository.rs
````rust
// file: src/database/repositories/settings_repository.rs
// description: Enhanced settings data access layer with graceful column handling

use crate::{
    database::connection::DatabaseConnection,
    types::{AppResult, UISettings},
};
use libsql::Value;
use std::sync::Arc;
use tracing::{error, warn};

pub struct SettingsRepository {
    conn: Arc<DatabaseConnection>,
}

impl SettingsRepository {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }

    pub async fn get_ui_settings(&self) -> AppResult<UISettings> {
        // First try the full query with all columns
        match self.try_get_full_settings().await {
            Ok(settings) => Ok(settings),
            Err(_) => {
                warn!("Failed to get full settings, trying basic settings");
                // Fall back to basic settings and provide defaults for missing columns
                self.get_basic_settings().await
            }
        }
    }

    async fn try_get_full_settings(&self) -> AppResult<UISettings> {
        let mut rows = self
            .conn
            .query(
                r#"
                    SELECT font_size, zoom_level, background_color, text_color, font_family,
                           theme_mode, show_article_stats, sidebar_width, text_body_font_size,
                           header_font_size, text_body_font, header_font, line_height,
                           paragraph_spacing, header_color, link_color, accent_color, corner_style
                    FROM user_preferences
                    ORDER BY id DESC
                    LIMIT 1
                    "#,
                libsql::params::Params::Positional(vec![]),
            )
            .await?;

        if let Some(row) = rows.next().await.map_err(|e| {
            crate::types::DatabaseError::Query(format!("Failed to fetch settings row: {}", e))
        })? {
            let settings = UISettings {
                // Basic settings
                font_size: row.get::<f64>(0).unwrap_or(14.0) as f32,
                zoom_level: row.get::<f64>(1).unwrap_or(1.0) as f32,
                background_color: row
                    .get::<String>(2)
                    .unwrap_or_else(|_| "#2b2b2b".to_string()),
                text_color: row
                    .get::<String>(3)
                    .unwrap_or_else(|_| "#ffffff".to_string()),
                font_family: row
                    .get::<String>(4)
                    .unwrap_or_else(|_| "default".to_string()),
                theme_mode: row.get::<String>(5).unwrap_or_else(|_| "dark".to_string()),
                show_article_stats: row.get::<i64>(6).unwrap_or(1) != 0,
                sidebar_width: row.get::<f64>(7).unwrap_or(300.0) as f32,

                // Enhanced settings from bibliotheca
                text_body_font_size: row.get::<f64>(8).unwrap_or(14.0) as f32,
                header_font_size: row.get::<f64>(9).unwrap_or(20.0) as f32,
                text_body_font: row
                    .get::<String>(10)
                    .unwrap_or_else(|_| "default".to_string()),
                header_font: row
                    .get::<String>(11)
                    .unwrap_or_else(|_| "default".to_string()),
                line_height: row.get::<f64>(12).unwrap_or(1.5) as f32,
                paragraph_spacing: row.get::<f64>(13).unwrap_or(8.0) as f32,
                header_color: row
                    .get::<String>(14)
                    .unwrap_or_else(|_| "#ffffff".to_string()),
                link_color: row
                    .get::<String>(15)
                    .unwrap_or_else(|_| "#4a9eff".to_string()),
                accent_color: row
                    .get::<String>(16)
                    .unwrap_or_else(|_| "#ff6b6b".to_string()),
                corner_style: row
                    .get::<String>(17)
                    .unwrap_or_else(|_| "rounded".to_string()),
            };

            settings.validate()?;
            Ok(settings)
        } else {
            // Insert default settings if none exist
            self.insert_default_settings().await?;
            Ok(UISettings::default())
        }
    }

    async fn get_basic_settings(&self) -> AppResult<UISettings> {
        // Try to get basic settings without corner_style column
        let mut rows = self
            .conn
            .query(
                r#"
                    SELECT font_size, zoom_level, background_color, text_color, font_family,
                           theme_mode, show_article_stats, sidebar_width, text_body_font_size,
                           header_font_size, text_body_font, header_font, line_height,
                           paragraph_spacing, header_color, link_color, accent_color
                    FROM user_preferences
                    ORDER BY id DESC
                    LIMIT 1
                    "#,
                libsql::params::Params::Positional(vec![]),
            )
            .await?;

        if let Some(row) = rows.next().await.map_err(|e| {
            crate::types::DatabaseError::Query(format!("Failed to fetch basic settings row: {}", e))
        })? {
            let settings = UISettings {
                // Basic settings
                font_size: row.get::<f64>(0).unwrap_or(14.0) as f32,
                zoom_level: row.get::<f64>(1).unwrap_or(1.0) as f32,
                background_color: row
                    .get::<String>(2)
                    .unwrap_or_else(|_| "#2b2b2b".to_string()),
                text_color: row
                    .get::<String>(3)
                    .unwrap_or_else(|_| "#ffffff".to_string()),
                font_family: row
                    .get::<String>(4)
                    .unwrap_or_else(|_| "default".to_string()),
                theme_mode: row.get::<String>(5).unwrap_or_else(|_| "dark".to_string()),
                show_article_stats: row.get::<i64>(6).unwrap_or(1) != 0,
                sidebar_width: row.get::<f64>(7).unwrap_or(300.0) as f32,

                // Enhanced settings from bibliotheca
                text_body_font_size: row.get::<f64>(8).unwrap_or(14.0) as f32,
                header_font_size: row.get::<f64>(9).unwrap_or(20.0) as f32,
                text_body_font: row
                    .get::<String>(10)
                    .unwrap_or_else(|_| "default".to_string()),
                header_font: row
                    .get::<String>(11)
                    .unwrap_or_else(|_| "default".to_string()),
                line_height: row.get::<f64>(12).unwrap_or(1.5) as f32,
                paragraph_spacing: row.get::<f64>(13).unwrap_or(8.0) as f32,
                header_color: row
                    .get::<String>(14)
                    .unwrap_or_else(|_| "#ffffff".to_string()),
                link_color: row
                    .get::<String>(15)
                    .unwrap_or_else(|_| "#4a9eff".to_string()),
                accent_color: row
                    .get::<String>(16)
                    .unwrap_or_else(|_| "#ff6b6b".to_string()),
                // Use default for missing corner_style
                corner_style: "rounded".to_string(),
            };

            // Try to update the database with the missing column
            match self.add_missing_corner_style_column().await {
                Ok(_) => warn!("Added missing corner_style column"),
                Err(e) => error!("Failed to add corner_style column: {}", e),
            }

            settings.validate()?;
            Ok(settings)
        } else {
            // Insert default settings if none exist
            self.insert_default_settings().await?;
            Ok(UISettings::default())
        }
    }

    async fn add_missing_corner_style_column(&self) -> AppResult<()> {
        self.conn
            .execute(
                "ALTER TABLE user_preferences ADD COLUMN corner_style TEXT DEFAULT 'rounded'",
                libsql::params::Params::Positional(vec![]),
            )
            .await?;
        Ok(())
    }

    pub async fn save_ui_settings(&self, settings: &UISettings) -> AppResult<()> {
        settings.validate()?;

        // Check if settings exist
        let count: i64 = {
            let mut rows = self
                .conn
                .query(
                    "SELECT COUNT(*) as count FROM user_preferences",
                    libsql::params::Params::Positional(vec![]),
                )
                .await?;

            if let Some(row) = rows.next().await.map_err(|e| {
                crate::types::DatabaseError::Query(format!(
                    "Failed to get preferences count: {}",
                    e
                ))
            })? {
                row.get::<i64>(0).unwrap_or(0)
            } else {
                0
            }
        };

        if count == 0 {
            self.insert_settings(settings).await
        } else {
            self.update_settings(settings).await
        }
    }

    async fn insert_default_settings(&self) -> AppResult<()> {
        let default_settings = UISettings::default();
        self.insert_settings(&default_settings).await
    }

    async fn insert_settings(&self, settings: &UISettings) -> AppResult<()> {
        self.conn
            .execute(
                r#"
                INSERT INTO user_preferences
                (font_size, zoom_level, background_color, text_color, font_family, theme_mode,
                 show_article_stats, sidebar_width, text_body_font_size, header_font_size,
                 text_body_font, header_font, line_height, paragraph_spacing, header_color,
                 link_color, accent_color, corner_style)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#,
                libsql::params::Params::Positional(vec![
                    Value::from(settings.font_size as f64),
                    Value::from(settings.zoom_level as f64),
                    Value::from(settings.background_color.clone()),
                    Value::from(settings.text_color.clone()),
                    Value::from(settings.font_family.clone()),
                    Value::from(settings.theme_mode.clone()),
                    Value::from(if settings.show_article_stats { 1 } else { 0 }),
                    Value::from(settings.sidebar_width as f64),
                    Value::from(settings.text_body_font_size as f64),
                    Value::from(settings.header_font_size as f64),
                    Value::from(settings.text_body_font.clone()),
                    Value::from(settings.header_font.clone()),
                    Value::from(settings.line_height as f64),
                    Value::from(settings.paragraph_spacing as f64),
                    Value::from(settings.header_color.clone()),
                    Value::from(settings.link_color.clone()),
                    Value::from(settings.accent_color.clone()),
                    Value::from(settings.corner_style.clone()),
                ]),
            )
            .await?;

        Ok(())
    }

    async fn update_settings(&self, settings: &UISettings) -> AppResult<()> {
        // Try the full update first
        match self.try_full_update(settings).await {
            Ok(_) => Ok(()),
            Err(_) => {
                // Fall back to update without corner_style if column doesn't exist
                warn!("Full update failed, trying basic update");
                self.try_basic_update(settings).await
            }
        }
    }

    async fn try_full_update(&self, settings: &UISettings) -> AppResult<()> {
        self.conn
            .execute(
                r#"
                UPDATE user_preferences SET
                    font_size = ?,
                    zoom_level = ?,
                    background_color = ?,
                    text_color = ?,
                    font_family = ?,
                    theme_mode = ?,
                    show_article_stats = ?,
                    sidebar_width = ?,
                    text_body_font_size = ?,
                    header_font_size = ?,
                    text_body_font = ?,
                    header_font = ?,
                    line_height = ?,
                    paragraph_spacing = ?,
                    header_color = ?,
                    link_color = ?,
                    accent_color = ?,
                    corner_style = ?,
                    updated_at = CURRENT_TIMESTAMP
                WHERE id = (SELECT id FROM user_preferences ORDER BY id DESC LIMIT 1)
                "#,
                libsql::params::Params::Positional(vec![
                    Value::from(settings.font_size as f64),
                    Value::from(settings.zoom_level as f64),
                    Value::from(settings.background_color.clone()),
                    Value::from(settings.text_color.clone()),
                    Value::from(settings.font_family.clone()),
                    Value::from(settings.theme_mode.clone()),
                    Value::from(if settings.show_article_stats { 1 } else { 0 }),
                    Value::from(settings.sidebar_width as f64),
                    Value::from(settings.text_body_font_size as f64),
                    Value::from(settings.header_font_size as f64),
                    Value::from(settings.text_body_font.clone()),
                    Value::from(settings.header_font.clone()),
                    Value::from(settings.line_height as f64),
                    Value::from(settings.paragraph_spacing as f64),
                    Value::from(settings.header_color.clone()),
                    Value::from(settings.link_color.clone()),
                    Value::from(settings.accent_color.clone()),
                    Value::from(settings.corner_style.clone()),
                ]),
            )
            .await?;

        Ok(())
    }

    async fn try_basic_update(&self, settings: &UISettings) -> AppResult<()> {
        self.conn
            .execute(
                r#"
                UPDATE user_preferences SET
                    font_size = ?,
                    zoom_level = ?,
                    background_color = ?,
                    text_color = ?,
                    font_family = ?,
                    theme_mode = ?,
                    show_article_stats = ?,
                    sidebar_width = ?,
                    text_body_font_size = ?,
                    header_font_size = ?,
                    text_body_font = ?,
                    header_font = ?,
                    line_height = ?,
                    paragraph_spacing = ?,
                    header_color = ?,
                    link_color = ?,
                    accent_color = ?,
                    updated_at = CURRENT_TIMESTAMP
                WHERE id = (SELECT id FROM user_preferences ORDER BY id DESC LIMIT 1)
                "#,
                libsql::params::Params::Positional(vec![
                    Value::from(settings.font_size as f64),
                    Value::from(settings.zoom_level as f64),
                    Value::from(settings.background_color.clone()),
                    Value::from(settings.text_color.clone()),
                    Value::from(settings.font_family.clone()),
                    Value::from(settings.theme_mode.clone()),
                    Value::from(if settings.show_article_stats { 1 } else { 0 }),
                    Value::from(settings.sidebar_width as f64),
                    Value::from(settings.text_body_font_size as f64),
                    Value::from(settings.header_font_size as f64),
                    Value::from(settings.text_body_font.clone()),
                    Value::from(settings.header_font.clone()),
                    Value::from(settings.line_height as f64),
                    Value::from(settings.paragraph_spacing as f64),
                    Value::from(settings.header_color.clone()),
                    Value::from(settings.link_color.clone()),
                    Value::from(settings.accent_color.clone()),
                ]),
            )
            .await?;

        // Try to add the missing column after basic update
        let _ = self.add_missing_corner_style_column().await;

        Ok(())
    }
}
````

## File: src/database/connection.rs
````rust
// file: src/database/connection.rs
// description: Database connection management

use crate::types::errors::DatabaseError;
use crate::types::AppResult;
use libsql::{Builder, Connection};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

pub struct DatabaseConnection {
    conn: Arc<RwLock<Connection>>,
}

impl DatabaseConnection {
    pub async fn new(db_path: &str) -> AppResult<Self> {
        info!("Connecting to database: {}", db_path);

        let db = Builder::new_local(db_path)
            .build()
            .await
            .map_err(|e| DatabaseError::Connection(e.to_string()))?;

        let conn = db
            .connect()
            .map_err(|e| DatabaseError::Connection(e.to_string()))?;

        let connection = Self {
            conn: Arc::new(RwLock::new(conn)),
        };

        // Initialize schema
        super::schema::initialize(&connection).await?;

        Ok(connection)
    }

    pub async fn execute(&self, sql: &str, params: libsql::params::Params) -> AppResult<u64> {
        let conn = self.conn.write().await;
        conn.execute(sql, params)
            .await
            .map_err(|e| DatabaseError::Query(e.to_string()).into())
    }

    pub async fn query(
        &self,
        sql: &str,
        params: libsql::params::Params,
    ) -> AppResult<libsql::Rows> {
        let conn = self.conn.read().await;
        conn.query(sql, params)
            .await
            .map_err(|e| DatabaseError::Query(e.to_string()).into())
    }
}
````

## File: src/database/mod.rs
````rust
// file: src/database/mod.rs
pub mod connection;
pub mod repositories;
pub mod schema;

pub use connection::DatabaseConnection;
````

## File: src/database/schema.rs
````rust
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
````

## File: src/services/article_service.rs
````rust
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
````

## File: src/services/database_service.rs
````rust
// file: src/services/database_service.rs
// description: Database operations service with graceful error handling

use crate::{
    config::AppConfig,
    database::{
        connection::DatabaseConnection,
        repositories::{ArticleRepository, ReadingHistoryRepository, SettingsRepository},
    },
    types::{AppResult, Article, UISettings},
};

use std::sync::Arc;
use tracing::{error, warn};

#[derive(Clone, Default)]
pub struct DatabaseService {
    article_repo: Option<Arc<ArticleRepository>>,
    settings_repo: Option<Arc<SettingsRepository>>,
    reading_history_repo: Option<Arc<ReadingHistoryRepository>>,
    is_available: bool,
}

impl DatabaseService {
    pub fn new(_config: &AppConfig) -> AppResult<Self> {
        // For backward compatibility, create a default disabled service
        Ok(Self::default())
    }

    pub async fn new_async(config: &AppConfig) -> AppResult<Self> {
        match DatabaseConnection::new(&config.database_path).await {
            Ok(connection) => {
                let conn = Arc::new(connection);

                let article_repo = Arc::new(ArticleRepository::new(conn.clone()));
                let settings_repo = Arc::new(SettingsRepository::new(conn.clone()));
                let reading_history_repo = Arc::new(ReadingHistoryRepository::new(conn.clone()));

                Ok(Self {
                    article_repo: Some(article_repo),
                    settings_repo: Some(settings_repo),
                    reading_history_repo: Some(reading_history_repo),
                    is_available: true,
                })
            }
            Err(e) => {
                error!("Failed to initialize database connection: {}", e);
                Ok(Self {
                    article_repo: None,
                    settings_repo: None,
                    reading_history_repo: None,
                    is_available: false,
                })
            }
        }
    }

    pub async fn save_article(&self, article: &Article) -> AppResult<String> {
        if !self.is_available {
            return Err(crate::types::AppError::Database(
                crate::types::DatabaseError::Connection("Database not available".to_string()),
            ));
        }

        if let Some(ref repo) = self.article_repo {
            repo.save(article).await
        } else {
            Err(crate::types::AppError::Database(
                crate::types::DatabaseError::Connection(
                    "Article repository not available".to_string(),
                ),
            ))
        }
    }

    pub async fn get_recent_articles(&self, limit: usize) -> AppResult<Vec<Article>> {
        if !self.is_available {
            warn!("Database not available, returning empty articles list");
            return Ok(Vec::new());
        }

        if let Some(ref repo) = self.article_repo {
            repo.get_recent(limit).await
        } else {
            Ok(Vec::new())
        }
    }

    pub async fn search_articles(&self, query: &str) -> AppResult<Vec<Article>> {
        if !self.is_available {
            warn!("Database not available, returning empty search results");
            return Ok(Vec::new());
        }

        if let Some(ref repo) = self.article_repo {
            repo.search(query).await
        } else {
            Ok(Vec::new())
        }
    }

    pub async fn delete_article(&self, title: &str) -> AppResult<bool> {
        if !self.is_available {
            return Err(crate::types::AppError::Database(
                crate::types::DatabaseError::Connection("Database not available".to_string()),
            ));
        }

        if let Some(ref repo) = self.article_repo {
            repo.delete_by_title(title).await
        } else {
            Ok(false)
        }
    }

    pub async fn get_ui_settings(&self) -> AppResult<UISettings> {
        if !self.is_available {
            warn!("Database not available, returning default UI settings");
            return Ok(UISettings::default());
        }

        if let Some(ref repo) = self.settings_repo {
            match repo.get_ui_settings().await {
                Ok(settings) => Ok(settings),
                Err(e) => {
                    error!("Failed to load UI settings from database: {}", e);
                    Ok(UISettings::default())
                }
            }
        } else {
            Ok(UISettings::default())
        }
    }

    pub async fn save_ui_settings(&self, settings: &UISettings) -> AppResult<()> {
        if !self.is_available {
            warn!("Database not available, cannot save UI settings");
            return Ok(()); // Silently ignore save attempts when DB is unavailable
        }

        if let Some(ref repo) = self.settings_repo {
            repo.save_ui_settings(settings).await
        } else {
            warn!("Settings repository not available");
            Ok(())
        }
    }

    pub async fn add_reading_session(&self, article_title: &str, duration: i32) -> AppResult<()> {
        if !self.is_available {
            return Ok(()); // Silently ignore when DB is unavailable
        }

        if let Some(ref repo) = self.reading_history_repo {
            repo.add_session(article_title, duration).await
        } else {
            Ok(())
        }
    }

    pub async fn get_reading_stats(&self) -> AppResult<(i32, i32, i64)> {
        if !self.is_available {
            return Ok((0, 0, 0));
        }

        if let Some(ref repo) = self.reading_history_repo {
            repo.get_stats().await
        } else {
            Ok((0, 0, 0))
        }
    }

    pub fn is_available(&self) -> bool {
        self.is_available
    }
}
````

## File: src/services/mod.rs
````rust
// file: src/services/mod.rs
pub mod article_service;
pub mod database_service;
pub mod search_service;
pub mod settings_service;

pub use article_service::ArticleService;
pub use database_service::DatabaseService;
pub use search_service::SearchService;
pub use settings_service::SettingsService;
````

## File: src/services/search_service.rs
````rust
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
````

## File: src/services/settings_service.rs
````rust
// file: src/services/settings_service.rs
// description: Settings management service

use crate::{
    app::runtime,
    services::DatabaseService,
    types::{AppResult, UISettings},
};
use std::sync::RwLock;
use tracing::{error, info};

pub struct SettingsService {
    database_service: DatabaseService,
    cached_settings: RwLock<Option<UISettings>>,
    runtime_handle: tokio::runtime::Handle,
}

impl SettingsService {
    pub fn new(database_service: DatabaseService) -> Self {
        let runtime_handle = runtime::get_runtime_handle();
        Self {
            database_service,
            cached_settings: RwLock::new(None),
            runtime_handle,
        }
    }

    pub fn get_ui_settings(&self) -> UISettings {
        // Check cache first
        if let Ok(cached) = self.cached_settings.read() {
            if let Some(ref settings) = *cached {
                return settings.clone();
            }
        }

        // Load from database using spawn_blocking for UI thread
        let database_service = self.database_service.clone();
        let result = self
            .runtime_handle
            .block_on(async { database_service.get_ui_settings().await });

        match result {
            Ok(mut settings) => {
                settings.sanitize_font_settings();

                // Update cache
                if let Ok(mut cached) = self.cached_settings.write() {
                    *cached = Some(settings.clone());
                }
                settings
            }
            Err(e) => {
                error!("Failed to load UI settings: {}", e);
                UISettings::default()
            }
        }
    }

    pub fn save_ui_settings(&self, settings: &UISettings) -> AppResult<()> {
        settings.validate()?;

        // Use the runtime handle to execute async operation
        let database_service = self.database_service.clone();
        let settings_clone = settings.clone();

        self.runtime_handle
            .block_on(async { database_service.save_ui_settings(&settings_clone).await })?;

        // Update cache
        if let Ok(mut cached) = self.cached_settings.write() {
            *cached = Some(settings.clone());
        }

        info!("UI settings saved successfully");
        Ok(())
    }

    pub async fn save_ui_settings_async(&self, settings: &UISettings) -> AppResult<()> {
        // Validate before saving
        settings.validate()?;

        // Save to database
        self.database_service.save_ui_settings(settings).await?;

        // Update cache
        if let Ok(mut cached) = self.cached_settings.write() {
            *cached = Some(settings.clone());
        }

        info!("UI settings saved successfully");
        Ok(())
    }

    pub fn apply_theme_preset(&self, theme: ThemePreset) -> AppResult<()> {
        let mut settings = self.get_ui_settings();

        match theme {
            ThemePreset::Dark => settings.apply_dark_theme(),
            ThemePreset::Light => settings.apply_light_theme(),
        }

        self.save_ui_settings(&settings)
    }
}

#[derive(Debug, Clone)]
pub enum ThemePreset {
    Dark,
    Light,
}
````

## File: src/types/article.rs
````rust
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
````

## File: src/types/errors.rs
````rust
// file: src/types/errors.rs
// description: Comprehensive error types

use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Error, Debug, Clone)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] DatabaseError),
    #[error("Network error: {0}")]
    Network(String),
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),
    #[error("IO error: {0}")]
    Io(String),
    #[error("Serialization error: {0}")]
    Serialization(String),
}

#[derive(Error, Debug, Clone)]
pub enum DatabaseError {
    #[error("Connection failed: {0}")]
    Connection(String),
    #[error("Query failed: {0}")]
    Query(String),
    #[error("Transaction failed: {0}")]
    Transaction(String),
    #[error("Schema error: {0}")]
    Schema(String),
}

#[derive(Error, Debug, Clone)]
pub enum ValidationError {
    #[error("Input is empty")]
    EmptyInput,
    #[error("Input too long: {current} characters, max {max}")]
    InputTooLong { current: usize, max: usize },
    #[error("Invalid characters in input")]
    InvalidCharacters,
    #[error("Invalid format: {0}")]
    InvalidFormat(String),
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Serialization(err.to_string())
    }
}
````

## File: src/types/mod.rs
````rust
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
````

## File: src/types/reading_passage.rs
````rust
// file: src/types/reading_passage.rs
// description: Generalized reading passage types that extend existing article system

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
````

## File: src/types/settings.rs
````rust
// file: src/types/settings.rs
// description: Enhanced UI settings with corner style and improved validation

use super::errors::{AppResult, ValidationError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UISettings {
    // Basic settings (existing)
    pub font_size: f32,
    pub zoom_level: f32,
    pub background_color: String,
    pub text_color: String,
    pub font_family: String,
    pub theme_mode: String,
    pub show_article_stats: bool,
    pub sidebar_width: f32,

    // Enhanced font settings (from bibliotheca)
    pub text_body_font_size: f32,
    pub header_font_size: f32,
    pub text_body_font: String,
    pub header_font: String,
    pub line_height: f32,
    pub paragraph_spacing: f32,
    pub header_color: String,
    pub link_color: String,
    pub accent_color: String,

    // New corner style option
    pub corner_style: String, // "rounded" or "square"
}

impl Default for UISettings {
    fn default() -> Self {
        Self {
            font_size: 14.0,
            zoom_level: 1.0,
            background_color: "#2b2b2b".to_string(),
            text_color: "#ffffff".to_string(),
            font_family: "default".to_string(),
            theme_mode: "dark".to_string(),
            show_article_stats: true,
            sidebar_width: 300.0,
            text_body_font_size: 14.0,
            header_font_size: 20.0,
            text_body_font: "default".to_string(),
            header_font: "default".to_string(),
            line_height: 1.5,
            paragraph_spacing: 8.0,
            header_color: "#ffffff".to_string(),
            link_color: "#4a9eff".to_string(),
            accent_color: "#ff6b6b".to_string(),
            corner_style: "rounded".to_string(),
        }
    }
}

impl UISettings {
    pub fn validate(&self) -> AppResult<()> {
        // Enhanced validation from bibliotheca
        if self.font_size < 8.0 || self.font_size > 32.0 {
            return Err(ValidationError::InvalidFormat(
                "Font size must be between 8.0 and 32.0".to_string(),
            )
            .into());
        }

        if self.text_body_font_size < 8.0 || self.text_body_font_size > 32.0 {
            return Err(ValidationError::InvalidFormat(
                "Text body font size must be between 8.0 and 32.0".to_string(),
            )
            .into());
        }

        if self.header_font_size < 12.0 || self.header_font_size > 48.0 {
            return Err(ValidationError::InvalidFormat(
                "Header font size must be between 12.0 and 48.0".to_string(),
            )
            .into());
        }

        if self.zoom_level < 0.5 || self.zoom_level > 3.0 {
            return Err(ValidationError::InvalidFormat(
                "Zoom level must be between 0.5 and 3.0".to_string(),
            )
            .into());
        }

        if self.line_height < 1.0 || self.line_height > 3.0 {
            return Err(ValidationError::InvalidFormat(
                "Line height must be between 1.0 and 3.0".to_string(),
            )
            .into());
        }

        if self.paragraph_spacing < 0.0 || self.paragraph_spacing > 50.0 {
            return Err(ValidationError::InvalidFormat(
                "Paragraph spacing must be between 0.0 and 50.0".to_string(),
            )
            .into());
        }

        if self.sidebar_width < 200.0 || self.sidebar_width > 800.0 {
            return Err(ValidationError::InvalidFormat(
                "Sidebar width must be between 200.0 and 800.0".to_string(),
            )
            .into());
        }

        // Validate corner style
        if !matches!(self.corner_style.as_str(), "rounded" | "square") {
            return Err(ValidationError::InvalidFormat(
                "Corner style must be 'rounded' or 'square'".to_string(),
            )
            .into());
        }

        // Validate color formats
        let colors = [
            (&self.background_color, "Background color"),
            (&self.text_color, "Text color"),
            (&self.header_color, "Header color"),
            (&self.link_color, "Link color"),
            (&self.accent_color, "Accent color"),
        ];

        for (color, name) in &colors {
            if parse_hex_color(color).is_none() {
                return Err(ValidationError::InvalidFormat(format!(
                    "{} must be in #RRGGBB format",
                    name
                ))
                .into());
            }
        }

        Ok(())
    }

    // Color getters
    pub fn get_background_color(&self) -> egui::Color32 {
        parse_hex_color(&self.background_color).unwrap_or(egui::Color32::from_rgb(43, 43, 43))
    }

    pub fn get_text_color(&self) -> egui::Color32 {
        parse_hex_color(&self.text_color).unwrap_or(egui::Color32::WHITE)
    }

    pub fn get_header_color(&self) -> egui::Color32 {
        parse_hex_color(&self.header_color).unwrap_or(egui::Color32::WHITE)
    }

    pub fn get_link_color(&self) -> egui::Color32 {
        parse_hex_color(&self.link_color).unwrap_or(egui::Color32::from_rgb(74, 158, 255))
    }

    pub fn get_accent_color(&self) -> egui::Color32 {
        parse_hex_color(&self.accent_color).unwrap_or(egui::Color32::from_rgb(255, 107, 107))
    }

    // Font size getters with zoom
    pub fn get_font_size(&self) -> f32 {
        (self.font_size * self.zoom_level).clamp(8.0, 48.0)
    }

    pub fn get_text_body_font_size(&self) -> f32 {
        (self.text_body_font_size * self.zoom_level).clamp(8.0, 48.0)
    }

    pub fn get_header_font_size(&self) -> f32 {
        (self.header_font_size * self.zoom_level).clamp(12.0, 64.0)
    }

    // Corner style helpers
    pub fn get_rounding(&self) -> egui::CornerRadius {
        match self.corner_style.as_str() {
            "rounded" => egui::CornerRadius::same(8),
            "square" => egui::CornerRadius::ZERO,
            _ => egui::CornerRadius::same(8), // default to rounded
        }
    }

    pub fn is_rounded(&self) -> bool {
        self.corner_style == "rounded"
    }

    // Font configuration - updated to use the font manager
    pub fn get_available_fonts() -> Vec<(String, String)> {
        // Get fonts from the font manager instead of hardcoded list
        crate::utils::fonts::get_available_fonts()
    }

    pub fn get_font_family(&self, font_name: &str) -> egui::FontFamily {
        // Use the font manager through the helper function
        crate::utils::fonts::get_font_family_for_name(font_name)
    }

    // **EXISTING TEXT STYLING METHODS** - These are retained from the original code
    pub fn apply_text_body_style(&self, mut text: egui::RichText) -> egui::RichText {
        text = text
            .size(self.get_text_body_font_size())
            .color(self.get_text_color())
            .family(self.get_font_family(&self.text_body_font));
        text
    }

    pub fn apply_header_style(&self, mut text: egui::RichText) -> egui::RichText {
        text = text
            .size(self.get_header_font_size())
            .color(self.get_header_color())
            .family(self.get_font_family(&self.header_font))
            .strong();
        text
    }

    pub fn apply_font_style(&self, mut text: egui::RichText) -> egui::RichText {
        text = text.size(self.get_font_size()).color(self.get_text_color());

        match self.font_family.as_str() {
            "terminus_nerd_mono" | "monospace" => text.family(self.get_font_family("monospace")),
            "serif" => text.family(self.get_font_family("serif")),
            "sans-serif" => text.family(self.get_font_family("sans-serif")),
            _ => text.family(self.get_font_family("default")),
        }
    }

    // **THEME PRESETS** - Existing methods from bibliotheca
    pub fn apply_dark_theme(&mut self) {
        self.background_color = "#2b2b2b".to_string();
        self.text_color = "#ffffff".to_string();
        self.header_color = "#ffffff".to_string();
        self.link_color = "#4a9eff".to_string();
        self.accent_color = "#ff6b6b".to_string();
        self.theme_mode = "dark".to_string();
    }

    pub fn apply_light_theme(&mut self) {
        self.background_color = "#f5f5f5".to_string();
        self.text_color = "#333333".to_string();
        self.header_color = "#1a1a1a".to_string();
        self.link_color = "#0066cc".to_string();
        self.accent_color = "#cc4444".to_string();
        self.theme_mode = "light".to_string();
    }

    pub fn apply_sepia_theme(&mut self) {
        self.background_color = "#f4f1e8".to_string();
        self.text_color = "#5c4b37".to_string();
        self.header_color = "#3d2f23".to_string();
        self.link_color = "#8b4513".to_string();
        self.accent_color = "#cd853f".to_string();
        self.theme_mode = "sepia".to_string();
    }

    pub fn apply_high_contrast_theme(&mut self) {
        self.background_color = "#000000".to_string();
        self.text_color = "#ffffff".to_string();
        self.header_color = "#ffffff".to_string();
        self.link_color = "#00ffff".to_string();
        self.accent_color = "#ffff00".to_string();
        self.theme_mode = "high_contrast".to_string();
    }

    // **FONT SIZE PRESETS** - Existing methods
    pub fn apply_small_font_preset(&mut self) {
        self.text_body_font_size = 12.0;
        self.header_font_size = 16.0;
        self.font_size = 12.0;
    }

    pub fn apply_medium_font_preset(&mut self) {
        self.text_body_font_size = 14.0;
        self.header_font_size = 20.0;
        self.font_size = 14.0;
    }

    pub fn apply_large_font_preset(&mut self) {
        self.text_body_font_size = 18.0;
        self.header_font_size = 26.0;
        self.font_size = 18.0;
    }

    pub fn apply_extra_large_font_preset(&mut self) {
        self.text_body_font_size = 22.0;
        self.header_font_size = 32.0;
        self.font_size = 22.0;
    }

    // Clean up legacy font settings that might cause warnings
    pub fn sanitize_font_settings(&mut self) {
        let available_fonts = Self::get_available_fonts();
        let available_font_keys: Vec<String> =
            available_fonts.iter().map(|(k, _)| k.clone()).collect();

        // Reset font settings to "default" if they reference unavailable fonts
        if !available_font_keys.contains(&self.font_family) {
            self.font_family = "default".to_string();
        }
        if !available_font_keys.contains(&self.text_body_font) {
            self.text_body_font = "default".to_string();
        }
        if !available_font_keys.contains(&self.header_font) {
            self.header_font = "default".to_string();
        }
    }
}

fn parse_hex_color(hex: &str) -> Option<egui::Color32> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }

    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;

    Some(egui::Color32::from_rgb(r, g, b))
}
````

## File: src/types/time_utils.rs
````rust
// file: src/types/time_utils.rs
// description: Time and timestamp utilities

use super::errors::{AppResult, ValidationError};
use chrono::{DateTime, TimeZone, Utc};

pub fn unix_to_datetime(unix_timestamp: i64) -> AppResult<DateTime<Utc>> {
    Utc.timestamp_opt(unix_timestamp, 0)
        .single()
        .ok_or_else(|| {
            ValidationError::InvalidFormat(format!("Invalid Unix timestamp: {}", unix_timestamp))
                .into()
        })
}

pub fn datetime_to_unix(datetime: &DateTime<Utc>) -> i64 {
    datetime.timestamp()
}

pub fn current_unix_timestamp() -> i64 {
    Utc::now().timestamp()
}

pub fn relative_time_from_unix(unix_timestamp: i64) -> String {
    match unix_to_datetime(unix_timestamp) {
        Ok(datetime) => {
            let now = Utc::now();
            let duration = now.signed_duration_since(datetime);

            if duration.num_days() > 0 {
                format!("{} days ago", duration.num_days())
            } else if duration.num_hours() > 0 {
                format!("{} hours ago", duration.num_hours())
            } else if duration.num_minutes() > 0 {
                format!("{} minutes ago", duration.num_minutes())
            } else {
                "Just now".to_string()
            }
        }
        Err(_) => "Invalid date".to_string(),
    }
}
````

## File: src/types/validation.rs
````rust
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
````

## File: src/ui/components/sidebar.rs
````rust
// file: src/ui/components/sidebar.rs
// description: Fixed sidebar with proper bookmark toggle functionality

use crate::{
    types::{Article, ArticleSubject},
    ui::events::UIEvent,
};
use egui;

pub struct Sidebar {
    selected_subject: ArticleSubject,
    custom_topic: String,
}

impl Sidebar {
    pub fn new() -> Self {
        Self {
            selected_subject: ArticleSubject::Technology,
            custom_topic: String::new(),
        }
    }
}

impl Default for Sidebar {
    fn default() -> Self {
        Self::new()
    }
}

impl Sidebar {
    pub fn draw(&mut self, ui: &mut egui::Ui) -> Vec<UIEvent> {
        self.draw_with_articles(ui, &[], &[])
    }

    pub fn draw_with_articles(
        &mut self,
        ui: &mut egui::Ui,
        recent_articles: &[Article],
        bookmarked_articles: &[Article],
    ) -> Vec<UIEvent> {
        let mut events = Vec::new();

        // Article Generation Section
        ui.heading("Generate New Article");
        ui.separator();

        // Subject selection
        ui.label("Select Subject:");
        for subject in ArticleSubject::all() {
            let is_selected = self.selected_subject == subject;

            if ui
                .selectable_label(is_selected, subject.display_name())
                .clicked()
            {
                self.selected_subject = subject.clone();
            }

            if is_selected {
                ui.small(subject.description());
                ui.add_space(4.0);
            }
        }

        ui.add_space(8.0);
        ui.label("Custom Topic (Optional):");
        ui.add(
            egui::TextEdit::multiline(&mut self.custom_topic)
                .desired_rows(3)
                .hint_text("Enter a specific topic or leave empty"),
        );

        ui.add_space(8.0);
        if ui.button("Generate Article").clicked() {
            let custom_topic = if self.custom_topic.trim().is_empty() {
                None
            } else {
                Some(self.custom_topic.clone())
            };

            events.push(UIEvent::GenerateArticle {
                subject: self.selected_subject.clone(),
                custom_topic,
            });
        }

        ui.add_space(16.0);
        ui.separator();

        // Recent Articles Section
        ui.heading("Recent Articles");
        ui.separator();

        if recent_articles.is_empty() {
            ui.label("No articles generated yet");
            ui.small("Generate your first article above!");
        } else {
            egui::ScrollArea::vertical()
                .max_height(400.0)
                .show(ui, |ui| {
                    for article in recent_articles {
                        self.draw_article_item(ui, article, bookmarked_articles, &mut events);
                    }
                });
        }

        ui.add_space(16.0);
        ui.separator();

        events
    }

    fn draw_article_item(
        &self,
        ui: &mut egui::Ui,
        article: &Article,
        bookmarked_articles: &[Article],
        events: &mut Vec<UIEvent>,
    ) {
        let is_bookmarked = bookmarked_articles.iter().any(|a| a.title == article.title);

        // Apply accent color highlighting for bookmarked articles
        let frame = if is_bookmarked {
            egui::Frame::group(ui.style())
                .stroke(egui::Stroke::new(2.0, ui.style().visuals.selection.bg_fill))
                .fill(ui.style().visuals.selection.bg_fill.gamma_multiply(0.1))
        } else {
            egui::Frame::group(ui.style())
        };

        frame.show(ui, |ui| {
            ui.set_max_width(ui.available_width());

            // Article title (clickable)
            let title_response = ui.add(
                egui::Label::new(egui::RichText::new(&article.title).strong())
                    .sense(egui::Sense::click())
                    .wrap(),
            );

            if title_response.clicked() {
                events.push(UIEvent::LoadArticle(article.clone()));
            }

            ui.add_space(4.0);

            // Article metadata
            ui.horizontal(|ui| {
                ui.small(article.subject.display_name());
                ui.separator();
                ui.small(format!("{}w", article.word_count));
                if is_bookmarked {
                    ui.separator();
                    ui.small("★ Bookmarked");
                }
            });

            ui.add_space(4.0);

            // Generation timestamp
            ui.small(article.generated_at.format("%m/%d %H:%M").to_string());

            ui.add_space(4.0);

            // Action buttons for each article
            ui.horizontal(|ui| {
                if ui
                    .small_button("Copy")
                    .on_hover_text("Copy as markdown")
                    .clicked()
                {
                    events.push(UIEvent::CopyArticle(article.clone()));
                }

                if ui
                    .small_button("Download")
                    .on_hover_text("Download as file")
                    .clicked()
                {
                    events.push(UIEvent::DownloadArticle(article.clone()));
                }

                // Bookmark toggle button
                let bookmark_text = if is_bookmarked {
                    "Unbookmark"
                } else {
                    "Bookmark"
                };

                if ui
                    .small_button(bookmark_text)
                    .on_hover_text(if is_bookmarked {
                        "Remove bookmark"
                    } else {
                        "Bookmark article"
                    })
                    .clicked()
                {
                    if is_bookmarked {
                        events.push(UIEvent::UnbookmarkArticle(article.clone()));
                    } else {
                        events.push(UIEvent::BookmarkArticle(article.clone()));
                    }
                }

                if ui
                    .small_button("Delete")
                    .on_hover_text("Delete article")
                    .clicked()
                {
                    events.push(UIEvent::DeleteArticle(article.title.clone()));
                }
            });
        });

        ui.add_space(8.0);
    }
}
````

## File: src/ui/components/status_bar.rs
````rust
// file: src/ui/components/status_bar.rs
// description: Status bar component

use crate::types::RequestStatus;
use egui;

pub struct StatusBar;

impl StatusBar {
    pub fn new() -> Self {
        Self
    }
}

impl Default for StatusBar {
    fn default() -> Self {
        Self::new()
    }
}

impl StatusBar {
    pub fn draw(&self, ui: &mut egui::Ui, status: &RequestStatus) {
        ui.horizontal(|ui| {
            let (status_text, color) = match status {
                RequestStatus::Idle => ("Ready", egui::Color32::GREEN),
                RequestStatus::Loading => ("Generating...", egui::Color32::YELLOW),
                RequestStatus::Success(_) => ("Article loaded", egui::Color32::GREEN),
                RequestStatus::Error(_) => ("Error", egui::Color32::RED),
            };

            ui.colored_label(color, status_text);
        });
    }
}
````

## File: src/ui/components/text_toolbar.rs
````rust
// file: src/ui/components/text_toolbar.rs
// description: Enhanced popup toolbar for text interactions with working dictionary and explanation

use crate::ui::events::UIEvent;
use egui;

#[derive(Default)]
pub struct TextToolbar {
    show: bool,
    position: egui::Pos2,
    selected_text: String,
    context: String,
}

impl TextToolbar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn show_at_position(&mut self, position: egui::Pos2, selected_text: String) {
        self.show = true;
        self.position = position;
        self.selected_text = selected_text.clone();
        self.context = selected_text; // For now, use selected text as context
    }

    pub fn hide(&mut self) {
        self.show = false;
        self.selected_text.clear();
        self.context.clear();
    }

    pub fn is_visible(&self) -> bool {
        self.show
    }

    pub fn draw(&mut self, ctx: &egui::Context) -> Vec<UIEvent> {
        let mut events = Vec::new();

        if !self.show {
            return events;
        }

        // Adjust position to stay within window bounds
        let window_rect = ctx.screen_rect();
        let mut adjusted_position = self.position;

        // Ensure toolbar doesn't go off screen
        if adjusted_position.x + 300.0 > window_rect.max.x {
            adjusted_position.x = window_rect.max.x - 300.0;
        }
        if adjusted_position.y - 80.0 < window_rect.min.y {
            adjusted_position.y = self.position.y + 40.0; // Show below instead of above
        }

        egui::Window::new("Text Tools")
            .id(egui::Id::new("text_toolbar"))
            .collapsible(false)
            .resizable(false)
            .title_bar(false)
            .fixed_pos(adjusted_position)
            .frame(egui::Frame::popup(&ctx.style()))
            .show(ctx, |ui| {
                ui.set_min_width(280.0);

                // Show selected text preview
                ui.horizontal(|ui| {
                    ui.label("Selected:");
                    ui.label(
                        egui::RichText::new(format!(
                            "\"{}\"",
                            if self.selected_text.len() > 30 {
                                format!("{}...", &self.selected_text[..27])
                            } else {
                                self.selected_text.clone()
                            }
                        ))
                        .italics()
                        .color(egui::Color32::LIGHT_GRAY),
                    );
                });

                ui.separator();

                // Action buttons
                ui.horizontal(|ui| {
                    if ui
                        .button("📖 Define")
                        .on_hover_text("Look up definition")
                        .clicked()
                    {
                        // Extract the first meaningful word for definition lookup
                        let word = self.extract_word_for_definition();
                        if !word.is_empty() {
                            events.push(UIEvent::LookupDefinition(word));
                        }
                        self.hide();
                    }

                    if ui
                        .button("💬 Explain")
                        .on_hover_text("Explain this text")
                        .clicked()
                    {
                        events.push(UIEvent::ExplainText {
                            text: self.selected_text.clone(),
                            context: self.context.clone(),
                        });
                        self.hide();
                    }

                    if ui
                        .button("📋 Copy")
                        .on_hover_text("Copy to clipboard")
                        .clicked()
                    {
                        ctx.copy_text(self.selected_text.clone());
                        self.hide();
                    }
                });

                ui.horizontal(|ui| {
                    if ui
                        .button("🔍 Search")
                        .on_hover_text("Search for this text")
                        .clicked()
                    {
                        events.push(UIEvent::SearchQuery(self.selected_text.clone()));
                        self.hide();
                    }

                    if ui.button("✕ Cancel").clicked() {
                        self.hide();
                    }
                });
            });

        // Auto-hide if user clicks elsewhere
        if ctx.input(|i| i.pointer.any_click()) {
            // Check if click was outside the toolbar window
            if let Some(pointer_pos) = ctx.pointer_latest_pos() {
                let toolbar_rect =
                    egui::Rect::from_min_size(adjusted_position, egui::Vec2::new(280.0, 80.0));

                if !toolbar_rect.contains(pointer_pos) {
                    self.hide();
                }
            }
        }

        events
    }

    fn extract_word_for_definition(&self) -> String {
        // Extract the first meaningful word from the selected text
        let words: Vec<&str> = self.selected_text.split_whitespace().collect();

        for word in words {
            let clean_word = word
                .trim_matches(|c: char| !c.is_alphabetic())
                .to_lowercase();

            // Only return words that are at least 2 characters and contain only letters
            if clean_word.len() >= 2 && clean_word.chars().all(|c| c.is_alphabetic()) {
                return clean_word;
            }
        }

        // Fallback: return the first word, cleaned
        if let Some(first_word) = self.selected_text.split_whitespace().next() {
            first_word
                .trim_matches(|c: char| !c.is_alphabetic())
                .to_lowercase()
        } else {
            String::new()
        }
    }

    // Helper method to set context separately if needed
    pub fn set_context(&mut self, context: String) {
        self.context = context;
    }

    // Method to check if the toolbar should be shown based on selection criteria
    pub fn should_show_for_text(text: &str) -> bool {
        let trimmed = text.trim();

        // Don't show for very short selections
        if trimmed.len() < 2 {
            return false;
        }

        // Don't show for selections that are just whitespace or punctuation
        if !trimmed.chars().any(|c| c.is_alphanumeric()) {
            return false;
        }

        // Don't show for very long selections (probably accidental)
        if trimmed.len() > 500 {
            return false;
        }

        true
    }
}
````

## File: src/ui/components/toolbar.rs
````rust
// file: src/ui/components/toolbar.rs
// description: Enhanced toolbar with updated menu structure

use crate::ui::events::UIEvent;
use egui;

pub struct Toolbar;

impl Toolbar {
    pub fn new() -> Self {
        Self
    }
}

impl Default for Toolbar {
    fn default() -> Self {
        Self::new()
    }
}

impl Toolbar {
    pub fn draw(&mut self, ctx: &egui::Context) -> Vec<UIEvent> {
        let mut events = Vec::new();

        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Settings").clicked() {
                        events.push(UIEvent::OpenSettings);
                        ui.close();
                    }

                    ui.separator();

                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        ui.close();
                    }
                });

                ui.menu_button("View", |ui| {
                    if ui.button("Toggle Sidebar").clicked() {
                        events.push(UIEvent::ToggleSidebar);
                        ui.close();
                    }

                    ui.separator();

                    if ui.button("Show Bookmarks").clicked() {
                        events.push(UIEvent::ToggleBookmarks);
                        ui.close();
                    }
                });

                ui.menu_button("Tools", |ui| {
                    if ui.button("Search Articles").clicked() {
                        events.push(UIEvent::OpenSearch);
                        ui.close();
                    }

                    ui.separator();

                    if ui.button("Debug & Diagnostics").clicked() {
                        events.push(UIEvent::OpenDebug);
                        ui.close();
                    }
                });

                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        // TODO: Show about dialog
                        ui.close();
                    }
                });
            });
        });

        events
    }
}
````

## File: src/ui/rendering/markdown_interactive.rs
````rust
// file: src/ui/rendering/markdown_interactive.rs
// description: Interactive markdown renderer with text selection support

use crate::types::UISettings;
use crate::ui::components::text_toolbar::TextToolbar;
use crate::ui::events::UIEvent;
use egui;

pub struct InteractiveMarkdownRenderer {
    text_toolbar: TextToolbar,
    selection_start: Option<usize>,
    selection_end: Option<usize>,
}

impl InteractiveMarkdownRenderer {
    pub fn new() -> Self {
        Self {
            text_toolbar: TextToolbar::new(),
            selection_start: None,
            selection_end: None,
        }
    }

    pub fn render_with_settings(
        &mut self,
        ui: &mut egui::Ui,
        content: &str,
        settings: &UISettings,
    ) -> Vec<UIEvent> {
        let mut events = Vec::new();

        // Draw the text toolbar first (it renders as a popup)
        let toolbar_events = self.text_toolbar.draw(ui.ctx());
        events.extend(toolbar_events);

        // Render the content with text selection capabilities
        self.render_selectable_content(ui, content, settings, &mut events);

        events
    }

    fn render_selectable_content(
        &mut self,
        ui: &mut egui::Ui,
        content: &str,
        settings: &UISettings,
        _events: &mut Vec<UIEvent>,
    ) {
        // Split content into paragraphs for better selection handling
        let paragraphs: Vec<&str> = content.split("\n\n").collect();

        for (para_idx, paragraph) in paragraphs.iter().enumerate() {
            if para_idx > 0 {
                ui.add_space(settings.paragraph_spacing);
            }

            self.render_selectable_paragraph(ui, paragraph, settings, _events);
        }
    }

    fn render_selectable_paragraph(
        &mut self,
        ui: &mut egui::Ui,
        paragraph: &str,
        settings: &UISettings,
        _events: &mut Vec<UIEvent>,
    ) {
        // Handle different markdown elements
        if paragraph.starts_with("# ") {
            let heading = paragraph.trim_start_matches("# ").trim();
            self.render_selectable_text(ui, heading, settings, _events, true);
        } else if paragraph.starts_with("## ") {
            let heading = paragraph.trim_start_matches("## ").trim();
            self.render_selectable_text(ui, heading, settings, _events, true);
        } else if paragraph.starts_with("### ") {
            let heading = paragraph.trim_start_matches("### ").trim();
            self.render_selectable_text(ui, heading, settings, _events, true);
        } else if paragraph.starts_with("- ") {
            // Bullet point
            ui.horizontal(|ui| {
                let bullet = settings.apply_text_body_style(egui::RichText::new("• "));
                ui.label(bullet);
                let text = paragraph.trim_start_matches("- ").trim();
                self.render_selectable_text(ui, text, settings, _events, false);
            });
        } else {
            // Regular paragraph
            self.render_selectable_text(ui, paragraph, settings, _events, false);
        }
    }

    fn render_selectable_text(
        &mut self,
        ui: &mut egui::Ui,
        text: &str,
        settings: &UISettings,
        _events: &mut Vec<UIEvent>,
        is_heading: bool,
    ) {
        // Create a selectable label
        let rich_text = if is_heading {
            settings.apply_header_style(egui::RichText::new(text))
        } else {
            settings.apply_text_body_style(egui::RichText::new(text))
        };

        let response = ui.add(
            egui::Label::new(rich_text)
                .wrap()
                .sense(egui::Sense::click_and_drag()),
        );

        // Handle text selection
        if response.drag_started() {
            self.selection_start = Some(0); // Simplified - you'd want to calculate actual position
        }

        if response.dragged() {
            // Update selection end
            self.selection_end = Some(text.len()); // Simplified
        }

        if response.drag_stopped() {
            // Handle text selection completion
            if let Some(pointer_pos) = ui.ctx().pointer_latest_pos() {
                let selected_text = self.get_selected_text_from_response(&response, text);

                if !selected_text.is_empty() && TextToolbar::should_show_for_text(&selected_text) {
                    self.text_toolbar
                        .show_at_position(pointer_pos, selected_text);
                }
            }
        }

        // Handle double-click for word selection
        if response.double_clicked() {
            if let Some(pointer_pos) = ui.ctx().pointer_latest_pos() {
                if let Some(word) = self.get_word_at_position(&response, text) {
                    if TextToolbar::should_show_for_text(&word) {
                        self.text_toolbar.show_at_position(pointer_pos, word);
                    }
                }
            }
        }
    }

    fn get_selected_text_from_response(&self, response: &egui::Response, text: &str) -> String {
        // This is a simplified implementation
        // In a real implementation, you'd calculate the actual selected portion based on mouse positions

        if response.dragged() {
            // For now, return the first sentence or word depending on drag distance
            let drag_delta = response.drag_delta();
            if drag_delta.length() > 50.0 {
                // Long drag - try to select a sentence
                self.get_first_sentence(text)
            } else {
                // Short drag - select first word
                self.get_first_word(text)
            }
        } else {
            String::new()
        }
    }

    fn get_word_at_position(&self, _response: &egui::Response, text: &str) -> Option<String> {
        // Simplified: return the first word
        // In a real implementation, you'd calculate which word was clicked based on position
        let word = self.get_first_word(text);
        if word.is_empty() {
            None
        } else {
            Some(word)
        }
    }

    fn get_first_word(&self, text: &str) -> String {
        text.split_whitespace()
            .next()
            .unwrap_or("")
            .trim_matches(|c: char| !c.is_alphabetic())
            .to_string()
    }

    fn get_first_sentence(&self, text: &str) -> String {
        // Find the first sentence (up to first period, exclamation, or question mark)
        if let Some(pos) = text.find(&['.', '!', '?'][..]) {
            let sentence = &text[..=pos];
            sentence.trim().to_string()
        } else {
            // If no sentence ending found, return first few words
            text.split_whitespace()
                .take(10)
                .collect::<Vec<_>>()
                .join(" ")
        }
    }
}
````

## File: src/ui/rendering/markdown.rs
````rust
// file: src/ui/rendering/markdown.rs
// description: Enhanced markdown parser with proper text flow and clean output

use crate::types::UISettings;
use egui;

// Enhanced EasyMark parser based on the working version
mod easy_mark {
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub enum Item<'a> {
        Newline,
        Text(Style, &'a str),
        Heading(&'a str),
        BulletPoint(&'a str),
        NumberedPoint(&'a str, &'a str),
        CodeBlock(&'a str),
        InlineCode(&'a str),
        Separator,
    }

    #[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
    pub struct Style {
        pub strong: bool,
        pub italics: bool,
        pub code: bool,
    }

    pub struct Parser<'a> {
        s: &'a str,
        start_of_line: bool,
        style: Style,
    }

    impl<'a> Parser<'a> {
        pub fn new(s: &'a str) -> Self {
            Self {
                s: s.trim(),
                start_of_line: true,
                style: Style::default(),
            }
        }
    }

    impl<'a> Iterator for Parser<'a> {
        type Item = Item<'a>;

        fn next(&mut self) -> Option<Self::Item> {
            loop {
                if self.s.is_empty() {
                    return None;
                }

                // Skip leading whitespace on lines that aren't start of line
                if !self.start_of_line && self.s.starts_with(' ') {
                    let trimmed = self.s.trim_start();
                    if trimmed.is_empty() {
                        self.s = "";
                        continue;
                    }
                    let to_consume = self.s.len() - trimmed.len();
                    if to_consume > 0 && to_consume <= 4 {
                        self.s = trimmed;
                        continue;
                    }
                }

                // Handle newlines - double newlines create paragraph breaks
                if self.s.starts_with("\n\n") {
                    self.s = &self.s[2..].trim_start();
                    self.start_of_line = true;
                    self.style = Style::default();
                    return Some(Item::Newline);
                } else if self.s.starts_with('\n') {
                    // Single newline - just move to next line within paragraph
                    self.s = &self.s[1..];
                    // Convert single newline to space for continuous text flow
                    if !self.s.is_empty() && !self.s.starts_with('\n') {
                        return Some(Item::Text(self.style, " "));
                    }
                    self.start_of_line = true;
                    continue;
                }

                if self.start_of_line {
                    // Handle different heading levels
                    if let Some(after) = self.s.strip_prefix("### ") {
                        let end = after.find('\n').unwrap_or(after.len());
                        let heading = &after[..end];
                        self.s = &after[end..];
                        self.start_of_line = false;
                        return Some(Item::Heading(heading));
                    } else if let Some(after) = self.s.strip_prefix("## ") {
                        let end = after.find('\n').unwrap_or(after.len());
                        let heading = &after[..end];
                        self.s = &after[end..];
                        self.start_of_line = false;
                        return Some(Item::Heading(heading));
                    } else if let Some(after) = self.s.strip_prefix("# ") {
                        let end = after.find('\n').unwrap_or(after.len());
                        let heading = &after[..end];
                        self.s = &after[end..];
                        self.start_of_line = false;
                        return Some(Item::Heading(heading));
                    }

                    // Bullet point
                    if let Some(after) = self.s.strip_prefix("- ") {
                        let end = after.find('\n').unwrap_or(after.len());
                        let text = &after[..end];
                        self.s = &after[end..];
                        self.start_of_line = false;
                        return Some(Item::BulletPoint(text));
                    }

                    // Numbered list
                    let n_digits = self.s.chars().take_while(|c| c.is_ascii_digit()).count();
                    if n_digits > 0 {
                        let remaining = &self.s[n_digits..];
                        if let Some(after) = remaining.strip_prefix(". ") {
                            let number = &self.s[..n_digits];
                            let end = after.find('\n').unwrap_or(after.len());
                            let text = &after[..end];
                            self.s = &after[end..];
                            self.start_of_line = false;
                            return Some(Item::NumberedPoint(number, text));
                        }
                    }

                    // Separator
                    if self.s.starts_with("---") {
                        let after = self.s.trim_start_matches('-');
                        self.s = after.strip_prefix('\n').unwrap_or(after);
                        self.start_of_line = false;
                        return Some(Item::Separator);
                    }

                    // Code block
                    if self.s.starts_with("```") {
                        let after = &self.s[3..];
                        if let Some(end_pos) = after.find("\n```") {
                            let code = &after[..end_pos];
                            self.s = &after[end_pos + 4..];
                            self.start_of_line = false;
                            return Some(Item::CodeBlock(code));
                        } else if let Some(end_pos) = after.find("```") {
                            let code = &after[..end_pos];
                            self.s = &after[end_pos + 3..];
                            self.start_of_line = false;
                            return Some(Item::CodeBlock(code));
                        }
                    }
                }

                // Inline code
                if self.s.starts_with('`') {
                    let after = &self.s[1..];
                    if let Some(end) = after.find('`') {
                        let code = &after[..end];
                        self.s = &after[end + 1..];
                        self.start_of_line = false;
                        return Some(Item::InlineCode(code));
                    }
                }

                // Bold text
                if self.s.starts_with("**") {
                    self.s = &self.s[2..];
                    self.start_of_line = false;
                    self.style.strong = !self.style.strong;
                    continue;
                }

                // Italic text
                if self.s.starts_with('*') && !self.s.starts_with("**") {
                    self.s = &self.s[1..];
                    self.start_of_line = false;
                    self.style.italics = !self.style.italics;
                    continue;
                }

                // Regular text - find the end of this text segment
                let end = self.s.find(&['*', '`', '\n'][..]).unwrap_or(self.s.len());

                if end == 0 {
                    // Handle special characters at the start
                    let text = &self.s[..1];
                    self.s = &self.s[1..];
                    self.start_of_line = false;
                    return Some(Item::Text(self.style, text));
                }

                let text = &self.s[..end];
                self.s = &self.s[end..];
                self.start_of_line = false;

                if !text.trim().is_empty() {
                    return Some(Item::Text(self.style, text));
                }
            }
        }
    }
}

pub struct MarkdownRenderer {
    // Store settings reference for consistent styling
}

impl MarkdownRenderer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&mut self, ui: &mut egui::Ui, content: &str) {
        self.render_with_settings(ui, content, &UISettings::default())
    }

    // Clean content for proper display
    fn clean_content(&self, content: &str) -> String {
        content
            .chars()
            .filter_map(|c| {
                match c {
                    // Keep standard printable ASCII characters
                    ' '..='~' => Some(c),

                    // Keep common whitespace
                    '\n' | '\t' => Some(c),

                    // Replace common problematic unicode with ASCII equivalents
                    '\u{2013}' | '\u{2014}' => Some('-'), // en-dash, em-dash
                    '\u{2018}' | '\u{2019}' => Some('\''), // smart quotes
                    '\u{201C}' | '\u{201D}' => Some('"'), // smart double quotes
                    '\u{2026}' => Some('.'),              // ellipsis -> single dot

                    // Replace bullets with simple ones
                    '\u{2022}' | '\u{2023}' | '\u{25E6}' | '\u{2043}' => Some('•'),

                    // Remove other control characters and problematic unicode
                    c if c.is_control() && c != '\n' && c != '\t' => None,
                    c if c as u32 > 127 && !matches!(c, '•') => {
                        // Replace other non-ASCII with space if it's not our allowed bullet
                        Some(' ')
                    }

                    _ => Some(c),
                }
            })
            .collect::<String>()
            // Clean up multiple spaces and normalize whitespace
            .lines()
            .map(|line| line.split_whitespace().collect::<Vec<_>>().join(" "))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn render_with_settings(
        &mut self,
        ui: &mut egui::Ui,
        content: &str,
        settings: &UISettings,
    ) {
        use easy_mark::{Item, Parser};

        // Clean the content first
        let cleaned_content = self.clean_content(content);

        let mut is_first_item = true;
        let mut current_paragraph_parts = Vec::new();

        for item in Parser::new(&cleaned_content) {
            match item {
                Item::Newline => {
                    // Render accumulated paragraph parts if any
                    if !current_paragraph_parts.is_empty() {
                        if !is_first_item {
                            ui.add_space(settings.paragraph_spacing);
                        }
                        self.render_paragraph_parts(ui, &current_paragraph_parts, settings);
                        current_paragraph_parts.clear();
                        is_first_item = false;
                    }
                }
                Item::Text(style, text) => {
                    if !text.trim().is_empty() {
                        current_paragraph_parts.push((style, text));
                    }
                }
                Item::Heading(text) => {
                    // Render any pending paragraph first
                    if !current_paragraph_parts.is_empty() {
                        if !is_first_item {
                            ui.add_space(settings.paragraph_spacing);
                        }
                        self.render_paragraph_parts(ui, &current_paragraph_parts, settings);
                        current_paragraph_parts.clear();
                        is_first_item = false;
                    }

                    if !is_first_item {
                        ui.add_space(settings.paragraph_spacing * 1.5);
                    }
                    let heading_text = settings.apply_header_style(egui::RichText::new(text));
                    ui.heading(heading_text);
                    ui.add_space(settings.paragraph_spacing);
                    is_first_item = false;
                }
                Item::BulletPoint(text) => {
                    // Render any pending paragraph first
                    if !current_paragraph_parts.is_empty() {
                        if !is_first_item {
                            ui.add_space(settings.paragraph_spacing);
                        }
                        self.render_paragraph_parts(ui, &current_paragraph_parts, settings);
                        current_paragraph_parts.clear();
                        is_first_item = false;
                    }

                    if !is_first_item {
                        ui.add_space(settings.paragraph_spacing / 2.0);
                    }
                    ui.horizontal(|ui| {
                        let bullet = settings.apply_text_body_style(egui::RichText::new("• "));
                        ui.label(bullet);

                        let bullet_text = settings.apply_text_body_style(egui::RichText::new(text));
                        ui.add(egui::Label::new(bullet_text).wrap());
                    });
                    is_first_item = false;
                }
                Item::NumberedPoint(number, text) => {
                    // Render any pending paragraph first
                    if !current_paragraph_parts.is_empty() {
                        if !is_first_item {
                            ui.add_space(settings.paragraph_spacing);
                        }
                        self.render_paragraph_parts(ui, &current_paragraph_parts, settings);
                        current_paragraph_parts.clear();
                        is_first_item = false;
                    }

                    if !is_first_item {
                        ui.add_space(settings.paragraph_spacing / 2.0);
                    }
                    ui.horizontal(|ui| {
                        let number_text = settings
                            .apply_text_body_style(egui::RichText::new(format!("{}. ", number)));
                        ui.label(number_text);

                        let point_text = settings.apply_text_body_style(egui::RichText::new(text));
                        ui.add(egui::Label::new(point_text).wrap());
                    });
                    is_first_item = false;
                }
                Item::CodeBlock(code) => {
                    // Render any pending paragraph first
                    if !current_paragraph_parts.is_empty() {
                        if !is_first_item {
                            ui.add_space(settings.paragraph_spacing);
                        }
                        self.render_paragraph_parts(ui, &current_paragraph_parts, settings);
                        current_paragraph_parts.clear();
                        is_first_item = false;
                    }

                    if !is_first_item {
                        ui.add_space(settings.paragraph_spacing);
                    }
                    ui.group(|ui| {
                        let code_text = egui::RichText::new(code)
                            .monospace()
                            .size(settings.get_text_body_font_size() * 0.9)
                            .color(egui::Color32::from_rgb(200, 200, 200))
                            .family(egui::FontFamily::Monospace);

                        ui.add(egui::Label::new(code_text).wrap());
                    });
                    ui.add_space(settings.paragraph_spacing);
                    is_first_item = false;
                }
                Item::InlineCode(code) => {
                    // Inline code is part of the current paragraph
                    current_paragraph_parts.push((
                        easy_mark::Style {
                            code: true,
                            ..Default::default()
                        },
                        code,
                    ));
                }
                Item::Separator => {
                    // Render any pending paragraph first
                    if !current_paragraph_parts.is_empty() {
                        if !is_first_item {
                            ui.add_space(settings.paragraph_spacing);
                        }
                        self.render_paragraph_parts(ui, &current_paragraph_parts, settings);
                        current_paragraph_parts.clear();
                        is_first_item = false;
                    }

                    if !is_first_item {
                        ui.add_space(settings.paragraph_spacing);
                    }
                    ui.separator();
                    ui.add_space(settings.paragraph_spacing);
                    is_first_item = false;
                }
            }
        }

        // Render any remaining paragraph parts
        if !current_paragraph_parts.is_empty() {
            if !is_first_item {
                ui.add_space(settings.paragraph_spacing);
            }
            self.render_paragraph_parts(ui, &current_paragraph_parts, settings);
        }
    }

    fn render_paragraph_parts(
        &self,
        ui: &mut egui::Ui,
        parts: &[(easy_mark::Style, &str)],
        settings: &UISettings,
    ) {
        ui.horizontal_wrapped(|ui| {
            for (style, text) in parts {
                if text.trim().is_empty() {
                    continue;
                }

                let mut rich_text = settings.apply_text_body_style(egui::RichText::new(*text));

                if style.strong {
                    rich_text = rich_text.strong();
                }
                if style.italics {
                    rich_text = rich_text.italics();
                }
                if style.code {
                    rich_text = rich_text
                        .monospace()
                        .background_color(egui::Color32::from_rgb(40, 40, 40))
                        .color(egui::Color32::from_rgb(255, 255, 255))
                        .family(egui::FontFamily::Monospace);
                }

                ui.add(egui::Label::new(rich_text).wrap());
            }
        });
    }
}
````

## File: src/ui/rendering/themes.rs
````rust
// file: src/ui/rendering/themes.rs
// description: Enhanced theme management with corner style support

use crate::types::UISettings;
use egui;

pub fn apply_theme(ctx: &egui::Context, settings: &UISettings) {
    let bg_color = settings.get_background_color();
    let text_color = settings.get_text_color();
    let accent_color = settings.get_accent_color();
    let rounding = settings.get_rounding();

    let mut style = (*ctx.style()).clone();

    // Basic colors
    style.visuals.window_fill = bg_color;
    style.visuals.panel_fill = bg_color;
    style.visuals.override_text_color = Some(text_color);

    // Corner rounding
    style.visuals.window_corner_radius = rounding;
    style.visuals.menu_corner_radius = rounding;

    // Widget rounding
    style.visuals.widgets.noninteractive.corner_radius = rounding;
    style.visuals.widgets.inactive.corner_radius = rounding;
    style.visuals.widgets.hovered.corner_radius = rounding;
    style.visuals.widgets.active.corner_radius = rounding;
    style.visuals.widgets.open.corner_radius = rounding;

    // Selection colors
    style.visuals.selection.bg_fill = accent_color.gamma_multiply(0.3);
    style.visuals.selection.stroke.color = accent_color;

    // Hyperlink color
    style.visuals.hyperlink_color = settings.get_link_color();

    // Button styling with corner preference
    if settings.is_rounded() {
        // More pronounced rounding for buttons in rounded mode
        style.visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(6);
        style.visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(6);
        style.visuals.widgets.active.corner_radius = egui::CornerRadius::same(6);
    }

    // Enhanced spacing
    style.spacing.item_spacing = egui::Vec2::splat(8.0);
    style.spacing.button_padding = egui::Vec2::new(12.0, 6.0);
    style.spacing.menu_margin = egui::Margin::same(8);
    style.spacing.window_margin = egui::Margin::same(8);

    ctx.set_style(style);
}
````

## File: src/ui/windows/debug.rs
````rust
// file: src/ui/windows/debug.rs
use crate::services::DatabaseService;
use egui;

pub struct DebugWindow {
    show: bool,
}

impl DebugWindow {
    pub fn new() -> Self {
        Self { show: false }
    }

    pub fn show(&mut self) {
        self.show = true;
    }

    pub fn draw(&mut self, ctx: &egui::Context, database_service: &DatabaseService) {
        if !self.show {
            return;
        }

        egui::Window::new("Debug & Diagnostics")
            .default_width(600.0)
            .resizable(true)
            .show(ctx, |ui| {
                ui.heading("System Information");
                ui.separator();

                ui.label(format!("App Version: {}", env!("CARGO_PKG_VERSION")));
                ui.label(format!("OS: {}", std::env::consts::OS));

                ui.add_space(10.0);
                ui.heading("Services Status");
                ui.separator();

                ui.label("Article Service: Connected");

                // Show actual database status
                let db_status = if database_service.is_available() {
                    "Connected"
                } else {
                    "NOT CONNECTED"
                };
                let color = if database_service.is_available() {
                    egui::Color32::GREEN
                } else {
                    egui::Color32::RED
                };
                ui.colored_label(color, format!("Database Service: {}", db_status));

                ui.add_space(10.0);
                if ui.button("Close").clicked() {
                    self.show = false;
                }
            });
    }
}
````

## File: src/ui/windows/definition.rs
````rust
// file: src/ui/windows/definition.rs
// description: Dictionary definition lookup window with proper error handling and environment reloading

use egui;
use serde::{Deserialize, Serialize};
use std::sync::mpsc;
use tracing::{error, info};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictionaryEntry {
    pub word: String,
    pub phonetics: Vec<Phonetic>,
    pub meanings: Vec<Meaning>,
    #[serde(default)]
    pub license: Option<License>,
    #[serde(rename = "sourceUrls", default)]
    pub source_urls: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Phonetic {
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub audio: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meaning {
    #[serde(rename = "partOfSpeech")]
    pub part_of_speech: String,
    pub definitions: Vec<Definition>,
    #[serde(default)]
    pub synonyms: Option<Vec<String>>,
    #[serde(default)]
    pub antonyms: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Definition {
    pub definition: String,
    #[serde(default)]
    pub synonyms: Option<Vec<String>>,
    #[serde(default)]
    pub antonyms: Option<Vec<String>>,
    #[serde(default)]
    pub example: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct License {
    pub name: String,
    pub url: String,
}

pub struct DefinitionWindow {
    show: bool,
    current_word: String,
    definition_data: Option<DictionaryEntry>,
    error_message: Option<String>,
    loading: bool,
    definition_receiver: Option<mpsc::Receiver<Result<DictionaryEntry, String>>>,
}

impl DefinitionWindow {
    pub fn new() -> Self {
        Self {
            show: false,
            current_word: String::new(),
            definition_data: None,
            error_message: None,
            loading: false,
            definition_receiver: None,
        }
    }

    pub fn lookup_word(&mut self, word: String) {
        // Reload environment to ensure we have the latest variables
        let _ = crate::config::environment::load_env_file();

        self.current_word = word.clone();
        self.definition_data = None;
        self.error_message = None;
        self.loading = true;
        self.show = true;

        // Start async lookup
        let (tx, rx) = mpsc::channel();
        self.definition_receiver = Some(rx);

        std::thread::spawn(move || {
            let result = Self::fetch_definition(&word);
            let _ = tx.send(result);
        });
    }

    fn fetch_definition(word: &str) -> Result<DictionaryEntry, String> {
        info!("Looking up definition for: {}", word);

        // Clean the word for lookup
        let clean_word = word
            .chars()
            .filter(|c| c.is_alphabetic())
            .collect::<String>()
            .to_lowercase();

        if clean_word.is_empty() {
            return Err("Invalid word for lookup".to_string());
        }

        // Check if the word is too short or invalid
        if clean_word.len() < 2 {
            return Err("Word too short for dictionary lookup".to_string());
        }

        match Self::make_definition_request(&clean_word) {
            Ok(definition) => Ok(definition),
            Err(e) => {
                error!("Failed to fetch definition: {}", e);
                Err(format!("Could not find definition for '{}'", word))
            }
        }
    }

    fn make_definition_request(word: &str) -> Result<DictionaryEntry, Box<dyn std::error::Error>> {
        let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", word);

        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .user_agent("reading-app/1.0")
            .build()?;

        info!("Making dictionary API request to: {}", url);

        let response = client.get(&url).send()?;

        if response.status().is_success() {
            let json_text = response.text()?;
            info!("Received dictionary response, parsing...");

            // Try to parse as array first (normal case)
            if let Ok(entries) = serde_json::from_str::<Vec<DictionaryEntry>>(&json_text) {
                if let Some(first_entry) = entries.into_iter().next() {
                    info!("Successfully parsed dictionary entry for: {}", word);
                    return Ok(first_entry);
                }
            }

            // Try to parse as single entry
            if let Ok(entry) = serde_json::from_str::<DictionaryEntry>(&json_text) {
                info!("Successfully parsed single dictionary entry for: {}", word);
                return Ok(entry);
            }

            error!("Could not parse dictionary response for word: {}", word);
            Err("Could not parse dictionary response".into())
        } else {
            let status_code = response.status().as_u16();
            error!("Dictionary API error {} for word: {}", status_code, word);

            match status_code {
                404 => Err("Word not found in dictionary".into()),
                429 => Err("Too many requests - please try again later".into()),
                _ => Err(format!("Dictionary service error: {}", response.status()).into()),
            }
        }
    }

    pub fn draw(&mut self, ctx: &egui::Context) {
        if !self.show {
            return;
        }

        // Check for received definition
        if let Some(ref receiver) = self.definition_receiver {
            if let Ok(result) = receiver.try_recv() {
                self.loading = false;
                match result {
                    Ok(definition) => {
                        info!("Definition received for: {}", self.current_word);
                        self.definition_data = Some(definition);
                        self.error_message = None;
                    }
                    Err(error) => {
                        error!(
                            "Definition lookup failed for {}: {}",
                            self.current_word, error
                        );
                        self.error_message = Some(error);
                        self.definition_data = None;
                    }
                }
                self.definition_receiver = None;
            }
        }

        egui::Window::new(format!("Definition: {}", self.current_word))
            .default_width(500.0)
            .default_height(400.0)
            .resizable(true)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    if self.loading {
                        ui.horizontal(|ui| {
                            ui.add(egui::Spinner::new());
                            ui.label("Looking up definition...");
                        });
                    } else if let Some(ref error) = self.error_message {
                        ui.colored_label(egui::Color32::RED, "Error:");
                        ui.label(error);

                        ui.add_space(10.0);
                        ui.label("Suggestions:");
                        ui.label("• Check the spelling");
                        ui.label("• Try a simpler form of the word");
                        ui.label("• Make sure it's an English word");

                        ui.add_space(10.0);
                        ui.label("Note: The dictionary lookup uses a free online API that may have occasional outages.");
                    } else if let Some(ref definition) = self.definition_data {
                        self.render_definition(ui, definition);
                    }
                });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    if ui.button("Close").clicked() {
                        self.show = false;
                    }

                    if !self.loading && ui.button("Lookup Again").clicked() {
                        self.lookup_word(self.current_word.clone());
                    }

                    if !self.current_word.is_empty() {
                        ui.separator();
                        ui.label("Search in:");

                        let search_urls = [
                            (
                                "Google",
                                format!(
                                    "https://www.google.com/search?q=define+{}",
                                    self.current_word
                                ),
                            ),
                            (
                                "Merriam-Webster",
                                format!(
                                    "https://www.merriam-webster.com/dictionary/{}",
                                    self.current_word
                                ),
                            ),
                            (
                                "Cambridge",
                                format!(
                                    "https://dictionary.cambridge.org/dictionary/english/{}",
                                    self.current_word
                                ),
                            ),
                        ];

                        for (name, url) in search_urls {
                            if ui.small_button(name).clicked() {
                                if let Err(e) = webbrowser::open(&url) {
                                    error!("Failed to open browser: {}", e);
                                }
                            }
                        }
                    }
                });
            });
    }

    fn render_definition(&self, ui: &mut egui::Ui, definition: &DictionaryEntry) {
        // Word header
        ui.heading(&definition.word);

        // Phonetics if available
        if !definition.phonetics.is_empty() {
            for phonetic in &definition.phonetics {
                if let Some(ref text) = phonetic.text {
                    ui.label(egui::RichText::new(format!("Pronunciation: {}", text)).italics());
                    ui.add_space(5.0);
                }
            }
        }

        ui.separator();
        ui.add_space(10.0);

        // Meanings
        for (meaning_idx, meaning) in definition.meanings.iter().enumerate() {
            if meaning_idx > 0 {
                ui.add_space(15.0);
            }

            // Part of speech
            ui.label(
                egui::RichText::new(&meaning.part_of_speech)
                    .strong()
                    .color(egui::Color32::from_rgb(100, 149, 237)),
            );

            ui.add_space(5.0);

            // Definitions
            for (def_idx, def) in meaning.definitions.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(format!("{}.", def_idx + 1));
                    ui.vertical(|ui| {
                        ui.label(&def.definition);

                        // Example if available
                        if let Some(ref example) = def.example {
                            ui.add_space(3.0);
                            ui.label(
                                egui::RichText::new(format!("Example: \"{}\"", example))
                                    .italics()
                                    .color(egui::Color32::GRAY),
                            );
                        }
                    });
                });
                ui.add_space(8.0);
            }

            // Synonyms if available
            if let Some(ref synonyms) = meaning.synonyms {
                if !synonyms.is_empty() {
                    ui.horizontal_wrapped(|ui| {
                        ui.label(egui::RichText::new("Synonyms:").strong());
                        for (i, synonym) in synonyms.iter().take(5).enumerate() {
                            if i > 0 {
                                ui.label(",");
                            }
                            ui.label(synonym);
                        }
                        if synonyms.len() > 5 {
                            ui.label(format!("... and {} more", synonyms.len() - 5));
                        }
                    });
                    ui.add_space(5.0);
                }
            }

            // Antonyms if available
            if let Some(ref antonyms) = meaning.antonyms {
                if !antonyms.is_empty() {
                    ui.horizontal_wrapped(|ui| {
                        ui.label(egui::RichText::new("Antonyms:").strong());
                        for (i, antonym) in antonyms.iter().take(5).enumerate() {
                            if i > 0 {
                                ui.label(",");
                            }
                            ui.label(antonym);
                        }
                        if antonyms.len() > 5 {
                            ui.label(format!("... and {} more", antonyms.len() - 5));
                        }
                    });
                }
            }
        }

        // Source attribution
        ui.add_space(20.0);
        ui.separator();
        ui.add_space(5.0);

        ui.small("Definitions provided by Free Dictionary API");
        if let Some(ref license) = definition.license {
            ui.small(format!("License: {}", license.name));
        }
    }
}

impl Default for DefinitionWindow {
    fn default() -> Self {
        Self::new()
    }
}
````

## File: src/ui/windows/explanation.rs
````rust
// file: src/ui/windows/explanation.rs
// description: Text explanation window using Groq API with environment variable reloading

use egui;
use serde_json::{json, Value};
use std::sync::mpsc;
use tracing::{error, info};

pub struct ExplanationWindow {
    show: bool,
    selected_text: String,
    context: String,
    explanation: String,
    loading: bool,
    explanation_receiver: Option<mpsc::Receiver<Result<String, String>>>,
}

impl ExplanationWindow {
    pub fn new() -> Self {
        Self {
            show: false,
            selected_text: String::new(),
            context: String::new(),
            explanation: String::new(),
            loading: false,
            explanation_receiver: None,
        }
    }

    pub fn explain_text(&mut self, text: String, context: String) {
        // Reload environment and get fresh API credentials
        let _ = crate::config::environment::load_env_file();

        let api_key = std::env::var("GROQ_API_KEY").unwrap_or_default();
        let base_url = std::env::var("GROQ_BASE_URL")
            .unwrap_or_else(|_| "https://api.groq.com/openai/v1".to_string());

        if api_key.trim().is_empty() {
            error!("GROQ_API_KEY not configured for explanation feature");
            self.explanation =
                "Error: GROQ_API_KEY not configured. Please check your .env file.".to_string();
            self.show = true;
            return;
        }

        self.selected_text = text;
        self.context = context;
        self.explanation = String::new();
        self.loading = true;
        self.show = true;

        info!(
            "Starting explanation request for text: '{}'",
            &self.selected_text[..std::cmp::min(50, self.selected_text.len())]
        );

        // Start async explanation request
        let (tx, rx) = mpsc::channel();
        self.explanation_receiver = Some(rx);

        let selected_text = self.selected_text.clone();
        let context = self.context.clone();

        std::thread::spawn(move || {
            let explanation =
                Self::fetch_explanation(&selected_text, &context, &api_key, &base_url);
            let _ = tx.send(explanation);
        });
    }

    fn fetch_explanation(
        text: &str,
        context: &str,
        api_key: &str,
        base_url: &str,
    ) -> Result<String, String> {
        info!("Requesting explanation for: {}", text);

        if api_key.trim().is_empty() {
            return Err("API key not configured".to_string());
        }

        if text.trim().is_empty() {
            return Err("No text provided for explanation".to_string());
        }

        match Self::make_explanation_request(text, context, api_key, base_url) {
            Ok(explanation) => {
                info!("Successfully received explanation");
                Ok(explanation)
            }
            Err(e) => {
                error!("Failed to fetch explanation: {}", e);
                Err(format!("Unable to provide explanation: {}", e))
            }
        }
    }

    fn make_explanation_request(
        text: &str,
        context: &str,
        api_key: &str,
        base_url: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let context_snippet = if context.len() > 500 {
            &context[..500]
        } else {
            context
        };

        let prompt = format!(
            "Please explain the following text in simple, clear terms. Provide context and clarification that would help someone understand it better.\n\nText to explain: \"{}\"\n\nSurrounding context: {}\n\nProvide a clear, helpful explanation in 2-3 paragraphs:",
            text, context_snippet
        );

        let request_body = json!({
            "messages": [
                {
                    "role": "system",
                    "content": "You are a helpful teacher who explains complex concepts in simple, clear language. Your explanations should be educational and easy to understand."
                },
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "model": "llama3-8b-8192",
            "temperature": 0.7,
            "max_tokens": 1000
        });

        info!("Making explanation API request to Groq");

        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .user_agent("reading-app/1.0")
            .build()?;

        let response = client
            .post(format!("{}/chat/completions", base_url))
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().unwrap_or_default();
            error!("Groq API error {}: {}", status, error_text);
            return Err(format!("API error {}: {}", status, error_text).into());
        }

        let response_json: Value = response.json()?;

        let explanation = response_json
            .get("choices")
            .and_then(|choices| choices.get(0))
            .and_then(|choice| choice.get("message"))
            .and_then(|message| message.get("content"))
            .and_then(|content| content.as_str())
            .ok_or("Invalid response format")?;

        if explanation.trim().is_empty() {
            return Err("Empty explanation received from API".into());
        }

        info!("Successfully received explanation from Groq API");
        Ok(explanation.to_string())
    }

    pub fn draw(&mut self, ctx: &egui::Context) {
        if !self.show {
            return;
        }

        // Check for received explanation
        if let Some(ref receiver) = self.explanation_receiver {
            if let Ok(result) = receiver.try_recv() {
                self.loading = false;
                match result {
                    Ok(explanation) => {
                        info!("Explanation received successfully");
                        self.explanation = explanation;
                    }
                    Err(error) => {
                        error!("Explanation request failed: {}", error);
                        self.explanation = format!("Error: {}", error);
                    }
                }
                self.explanation_receiver = None;
            }
        }

        egui::Window::new("Text Explanation")
            .default_width(600.0)
            .default_height(450.0)
            .resizable(true)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.heading("Selected Text:");
                        ui.add_space(5.0);

                        // Show selected text in a frame
                        egui::Frame::group(ui.style())
                            .fill(ui.style().visuals.code_bg_color)
                            .inner_margin(egui::Margin::same(10))
                            .show(ui, |ui| {
                                ui.add(
                                    egui::Label::new(
                                        egui::RichText::new(&self.selected_text)
                                            .italics()
                                            .color(egui::Color32::LIGHT_GRAY),
                                    )
                                    .wrap(),
                                );
                            });

                        ui.add_space(15.0);
                        ui.separator();
                        ui.add_space(10.0);

                        ui.heading("Explanation:");
                        ui.add_space(5.0);

                        if self.loading {
                            ui.horizontal(|ui| {
                                ui.add(egui::Spinner::new());
                                ui.label("Generating explanation...");
                            });
                            ui.add_space(5.0);
                            ui.label("This may take a few seconds.");
                        } else if !self.explanation.is_empty() {
                            // Show explanation with proper formatting
                            if self.explanation.starts_with("Error:") {
                                ui.colored_label(egui::Color32::RED, &self.explanation);

                                if self.explanation.contains("API key") {
                                    ui.add_space(10.0);
                                    ui.label("To enable explanations:");
                                    ui.label("1. Create a .env file in your project root");
                                    ui.label("2. Add: GROQ_API_KEY=your_api_key_here");
                                    ui.label("3. Get a free API key from console.groq.com");
                                }
                            } else {
                                ui.add(egui::Label::new(&self.explanation).wrap());
                            }
                        } else {
                            ui.label("No explanation available.");
                        }
                    });
                });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    if ui.button("Close").clicked() {
                        self.show = false;
                    }

                    if !self.loading
                        && !self.selected_text.is_empty()
                        && !self.explanation.starts_with("Error:")
                    {
                        if ui.button("Explain Again").clicked() {
                            self.explain_text(self.selected_text.clone(), self.context.clone());
                        }
                    }

                    // Check current API key status and show helpful info
                    let _ = crate::config::environment::load_env_file();
                    let api_key = std::env::var("GROQ_API_KEY").unwrap_or_default();

                    if api_key.trim().is_empty() {
                        ui.separator();
                        ui.colored_label(
                            egui::Color32::YELLOW,
                            "Note: Configure GROQ_API_KEY to enable explanations",
                        );
                    } else {
                        ui.separator();
                        ui.colored_label(egui::Color32::GREEN, "API configured");
                    }
                });
            });
    }
}

impl Default for ExplanationWindow {
    fn default() -> Self {
        Self::new()
    }
}
````

## File: src/ui/windows/mod.rs
````rust
// file: src/ui/windows/mod.rs
pub mod debug;
pub mod definition;
pub mod explanation;
pub mod search;
pub mod settings;

pub use debug::DebugWindow;
pub use definition::DefinitionWindow;
pub use explanation::ExplanationWindow;
pub use search::SearchWindow;
pub use settings::SettingsWindow;
````

## File: src/ui/windows/search.rs
````rust
// file: src/ui/windows/search.rs
// description: Search window

use crate::services::DatabaseService;
use egui;

pub struct SearchWindow {
    show: bool,
    query: String,
}

impl SearchWindow {
    pub fn new() -> Self {
        Self {
            show: false,
            query: String::new(),
        }
    }
}

impl Default for SearchWindow {
    fn default() -> Self {
        Self::new()
    }
}

impl SearchWindow {
    pub fn show(&mut self) {
        self.show = true;
    }

    pub fn draw(&mut self, ctx: &egui::Context, _database_service: &DatabaseService) {
        if !self.show {
            return;
        }

        egui::Window::new("Search Articles")
            .default_width(500.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Search:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.query)
                            .hint_text("Enter search terms..."),
                    );

                    if ui.button("Search").clicked() {
                        // Perform search
                    }
                });

                ui.separator();

                // TODO: Display search results

                ui.add_space(10.0);
                if ui.button("Close").clicked() {
                    self.show = false;
                }
            });
    }
}
````

## File: src/ui/windows/settings.rs
````rust
// file: src/ui/windows/settings.rs
// description: Enhanced settings window with corner style option and improved layout

use crate::services::SettingsService;
use crate::types::UISettings;
use crate::ui::events::UIEvent;
use egui;
use tracing::{error, info};

pub struct SettingsWindow {
    show: bool,
}

impl SettingsWindow {
    pub fn new() -> Self {
        Self { show: false }
    }
}

impl Default for SettingsWindow {
    fn default() -> Self {
        Self::new()
    }
}

impl SettingsWindow {
    pub fn show(&mut self) {
        self.show = true;
    }

    pub fn draw(
        &mut self,
        ctx: &egui::Context,
        settings_service: &mut SettingsService,
    ) -> Vec<UIEvent> {
        let mut events = Vec::new();

        if !self.show {
            return events;
        }

        egui::Window::new("Settings")
            .default_width(750.0)
            .default_height(650.0)
            .resizable(true)
            .show(ctx, |ui| {
                let mut ui_settings = settings_service.get_ui_settings();
                let mut changed = false;

                egui::ScrollArea::vertical().show(ui, |ui| {
                    let heading_text = ui_settings
                        .apply_font_style(egui::RichText::new("UI Customization").strong());
                    ui.heading(heading_text);
                    ui.separator();

                    // Interface Style
                    ui.collapsing("Interface Style", |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Corner Style:");
                            egui::ComboBox::from_id_salt("corner_style")
                                .selected_text(&ui_settings.corner_style)
                                .show_ui(ui, |ui| {
                                    let styles = [
                                        ("rounded", "Rounded Corners"),
                                        ("square", "Square Corners"),
                                    ];
                                    for (value, display) in styles {
                                        if ui
                                            .selectable_value(
                                                &mut ui_settings.corner_style,
                                                value.to_string(),
                                                display,
                                            )
                                            .changed()
                                        {
                                            changed = true;
                                        }
                                    }
                                });
                        });

                        ui.horizontal(|ui| {
                            ui.label("Theme Mode:");
                            egui::ComboBox::from_id_salt("theme_mode")
                                .selected_text(&ui_settings.theme_mode)
                                .show_ui(ui, |ui| {
                                    let modes = [
                                        ("dark", "Dark"),
                                        ("light", "Light"),
                                        ("sepia", "Sepia"),
                                        ("high_contrast", "High Contrast"),
                                    ];
                                    for (value, display) in modes {
                                        if ui
                                            .selectable_value(
                                                &mut ui_settings.theme_mode,
                                                value.to_string(),
                                                display,
                                            )
                                            .changed()
                                        {
                                            changed = true;
                                        }
                                    }
                                });
                        });
                    });

                    // Font Settings
                    ui.collapsing("Font Settings", |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Text Body Font Size:");
                            if ui
                                .add(
                                    egui::Slider::new(
                                        &mut ui_settings.text_body_font_size,
                                        8.0..=32.0,
                                    )
                                    .suffix("pt"),
                                )
                                .changed()
                            {
                                changed = true;
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("Header Font Size:");
                            if ui
                                .add(
                                    egui::Slider::new(
                                        &mut ui_settings.header_font_size,
                                        12.0..=48.0,
                                    )
                                    .suffix("pt"),
                                )
                                .changed()
                            {
                                changed = true;
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("Text Body Font:");
                            egui::ComboBox::from_id_salt("text_body_font")
                                .selected_text(&ui_settings.text_body_font)
                                .show_ui(ui, |ui| {
                                    // Use the font manager to get available fonts
                                    let available_fonts =
                                        crate::utils::fonts::get_available_fonts();
                                    for (value, display) in available_fonts {
                                        if ui
                                            .selectable_value(
                                                &mut ui_settings.text_body_font,
                                                value.to_string(),
                                                display,
                                            )
                                            .changed()
                                        {
                                            changed = true;
                                        }
                                    }
                                });
                        });

                        ui.horizontal(|ui| {
                            ui.label("Header Font:");
                            egui::ComboBox::from_id_salt("header_font")
                                .selected_text(&ui_settings.header_font)
                                .show_ui(ui, |ui| {
                                    // Use the font manager to get available fonts
                                    let available_fonts =
                                        crate::utils::fonts::get_available_fonts();
                                    for (value, display) in available_fonts {
                                        if ui
                                            .selectable_value(
                                                &mut ui_settings.header_font,
                                                value.to_string(),
                                                display,
                                            )
                                            .changed()
                                        {
                                            changed = true;
                                        }
                                    }
                                });
                        });

                        ui.horizontal(|ui| {
                            ui.label("General Font Family:");
                            egui::ComboBox::from_id_salt("font_family")
                                .selected_text(&ui_settings.font_family)
                                .show_ui(ui, |ui| {
                                    // Use the font manager to get available fonts
                                    let available_fonts =
                                        crate::utils::fonts::get_available_fonts();
                                    for (value, display) in available_fonts {
                                        if ui
                                            .selectable_value(
                                                &mut ui_settings.font_family,
                                                value.to_string(),
                                                display,
                                            )
                                            .changed()
                                        {
                                            changed = true;
                                        }
                                    }
                                });
                        });

                        ui.horizontal(|ui| {
                            ui.label("General Font Size:");
                            if ui
                                .add(
                                    egui::Slider::new(&mut ui_settings.font_size, 8.0..=32.0)
                                        .suffix("pt"),
                                )
                                .changed()
                            {
                                changed = true;
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("Line Height:");
                            if ui
                                .add(
                                    egui::Slider::new(&mut ui_settings.line_height, 1.0..=3.0)
                                        .suffix("x"),
                                )
                                .changed()
                            {
                                changed = true;
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("Paragraph Spacing:");
                            if ui
                                .add(
                                    egui::Slider::new(
                                        &mut ui_settings.paragraph_spacing,
                                        0.0..=50.0,
                                    )
                                    .suffix("px"),
                                )
                                .changed()
                            {
                                changed = true;
                            }
                        });

                        // Font size presets
                        ui.separator();
                        ui.label("Font Size Presets:");
                        ui.horizontal_wrapped(|ui| {
                            if ui.button("Small").clicked() {
                                ui_settings.apply_small_font_preset();
                                changed = true;
                            }
                            if ui.button("Medium").clicked() {
                                ui_settings.apply_medium_font_preset();
                                changed = true;
                            }
                            if ui.button("Large").clicked() {
                                ui_settings.apply_large_font_preset();
                                changed = true;
                            }
                            if ui.button("Extra Large").clicked() {
                                ui_settings.apply_extra_large_font_preset();
                                changed = true;
                            }
                        });
                    });

                    // Layout Settings
                    ui.collapsing("Layout Settings", |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Zoom Level:");
                            if ui
                                .add(
                                    egui::Slider::new(&mut ui_settings.zoom_level, 0.5..=3.0)
                                        .suffix("x"),
                                )
                                .changed()
                            {
                                changed = true;
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("Sidebar Width:");
                            if ui
                                .add(
                                    egui::Slider::new(
                                        &mut ui_settings.sidebar_width,
                                        200.0..=800.0,
                                    )
                                    .suffix("px"),
                                )
                                .changed()
                            {
                                changed = true;
                            }
                        });
                    });

                    // Color Settings
                    ui.collapsing("Color Settings", |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Background:");
                            let mut bg_color = ui_settings.get_background_color();
                            if ui.color_edit_button_srgba(&mut bg_color).changed() {
                                ui_settings.background_color = format!(
                                    "#{:02X}{:02X}{:02X}",
                                    bg_color.r(),
                                    bg_color.g(),
                                    bg_color.b()
                                );
                                changed = true;
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("Text Color:");
                            let mut text_color = ui_settings.get_text_color();
                            if ui.color_edit_button_srgba(&mut text_color).changed() {
                                ui_settings.text_color = format!(
                                    "#{:02X}{:02X}{:02X}",
                                    text_color.r(),
                                    text_color.g(),
                                    text_color.b()
                                );
                                changed = true;
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("Header Color:");
                            let mut header_color = ui_settings.get_header_color();
                            if ui.color_edit_button_srgba(&mut header_color).changed() {
                                ui_settings.header_color = format!(
                                    "#{:02X}{:02X}{:02X}",
                                    header_color.r(),
                                    header_color.g(),
                                    header_color.b()
                                );
                                changed = true;
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("Link Color:");
                            let mut link_color = ui_settings.get_link_color();
                            if ui.color_edit_button_srgba(&mut link_color).changed() {
                                ui_settings.link_color = format!(
                                    "#{:02X}{:02X}{:02X}",
                                    link_color.r(),
                                    link_color.g(),
                                    link_color.b()
                                );
                                changed = true;
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("Accent Color:");
                            let mut accent_color = ui_settings.get_accent_color();
                            if ui.color_edit_button_srgba(&mut accent_color).changed() {
                                ui_settings.accent_color = format!(
                                    "#{:02X}{:02X}{:02X}",
                                    accent_color.r(),
                                    accent_color.g(),
                                    accent_color.b()
                                );
                                changed = true;
                            }
                        });

                        ui.separator();
                        ui.label("Theme Presets:");
                        ui.horizontal_wrapped(|ui| {
                            if ui.button("Dark Theme").clicked() {
                                ui_settings.apply_dark_theme();
                                changed = true;
                            }
                            if ui.button("Light Theme").clicked() {
                                ui_settings.apply_light_theme();
                                changed = true;
                            }
                            if ui.button("Sepia").clicked() {
                                ui_settings.apply_sepia_theme();
                                changed = true;
                            }
                            if ui.button("High Contrast").clicked() {
                                ui_settings.apply_high_contrast_theme();
                                changed = true;
                            }
                        });
                    });

                    // Display Options
                    ui.collapsing("Display Options", |ui| {
                        if ui
                            .checkbox(
                                &mut ui_settings.show_article_stats,
                                "Show Article Statistics",
                            )
                            .changed()
                        {
                            changed = true;
                        }
                    });

                    ui.separator();

                    // Action buttons
                    ui.horizontal(|ui| {
                        if ui.button("Reset to Defaults").clicked() {
                            ui_settings = UISettings::default();
                            changed = true;
                        }

                        if ui.button("Close Settings").clicked() {
                            self.show = false;
                        }
                    });

                    // Save settings if changed
                    if changed {
                        if let Err(e) = ui_settings.validate() {
                            error!("Settings validation failed: {}", e);
                            ui.colored_label(
                                egui::Color32::RED,
                                format!("Validation Error: {}", e),
                            );
                        } else {
                            match settings_service.save_ui_settings(&ui_settings) {
                                Ok(_) => {
                                    info!("Settings saved successfully");
                                    events.push(UIEvent::SettingsChanged);
                                }
                                Err(e) => {
                                    error!("Failed to save settings: {}", e);
                                    ui.colored_label(
                                        egui::Color32::RED,
                                        format!("Save Error: {}", e),
                                    );
                                }
                            }
                        }
                    }
                });
            });

        events
    }
}
````

## File: src/ui/events.rs
````rust
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
````

## File: src/ui/mod.rs
````rust
// file: src/ui/mod.rs

pub mod components;
pub mod events;
pub mod rendering;
pub mod windows;

pub use events::UIEvent;
````

## File: src/utils/fonts.rs
````rust
// file: src/utils/fonts.rs
// description: Working font configuration with safe customization

use egui::{FontData, FontDefinitions, FontFamily};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tracing::{error, info, warn};

pub struct FontManager {
    available_fonts: HashMap<String, String>, // name -> path
    loaded_font_keys: Vec<String>,            // Successfully loaded font keys
}

impl FontManager {
    pub fn new() -> Self {
        let mut manager = Self {
            available_fonts: HashMap::new(),
            loaded_font_keys: Vec::new(),
        };
        manager.scan_system_fonts();
        manager
    }

    fn scan_system_fonts(&mut self) {
        let font_paths: Vec<String> = if cfg!(target_os = "macos") {
            let home = std::env::var("HOME").unwrap_or_default();
            vec![
                "/System/Library/Fonts".to_string(),
                "/Library/Fonts".to_string(),
                format!("{}/.fonts", home),
                format!("{}/Library/Fonts", home),
            ]
        } else if cfg!(target_os = "windows") {
            let user_profile = std::env::var("USERPROFILE").unwrap_or_default();
            vec![
                "C:/Windows/Fonts".to_string(),
                format!("{}/.fonts", user_profile),
            ]
        } else {
            let home = std::env::var("HOME").unwrap_or_default();
            vec![
                "/usr/share/fonts".to_string(),
                "/usr/local/share/fonts".to_string(),
                format!("{}/.fonts", home),
                format!("{}/.local/share/fonts", home),
            ]
        };

        for font_dir in &font_paths {
            self.scan_directory(font_dir);
        }

        info!("Found {} system fonts", self.available_fonts.len());
    }

    fn scan_directory(&mut self, dir_path: &str) {
        let path = Path::new(dir_path);
        if !path.exists() || !path.is_dir() {
            return;
        }

        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(extension) = path.extension() {
                        let ext = extension.to_string_lossy().to_lowercase();
                        if matches!(ext.as_str(), "ttf" | "otf") {
                            if let Some(name) = path.file_stem() {
                                let font_name = name.to_string_lossy().to_string();
                                let font_path = path.to_string_lossy().to_string();

                                // Special handling for TerminessTTF Nerd Font
                                if font_name.contains("TerminessTTF")
                                    || font_name.contains("Terminess")
                                {
                                    self.available_fonts
                                        .insert("terminus_nerd_mono".to_string(), font_path);
                                }
                                // Look for common system fonts by name patterns
                                else if font_name.to_lowercase().contains("times") {
                                    self.available_fonts
                                        .insert("times_font".to_string(), font_path);
                                } else if font_name.to_lowercase().contains("arial") {
                                    self.available_fonts
                                        .insert("arial_font".to_string(), font_path);
                                } else if font_name.to_lowercase().contains("helvetica") {
                                    self.available_fonts
                                        .insert("helvetica_font".to_string(), font_path);
                                }
                                // Store other fonts with sanitized names
                                else {
                                    let sanitized_name = font_name
                                        .replace(" ", "_")
                                        .replace("(", "")
                                        .replace(")", "")
                                        .to_lowercase();
                                    self.available_fonts.insert(sanitized_name, font_path);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn get_available_fonts(&self) -> Vec<(String, String)> {
        let mut fonts = vec![
            ("default".to_string(), "Default (Proportional)".to_string()),
            ("monospace".to_string(), "System Monospace".to_string()),
        ];

        // Add successfully loaded fonts
        for font_key in &self.loaded_font_keys {
            let display_name = match font_key.as_str() {
                "terminus_nerd_mono" => "TerminessTTF Nerd Font Mono".to_string(),
                "times_font" => "Times".to_string(),
                "arial_font" => "Arial".to_string(),
                "helvetica_font" => "Helvetica".to_string(),
                _ => font_key.replace("_", " ").to_string(),
            };
            fonts.push((font_key.clone(), display_name));
        }

        fonts.sort_by(|a, b| a.1.cmp(&b.1));
        fonts
    }

    pub fn load_font_data(&self, font_name: &str) -> Option<FontData> {
        if let Some(font_path) = self.available_fonts.get(font_name) {
            match fs::read(font_path) {
                Ok(data) => {
                    info!("Loaded font: {} from {}", font_name, font_path);
                    Some(FontData::from_owned(data))
                }
                Err(e) => {
                    error!("Failed to read font file {}: {}", font_path, e);
                    None
                }
            }
        } else {
            None
        }
    }

    pub fn has_font(&self, font_name: &str) -> bool {
        self.available_fonts.contains_key(font_name)
    }

    pub fn register_loaded_font(&mut self, font_key: String) {
        if !self.loaded_font_keys.contains(&font_key) {
            self.loaded_font_keys.push(font_key);
        }
    }
}

// Global font manager instance
pub fn get_font_manager() -> &'static std::sync::Mutex<FontManager> {
    use std::sync::{Mutex, OnceLock};

    static FONT_MANAGER_CELL: OnceLock<Mutex<FontManager>> = OnceLock::new();
    FONT_MANAGER_CELL.get_or_init(|| Mutex::new(FontManager::new()))
}

pub fn configure(cc: &eframe::CreationContext<'_>) {
    let ctx = &cc.egui_ctx;
    let font_manager_mutex = get_font_manager();
    let mut font_manager = font_manager_mutex.lock().unwrap();

    let mut fonts = FontDefinitions::default();

    // Load system fonts that were found and CAN BE LOADED
    let font_keys_to_try: Vec<String> = font_manager.available_fonts.keys().cloned().collect();

    for font_key in font_keys_to_try {
        if let Some(font_data) = font_manager.load_font_data(&font_key) {
            // Insert the font data with a unique key
            fonts.font_data.insert(font_key.clone(), font_data.into());

            // Create a font family for this font
            fonts.families.insert(
                FontFamily::Name(font_key.clone().into()),
                vec![font_key.clone()],
            );

            // Track that this font was successfully loaded
            font_manager.register_loaded_font(font_key);
        }
    }
    ctx.set_fonts(fonts);
}

// Helper functions for UISettings
pub fn get_available_fonts() -> Vec<(String, String)> {
    let font_manager_mutex = get_font_manager();
    let font_manager = font_manager_mutex.lock().unwrap();
    font_manager.get_available_fonts()
}

pub fn get_font_family_for_name(font_name: &str) -> egui::FontFamily {
    let font_manager_mutex = get_font_manager();
    let font_manager = font_manager_mutex.lock().unwrap();

    match font_name {
        "default" => egui::FontFamily::Proportional,
        "monospace" => egui::FontFamily::Monospace,
        _ => {
            // Check if it's a system font we actually loaded
            if font_manager
                .loaded_font_keys
                .contains(&font_name.to_string())
            {
                egui::FontFamily::Name(font_name.into())
            } else {
                // Safe fallback - use built-in fonts only
                warn!("Font '{}' not loaded, falling back to default", font_name);
                egui::FontFamily::Proportional
            }
        }
    }
}
````

## File: src/utils/logging.rs
````rust
// file: src/utils/logging.rs
// description: Logging configuration

use crate::types::AppResult;
use std::sync::Once;
use tracing_subscriber::{fmt, EnvFilter};

static INIT: Once = Once::new();

pub fn init() -> AppResult<()> {
    INIT.call_once(|| {
        if std::env::var("RUST_LOG").is_err() {
            std::env::set_var("RUST_LOG", "reading_app=info,warn");
        }

        let filter = EnvFilter::from_default_env();
        let subscriber = fmt()
            .with_env_filter(filter)
            .with_ansi(true)
            .with_target(false)
            .compact()
            .finish();

        tracing::subscriber::set_global_default(subscriber)
            .expect("Failed to set global default subscriber");
    });

    Ok(())
}
````

## File: src/utils/mod.rs
````rust
// file: src/utils/mod.rs
pub mod fonts;
pub mod logging;
````

## File: src/lib.rs
````rust
// file: src/lib.rs
// description: Library root with public exports

pub mod app;
pub mod client;
pub mod config;
pub mod database;
pub mod services;
pub mod types;
pub mod ui;
pub mod utils;

// Re-export commonly used types
pub use types::{
    article::{Article, ArticleSubject},
    errors::{AppError, AppResult},
    settings::UISettings,
};

// Re-export main application
pub use app::App;
````

## File: src/main.rs
````rust
// file: src/main.rs
// description: Application entry point with environment setup and runtime management

use reading_app::{
    app::{runtime, App},
    config::AppConfig,
    utils::logging,
};
use std::process;
use tracing::info;

fn main() -> eframe::Result<()> {
    // Initialize logging first
    if let Err(e) = logging::init() {
        eprintln!("Failed to initialize logging: {}", e);
        process::exit(1);
    }

    info!(
        "Starting Reading Application v{}",
        env!("CARGO_PKG_VERSION")
    );

    // Initialize global async runtime early
    runtime::init_runtime();

    // Load configuration
    let config = match AppConfig::load() {
        Ok(config) => {
            info!("Configuration loaded successfully");
            config
        }
        Err(e) => {
            eprintln!("Configuration error: {}", e);
            eprintln!("Make sure you have a .env file with GROQ_API_KEY set");
            eprintln!("Example .env file content:");
            eprintln!("GROQ_API_KEY=your_api_key_here");
            eprintln!("DATABASE_PATH=reading_app.db");
            process::exit(1);
        }
    };

    // Set up panic handler
    std::panic::set_hook(Box::new(|panic_info| {
        tracing::error!("Application panicked: {}", panic_info);
        if let Some(location) = panic_info.location() {
            tracing::error!(
                "Panic occurred in file '{}' at line {}",
                location.file(),
                location.line()
            );
        }
    }));

    // Configure native options
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1400.0, 900.0])
            .with_min_inner_size([1000.0, 700.0])
            .with_title("Reading Article Application")
            .with_icon(eframe::egui::IconData::default()),
        persist_window: true,
        centered: true,
        ..Default::default()
    };

    // Run the application
    eframe::run_native(
        "Reading Article Application",
        native_options,
        Box::new(move |cc| {
            reading_app::utils::fonts::configure(cc);
            Ok(Box::new(App::new(config)))
        }),
    )
}
````

## File: Cargo.toml
````toml
[package]
name = "reading_app"
version = "0.1.0"
edition = "2021"

[dependencies]
anyhow = "1.0.99"
chrono = { version = "0.4.41", features = ["serde"] }
dirs = "6.0.0"
dotenvy = "0.15.7"
eframe = { version = "0.32.2", features = ["default_fonts", "glow", "persistence"] }
egui = "0.32.2"
env_logger = "0.11.8"
futures = "0.3.31"
libsql = "0.9.23"
reqwest = { version = "0.12.23", features = ["json", "blocking"] }
rustc_version_runtime = "0.3.0"
serde = { version = "1.0.219", features = ["serde_derive"] }
serde_json = "1.0.143"
thiserror = "2.0.16"
tokio = { version = "1.47.1", features = ["full", "macros"] }
tracing = "0.1.41"
tracing-subscriber = { version = "0.3.20", features = ["env-filter", "fmt", "ansi"] }
uuid = { version = "1.18.1", features = ["v4"] }
webbrowser = "1.0.1"
````

## File: src/ui/components/article_viewer.rs
````rust
// file: src/ui/components/article_viewer.rs
// description: Enhanced article content display component with interactive text selection

use crate::{
    types::{Article, ContentType, RequestStatus, UISettings},
    ui::events::UIEvent,
    ui::rendering::markdown::MarkdownRenderer,
    ui::rendering::markdown_interactive::InteractiveMarkdownRenderer,
};
use egui;

pub struct ArticleViewer {
    markdown_renderer: MarkdownRenderer,
    interactive_renderer: InteractiveMarkdownRenderer,
    interactive_mode: bool,
}

impl ArticleViewer {
    pub fn new() -> Self {
        Self {
            markdown_renderer: MarkdownRenderer::new(),
            interactive_renderer: InteractiveMarkdownRenderer::new(),
            interactive_mode: true, // Default to interactive mode for text selection
        }
    }

    pub fn set_interactive_mode(&mut self, interactive: bool) {
        self.interactive_mode = interactive;
    }

    pub fn draw(&mut self, ui: &mut egui::Ui, status: &RequestStatus) -> Vec<UIEvent> {
        self.draw_with_settings(ui, status, &UISettings::default())
    }

    pub fn draw_with_settings(
        &mut self,
        ui: &mut egui::Ui,
        status: &RequestStatus,
        settings: &UISettings,
    ) -> Vec<UIEvent> {
        let mut events = Vec::new();

        match status {
            RequestStatus::Idle => {
                self.draw_welcome_screen(ui, settings);
            }
            RequestStatus::Loading => {
                self.draw_loading_screen(ui, settings);
            }
            RequestStatus::Success(content_type) => {
                let content_events = self.draw_content(ui, content_type, settings);
                events.extend(content_events);
            }
            RequestStatus::Error(error) => {
                self.draw_error_screen(ui, error, settings);
            }
        }

        events
    }

    fn draw_welcome_screen(&self, ui: &mut egui::Ui, settings: &UISettings) {
        ui.vertical_centered(|ui| {
            ui.add_space(50.0);
            let app_title = settings.apply_header_style(
                egui::RichText::new("Reading App")
                    .strong()
                    .size(settings.get_font_size() * 2.0),
            );
            ui.heading(app_title);
            ui.add_space(20.0);
            ui.separator();
            ui.add_space(20.0);

            let instruction_text =
                settings.apply_text_body_style(egui::RichText::new("Get started by:"));
            ui.label(instruction_text);

            let bullet_points = [
                "• Select a subject from the sidebar",
                "• Optionally specify a custom topic",
                "• Click 'Generate Article' to create content",
            ];

            for point in bullet_points {
                let point_text = settings.apply_text_body_style(egui::RichText::new(point));
                ui.label(point_text);
            }

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(10.0);
            let info_text = settings.apply_text_body_style(
                egui::RichText::new(
                    "Articles are generated using Groq AI and stored in your local database",
                )
                .size(settings.get_font_size() * 0.85),
            );
            ui.label(info_text);
        });
    }

    fn draw_loading_screen(&self, ui: &mut egui::Ui, settings: &UISettings) {
        ui.vertical_centered(|ui| {
            ui.add_space(100.0);
            let loading_title =
                settings.apply_header_style(egui::RichText::new("Generating Article...").strong());
            ui.heading(loading_title);
            ui.add_space(20.0);
            ui.add(egui::Spinner::new().size(40.0));
            ui.add_space(20.0);
            let wait_text = settings.apply_text_body_style(egui::RichText::new(
                "Please wait while we generate your article using Groq AI.",
            ));
            ui.label(wait_text);
            let duration_text = settings.apply_text_body_style(
                egui::RichText::new(
                    "This usually takes 10-30 seconds depending on article length.",
                )
                .size(settings.get_font_size() * 0.85),
            );
            ui.label(duration_text);

            ui.add_space(20.0);
            if ui.button("Cancel").clicked() {
                // TODO: Handle cancellation
            }
        });
    }

    fn draw_content(
        &mut self,
        ui: &mut egui::Ui,
        content_type: &ContentType,
        settings: &UISettings,
    ) -> Vec<UIEvent> {
        match content_type {
            ContentType::Article { content, .. } => {
                if self.interactive_mode {
                    self.draw_interactive_article(ui, content, settings)
                } else {
                    self.draw_article(ui, content, settings);
                    Vec::new()
                }
            }
            ContentType::ReadingPassage { content, .. } => {
                if self.interactive_mode {
                    self.draw_interactive_reading_passage(ui, content, settings)
                } else {
                    self.draw_reading_passage(ui, content, settings);
                    Vec::new()
                }
            }
        }
    }

    fn draw_article(&mut self, ui: &mut egui::Ui, article: &Article, settings: &UISettings) {
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                // Article header with enhanced styling
                let title_text = settings.apply_header_style(egui::RichText::new(&article.title));
                ui.heading(title_text);
                ui.add_space(settings.paragraph_spacing);

                // Article metadata
                ui.horizontal(|ui| {
                    let meta_style = |text: String| {
                        settings.apply_text_body_style(egui::RichText::new(text).weak())
                    };

                    ui.label(meta_style(format!(
                        "Subject: {}",
                        article.subject.display_name()
                    )));
                    ui.separator();

                    let formatted_time = article
                        .generated_at
                        .format("%Y-%m-%d %H:%M UTC")
                        .to_string();
                    ui.label(meta_style(format!("Generated: {}", formatted_time)));

                    if settings.show_article_stats {
                        ui.separator();
                        ui.label(meta_style(format!("Words: {}", article.word_count)));
                        ui.separator();
                        ui.label(meta_style(format!(
                            "Read time: {}m",
                            article.estimated_read_time
                        )));
                    }
                });

                ui.separator();
                ui.add_space(settings.paragraph_spacing * 2.0);

                // Article content with enhanced EasyMark formatting
                self.markdown_renderer
                    .render_with_settings(ui, &article.content, settings);
            });
    }

    fn draw_interactive_article(
        &mut self,
        ui: &mut egui::Ui,
        article: &Article,
        settings: &UISettings,
    ) -> Vec<UIEvent> {
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                // Article header (non-interactive)
                let title_text = settings.apply_header_style(egui::RichText::new(&article.title));
                ui.heading(title_text);
                ui.add_space(settings.paragraph_spacing);

                // Article metadata (non-interactive)
                ui.horizontal(|ui| {
                    let meta_style = |text: String| {
                        settings.apply_text_body_style(egui::RichText::new(text).weak())
                    };

                    ui.label(meta_style(format!(
                        "Subject: {}",
                        article.subject.display_name()
                    )));
                    ui.separator();

                    let formatted_time = article
                        .generated_at
                        .format("%Y-%m-%d %H:%M UTC")
                        .to_string();
                    ui.label(meta_style(format!("Generated: {}", formatted_time)));

                    if settings.show_article_stats {
                        ui.separator();
                        ui.label(meta_style(format!("Words: {}", article.word_count)));
                        ui.separator();
                        ui.label(meta_style(format!(
                            "Read time: {}m",
                            article.estimated_read_time
                        )));
                    }
                });

                ui.separator();
                ui.add_space(settings.paragraph_spacing * 2.0);

                // Mode toggle
                ui.horizontal(|ui| {
                    ui.label("Text Mode:");
                    if ui
                        .radio(self.interactive_mode, "Interactive (with text selection)")
                        .clicked()
                    {
                        self.interactive_mode = true;
                    }
                    if ui
                        .radio(!self.interactive_mode, "Static (read-only)")
                        .clicked()
                    {
                        self.interactive_mode = false;
                    }
                });

                ui.add_space(settings.paragraph_spacing);

                // Interactive article content
                self.interactive_renderer
                    .render_with_settings(ui, &article.content, settings)
            })
            .inner
    }

    fn draw_reading_passage(
        &mut self,
        ui: &mut egui::Ui,
        passage: &crate::types::reading_passage::ReadingPassage,
        settings: &UISettings,
    ) {
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                // Passage header with enhanced styling
                let title_text = settings.apply_header_style(egui::RichText::new(&passage.title));
                ui.heading(title_text);
                ui.add_space(settings.paragraph_spacing);

                // Passage metadata
                ui.horizontal(|ui| {
                    let meta_style = |text: String| {
                        settings.apply_text_body_style(egui::RichText::new(text).weak())
                    };

                    ui.label(meta_style(format!(
                        "Subject: {:?}",
                        passage.subject_category
                    )));
                    ui.separator();
                    ui.label(meta_style(format!(
                        "Difficulty: {:?}",
                        passage.difficulty_level
                    )));
                    ui.separator();

                    if settings.show_article_stats {
                        ui.label(meta_style(format!("Words: {}", passage.word_count)));
                        ui.separator();
                        ui.label(meta_style(format!(
                            "Questions: {}",
                            passage.questions.len()
                        )));
                    }
                });

                ui.separator();
                ui.add_space(settings.paragraph_spacing * 2.0);

                // Passage content with enhanced EasyMark formatting
                self.markdown_renderer
                    .render_with_settings(ui, &passage.content, settings);
            });
    }

    fn draw_interactive_reading_passage(
        &mut self,
        ui: &mut egui::Ui,
        passage: &crate::types::reading_passage::ReadingPassage,
        settings: &UISettings,
    ) -> Vec<UIEvent> {
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                // Passage header (non-interactive)
                let title_text = settings.apply_header_style(egui::RichText::new(&passage.title));
                ui.heading(title_text);
                ui.add_space(settings.paragraph_spacing);

                // Passage metadata (non-interactive)
                ui.horizontal(|ui| {
                    let meta_style = |text: String| {
                        settings.apply_text_body_style(egui::RichText::new(text).weak())
                    };

                    ui.label(meta_style(format!(
                        "Subject: {:?}",
                        passage.subject_category
                    )));
                    ui.separator();
                    ui.label(meta_style(format!(
                        "Difficulty: {:?}",
                        passage.difficulty_level
                    )));
                    ui.separator();

                    if settings.show_article_stats {
                        ui.label(meta_style(format!("Words: {}", passage.word_count)));
                        ui.separator();
                        ui.label(meta_style(format!(
                            "Questions: {}",
                            passage.questions.len()
                        )));
                    }
                });

                ui.separator();
                ui.add_space(settings.paragraph_spacing * 2.0);

                // Mode toggle
                ui.horizontal(|ui| {
                    ui.label("Text Mode:");
                    if ui
                        .radio(self.interactive_mode, "Interactive (with text selection)")
                        .clicked()
                    {
                        self.interactive_mode = true;
                    }
                    if ui
                        .radio(!self.interactive_mode, "Static (read-only)")
                        .clicked()
                    {
                        self.interactive_mode = false;
                    }
                });

                ui.add_space(settings.paragraph_spacing);

                // Interactive passage content
                self.interactive_renderer
                    .render_with_settings(ui, &passage.content, settings)
            })
            .inner
    }

    fn draw_error_screen(
        &self,
        ui: &mut egui::Ui,
        error: &crate::types::AppError,
        settings: &UISettings,
    ) {
        ui.vertical_centered(|ui| {
            ui.add_space(50.0);
            let error_title = settings.apply_header_style(egui::RichText::new("Error").strong());
            ui.heading(error_title);
            ui.add_space(20.0);
            ui.separator();
            ui.add_space(20.0);

            ui.colored_label(egui::Color32::RED, "Failed to generate article:");
            ui.add_space(10.0);

            // Better error display with wrapping
            ui.horizontal_wrapped(|ui| {
                let error_text =
                    settings.apply_text_body_style(egui::RichText::new(&error.to_string()));
                ui.label(error_text);
            });

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                if ui.button("Try Again").clicked() {
                    // TODO: Handle retry
                }

                if ui.button("Check Settings").clicked() {
                    // TODO: Handle settings
                }
            });
        });
    }

    // Helper method to get current interactive mode state
    pub fn is_interactive_mode(&self) -> bool {
        self.interactive_mode
    }

    // Method to toggle between interactive and static modes
    pub fn toggle_interactive_mode(&mut self) {
        self.interactive_mode = !self.interactive_mode;
    }
}

impl Default for ArticleViewer {
    fn default() -> Self {
        Self::new()
    }
}
````

## File: src/ui/components/mod.rs
````rust
// file: src/ui/components/mod.rs

pub mod article_viewer;
pub mod sidebar;
pub mod status_bar;
pub mod text_toolbar;
pub mod toolbar;

pub use article_viewer::ArticleViewer;
pub use sidebar::Sidebar;
pub use status_bar::StatusBar;
pub use text_toolbar::TextToolbar;
pub use toolbar::Toolbar;
````

## File: src/ui/rendering/mod.rs
````rust
// file: src/ui/rendering/mod.rs

pub mod markdown;
pub mod markdown_interactive;
pub mod themes;
````
