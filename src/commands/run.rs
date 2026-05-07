use std::process::Command;
use std::path::Path;
use std::error::Error;
use colored::Colorize;
use super::venv;

pub fn run(path: &str) -> Result<(), Box<dyn Error>> {
    if !Path::new(".venv").is_dir() {
        venv::run("")?;
    }

    if path.is_empty() {
        start_app(None)?;
    } else {
        start_app(Some(path))?;
    }
    
    Ok(())
}

fn start_app(path: Option<&str>) -> Result<(), Box<dyn Error>> {
    println!("{}", "Starting application...".bright_black());
    
    let python_path;
    match path {
        None => {
            if cfg!(target_os = "windows") {
                python_path = "src\\main.py";
            } else {
                python_path = "src/main.py";
            }
        }
        Some(v) => {
            python_path = v;
        }
    }
    
    if cfg!(target_os = "windows") {
        let mut cmd = Command::new(".venv\\Scripts\\python.exe");
        cmd.arg(python_path);
        cmd.status()?;
    } else {
        let mut cmd = Command::new(".venv/bin/python");
        cmd.arg(python_path);
        cmd.status()?;
    }

    Ok(())
}