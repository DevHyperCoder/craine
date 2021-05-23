use std::path::PathBuf;
use structopt::StructOpt;

/// HTML Compiler with react like components
#[derive(StructOpt, Debug)]
pub struct Config {
    /// Subcommand to execute
    #[structopt(subcommand)]
    pub cmd: Command,
}

/// Command list enum
#[derive(StructOpt, Debug)]
pub enum Command {
    /// Compilation command
    Compile {
        /// Path to a craine workspace
        #[structopt(short, long)]
        path: PathBuf,

        /// Auto compile or not
        #[structopt(long)]
        autorun: bool,
    },
    /// Initializes a new craine project in the given <path>
    Init {
        /// Path to a directory
        #[structopt(short, long)]
        path: PathBuf,
    },
}
