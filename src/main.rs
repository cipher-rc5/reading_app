// file: src/main.rs
// description: Application entry point with environment setup and runtime management

use reading_app::{
    app::{App, AppInitialization, AppRuntime},
    config::AppConfig,
    services::{ArticleService, DatabaseService, SettingsService},
    utils::{fonts::FontRegistry, logging},
};
use std::process;
use tracing::{error, info};

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

    let runtime = match AppRuntime::new() {
        Ok(runtime) => runtime,
        Err(err) => {
            eprintln!("Failed to initialize async runtime: {}", err);
            process::exit(1);
        }
    };

    let font_registry = FontRegistry::new();

    let article_service = ArticleService::new(&config).unwrap_or_else(|err| {
        error!("Failed to initialize article service: {}", err);
        ArticleService::default()
    });

    let database_service = runtime
        .block_on(DatabaseService::new_async(&config))
        .unwrap_or_else(|err| {
            error!("Failed to initialize database service: {}", err);
            DatabaseService::default()
        });

    let settings_service = SettingsService::new(
        database_service.clone(),
        runtime.clone(),
        font_registry.clone(),
    );
    let ui_settings = settings_service.get_ui_settings();

    let recent_articles = runtime
        .block_on(database_service.get_recent_articles(20))
        .unwrap_or_else(|err| {
            error!("Failed to load recent articles: {}", err);
            Vec::new()
        });

    let app_initialization = AppInitialization {
        runtime,
        article_service,
        database_service,
        settings_service,
        font_registry: font_registry.clone(),
        ui_settings,
        recent_articles,
    };

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
            font_registry.configure(cc);
            Ok(Box::new(App::new(app_initialization)))
        }),
    )
}
