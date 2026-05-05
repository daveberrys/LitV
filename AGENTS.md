# litv - Python Package Manager CLI (Rust)

A simplified pip-like CLI tool written in Rust. Manages Python dependencies, virtual environments, and package installation from PyPI.

## Commands

```bash
cargo run -- add <package>      # Add package + deps, install to venv
cargo run -- add                # Install all packages from pyproject.toml
cargo run -- remove <package>  # Remove package from pyproject.toml and venv
cargo run -- init              # Initialize project structure
cargo run -- run <script>      # Run Python script using venv Python
```

## Key Files

- `src/cli.rs` - CLI definition using clap
- `src/commands/add.rs` - Package installation logic
- `src/commands/remove.rs` - Package removal logic
- `src/commands/init.rs` - Project initialization
- `src/commands/run.rs` - Python script execution

## Dependencies (Cargo.toml)

- `reqwest` - HTTP client for PyPI API calls
- `zip` - Extract wheel files
- `toml` - Parse/write pyproject.toml
- `clap` - CLI argument parsing

## Implementation Details

1. **Virtual Environment**: Creates `.venv` in project root

2. **pyproject.toml format**:
   ```toml
   [litv]
   dependencies = ["flask==3.1.3", "requests==2.33.1"]
   ```

3. **Add workflow**:
   - Fetches package from `https://pypi.org/pypi/{package}/json`
   - Gets `requires_dist` for dependencies
   - Creates `.venv` if needed
   - Downloads wheel to temp dir, extracts to `site-packages`
   - Adds package + deps to pyproject.toml
   - Installs all deps automatically

4. **Remove workflow**:
   - Removes from `[litv]` dependencies in pyproject.toml
   - Deletes package folder and `.dist-info` from venv

5. **Platform paths**:
   - Windows: `.venv/Lib/site-packages`
   - Unix: `.venv/lib/python3.x/site-packages`

## Build & Run

```bash
cargo build              # Debug build
cargo run add flask      # Add flask
cargo run remove flask   # Remove flask
```