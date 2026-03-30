# Repository Guidelines

## Project Structure & Module Organization
`src/` contains the Rust crate. `main.rs` is the thin binary entry point; core behavior lives in modules such as `cli.rs`, `config.rs`, `system.rs`, `template.rs`, and `fs/` for filesystem parsing helpers. `templates/` holds the built-in MOTD templates (`motd.json.tmpl`, `motd.md.tmpl`) compiled into the binary with `include_str!`. `test/` stores fixture data such as `pacman.log`. Build output goes to `target/` and should not be committed.

## Build, Test, and Development Commands
Use Cargo directly during development:

- `cargo build` builds the crate in debug mode.
- `cargo run -- json` runs the CLI locally with the `json` template.
- `cargo test` runs unit tests.
- `cargo +nightly fmt` applies repository formatting. The checked-in
  `rustfmt.toml` uses nightly-only options, so plain `cargo fmt` emits
  warnings and does not fully honor repository formatting settings.
- `cargo clippy --all-targets --all-features` checks for lint issues before a PR.

For local installation, `make install` builds `target/release/markotd` and copies it to `~/.bin/markotd`. `make clean` removes build artifacts.

## Coding Style & Naming Conventions
This project uses Rust 2024 edition. Follow `rustfmt.toml` with the
nightly formatter: 80-column width, grouped/reordered imports, and field
init shorthand where possible. Use `snake_case` for functions, modules,
and files; use `UpperCamelCase` for structs and enums. Keep `main.rs`
minimal and place reusable logic in `lib.rs` modules. Prefer explicit
error propagation with `color-eyre::Result`.

## Testing Guidelines
Unit tests currently live next to implementation code, for example in `src/fs/filesystem.rs`. Add focused tests beside the module you change unless a separate fixture in `test/` makes more sense. Name tests by behavior, such as `test_filesystem_mount_spaces`. Run `cargo test` before opening a PR and note any platform-specific failures clearly.

## Commit & Pull Request Guidelines
Recent history favors short, imperative commit subjects such as `Update deps` and `New template system`. Keep commits scoped and readable; avoid bundling refactors with behavior changes. PRs should explain the user-visible change, list validation steps, and link any related issue. Include sample output when changing templates or CLI behavior.

## Configuration Tips
Runtime behavior is driven by environment variables like `NOTIFY_UPDATE_HOURS`, `DUR_DIV`, `DF_WHITELIST`, and `DF_BLACKLIST`. When changing configuration handling, update both `README.md` and the template context in `src/template.rs` if exposed fields change.
