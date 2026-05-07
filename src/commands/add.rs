use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::fs;
use reqwest::blocking;
use colored::Colorize;
use zip::ZipArchive;
use super::venv;

#[derive(serde::Deserialize)]
struct PyPiResponse {
    info: PackageInfo,
    urls: Vec<PackageUrl>,
}

#[derive(serde::Deserialize)]
struct PackageInfo {
    #[allow(dead_code)]
    name: String,
    version: String,
    requires_dist: Option<Vec<String>>,
}

#[derive(serde::Deserialize)]
struct PackageUrl {
    url: String,
}

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

pub fn run(packages: &[String], install: bool) -> Result<(), Box<dyn Error>> {
    let current_dir = env::current_dir()?;
    let pyproject_path = current_dir.join("pyproject.toml");
    let venv_dir = current_dir.join(".venv");

    if !venv_dir.exists() {
        create_venv(&venv_dir)?;
    }

    let site_packages = get_site_packages(&venv_dir)?;

    if packages.is_empty() {
        let deps = read_dependencies(&pyproject_path)?;
        if deps.is_empty() {
            println!("{} {}",
                "No dependencies found in pyproject.toml. Add packages with:".white(),
                "litv add <package>".bold().green()
            );
            return Ok(());
        }
        println!("{} {}",
            "Installing dependencies from".bright_black(),
            "pyproject.toml...".bright_black().bold()
        );
        install_from_pyproject(&pyproject_path, &site_packages)?;
        println!("{}", "Installation complete!".green().bold());
        return Ok(());
    }

    let mut dependencies = read_dependencies(&pyproject_path)?;

    for package in packages {
        let (version, package_deps) = install_package_and_get_deps(&site_packages, package)?;
        
        if install {
            install_extras_and_deps(&site_packages, package, &package_deps, &mut dependencies)?;
        }

        add_dependency(&mut dependencies, package, &version);
        
        for dep in package_deps {
            let dep_name = extract_package_name(&dep);
            if dep_name.is_empty() || dep_name == "python" || dep_name.contains('-') || dep.contains('[') {
                continue;
            }
            if let Ok(dep_version) = get_package_version(&dep_name) {
                add_dependency(&mut dependencies, &dep_name, &dep_version);
            }
        }
    }

    write_pyproject(&pyproject_path, &dependencies)?;

    if install {
        println!("{}", "Installation complete!".green().bold());
    } else {
        println!("{} {} {}",
            "To install the packages, just run".white(),
            "litv add".bold().green(),
            "to install them!".white()
        );
    }
    
    Ok(())
}

fn create_venv(_venv_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    venv::run("")?;
    Ok(())
}

fn get_site_packages(venv_dir: &PathBuf) -> Result<PathBuf, Box<dyn Error>> {
    #[cfg(target_os = "windows")]
    { Ok(venv_dir.join("Lib").join("site-packages")) }
    #[cfg(not(target_os = "windows"))]
    { Ok(venv_dir.join("lib").join("python3.12").join("site-packages")) }
}

fn read_dependencies(pyproject_path: &PathBuf) -> Result<Vec<String>, Box<dyn Error>> {
    if !pyproject_path.exists() {
        return Ok(vec![]);
    }
    let content = fs::read_to_string(pyproject_path)?;
    let pyproject: PyProjectToml = toml::from_str(&content).unwrap_or(PyProjectToml { litv: None });
    Ok(pyproject.litv.and_then(|p| p.dependencies).unwrap_or_default())
}

fn write_pyproject(path: &PathBuf, dependencies: &[String]) -> Result<(), Box<dyn Error>> {
    println!("{} {}", "Writing packages required dependencies to:".bright_black(), "pyproject.toml".bold().bright_black());
    let content = if path.exists() {
        let existing = fs::read_to_string(path)?;
        let mut pyproject: PyProjectToml = toml::from_str(&existing).unwrap_or(PyProjectToml { litv: None });
        if let Some(ref mut project) = pyproject.litv {
            project.dependencies = Some(dependencies.to_vec());
        } else {
            pyproject.litv = Some(Project { name: None, version: None, description: None, python_version: None, dependencies: Some(dependencies.to_vec()) });
        }
        toml::to_string_pretty(&pyproject).unwrap_or(existing)
    } else {
        let pyproject = PyProjectToml {
            litv: Some(Project { name: None, version: None, description: None, python_version: None, dependencies: Some(dependencies.to_vec()) }),
        };
        toml::to_string_pretty(&pyproject)?
    };
    fs::write(path, content)?;
    Ok(())
}

fn install_from_pyproject(pyproject_path: &PathBuf, site_packages: &PathBuf) -> Result<(), Box<dyn Error>> {
    if !pyproject_path.exists() {
        println!("{} {}", "No pyproject.toml found. Add packages with:".red(), "litv add <package>".red().bold());
        return Ok(());
    }
    let deps = read_dependencies(pyproject_path)?;
    for dep in deps {
        let name = dep.split("==").next().unwrap_or(&dep);
        install_package(site_packages, name)?;
    }
    Ok(())
}

fn install_package_and_get_deps(site_packages: &PathBuf, package: &str) -> Result<(String, Vec<String>), Box<dyn Error>> {
    let (download_url, version, dependencies) = get_package_info(package)?;

    let temp_dir = std::env::temp_dir().join(format!("litv_{}", package));
    if temp_dir.exists() {
        let _ = fs::remove_dir_all(&temp_dir);
    }
    fs::create_dir_all(&temp_dir)?;

    let temp_wheel = temp_dir.join("package.whl");
    let response = blocking::get(&download_url)?;
    if !response.status().is_success() {
        return Err(format!("Download failed: HTTP {}", response.status()).into());
    }
    let data = response.bytes()?;
    fs::write(&temp_wheel, data)?;

    extract_wheel(&temp_wheel, site_packages)?;

    let _ = fs::remove_dir_all(&temp_dir);

    println!("{} {}={}", "+".green(), package.white(), version.white());
    Ok((version, dependencies))
}

fn install_package(site_packages: &PathBuf, package: &str) -> Result<String, Box<dyn Error>> {
    let (version, _deps) = install_package_and_get_deps(site_packages, package)?;
    Ok(version)
}

fn add_dependency(dependencies: &mut Vec<String>, package: &str, version: &str) {
    let new_dep = format!("{}=={}", package, version);
    let package_with_eq = format!("{}==", package);
    
    for dep in dependencies.iter() {
        if dep.starts_with(&package_with_eq) || dep == package {
            return;
        }
    }
    dependencies.push(new_dep);
}

fn extract_package_name(dep: &str) -> String {
    let base = dep.split(';').next().unwrap_or(dep);
    let name_end = base.find(|c: char| matches!(c, '(' | '[' | '>' | '<' | '=' | '~')).unwrap_or(base.len());
    base[..name_end].trim().to_string()
}

fn install_extras_and_deps(
    site_packages: &PathBuf,
    package: &str,
    _package_deps: &[String],
    dependencies: &mut Vec<String>,
) -> Result<(), Box<dyn Error>> {
    let extras = extract_extras(package);
    
    if extras.is_empty() {
        return Ok(());
    }

    let all_deps = get_package_all_deps(package)?;

    for extra in extras {
        for dep in &all_deps {
            if dep.contains(&format!("extra == \"{}\"", extra)) {
                let dep_name = extract_package_name(dep);
                if dep_name.is_empty() || dep_name == "python" {
                    continue;
                }
                if let Ok(dep_version) = get_package_version(&dep_name) {
                    add_dependency(dependencies, &dep_name, &dep_version);
                    install_package(site_packages, &dep_name)?;
                }
            }
        }
    }

    Ok(())
}

fn extract_extras(package: &str) -> Vec<String> {
    if let Some(start) = package.find('[') {
        if let Some(end) = package.find(']') {
            let extras_str = &package[start + 1..end];
            return extras_str
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
        }
    }
    vec![]
}

fn get_package_all_deps(package: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let name = package.split('[').next().unwrap_or(package);
    let url = format!("https://pypi.org/pypi/{}/json", name);
    let response = blocking::get(&url)?.json::<PyPiResponse>()?;
    let deps = response.info.requires_dist.unwrap_or_default();
    Ok(deps)
}

fn get_package_info(package: &str) -> Result<(String, String, Vec<String>), Box<dyn Error>> {
    let url = format!("https://pypi.org/pypi/{}/json", package);
    let response = blocking::get(&url)?.json::<PyPiResponse>()?;
    
    let wheel_url = response.urls.iter()
        .find(|u| u.url.ends_with(".whl"))
        .map(|u| u.url.clone())
        .ok_or_else(|| format!("No wheel file found for {}", package))?;
    
    let deps = response.info.requires_dist.unwrap_or_default();
    let dep_names: Vec<String> = deps.iter().map(|d| d.split(';').next().unwrap_or(d).trim().to_string()).collect();
    Ok((wheel_url, response.info.version.clone(), dep_names))
}

fn get_package_version(package: &str) -> Result<String, Box<dyn Error>> {
    let url = format!("https://pypi.org/pypi/{}/json", package);
    let response = blocking::get(&url)?.json::<PyPiResponse>()?;
    Ok(response.info.version)
}

fn extract_wheel(wheel_path: &PathBuf, dest_dir: &PathBuf) -> Result<(), Box<dyn Error>> {
    let file = fs::File::open(wheel_path)?;
    let mut archive = ZipArchive::new(file)?;
    for i in 0..archive.len() {
        let mut f = archive.by_index(i)?;
        let outpath = dest_dir.join(f.name());
        if f.name().ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                let _ = fs::create_dir_all(p);
            }
            let mut outfile = fs::File::create(&outpath)?;
            std::io::copy(&mut f, &mut outfile)?;
        }
    }
    Ok(())
}