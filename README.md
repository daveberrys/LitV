# LitV [ VERY VERY ALPHA ]
A [`uv`](https://docs.astral.sh/uv/) inspired tool made with rust. A python helper if you will.

## Where do I get it?
- Linux Build: https://nightly.link/daveberrys/LitV/workflows/nightly/main/litv-Linux.zip
- Windows Build: https://nightly.link/daveberrys/LitV/workflows/nightly/main/litv-Windows.zip
- macOS (arm): https://nightly.link/daveberrys/LitV/workflows/nightly/main/litv-macOS-arm.zip
- macOS (intel): https://nightly.link/daveberrys/LitV/workflows/nightly/main/litv-macOS-Intel.zip

## How do I use it?
Since this is a CLI tool, you run it from the terminal. Here are the arguments:
<details>
<summary>Click to see the arguments (it's a long one...)</summary>

- `litv init`
    - Initialize a new project in the folder you're in
    - Extras:
        - `litv run <project_name>`
            - Initializes a new project in a new directory with the given name.
            - `project_name` is optional.
- `litv run`
    - Checks if you have `.venv` directory.'
        - If not, initializes a new virtual environment by doing `litv venv`.
    - Run `src/main.py` through the virtual environment
    - Extras:
        - `litv run <file>`
            - To run a specific file through the virtual environment.
        - `litv run -- <args>`
            - Passthrough arguments to the python script.
        - `litv run <file> <args>`
            - To run a specific file through the virtual environment with passthrough arguments.
- `litv add`
    - Checks:
        - If you have `.venv` directory.
            - If not, initializes a new virtual environment by doing `litv venv`.
        - If `pyproject.toml` exists.
            - If not, it will prompt the user that pyproject.toml does not exist and the user must create one by doing `litv init` or creating themselves.
    - If no other argument is given, downloads every single package from `pyproject.toml`.
    - Extras:
        - `litv add <package>`
            - `<package>` is the name of the package to add to `pyproject.toml`.
        - `litv add <package> -i`
            - `-i` is a optional flag that will install the package and then place the package you installed to `pyproject.toml`.
        - `litv add <package> -b`
            - `-b` is a optional flag that will use `pip` to install the package and then place the package you installed to `pyproject.toml`.
- `litv remove <package>`
    - `<package>` is the name of the package to remove from `pyproject.toml`.
    - If `<package>` is empty, the user will be prompted to add a argument.
    - Extras:
        - `litv remove <package> -b`
            - `-b` is a optional flag that will use `pip` to remove the package and then remove the package you installed to `pyproject.toml`.
- `litv venv`
    - Creates a new virtual environment.
    - Extras:
        - `litv venv --<python_version>`
            - `<python_version>` is the version of Python to use for the virtual environment. (eg: `litv venv --3.13`)

</details>

## Is this LLM Assisted?
Yes. I needed help with most of the code. I'm being fully transparent about this.

######

"This was JUST an excuse for me to learn rust... Probably."
-Daveberry 2026.