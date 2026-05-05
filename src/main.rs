mod cli;
mod commands;

use cli::{Args, Command};
use commands::init::run as init_run;
use commands::run::run as run_run;
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
        
        None => {
            println!("Verbose: {}", args.verbose);
        }
    }
}