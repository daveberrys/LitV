use std::process::Command;
use std::path::Path;
use std::error::Error;
use colored::Colorize;

pub fn run(_path: &str) -> Result<(), Box<dyn Error>> {
    if Path::new(".venv").is_dir() {
        start_app()?;
    } else {
        initialize_venv()?;
    }

    Ok(())
}

fn initialize_venv() -> Result<(), Box<dyn Error>> {
    println!("{}", "Creating virtual environment...".white());

    let mut venv = Command::new("python");
    venv.arg("-m");
    venv.arg("venv");
    venv.arg(".venv");
    venv.status()?;

    if Path::new("requirements.txt").is_file() {
        let mut pip = Command::new("pip");
        pip.arg("install");
        pip.arg("-r");
        pip.arg("requirements.txt");
        let _ = pip.status();
    }

    start_app()
}

fn start_app() -> Result<(), Box<dyn Error>> {
    println!("{}", "Starting application...".white());

    let python = ".venv\\Scripts\\python.exe";
    let script = "src\\main.py";

    let mut cmd = if cfg!(target_os = "windows") {
        let mut c = Command::new("cmd");
        c.arg("/C");
        c.arg(format!("{} {}", python, script));
        c
    } else {
        let mut c = Command::new(".venv/bin/python");
        c.arg("src/main.py");
        c
    };

    let output = cmd.output()?;

    println!("{}", String::from_utf8_lossy(&output.stdout));
    if !output.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr).red());
    }

    Ok(())
}