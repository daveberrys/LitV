use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::fs;
use std::process::Command;
use colored::Colorize;

#[derive(serde::Deserialize, serde::Serialize)]
struct PyProjectToml {
    litv: Option<Project>,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Project {
    name: Option<String>,
    version: Option<String>,
    description: Option<String>,
    python_version: Option<String>,
    dependencies: Option<Vec<String>>,
}

pub fn run(packages: &[String], backup: bool) -> Result<(), Box<dyn Error>> {
    let current_dir = env::current_dir()?;
    let pyproject_path = current_dir.join("pyproject.toml");
    let venv_dir = current_dir.join(".venv");

    if packages.is_empty() {
        println!("{}", "Please specify packages to remove".red());
        return Ok(());
    }
    
    let mut dependencies = read_dependencies(&pyproject_path)?;
    
    for package in packages {
        remove_from_pyproject(package, &mut dependencies, &pyproject_path)?;
        
        if venv_dir.exists() {
            let site_packages = get_site_packages(&venv_dir)?;

            if backup {
                pip_remove(package)?;
            } else {
                remove_package_from_venv(&site_packages, package)?;
            }
        }
    }

    println!("{} Removed packages: {:?}", "-".red(), packages);
    Ok(())
}

fn read_dependencies(pyproject_path: &PathBuf) -> Result<Vec<String>, Box<dyn Error>> {
    if !pyproject_path.exists() {
        return Ok(vec![]);
    }
    let content = fs::read_to_string(pyproject_path)?;
    let pyproject: PyProjectToml = toml::from_str(&content).unwrap_or(PyProjectToml { litv: None });
    Ok(pyproject.litv.and_then(|p| p.dependencies).unwrap_or_default())
}

fn remove_from_pyproject(package: &str, dependencies: &mut Vec<String>, path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let package_lower = package.to_lowercase();
    
    dependencies.retain(|dep| {
        let dep_name = dep.split("==").next().unwrap_or(dep).to_lowercase();
        dep_name != package_lower
    });

    if path.exists() {
        let existing = fs::read_to_string(path)?;
        let mut pyproject: PyProjectToml = toml::from_str(&existing).unwrap_or(PyProjectToml { litv: None });
        
        if let Some(ref mut project) = pyproject.litv {
            project.dependencies = Some(dependencies.to_vec());
        }
        
        let content = toml::to_string_pretty(&pyproject).unwrap_or(existing);
        fs::write(path, content)?;
    }

    Ok(())
}

fn remove_package_from_venv(site_packages: &PathBuf, package: &str) -> Result<(), Box<dyn Error>> {
    let package_folder = site_packages.join(package);

    if package_folder.exists() {
        fs::remove_dir_all(&package_folder)?;
        println!("{} Removed from venv: {}", "-".red(), package.white());
    }

    for entry in fs::read_dir(&site_packages)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().to_lowercase();
        if name.starts_with(&package.to_lowercase()) && name.ends_with(".dist-info") {
            fs::remove_dir_all(entry.path())?;
            println!("{} Removed dist-info: {}", "-".red(), name.white());
        }
    }

    Ok(())
}

fn get_site_packages(venv_dir: &PathBuf) -> Result<PathBuf, Box<dyn Error>> {
    #[cfg(target_os = "windows")]
    { Ok(venv_dir.join("Lib").join("site-packages")) }
    #[cfg(not(target_os = "windows"))]
    { Ok(venv_dir.join("lib").join("python3.12").join("site-packages")) }
}

fn pip_remove(package: &str) -> Result<(), Box<dyn Error>> {
    let python_path;
    if cfg!(target_os = "windows") {
        python_path = ".venv\\Scripts\\python.exe".to_string();
    } else {
        python_path = ".venv/bin/python".to_string();
    }

    let status = Command::new(python_path)
        .arg("-m")
        .arg("pip")
        .arg("uninstall")
        .arg("-y")
        .arg(package)
        .status()?;
    
    if !status.success() {
        return Err(format!("pip uninstall failed for {} with status: {}", package, status).into());
    }
    
    Ok(())
}