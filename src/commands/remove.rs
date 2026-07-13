use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;

use colored::Colorize;

use super::venv;

const REQUIREMENTS_FILE: &str = "requirements.txt";

/// Uninstalls all requested packages in one pip invocation, then removes their
/// direct requirement entries from requirements.txt.
pub fn run(packages: &[String]) -> Result<(), Box<dyn Error>> {
    if packages.is_empty() {
        return Err("Please specify at least one package to remove".into());
    }

    venv::ensure()?;
    let status = Command::new(venv::python_path())
        .args(["-m", "pip", "uninstall", "-y"])
        .args(packages)
        .status()?;
    if !status.success() {
        return Err(format!("pip uninstall failed with status: {status}").into());
    }

    remove_from_requirements(Path::new(REQUIREMENTS_FILE), packages)?;
    println!("{} Removed packages: {}", "-".red(), packages.join(", "));
    Ok(())
}

fn remove_from_requirements(path: &Path, packages: &[String]) -> Result<(), Box<dyn Error>> {
    if !path.exists() {
        return Ok(());
    }

    let requested: Vec<String> = packages.iter().map(|package| normalized_name(package)).collect();
    let content = fs::read_to_string(path)?;
    let filtered = content
        .lines()
        .filter(|line| !requested.iter().any(|package| normalized_name(line) == *package))
        .collect::<Vec<_>>()
        .join("\n");
    let output = if filtered.is_empty() { filtered } else { format!("{filtered}\n") };
    fs::write(path, output)?;
    println!("{} {}", "Updated".bright_black(), REQUIREMENTS_FILE.bold());
    Ok(())
}

fn normalized_name(requirement: &str) -> String {
    let name = requirement
        .trim()
        .split(|character: char| matches!(character, '[' | '=' | '!' | '<' | '>' | '~' | ';' | ' ' | '\t'))
        .next()
        .unwrap_or_default();
    name.to_ascii_lowercase().replace(['_', '.'], "-")
}
