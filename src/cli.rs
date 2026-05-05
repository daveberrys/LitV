use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "litv")]
#[command(about = "A CLI tool", long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Initialize a new LitV project
    Init {
        /// Path to initialize the project in
        #[arg(default_value = ".")]
        path: String,
    },

    /// Run the LitV application
    Run {
        /// Path to the project to run
        #[arg(default_value = "src/main.py")]
        path: String,
    },
}

impl Args {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}