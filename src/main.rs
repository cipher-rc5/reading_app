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
