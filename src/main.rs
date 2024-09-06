use clap::Parser;
use pdb_downloader::PdbDownloader;
use std::{path::PathBuf, process::exit};

#[derive(Parser)]
struct Args {
    #[arg(short = 'i', long)]
    pe: PathBuf,
    #[arg(short, long)]
    out: PathBuf,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let downloader = match PdbDownloader::new(args.pe, args.out) {
        Ok(downloader) => downloader,
        Err(e) => {
            eprintln!("error: {:?}", e);
            exit(1);
        }
    };
    match downloader.download().await {
        Ok(_) => {
            println!("PDB downloaded");
        }
        Err(e) => {
            eprintln!("error: {}", e);
            exit(1);
        }
    }
}
