# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust workspace project called "bigly" that serves as a starter template. The project demonstrates modular Rust architecture with separate crates for different concerns:

- **Main binary** (`src/main.rs`): Entry point with panic handling, logging setup, and CLI delegation
- **CLI crate** (`cli/`): Command-line interface using clap for argument parsing
- **Core crate** (`core/`): Business logic with commands for hazard generation, error simulation, and config display
- **Utils crate** (`utils/`): Shared utilities for configuration, logging, and error handling

## Common Commands

The project uses `just` as a command runner. Key commands from the justfile:

```bash
# Build and test
cargo build                 # Build the project
just run-tests              # Run all tests
just run-test <TEST_NAME>   # Run a specific test
just debug <TEST_NAME>      # Run test with debug features

# Code quality
just lint                   # Run clippy linting
cargo fmt                   # Format code (uses rustfmt.toml config)

# Performance and analysis
just bench                  # Run benchmarks
just graph                  # Generate dependency graph (requires graphviz)

# Cleanup
just clean                  # Clean build artifacts and temp files

# Docker
just docker                 # Build Docker image
```

## Architecture

### Workspace Structure
- Root `Cargo.toml` defines a workspace with three member crates
- Each crate has its own `Cargo.toml` and can be developed independently
- Dependencies flow: main → cli → core, with utils as a shared dependency

### Configuration System
- Uses TOML configuration files with a default config embedded in the binary
- Configuration can be overridden via CLI `--config` flag
- Default config is in `src/resources/default_config.toml`

### Error Handling
- Custom error types defined in `utils/src/error.rs`
- Uses `Result<T>` return types throughout the codebase
- Panic handling differs between debug/release builds (better-panic vs human-panic)

### Logging
- Uses `slog` for structured logging with `log` macro compatibility
- Logger setup in `utils/src/logger.rs`
- Global logger initialized in main.rs

### CLI Design
- Built with clap v3 syntax
- Subcommands: `hazard`, `error`, `config`
- Global `--config` option for custom configuration files
- ArgRequiredElseHelp setting shows help when no args provided

## Development Notes

- Project uses Rust 2018 edition
- Optimized profiles for different build types (dev, release, test, bench)
- Integration tests in `tests/` directory
- Benchmarks in `core/benches/`
- Docker support with multi-stage builds