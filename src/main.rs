use clap::Parser;
use std::error::Error;
use std::path::PathBuf;
use std::fs::{self, DirEntry};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short = 'd', long, value_name = "DIRECTORY")]
    search_dir: Option<PathBuf>,
    #[arg(value_name = "SEARCH STRING")]
    searcher: String,
}

impl Cli {
    fn read_dir(&self) -> Result<Vec<Result<DirEntry, std::io::Error>>, std::io::Error> {
        let files: Vec<_> = fs::read_dir(self.search_dir.clone().unwrap_or(".".into()))?.collect();
        Ok(files)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    Ok(())
}
