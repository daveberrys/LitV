# LitV [ VERY VERY ALPHA ]

A small Rust wrapper around `python -m venv` and `pip` for Python projects. LitV stores direct dependencies in `requirements.txt`.

## Where do I get it?
- Linux Build: https://nightly.link/daveberrys/LitV/workflows/nightly/main/litv-Linux.zip
- Windows Build: https://nightly.link/daveberrys/LitV/workflows/nightly/main/litv-Windows.zip
- macOS (arm): https://nightly.link/daveberrys/LitV/workflows/nightly/main/litv-macOS-arm.zip
- macOS (intel): https://nightly.link/daveberrys/LitV/workflows/nightly/main/litv-macOS-Intel.zip

## How do I use it?

LitV uses the `.venv` directory in the current project. `add`, `remove`, and `run` create it with `python -m venv .venv` when it is missing.

```bash
litv init [path]                 # Create src/, requirements.txt, and project files
litv venv                        # Run python -m venv .venv
litv add requests flask          # pip install requests flask; record both requirements
litv add                         # pip install -r requirements.txt
litv remove requests flask       # pip uninstall -y requests flask; remove both requirements
litv run                         # Run src/main.py with .venv's Python
litv run script.py -- --flag     # Run another script and pass its arguments through
```

Package arguments are passed to pip as separate arguments, so multi-package commands work the same way as `pip install requests flask` and `pip uninstall -y requests flask`.

## Is this LLM Assisted?
Yes. I needed help with most of the code. I'm being fully transparent about this.

######

"This was JUST an excuse for me to learn rust... Probably."
-Daveberry 2026.
