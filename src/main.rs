mod providers;
mod formats;

use crate::providers::mods::direct::dl_mod;
use crate::formats::ishimi::schemas::IshimiFormat;
use clap::Parser;
use std::fs;
use std::path::Path;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the mod config
    path: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let file_str = &*fs::read_to_string(&args.path)?;

    println!("{:}", file_str);

    let file: IshimiFormat = toml::from_str(file_str)?;

    println!("{:?}", &file);

    let url = &file.mods.as_ref().unwrap()[0].direct.as_ref().unwrap()[0].url;

    let slash_separated_url = url.split('/').collect::<Vec<&str>>();
    let path = Path::new(slash_separated_url.last().unwrap());

    println!("Start downloading the file: {}", path.to_str().unwrap());
    dl_mod(url, path).await?;
    println!("Downloaded!");

    Ok(())
}
