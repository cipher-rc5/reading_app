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
