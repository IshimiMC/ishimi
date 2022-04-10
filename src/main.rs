mod download;
mod format;

use std::path::Path;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the mod config
    path: String,
}

fn main() {
    let args = Args::parse();

    let b: bool = Path::new().is_dir();
}
