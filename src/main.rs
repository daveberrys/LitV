mod cli;
mod commands;

use cli::{Args, Command};
use commands::add::run as add_run;
use commands::init::run as init_run;
use commands::remove::run as remove_run;
use commands::run::run as run_run;
use commands::venv::run as venv_run;

use colored::Colorize;

fn main() {
    let args = Args::parse_args();

    match args.command {
        Some(Command::Init { path }) => {
            if let Err(e) = init_run(&path) {
                eprintln!("Init failed: {}", e.to_string().red());
            }
        }

        Some(Command::Run { path, args }) => {
            if let Err(e) = run_run(&path, args) {
                eprintln!("Run failed: {}", e.to_string().red());
            }
        }

        Some(Command::Add { name }) => {
            let packages = name.unwrap_or_default();
            if let Err(e) = add_run(&packages) {
                eprintln!("Add failed: {}", e.to_string().red());
            }
        }

        Some(Command::Remove { name }) => {
            let packages = name.unwrap_or_default();
            if let Err(e) = remove_run(&packages) {
                eprintln!("Remove failed: {}", e.to_string().red());
            }
        }

        Some(Command::Venv) => {
            if let Err(e) = venv_run() {
                eprintln!("Venv failed: {}", e.to_string().red());
            }
        }

        None => {
            println!("{}{} {}", "Lit".bright_yellow().bold(), "V".bright_black().bold(), "available command list".white());
            println!("{} {}", "litv init".cyan(), "to initialize a LitV project".white());
            println!("{} {}", "litv run <pyfile> <args>".cyan(), "to run a Python file with LitV".white());
            println!("{} {}", "litv add <packages>".cyan(), "to add packages to the virtual environment".white());
            println!("{} {}", "litv remove <packages>".cyan(), "to remove packages from the virtual environment".white());
            println!("{} {}", "litv venv".cyan(), "to manage the virtual environment".white());
            println!("{} {}", "check our github at".white(), "https://github.com/daveberrys/litv".cyan());
        }
    }
}
