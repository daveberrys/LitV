mod cli;
mod commands;

use cli::{Args, Command};
use commands::init::run as init_run;
use commands::run::run as run_run;
use commands::add::run as add_run;
use commands::remove::run as remove_run;

use colored::Colorize;

fn main() {
    let args = Args::parse_args();

    match args.command {
        Some(Command::Init { path }) => {
            if let Err(e) = init_run(&path) {
                eprintln!("Init failed: {}", e.to_string().red());
            }
        }

        Some(Command::Run { path }) => {
            if let Err(e) = run_run(&path) {
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

        None => {
            println!("Verbose: {}", args.verbose);
        }
    }
}