use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::Command;

use colored::Colorize;

const VENV_DIR: &str = ".venv";

/// Creates or refreshes the project virtual environment using Python's standard
/// library implementation.
pub fn run() -> Result<(), Box<dyn Error>> {
    println!("{}", "Creating virtual environment...".bright_black());
    let status = Command::new("python")
        .args(["-m", "venv", VENV_DIR])
        .status()?;
    if !status.success() {
        return Err(format!("python -m venv {VENV_DIR} failed with status: {status}").into());
    }
    println!("{} {}", "Virtual environment ready at".green().bold(), VENV_DIR);
    Ok(())
}

/// Ensures the project has a virtual environment before invoking its Python or
/// pip executables.
pub fn ensure() -> Result<(), Box<dyn Error>> {
    if !Path::new(VENV_DIR).is_dir() {
        println!("{}", "No .venv found; creating one...".yellow());
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
