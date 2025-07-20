# Bigly

```
                                                              
                          .-"""""-.
                        .'         '.
                       /   ,-.   ,-. \
                      |   ( o ) ( o ) |
                       \   '-'   '-' /
                        '.  \___/  .'
                          '-.___.-'
                        /           \
                       |    B I G    |
                       |    L Y !    |
                        \___________/
                           |     |
                          /       \
                         /_________\
                        |  ∩-----∩  |
                        | ( o   o ) |
                        |  >  -  <  |
                        | \  ___  / |
                         \ '-----' /
                          '-......-'

         A CLI tool that greps all files under the current directory
```

## Overview

Bigly is a powerful command-line search tool that helps you find text across all files in your current directory and subdirectories. It's designed to be fast, simple, and effective for searching through codebases and file collections.

## Features

- 🔍 **Fast recursive search** - Search through all files in current directory
- 📁 **File-specific targeting** - Target specific files with `--file` option
- 🚫 **Smart filtering** - Automatically skips hidden files, binary files, and build artifacts
- 📋 **Clean output** - Shows filename, line number, and matching line
- ⚡ **Simple usage** - Works with or without quotes around search terms

## Installation

```bash
# Install from source
cargo install --path .

# Or run directly
cargo run -- "search_term"
```

## Usage

```bash
# Search for text in all files
bigly "hello"

# Search in a specific file
bigly --file filename.txt "hello"

# Show help
bigly --help

# Show version
bigly --version
```

### Example Output

```bash
$ bigly "just"
CLAUDE.md:16:The project uses `just` as a command runner. Key commands from the justfile:
CLAUDE.md:21:just run-tests              # Run all tests
justfile:6:run-test TEST:
```

## Additional Commands

Bigly also includes some utility subcommands:

```bash
# Generate a random result
bigly hazard

# Show current configuration
bigly config

# Simulate an error (for testing)
bigly error
```

## Architecture

Bigly is built as a Rust workspace with modular architecture:

- **CLI crate** - Command-line interface and argument parsing
- **Core crate** - Business logic and search functionality  
- **Utils crate** - Shared utilities for configuration and logging

## Development

```bash
# Build the project
cargo build

# Run tests
cargo test --all

# Run linting
cargo clippy

# Format code
cargo fmt
```

## License

MIT License - see LICENSE file for details.

---

*"It's going to be bigly!"* 🚀
