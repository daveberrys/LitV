use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::fs;
use std::process::Command;
use reqwest::blocking;
use colored::Colorize;
use zip::ZipArchive;
use flate2::read::GzDecoder;
use tar::Archive as TarArchive;
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
    project: Option<Project>,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Project {
    name: Option<String>,
    version: Option<String>,
    description: Option<String>,
    python_version: Option<String>,
    dependencies: Option<Vec<String>>,
}

pub fn run(packages: &[String], install: bool, backup: bool) -> Result<(), Box<dyn Error>> {
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

        if backup {
            println!("{} {}",
                "Installing dependencies from".bright_black(),
                "pyproject.toml using pip...".bright_black().bold()
            );
            for dep in &deps {
                pip_install(dep)?;
            }
        } else {
            println!("{} {}",
                "Installing dependencies from".bright_black(),
                "pyproject.toml...".bright_black().bold()
            );
            install_from_pyproject(&pyproject_path, &site_packages)?;
        }

        println!("{}", "Installation complete!".green().bold());
        return Ok(());
    }

    let mut dependencies = read_dependencies(&pyproject_path)?;

    for package in packages {
        let (version, package_deps) = if backup {
            let (_, v, deps) = get_package_info(package)?;
            pip_install(package)?;
            (v, deps)
        } else if install {
            install_package_and_get_deps(&site_packages, package)?
        } else {
            let (_, v, deps) = get_package_info(package)?;
            (v, deps)
        };
        
        if install && !backup {
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

    if install || backup {
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
    #[cfg(not(target_os = "windows"))] {
        let lib_dir = venv_dir.join("lib");
        for entry in fs::read_dir(lib_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name.starts_with("python") {
                        return Ok(path.join("site-packages"));
                    }
                }
            }
        }
        Err("Could not find python directory in .venv/lib".into())
    }
}

fn read_dependencies(pyproject_path: &PathBuf) -> Result<Vec<String>, Box<dyn Error>> {
    if !pyproject_path.exists() {
        return Ok(vec![]);
    }
    let content = fs::read_to_string(pyproject_path)?;
    let pyproject: PyProjectToml = toml::from_str(&content).unwrap_or(PyProjectToml { project: None });
    Ok(pyproject.project.and_then(|p| p.dependencies).unwrap_or_default())
}

fn write_pyproject(path: &PathBuf, dependencies: &[String]) -> Result<(), Box<dyn Error>> {
    println!("{} {}", "Writing packages required dependencies to:".bright_black(), "pyproject.toml".bold().bright_black());
    let content = if path.exists() {
        let existing = fs::read_to_string(path)?;
        let mut pyproject: PyProjectToml = toml::from_str(&existing).unwrap_or(PyProjectToml { project: None });
        if let Some(ref mut project) = pyproject.project {
            project.dependencies = Some(dependencies.to_vec());
        } else {
            pyproject.project = Some(Project { name: None, version: None, description: None, python_version: None, dependencies: Some(dependencies.to_vec()) });
        }
        toml::to_string_pretty(&pyproject).unwrap_or(existing)
    } else {
        let pyproject = PyProjectToml {
            project: Some(Project { name: None, version: None, description: None, python_version: None, dependencies: Some(dependencies.to_vec()) }),
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
        install_package(site_packages, &dep)?;
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

    let is_wheel = download_url.ends_with(".whl");
    let temp_file = temp_dir.join(if is_wheel { "package.whl" } else { "package.tar.gz" });
    let response = blocking::get(&download_url)?;
    if !response.status().is_success() {
        return Err(format!("Download failed: HTTP {}", response.status()).into());
    }
    let data = response.bytes()?;
    fs::write(&temp_file, data)?;

    if is_wheel {
        extract_wheel(&temp_file, site_packages)?;
    } else {
        extract_tarball(&temp_file, site_packages)?;
    }

    let _ = fs::remove_dir_all(&temp_dir);

    println!("{} {}", "+".green(), package.white());
    Ok((version, dependencies))
}

fn install_package(site_packages: &PathBuf, package: &str) -> Result<String, Box<dyn Error>> {
    let (version, _deps) = install_package_and_get_deps(site_packages, package)?;
    Ok(version)
}

fn add_dependency(dependencies: &mut Vec<String>, package: &str, version: &str) {
    let name = package.split("==").next().unwrap_or(package);
    let new_dep = format!("{}=={}", name, version);
    let package_with_eq = format!("{}==", name);
    
    for dep in dependencies.iter() {
        if dep.starts_with(&package_with_eq) || dep.starts_with(name) {
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
    let (name, requested_version) = if let Some((n, v)) = package.split_once("==") {
        (n, Some(v.to_string()))
    } else {
        (package, None)
    };
    
    let url = if let Some(ref ver) = requested_version {
        format!("https://pypi.org/pypi/{}/{}/json", name, ver)
    } else {
        format!("https://pypi.org/pypi/{}/json", name)
    };
    
    let response = match blocking::get(&url) {
        Ok(resp) if resp.status().is_success() => resp.json::<PyPiResponse>()?,
        _ if requested_version.is_some() => {
            let fallback_url = format!("https://pypi.org/pypi/{}/json", name);
            let fallback_resp = blocking::get(&fallback_url)?.json::<PyPiResponse>()?;
            eprintln!("Warning: version {} not found for {}, using latest: {}", requested_version.as_ref().unwrap(), name, fallback_resp.info.version);
            fallback_resp
        }
        _ => return Err(format!("Failed to fetch package info for {}", package).into()),
    };
    
    let download_url = response.urls.iter()
        .find(|u| u.url.ends_with(".whl"))
        .or_else(|| response.urls.iter().find(|u| u.url.ends_with(".tar.gz")))
        .map(|u| u.url.clone())
        .ok_or_else(|| format!("No wheel or tar.gz file found for {}", package))?;
    
    let version = response.info.version.clone();
    let deps = response.info.requires_dist.unwrap_or_default();
    let dep_names: Vec<String> = deps.iter().map(|d| d.split(';').next().unwrap_or(d).trim().to_string()).collect();
    Ok((download_url, version, dep_names))
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

fn extract_tarball(tarball_path: &PathBuf, dest_dir: &PathBuf) -> Result<(), Box<dyn Error>> {
    let file = fs::File::open(tarball_path)?;
    let decoder = GzDecoder::new(file);
    let mut archive = TarArchive::new(decoder);
    
    let mut top_dir: Option<PathBuf> = None;
    
    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?.into_owned();
        
        if top_dir.is_none() {
            if let Some(first) = path.components().next() {
                let td = PathBuf::from(first.as_os_str());
                top_dir = Some(td.clone());
                if !dest_dir.join(&td).exists() {
                    let _ = fs::create_dir_all(dest_dir.join(&td));
                }
            }
        }
        
        let relative = if let Some(ref td) = top_dir {
            path.strip_prefix(td).unwrap_or(&path).to_path_buf()
        } else {
            path.clone()
        };
        
        let outpath = dest_dir.join(&relative);
        
        if entry.header().entry_type().is_dir() {
            let _ = fs::create_dir_all(&outpath);
            continue;
        }
        
        if let Some(p) = outpath.parent() {
            let _ = fs::create_dir_all(p);
        }
        entry.unpack(&outpath)?;
    }
    
    Ok(())
}

fn pip_install(package: &str) -> Result<(), Box<dyn Error>> {
    let python_path;
    if cfg!(target_os = "windows") {
        python_path = ".venv\\Scripts\\python.exe".to_string();
    } else {
        python_path = ".venv/bin/python".to_string();
    }
    
    let status = Command::new(python_path)
        .arg("-m")
        .arg("pip")
        .arg("install")
        .arg("--no-user")
        .arg(package)
        .status()?;

    if !status.success() {
        return Err(format!("pip install failed for {} with status: {}", package, status).into());
    }

    Ok(())
}