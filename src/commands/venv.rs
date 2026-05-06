use std::process::Command;
use colored::Colorize;
use std::error::Error;
use std::path::PathBuf;

pub fn run(version: &str) -> Result<(), Box<dyn Error>> {
    check_os()?;

    println!("{}", "Creating a LitV Virtual Environment...".bright_black());
    
    let python_path = if version.is_empty() {
        detect_latest_python()?
    } else {
        run_py_command(&[version, "-c", "import sys; print(sys.executable)"])?
    };
    
    let python_exe = python_path.trim();
    let python_path_buf = PathBuf::from(python_exe);
    let python_dir = python_path_buf.parent()
        .ok_or_else(|| "Invalid Python path".to_string())?;
    
    let venv_path = std::env::current_dir()?.join(".venv");
    
    if venv_path.exists() {
        println!("{}", "Removing existing .venv...".yellow());
        std::fs::remove_dir_all(&venv_path)?;
    }
    
    std::fs::create_dir(&venv_path)?;
    
    let lib_src = python_dir.join("Lib");
    let venv_lib = venv_path.join("Lib");
    let scripts_src = python_dir.join("Scripts");
    let venv_scripts = venv_path.join("Scripts");
    
    println!("{}", "Creating junction for Lib...".bright_black());
    if let Err(_) = run_junction(&venv_lib, &lib_src) {
        println!("{} Falling back to copy", "Junction failed:".yellow());
        fallback_copy(&venv_lib, &lib_src)?;
    }
    
    println!("{}", "Creating junction for Scripts...".bright_black());
    if let Err(_) = run_junction(&venv_scripts, &scripts_src) {
        println!("{} Falling back to copy", "Junction failed:".yellow());
        fallback_copy(&venv_scripts, &scripts_src)?;
    }
    
    println!("{}", "Creating pyvenv.cfg...".bright_black());
    let cfg_content = format!(
        "home = {}\ninclude-system-site-packages = false\nversion = 3.14.0\n",
        python_exe.replace('\\', "\\\\")
    );
    std::fs::write(venv_path.join("pyvenv.cfg"), cfg_content)?;
    
    let venv_python = venv_path.join("Scripts").join("python.exe");
    if !venv_python.exists() {
        std::fs::copy(python_exe, &venv_python)?;
    }
    
    println!("{}", "Virtual environment created!".green().bold());
    println!("{}", format!("    at: {}", venv_path.display()).white());
    
    Ok(())
}

fn detect_latest_python() -> Result<String, Box<dyn Error>> {
    let output = Command::new("py").args(["-0"]).output()?;
    if !output.status.success() {
        return Err("No Python found".into());
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();
    
    if lines.is_empty() || lines.len() < 2 {
        return Err("No Python versions found".into());
    }
    
    let latest = lines[1].trim();
    let version = latest.split(' ').next().unwrap_or(latest);
    
    run_py_command(&[version, "-c", "import sys; print(sys.executable)"])
}

fn run_py_command(args: &[&str]) -> Result<String, Box<dyn Error>> {
    let output = Command::new("py").args(args).output()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("py command failed: {}", stderr).into());
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn run_junction(target: &std::path::Path, source: &std::path::Path) -> Result<(), Box<dyn Error>> {
    let target_str = target.to_string_lossy().replace('/', "\\");
    let source_str = source.to_string_lossy().replace('/', "\\");
    
    let output = Command::new("cmd")
        .args(["/C", &format!("mklink /J \"{}\" \"{}\"", target_str, source_str)])
        .output()?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Junction error: {}", stderr).into());
    }
    Ok(())
}

fn fallback_copy(target: &std::path::Path, source: &std::path::Path) -> Result<(), Box<dyn Error>> {
    if source.is_dir() {
        std::fs::create_dir_all(target)?;
        for entry in std::fs::read_dir(source)? {
            let entry = entry?;
            let dest = target.join(entry.file_name());
            if entry.path().is_dir() {
                fallback_copy(&dest, &entry.path())?;
            } else {
                std::fs::copy(entry.path(), dest)?;
            }
        }
    } else {
        std::fs::copy(source, target)?;
    }
    Ok(())
}

fn check_os() -> Result<(), Box<dyn Error>> {
    #[cfg(not(target_os = "windows"))]
    {
        println!("{}", "Sorry to say this, but your current os (which may be unix) is not supported!".red());
        println!("{}", "As of right now, only Windows is supported.".red());
    }
    Ok(())
}