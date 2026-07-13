use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;

use colored::Colorize;

use super::venv;

const REQUIREMENTS_FILE: &str = "requirements.txt";

/// Installs the supplied requirements with the project's virtual environment and
/// records direct requirements in requirements.txt.
pub fn run(packages: &[String]) -> Result<(), Box<dyn Error>> {
    venv::ensure()?;

    if packages.is_empty() {
        let requirements = Path::new(REQUIREMENTS_FILE);
        if !requirements.exists() {
            println!("{} {}", "No requirements.txt found. Add packages with:".yellow(), "litv add <package>".green().bold());
            return Ok(());
        }

        run_pip(["install", "-r", REQUIREMENTS_FILE])?;
        println!("{}", "Installation complete!".green().bold());
        return Ok(());
    }

    run_pip(std::iter::once("install").chain(packages.iter().map(String::as_str)))?;
    add_to_requirements(Path::new(REQUIREMENTS_FILE), packages)?;
    println!("{}", "Installation complete!".green().bold());
    Ok(())
}

fn run_pip<'a>(args: impl IntoIterator<Item = &'a str>) -> Result<(), Box<dyn Error>> {
    let status = Command::new(venv::python_path())
        .arg("-m")
        .arg("pip")
        .args(args)
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("pip failed with status: {status}").into())
    }
}

fn add_to_requirements(path: &Path, packages: &[String]) -> Result<(), Box<dyn Error>> {
    let mut requirements = if path.exists() {
        fs::read_to_string(path)?
    } else {
        String::new()
    };

    for package in packages {
        if !requirements.lines().any(|line| line.trim() == package) {
            if !requirements.is_empty() && !requirements.ends_with('\n') {
                requirements.push('\n');
            }
            requirements.push_str(package);
            requirements.push('\n');
        }
    }

    fs::write(path, requirements)?;
    println!("{} {}", "Updated".bright_black(), REQUIREMENTS_FILE.bold());
    Ok(())
}
