# AGENTS.md

## Build, Lint, and Test Commands

- Build: `cargo build`
- Run: `cargo run`
- Test: `cargo test`
- Run single test: `cargo test <test_name>`
- Lint: `cargo clippy`
- Format: `dprint fmt`

## Code Style Guidelines

- Use Rust naming conventions (snake_case for functions, camelCase for types)
- Prefer `anyhow` and `thiserror` for error handling
- Use `tracing` for logging
- Format with dprint using the provided configuration
- Type annotations required for function parameters and return types

## Cursor/Copilot Rules

- Formatting rules defined in dprint.json
- TypeScript/JavaScript formatting with 2-space indents
- JSON and TOML formatting rules included
