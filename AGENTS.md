# Development Guide for Meditari

## Rules

When creating a shell for a command that needs to be executed in side the stoic repo, always use the stoic_shell function.

## Build & Test Commands
- **Build**: `cargo build`
- **Test all**: `cargo test` or `just test`
- **Test single**: `cargo test <test_name>`
- **Lint**: `just lint` (runs fmt check + clippy with -Dwarnings)
- **Format**: `just fmt` (runs cargo fmt + clippy fix)
- **Verify all**: `just verify` (test_run + test + lint)

## Code Style
- **Edition**: Rust 2024
- **Imports**: Use `use` statements at top, group by std/external/internal
- **Modules**: Public modules that are commands in mod.rs with `pub mod`, re-export with `pub use`
- **Error handling**: Use `.unwrap()` for expected success, `eprintln!` + `exit(1)` for fatal errors
- **Shell commands**: Use xshell `cmd!` macro for external commands
- **Naming**: snake_case for functions/variables, PascalCase for types/enums
- **Constants**: SCREAMING_SNAKE_CASE with explicit type when needed
- **CLI**: Use clap derive API with `#[derive(Parser)]` and `#[command()]` attributes