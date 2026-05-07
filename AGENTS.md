# litv - Python Package Manager CLI (Rust)

A simplified pip-like CLI tool written in Rust. Manages Python dependencies, virtual environments, and package installation from PyPI.

## Commands

```bash
cargo run -- add <package>         # Add package to pyproject.toml (no install)
cargo run -- add <package> -i      # Add package AND install immediately
cargo run -- add                   # Install all packages from pyproject.toml
cargo run -- remove <package>      # Remove package from pyproject.toml and venv
cargo run -- init                  # Initialize project structure
cargo run -- run                   # Run src/main.py using venv Python
cargo run -- run <script>          # Run specified script using venv Python
cargo run -- run -- <argument>     # Run python program and passthrough arguments
cargo run -- venv                  # Create virtual environment
```

## Key Files

- `src/cli.rs` - CLI argument definitions using clap
- `src/main.rs` - Entry point, parses args and calls commands
- `src/commands/mod.rs` - Re-exports all command modules
- `src/commands/add.rs` - Package fetching, installation, and extras handling
- `src/commands/remove.rs` - Package removal from pyproject.toml and venv
- `src/commands/init.rs` - Project structure initialization
- `src/commands/run.rs` - Python script execution using venv
- `src/commands/venv.rs` - Virtual environment creation (Windows junctions)

## Dependencies (Cargo.toml)

- `reqwest` - HTTP client for PyPI API calls
- `zip` - Extract wheel files
- `toml` - Parse/write pyproject.toml
- `clap` - CLI argument parsing
- `colored` - Terminal colors and styling

## Data Structures

### PyPiResponse (from PyPI API)
- `info` - PackageInfo (name, version, requires_dist)
- `urls` - Vec<PackageUrl> (wheel download URLs)

### PyProjectToml
- `litv` - Project section with name, version, description, python_version, dependencies

## Implementation Details

### Virtual Environment
- Creates `.venv` in project root
- Windows: Uses junction links to Python's Lib/Scripts directories for efficiency
- Falls back to copying if junctions fail
- Platform path: `.venv/Lib/site-packages` (Windows), `.venv/lib/python3.x/site-packages` (Unix)

### pyproject.toml format
```toml
[litv]
name = "project-name"
version = "0.1.0"
dependencies = ["flask==3.1.3", "requests==2.33.1"]
```

### Add workflow (without -i)
1. Validates package name
2. Adds package + version to pyproject.toml
3. Does NOT install - user runs `litv add` to install

### Add workflow (with -i)
1. Validates package name
2. Fetches package from PyPI JSON API
3. Checks for extras (e.g., `package[extra]`) and installs those deps too
4. Fetches `requires_dist` for both main package and extras
5. Installs main package AND all dependencies to venv site-packages
6. Adds everything to pyproject.toml
7. Prints "Installation complete!"

### Remove workflow
1. Reads pyproject.toml dependencies
2. Filters out the package (case-insensitive)
3. Writes updated dependencies back to pyproject.toml
4. Deletes package folder and `.dist-info` from venv site-packages

### Init workflow
1. Creates directory structure (src/)
2. Creates README.md, .gitignore, pyproject.toml, src/main.py
3. Uses template content for each file

### Run workflow
1. Checks if .venv exists, creates if not
2. Executes venv Python with src/main.py
3. Shows stdout/stderr from the script

### Venv workflow (Windows only)
1. Detects latest Python version using `py -0`
2. Creates .venv directory
3. Creates junction links to Python's Lib and Scripts (fast) or copies (fallback)
4. Writes pyvenv.cfg configuration
5. Copies python.exe to .venv/Scripts/

## Build & Run

```bash
cargo build              # Debug build
cargo run add flask     # Add flask to pyproject.toml (no install)
cargo run add flask -i   # Add and install flask immediately
cargo run add           # Install all deps from pyproject.toml
cargo run remove flask # Remove flask from project and venv
```