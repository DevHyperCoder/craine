use structopt::StructOpt;
use std::path::PathBuf;

/// HTML Compiler with react like components
#[derive(StructOpt,Debug)]
pub struct Config {
    /// Path to a craine workspace
    #[structopt(short,long)]
    pub path: PathBuf,
}
