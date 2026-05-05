use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::fs;
use reqwest::blocking;
use colored::Colorize;

#[derive(serde::Deserialize)]
struct PackageInfo {
    version: String,
}

#[derive(serde::Deserialize)]
struct PackageUrl {
    url: String,
}

#[derive(serde::Deserialize)]
struct PyPiResponse {
    urls: Vec<PackageUrl>,
    info: PackageInfo,
}

pub fn run(package: &str) -> Result<(), Box<dyn Error>> {
    let cache_dir = get_cache_folder();
    
    fs::create_dir_all(&cache_dir)?;
    download_package(&cache_dir, package)?;
    
    Ok(())
}

fn get_cache_folder() -> PathBuf {
    let home = env::var("APPDATA")
        .or_else(|_| env::var("HOME"))
        .unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join("dev.pages.codedave.litv").join("cached_packages")
}

fn download_package(cache_dir: &PathBuf, package: &str) -> Result<(), Box<dyn Error>> {
    let package_folder = cache_dir.join(package);
    fs::create_dir_all(&package_folder)?;
    
    let (download_link, package_version) = get_download_link(package)?;
    let data = blocking::get(&download_link)?.bytes()?;
    fs::write(package_folder.join("package.whl"), data)?;

    println!("{} {}={}", "+".green(), package.white(), package_version.white());
    Ok(())
}

fn get_download_link(package: &str) -> Result<(String, String), Box<dyn Error>> {
    let url = format!("https://pypi.org/pypi/{}/json", package);
    let response = blocking::get(&url)?.json::<PyPiResponse>()?;
    
    Ok((response.urls[0].url.clone(), response.info.version.clone()))
}