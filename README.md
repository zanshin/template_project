# Rust Project Template

A batteries-included starter template for Rust CLI projects with logging, error handling, and testing infrastructure.

## What's Included

- **Error handling** — `thiserror`-based error types in `src/error.rs`
- **Structured logging** — Terminal and file logging via `simplelog` in `src/messages.rs`
- **Library + Binary structure** — Clean separation between `src/lib.rs` and `src/main.rs`
- **Integration tests** — Test harness in `tests/integration_test.rs`
- **Optimized release builds** — LTO, single codegen-unit, stripped binaries

## Getting Started

1. Copy this template into your new project directory
2. Complete the setup checklist in `CLAUDE.md`:
   - Rename `name` in `Cargo.toml`
   - Update `use template_project::` references in `src/main.rs`
   - Replace placeholder error variants in `src/error.rs`
   - Replace placeholder log calls in `src/main.rs`
   - Update this README with your project details

## Development

```bash
# Run with logging
cargo run

# Run tests
cargo test

# Lint and format
cargo clippy
cargo fmt
```

## Architecture

- `src/lib.rs` — Public API surface; re-exports `Error` and `Result`
- `src/error.rs` — Application error types
- `src/messages.rs` — Logging initialization
- `src/main.rs` — Binary entry point
- `tests/integration_test.rs` — Integration test suite

## License

[LICENSE](LICENSE)
