mod cli;
mod commands;

use cli::{Args, Command};
use commands::init::run as init_run;
use commands::run::run as run_run;
use commands::add::run as add_run;
use commands::remove::run as remove_run;
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

        Some(Command::Add { name, install }) => {
            let packages = name.unwrap_or_default();
            if let Err(e) = add_run(&packages, install) {
                eprintln!("Add failed: {}", e.to_string().red());
            }
        }

        Some(Command::Remove { name }) => {
            let packages = name.unwrap_or_default();
            if let Err(e) = remove_run(&packages) {
                eprintln!("Remove failed: {}", e.to_string().red());
            }
        }

        Some(Command::Venv { version }) => {
            if let Err(e) = venv_run(&version) {
                eprintln!("Venv failed: {}", e.to_string().red());
            }
        }
        
        None => {
            println!("Verbose: {}", args.verbose);
        }
    }
}