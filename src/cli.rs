use std::path::PathBuf;

use clap::Parser;

// cli options for 'fd'
#[derive(Parser)]
pub struct Opts {
    /// Base Directory for search
    #[arg(long = "file")]
    pub dir: Option<PathBuf>,
}
