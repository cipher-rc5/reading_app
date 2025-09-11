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
