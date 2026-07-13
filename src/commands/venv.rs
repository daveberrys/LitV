use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::Command;

use colored::Colorize;

const VENV_DIR: &str = ".venv";

/// Creates or refreshes the project virtual environment using Python's standard
/// library implementation.
pub fn run() -> Result<(), Box<dyn Error>> {
    println!("{}", "Creating virtual environment...".bright_black());
    let (launcher, version) = find_python_launcher()?;
    println!("{} {} ({})", "Using".bright_black(), launcher, version);

    let status = Command::new(launcher)
        .args(["-m", "venv", VENV_DIR])
        .status()?;
    if !status.success() {
        return Err(format!("{launcher} -m venv {VENV_DIR} failed with status: {status}").into());
    }

    println!(
        "{} {}",
        "Virtual environment ready at".green().bold(),
        VENV_DIR
    );
    Ok(())
}

/// Finds the first available Python launcher in LitV's cross-platform order.
/// A launcher is accepted only when `--version` succeeds and produces output.
fn find_python_launcher() -> Result<(&'static str, String), Box<dyn Error>> {
    for launcher in ["py", "python", "python3"] {
        let Ok(output) = Command::new(launcher).arg("--version").output() else {
            continue;
        };

        if !output.status.success() {
            continue;
        }

        let version = String::from_utf8_lossy(&output.stdout).trim().to_owned();
        let version = if version.is_empty() {
            String::from_utf8_lossy(&output.stderr).trim().to_owned()
        } else {
            version
        };

        if !version.is_empty() {
            return Ok((launcher, version));
        }
    }

    Err("Could not find a usable Python launcher. Checked `py`, `python`, and `python3` with `--version`.".into())
}

/// Ensures the project has a virtual environment before invoking its Python or
/// pip executables.
pub fn ensure() -> Result<(), Box<dyn Error>> {
    if !Path::new(VENV_DIR).is_dir() || !python_path().is_file() {
        println!("{}", "No usable .venv found; creating one...".yellow());
        run()?;
    }
    Ok(())
}

pub fn python_path() -> PathBuf {
    if cfg!(windows) {
        PathBuf::from(VENV_DIR).join("Scripts").join("python.exe")
    } else {
        PathBuf::from(VENV_DIR).join("bin").join("python")
    }
}
