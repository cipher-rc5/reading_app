// file: src/app/application.rs
// description: app with proper database initialization and window integration

use crate::{
    app::runtime::AppRuntime,
    services::{ArticleService, DatabaseService, SettingsService},
    types::{Article, ContentType, RequestStatus, UISettings},
    ui::{components::*, events::UIEvent, windows::*},
    utils::fonts::FontRegistry,
};
use eframe::egui;
use tracing::{error, info};

pub struct AppInitialization {
    pub runtime: AppRuntime,
    pub article_service: ArticleService,
    pub database_service: DatabaseService,
    pub settings_service: SettingsService,
    pub font_registry: FontRegistry,
    pub ui_settings: UISettings,
    pub recent_articles: Vec<Article>,
}

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
    runtime: AppRuntime,

    // Enhanced UI state
    ui_settings: UISettings,
    settings_changed: bool,

    // New UI state
    sidebar_collapsed: bool,
    recent_articles: Vec<Article>,
    selected_article: Option<Article>,
    bookmarked_articles: Vec<Article>,
    show_bookmarks: bool,

    // Font management
    font_registry: FontRegistry,
}

impl App {
    pub fn new(init: AppInitialization) -> Self {
        info!("Initializing application with explicit dependencies");

        let AppInitialization {
            runtime,
            article_service,
            database_service,
            settings_service,
            font_registry,
            ui_settings,
            recent_articles,
        } = init;

        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

        Self {
            article_service,
            database_service,
            settings_service,
            sidebar: Sidebar::new(),
            article_viewer: ArticleViewer::new(),
            toolbar: Toolbar::new(),
            status_bar: StatusBar::new(),
            settings_window: SettingsWindow::new(),
            search_window: SearchWindow::new(),
            debug_window: DebugWindow::new(),
            definition_window: DefinitionWindow::new(),
            explanation_window: ExplanationWindow::new(),
            current_status: RequestStatus::Idle,
            message_receiver: Some(rx),
            message_sender: Some(tx),
            runtime,
            ui_settings,
            settings_changed: false,
            sidebar_collapsed: false,
            recent_articles,
            selected_article: None,
            bookmarked_articles: Vec::new(),
            show_bookmarks: false,
            font_registry,
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
                self.current_status = RequestStatus::Success(Box::new(content_type));
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
                self.current_status = RequestStatus::Success(Box::new(content_type));
            }
            UIEvent::DeleteArticle(title) => {
                // Remove from recent articles
                self.recent_articles.retain(|a| a.title != title);
                // Remove from bookmarks
                self.bookmarked_articles.retain(|a| a.title != title);
                // If the deleted article was selected, clear selection
                if self
                    .selected_article
                    .as_ref()
                    .is_some_and(|selected| selected.title == title)
                {
                    self.selected_article = None;
                    self.current_status = RequestStatus::Idle;
                }
            }
            UIEvent::SearchQuery(query) => {
                let database_service = self.database_service.clone();
                let tx = self.message_sender.as_ref().unwrap().clone();

                self.runtime.spawn(async move {
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
                    self.current_status = RequestStatus::Success(Box::new(content_type));
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

                self.runtime.spawn(async move {
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
            // Ignore unimplemented events until their workflows are available
            _ => {}
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
            self.runtime.spawn(async move {
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
                            &self.font_registry,
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
        let settings_events =
            self.settings_window
                .draw(ctx, &mut self.settings_service, &self.font_registry);
        for event in settings_events {
            self.handle_ui_event(event, ctx);
        }

        let search_events = self.search_window.draw(ctx);
        for event in search_events {
            self.handle_ui_event(event, ctx);
        }
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
        let fonts = &self.font_registry;

        // Article header
        ui.horizontal(|ui| {
            let title_text = self
                .ui_settings
                .apply_header_style(fonts, egui::RichText::new(&article.title));
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
                    .apply_text_body_style(fonts, egui::RichText::new(text).weak())
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
                markdown_renderer.render_with_settings(
                    ui,
                    &article.content,
                    &self.ui_settings,
                    fonts,
                );

                // Bottom padding
                ui.add_space(40.0);
            });

        events
    }
}
