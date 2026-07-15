# litv - Python Package Manager CLI (Rust)

A simplified pip-like CLI tool written in Rust. It manages Python dependencies with `pip`, `requirements.txt`, and virtual environments created by Python.

## Commands

```bash
cargo run -- add <package> [...]   # Install with pip and add to requirements.txt
cargo run -- add                   # Install all packages from requirements.txt
cargo run -- remove <package> [...] # Uninstall with pip and remove requirements
cargo run -- init                  # Initialize project structure
cargo run -- run                   # Run src/main.py using venv Python
cargo run -- run <script>          # Run specified script using venv Python
cargo run -- run -- <argument>     # Run python program and passthrough arguments
cargo run -- venv                  # Create virtual environment
cargo run -- venv <version>        # Create virtual environment with specific Python version
```

## Key Files

- `src/cli.rs` - CLI argument definitions using clap
- `src/main.rs` - Entry point, parses args and calls commands
- `src/commands/mod.rs` - Re-exports all command modules
- `src/commands/add.rs` - pip installation and requirements.txt updates
- `src/commands/remove.rs` - pip uninstallation and requirements.txt updates
- `src/commands/init.rs` - Project structure initialization
- `src/commands/run.rs` - Python script execution using venv
- `src/commands/venv.rs` - Virtual environment creation with `python -m venv .venv`

## Dependencies (Cargo.toml)

- `clap` - CLI argument parsing
- `colored` - Terminal colors and styling

## Implementation Details

### Virtual Environment
- Creates `.venv` in project root
- Uses `python -m venv .venv`
- `add`, `remove`, and `run` create it if it is missing

### Add workflow
1. Ensures `.venv` exists.
2. Runs `.venv`'s `python -m pip install` with every supplied package argument.
3. Writes the direct supplied requirements to `requirements.txt` after pip succeeds.
4. With no package arguments, runs `pip install -r requirements.txt`.

### Remove workflow
1. Ensures `.venv` exists.
2. Runs `.venv`'s `python -m pip uninstall -y` with every supplied package argument.
3. Removes matching direct requirements case-insensitively from `requirements.txt`.

### Init workflow
1. Creates directory structure (src/)
2. Creates README.md, .gitignore, requirements.txt, src/main.py
3. Uses template content for each file

### Run workflow
1. Checks if .venv exists, creates if not
2. Executes venv Python with src/main.py
3. Shows stdout/stderr from the script

### Venv workflow
1. Runs `python -m venv .venv`.

## Build & Run

```bash
cargo build             # Debug build
cargo run add flask     # Install flask and add it to requirements.txt
cargo run add           # Install all dependencies from requirements.txt
cargo run remove flask  # Remove flask from project and venv
```
