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
        #[arg(default_value = "src/main.py", allow_hyphen_values = true, trailing_var_arg = true)]
        path: String,

        /// Arguments to pass to the Python script
        #[arg(last = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// Adds a new dependency to the project and cache.
    Add {
        /// Name of the dependency to add (omit to install all from pyproject.toml)
        #[arg(num_args = 1..)]
        name: Option<Vec<String>>,

        /// Install the package immediately
        #[arg(short, long)]
        install: bool,
    },

    /// Removes a dependency from the project and cache.
    Remove {
        /// Name of the dependency to remove
        #[arg(num_args = 1..)]
        name: Option<Vec<String>>,
    },

    /// Creates a virtual environment for the project.
    Venv {
        /// Version of Python to use for the virtual environment
        #[arg(default_value = "-3.14", allow_hyphen_values = true)]
        version: String,
    },
}

impl Args {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}