use clap::Parser;
use std::borrow::Cow;
use std::error::Error;
use std::fs::{self, DirEntry};
use std::ops::Deref;
use std::path::PathBuf;

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

    fn read_dir_unwrapped(&self) -> Result<Vec<DirEntry>, Box<dyn Error>> {
        let files = self.read_dir()?;
        files
            .into_iter()
            .map(|file| file.map_err(|e| Box::new(e) as Box<dyn Error>))
            .collect()
    }

    fn list_dir(&self) -> Result<Vec<std::ffi::OsString>, Box<dyn Error>> {
        let entries = self.read_dir_unwrapped()?;
        Ok(entries.iter().map(|entry| entry.file_name()).collect())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let mock_files = vec!["hello", "there", "how", "are", "you?"];
    dbg!(fuzzy_search(&mock_files, &cli.searcher));
    Ok(())
}

fn fuzzy_search<'a>(search_space: &'a [&'a str], searcher: &'a str) -> Vec<&'a str> {
    let mut search_space: Vec<&str> = search_space
        .iter()
        .filter(|item| strsim::normalized_damerau_levenshtein(item, searcher) > 0.0)
        .copied()
        .collect();
    fuzzy_sort(&mut search_space, searcher);
    search_space
}

fn fuzzy_sort<'a>(search_space: &mut [&'a str], searcher: &'a str) {
    search_space.sort_unstable_by_key(|&item| {
        std::cmp::Reverse(ordered_float::OrderedFloat(
            strsim::normalized_damerau_levenshtein(item, searcher),
        ))
    });
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        assert_eq!(2, 2);
    }
}
