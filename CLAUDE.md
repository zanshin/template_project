# template_project

My name for this project is **Rowan**.

## Project purpose

Bare-bones Rust starter template. Copy this into a new `cargo new` project and
rename `template_project` throughout to the real project name.

## Checklist when copying this template

- [ ] Rename `name` in `Cargo.toml`
- [ ] Rename `use template_project::` references in `src/main.rs`
- [ ] Update `README.md`
- [ ] Replace placeholder error variants in `src/error.rs`
- [ ] Replace placeholder log calls in `src/main.rs`

## Architecture

- `src/lib.rs` — public API; re-exports `Error` and `Result`
- `src/error.rs` — `thiserror`-derived error enum
- `src/messages.rs` — logging initialisation (terminal + file via `simplelog`)
- `src/main.rs` — thin binary entry point
- `tests/integration_test.rs` — integration tests against the public API

## Conventions

- Follow TDD: write a failing test before adding each new feature.
- `cargo fmt`, `cargo clippy`, and `cargo test` must all pass before committing.
- Never use `--no-verify` when committing.
