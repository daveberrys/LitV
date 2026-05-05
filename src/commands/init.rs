use std::fs;
use std::io;
use std::path::Path;
use colored::Colorize;

const CONTENT_README: &str = r#"# A LitV Project.
Check out LitV in our github.
"#;

const CONTENT_GITIGNORE: &str = r#"*venv*
*.pyc
__pycache__
"#;

const CONTENT_PYPROJECT: &str = r#"[litv]
name = "{}"
version = "0.1.0"
description = "An initialized LitV project"
python_version = "latest"
is_litv_project = true
dependencies = []
"#;

const CONTENT_MAIN: &str = r#"def main():
    print("Hello from LitV!")
"#;

pub fn run(path: &str) -> Result<(), io::Error> {
    let base_path = Path::new(path);

    if path != "." && path != "" {
        fs::create_dir(base_path)?;
        fs::create_dir(base_path.join("src"))?;
    }

    let project_name = base_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("litv-project");

    let pyproject_content = CONTENT_PYPROJECT.replace("{}", project_name);

    println!("{}", format!("Creating a LitV project in {}...", path).bright_black());

    create_file(base_path.join("README.md"), CONTENT_README)?;
    create_file(base_path.join(".gitignore"), CONTENT_GITIGNORE)?;
    create_file(base_path.join("pyproject.toml"), &pyproject_content)?;
    create_file(base_path.join("src/main.py"), CONTENT_MAIN)?;

    println!("{} {} {}", "Finished!".green(), "Check out your project in".white(), path.green());
    Ok(())
}

fn create_file<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, content: C) -> Result<(), io::Error> {
    fs::write(path.as_ref(), content.as_ref())?;
    println!("{} {}", "+".green(), path.as_ref().display().to_string().white());
    Ok(())
}