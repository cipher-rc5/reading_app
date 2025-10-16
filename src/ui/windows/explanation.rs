// file: src/ui/windows/explanation.rs
// description: Text explanation window using Groq API with environment variable reloading

use egui;
use serde_json::{Value, json};
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
        if let Some(result) = self
            .explanation_receiver
            .as_ref()
            .and_then(|receiver| receiver.try_recv().ok())
        {
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
                        && ui.button("Explain Again").clicked()
                    {
                        self.explain_text(self.selected_text.clone(), self.context.clone());
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
