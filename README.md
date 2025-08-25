# meditari

A Rust CLI tool for managing daily notes in a Git repository with stoic practices.

## Prerequisites

- **Ghostty Terminal**: The application currently requires Ghostty terminal to open editors
- **Git**: For repository management and version control
- **GitHub Account**: To store your stoic notes repository

## CLI Usage

```bash
med [OPTIONS] [OWNER] [EDITOR] [COMMAND]
```

### Arguments

- `OWNER`: Optional name to operate on (defaults to git username)
- `EDITOR`: Optional editor command (auto-detected if not provided)

### Commands

#### Default (no command)

Opens your editor to create a new note for today.

```bash
med
med john
med john vim
```

#### `content <CONTENT>`

Create a note with the specified content directly without opening an editor.

```bash
med content "My daily reflection"
med content "Today I learned about Rust ownership"
```

#### `test`

Creates a test file and then resets (removes all markdown files). Useful for testing the workflow.

```bash
med test
```

#### `reset`

Removes all markdown files from the repository and commits the change. Can be reverted with the `revert` command.

```bash
med reset
```

#### `revert`

Reverts the last commit in the repository. Useful for undoing a `reset` operation.

```bash
med revert
```

## How It Works

1. **Repository Management**: The tool clones or accesses a `stoic` repository in your home directory
2. **Daily Notes**: Creates notes organized by date in a `YYYY/MM/DD/` directory structure
3. **File Naming**: Notes are named with timestamp: `YYYY_MM_DD_HH_MM_SS.md`
4. **Auto-commit**: Automatically commits and pushes your notes to the repository
5. **Editor Integration**: Opens your preferred editor via Ghostty terminal (currently requires Ghostty)

## Examples

```bash
# Create a new daily note (opens editor)
med

# Create a note with specific content
med content "Practiced mindfulness today"

# Reset all notes (careful!)
med reset

# Undo the reset
med revert

# Test the workflow
med test
```

---

## Development Commands

### Just Commands

#### `just run [args]`

Run the application with optional arguments.

```bash
just run
just run --help
just run test
```

#### `just verify`

Run the complete verification pipeline including tests and linting.

```bash
just verify
```

#### `just test`

Run the test suite.

```bash
just test
```

#### `just test_run`

Run the application in test mode.

```bash
just test_run
```

#### `just lint`

Run static code analysis including format checking and clippy linting.

```bash
just lint
```

#### `just fmt`

Format code and apply automatic fixes.

```bash
just fmt
```

### Direct Cargo Commands

#### `cargo build`

Build the project.

```bash
cargo build
cargo build --release
```

#### `cargo run [args]`

Run the application directly via cargo.

```bash
cargo run
cargo run -- --help
```

#### `cargo test`

Run tests directly via cargo.

```bash
cargo test
```

#### `cargo fmt`

Format code.

```bash
cargo fmt
```

#### `cargo clippy`

Run clippy linting.

```bash
cargo clippy
```

## Environment Variables

- `RUST_LOG`: Set to "debug" by default when using just commands
