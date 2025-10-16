# reading_app - README

Sophisticated desktop reading application built with Rust, featuring AI-powered content generation, interactive reading experiences, and comprehensive learning analytics. This application leverages the unique capabilities of gpt-oss-20b via Groq's API for optimal educational content creation.

![Reading App Interface](https://github.com/cipher-rc5/reading_app/blob/master/docs/app_usage.png)

## Key Features

### AI-Powered Content Generation

- Dynamic Article Generation: Create educational articles on any topic using advanced AI
- Reading Passage Creation: Generate structured reading comprehension passages with questions
- Adaptive Difficulty Levels: Content tailored to different skill levels (Foundation to Elite)
- Multi-Subject Support: Coverage across Sciences, Humanities, Social Sciences, and more

### Rich User Interface

- Modern GUI: Built with egui for native desktop experience
- Responsive Design: Adaptive layout with collapsible sidebar
- Theme Support: Customizable UI themes and font settings
- Interactive Features: Word definitions, text explanations, and contextual help

### Learning Analytics

- Reading Progress Tracking: Monitor reading time and progress
- Performance Analytics: Track comprehension and learning patterns
- Bookmark Management: Save and organize favorite content
- Search Functionality: Full-text search across all content

### Data Management

- Local Database: SQLite-based storage with libSQL
- Content Persistence: Save articles and reading passages locally
- Settings Management: Persistent UI preferences and configurations
- Export Capabilities: Download content as markdown files

## Architecture Overview

### Core Technologies

- **Language**: Rust 2021 Edition
- **GUI Framework**: egui + eframe (native desktop)
- **Database**: libSQL (SQLite-compatible)
- **AI Integration**: Groq API with gpt-oss-20b
- **Async Runtime**: Tokio for concurrent operations
- **HTTP Client**: reqwest for API communication

### Directory Structure

```
src/
├── app/                            # Main application logic
│   ├── app.rs                      # Core application state and UI
│   ├── mod.rs                      # App module exports
│   └── runtime.rs                  # Global async runtime management
├── client/                         # External API integrations
│   ├── groq_client.rs              # Groq API client (gpt-oss-20b)
│   ├── config.rs                   # Client configuration
│   ├── mod.rs                      # Client module exports
│   └── retry.rs                    # Request retry logic
├── config/                         # Application configuration
│   ├── app_config.rs               # Main configuration structure
│   ├── environment.rs              # Environment variable handling
│   └── mod.rs                      # Config module exports
├── database/                       # Data persistence layer
│   ├── connection.rs               # Database connection management
│   ├── models.rs                   # Data models and schemas
│   ├── schema.rs                   # Database schema definitions
│   ├── mod.rs                      # Database module exports
│   └── repositories/               # Data access objects
│       ├── article_repository.rs
│       ├── reading_history_repository.rs
│       ├── settings_repository.rs
│       └── mod.rs
├── services/                       # Business logic layer
│   ├── article_service.rs          # Article generation and management
│   ├── database_service.rs         # Database operations
│   ├── search_service.rs           # Search functionality
│   ├── settings_service.rs         # Settings management
│   └── mod.rs                      # Services module exports
├── types/                          # Type definitions and domain models
│   ├── article.rs                  # Article data structures
│   ├── errors.rs                   # Error types and handling
│   ├── reading_passage.rs          # Reading passage structures
│   ├── settings.rs                 # UI settings and preferences
│   ├── time_utils.rs               # Time and date utilities
│   ├── validation.rs               # Input validation logic
│   └── mod.rs                      # Types module exports
├── ui/                             # User interface components
│   ├── components/                 # Reusable UI components
│   │   ├── article_viewer.rs
│   │   ├── sidebar.rs
│   │   ├── status_bar.rs
│   │   ├── text_toolbar.rs
│   │   ├── toolbar.rs
│   │   └── mod.rs
│   ├── rendering/                  # Content rendering
│   │   ├── markdown.rs             # Markdown rendering
│   │   ├── themes.rs               # UI themes and styling
│   │   └── mod.rs
│   ├── windows/                    # Modal windows and dialogs
│   │   ├── debug.rs                # Debug information window
│   │   ├── definition.rs           # Word definition lookup
│   │   ├── explanation.rs          # Text explanation window
│   │   ├── search.rs               # Search interface
│   │   ├── settings.rs             # Settings configuration
│   │   └── mod.rs
│   ├── events.rs                   # UI event definitions
│   └── mod.rs                      # UI module exports
├── utils/                          # Utility functions
│   ├── fonts.rs                    # Font management
│   ├── logging.rs                  # Logging configuration
│   └── mod.rs                      # Utils module exports
├── lib.rs                          # Library root and public exports
└── main.rs                         # Application entry point
```

## Installation & Setup

### Prerequisites

- **Rust**: 1.70+ (2021 edition)
- **System Dependencies**:
  - SQLite development libraries
  - OpenSSL development libraries

### Quick Start

1. **Clone the repository**
   ```bash
   git clone https://github.com/cipher-rc5/reading_app.git
   cd reading_app
   ```

2. **Install dependencies**
   ```bash
   cargo build --release
   ```

3. **Configure environment**
   ```bash
   cp .env.example .env
   ```
   Edit `.env` with your Groq API key

4. **Run the application**
   ```bash
   cargo run
   ```

> **Note**: This application currently employs Groq as an inference provider, please visit their [website](https://groq.com) for API-Keys

### Configuration

Create a `.env` file in the project root:

```bash
# Required: Groq API Configuration
GROQ_API_KEY=your_groq_api_key_here
GROQ_BASE_URL=https://api.groq.com/openai/v1

# Optional: Logging Configuration
RUST_LOG=info
```

## AI Integration: gpt-oss-20b via Groq

### Why gpt-oss-20b?

This application leverages gpt-oss-20b, a cutting-edge open-weight Mixture of Experts (MoE) model, through Groq's optimized inference platform. This choice provides several unique advantages for educational applications:

#### Tool-Calling Capabilities

- **Structured Content Generation**: Advanced JSON mode for consistent, parseable educational content
- **Question Generation**: Sophisticated comprehension question creation with multiple choice options
- **Metadata Extraction**: Automatic difficulty assessment and learning objective identification
- **Adaptive Content**: Dynamic content adjustment based on user performance and preferences

#### Open-Weight MoE Design Benefits

- **Educational Transparency**: Open-source model allows for community validation and improvement
- **Cost-Effective Scaling**: Efficient inference through expert routing reduces computational costs
- **Specialized Expertise**: Different model experts handle specific educational domains optimally
- **Future-Proof Architecture**: Community-driven improvements and fine-tuning capabilities

#### ⚡ Groq Platform Advantages

- **Ultra-Low Latency**: Optimized for real-time educational interactions
- **High Throughput**: Handles multiple concurrent users efficiently
- **Reliable Infrastructure**: Enterprise-grade API reliability for educational environments
- **Cost Optimization**: Competitive pricing for sustained educational use

### Content Generation Features

#### Article Generation

- **Subject Coverage**: 15+ academic subjects from Science to Literature
- **Quality Assurance**: Outputs are stored to a user's local SQLite database and can readily be reviewed
- **Adaptive Length**: 800-1200 words optimized for reading comprehension
- **Structured Format**: Clear headings, sections, and logical flow

#### Reading Passages

- **Difficulty Levels**: 4-tier system (Foundation → Elite)
- **Lexile Integration**: Standardized readability metrics
- **Question Types**: 8 standardized comprehension question types
- **Progress Tracking**: Individual question performance analytics

#### Educational Standards

- **Common Core Alignment**: Reading comprehension standards compliance
- **Differentiated Instruction**: Multiple difficulty levels for diverse learners
- **Assessment Integration**: Standardized test preparation support
- **Progress Monitoring**: Detailed learning analytics and recommendations

## Usage Guide

### Getting Started

1. **Launch the Application**
   ```bash
   cargo run
   ```

2. **Initial Setup**
   - Configure your Groq API key in settings
   - Adjust UI preferences (theme, fonts, layout)
   - Set up your learning profile

### Core Features

#### Content Generation

- **Generate Articles**: Select subject → Enter custom topic → Generate
- **Create Reading Passages**: Choose difficulty level → Select subject category → Generate
- **Custom Topics**: Override default topics for specific learning objectives

#### Interactive Reading

- **Word Definitions**: Double-click words for instant definitions
- **Text Explanations**: Select text for contextual explanations
- **Bookmark Management**: Save important passages and articles
- **Progress Tracking**: Automatic reading time and progress monitoring

#### Learning Analytics

- **Performance Dashboard**: View reading statistics and trends
- **Skill Assessment**: Track comprehension across different question types
- **Progress Reports**: Detailed analytics on learning patterns
- **Recommendations**: AI-powered suggestions for next content

#### Customization

- **Theme Selection**: Light/Dark themes with custom color schemes
- **Font Management**: Multiple font options with size customization
- **Layout Options**: Adjustable sidebar, window sizing, and spacing
- **Keyboard Shortcuts**: Efficient navigation and content interaction

## Development

### Building from Source

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run
```

### Development Commands

```bash
# Lint code
cargo clippy

# Format code
dprint fmt

# Check compilation
cargo check

# Generate documentation
cargo doc --open

# One line to clean and run
cargo clean && cargo build --release && RUST_LOG=debug cargo run
```

### Key Dependencies

- **egui/eframe**: Native GUI framework
- **tokio**: Async runtime for concurrent operations
- **libsql**: SQLite database with edge replication
- **reqwest**: HTTP client for API communication
- **serde**: Serialization/deserialization
- **tracing**: Structured logging and diagnostics

## Contributing

### Development Setup

1. **Fork and Clone**
   ```bash
   git clone https://github.com/cipher-rc5/reading_app.git
   cd reading_app
   ```

2. **Development Dependencies**
   ```bash
   # Install dprint for code formatting
   # Install cargo tools for development
   cargo install cargo-edit cargo-watch
   ```

3. **Code Standards**
   - Follow Rust 2021 idioms and patterns
   - Use cargo clippy for linting
   - Format code with dprint fmt
   - Write comprehensive tests for new features

### Architecture Guidelines

- **Separation of Concerns**: Clear boundaries between UI, business logic, and data layers
- **Async Best Practices**: Proper error handling and resource management
- **Type Safety**: Leverage Rust's type system for compile-time guarantees
- **Performance**: Optimize for responsive UI and efficient resource usage

### Testing Strategy - still in progress

```bash
# Unit tests
cargo test --lib

# Integration tests
cargo test --test integration

# All tests with coverage
cargo test --all-targets
```

## Roadmap

### Phase 1: Core Features

- [x] AI-powered content generation
- [x] Interactive reading interface
- [x] Local data persistence
- [x] Basic learning analytics

### Phase 2: Enhanced Learning (In Progress)

- [ ] Advanced question types and assessment
- [ ] Collaborative features and sharing
- [ ] Offline content synchronization
- [ ] Multi-language support

### Phase 3: Advanced Analytics

- [ ] Machine learning-based recommendations
- [ ] Detailed learning path generation
- [ ] Integration with learning management systems
- [ ] Advanced progress visualization

## Author

**Author**: ℭ𝔦𝔭𝔥𝔢𝔯
**GitHub**: [cipher-rc5](https://github.com/cipher-rc5)
**Telegram**: cipher0091

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- **OpenAI**: For building and making GPT-OSS an open-weights model for continued development
- **Groq**: For providing access to gpt-oss-20b and their optimized inference platform
- **Rust Community**: For the excellent ecosystem and tooling
- **Educational Technology Community**: For inspiration and best practices

## Support

For support, feature requests, or bug reports:

- **Issues**: [GitHub Issues](https://github.com/cipher-rc5/reading_app/issues)
- **Discussions**: [GitHub Discussions](https://github.com/cipher-rc5/reading_app/discussions)
- **Documentation**: [Wiki](https://github.com/cipher-rc5/reading_app/wiki)

---

Built with [Rust](https://www.rust-lang.org/) and powered by [gpt-oss-20b](https://openai.com/index/gpt-oss-model-card/) via [Groq](https://groq.com/)
