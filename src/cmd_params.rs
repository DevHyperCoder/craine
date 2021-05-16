use std::path::PathBuf;
use structopt::StructOpt;

/// HTML Compiler with react like components
#[derive(StructOpt, Debug)]
pub struct Config {
    /// Path to a craine workspace
    #[structopt(short, long)]
    pub path: PathBuf,

    /// Auto compile or not
    #[structopt(long)]
    pub autorun: bool,
}
