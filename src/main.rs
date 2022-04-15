mod download;
mod format;

use std::path::Path;
use clap::Parser;
use std::fs;
use format::ishimi::schemas::IshimiFormat;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the mod config
    path: String,
}

fn main() {
    let args = Args::parse();

    let file_str = &*fs::read_to_string(&args.path).unwrap();

    println!("{:}", file_str);

    let file: IshimiFormat = toml::from_str(file_str).unwrap();

    println!("{:?}", file);
}
