use clap::Parser;
use std::env::current_dir;
use std::error::Error;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short = 'd', long, value_name = "DIRECTORY")]
    search_dir: Option<PathBuf>,
    #[arg(value_name = "SEARCH STRING")]
    searcher: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    Ok(())
}
