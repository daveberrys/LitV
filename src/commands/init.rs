use colored::Colorize;
use std::error::Error;
use std::fs;
use std::io;
use std::path::Path;

const CONTENT_README: &str = r#"# A LitV Project.
Check out LitV in our github."#;

const CONTENT_GITIGNORE: &str = r#"*venv*
*.pyc
__pycache__"#;

const CONTENT_REQUIREMENTS: &str = "";

const CONTENT_MAIN: &str = r#"def main():
    print("Hello from LitV!")

if __name__ == "__main__":
    main()"#;

pub fn run(path: &str) -> Result<(), Box<dyn Error>> {
    let base_path = Path::new(path);

    if path != "." && path != "" {
        fs::create_dir(base_path)?;
    }
    fs::create_dir(base_path.join("src"))?;

    println!(
        "{}",
        format!("Creating a LitV project in {}...", path).bright_black()
    );

    create_file(base_path.join("README.md"), CONTENT_README)?;
    create_file(base_path.join(".gitignore"), CONTENT_GITIGNORE)?;
    create_file(base_path.join("requirements.txt"), CONTENT_REQUIREMENTS)?;
    create_file(base_path.join("src/main.py"), CONTENT_MAIN)?;

    println!(
        "{} {} {}",
        "Finished!".green(),
        "Check out your project in".white(),
        path.green()
    );
    Ok(())
}

fn create_file<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, content: C) -> Result<(), io::Error> {
    fs::write(path.as_ref(), content.as_ref())?;
    println!(
        "{} {}",
        "+".green(),
        path.as_ref().display().to_string().white()
    );
    Ok(())
}
