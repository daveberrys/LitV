use super::venv;
use colored::Colorize;
use std::error::Error;
use std::process::Command;

pub fn run(path: &str, args: Vec<String>) -> Result<(), Box<dyn Error>> {
    venv::ensure()?;

    if path.is_empty() {
        start_app(None, args)?;
    } else {
        start_app(Some(path), args)?;
    }

    Ok(())
}

fn start_app(path: Option<&str>, args: Vec<String>) -> Result<(), Box<dyn Error>> {
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

    let status = Command::new(venv::python_path())
        .arg(python_path)
        .args(args)
        .status()?;
    if !status.success() {
        return Err(format!("Python script failed with status: {status}").into());
    }

    Ok(())
}
