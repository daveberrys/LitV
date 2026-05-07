use std::process::Command;
use std::path::Path;
use std::error::Error;
use colored::Colorize;
use super::venv;

pub fn run(_path: &str) -> Result<(), Box<dyn Error>> {
    if !Path::new(".venv").is_dir() {
        venv::run("")?;
    }
    start_app()?;

    Ok(())
}

fn start_app() -> Result<(), Box<dyn Error>> {
    println!("{}", "Starting application...".bright_black());

    if cfg!(target_os = "windows") {
        let mut cmd = Command::new(".venv\\Scripts\\python.exe");
        cmd.arg("src\\main.py");
        cmd.status()?;
    } else {
        let mut cmd = Command::new(".venv/bin/python");
        cmd.arg("src/main.py");
        cmd.status()?;
    }

    Ok(())
}