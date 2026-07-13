use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "litv")]
#[command(about = "Your next helper tool for your next python project.", long_about = None)]
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
        #[arg(
            default_value = "src/main.py",
            allow_hyphen_values = true,
            trailing_var_arg = true
        )]
        path: String,

        /// Arguments to pass to the Python script
        #[arg(last = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// Installs packages with pip and records them in requirements.txt.
    Add {
        /// Packages to install (omit to install requirements.txt)
        #[arg(num_args = 1..)]
        name: Option<Vec<String>>,
    },

    /// Uninstalls packages with pip and removes them from requirements.txt.
    Remove {
        /// Packages to remove
        #[arg(required = true, num_args = 1..)]
        name: Option<Vec<String>>,
    },

    /// Creates a virtual environment with `python -m venv .venv`.
    Venv,
}

impl Args {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
