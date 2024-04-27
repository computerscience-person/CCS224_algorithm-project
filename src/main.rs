use clap::Parser;
use std::borrow::Cow;
use std::error::Error;
use std::fs::{self, DirEntry};
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

    // fn list_entries(&self) -> Result<Vec<Cow<'_, str>>, Box<dyn Error>> {
    //     Ok(self
    //         .read_dir()
    //         .map_err(Box::new)?
    //         .into_iter()
    //         .filter_map(Result::ok)
    //         .map(|entry| {
    //          let name = entry.file_name().to_owned();
    //          <OsStr as AsRef<OsStr>>::as_ref(&name).to_string_lossy()
    //         })
    //         .collect())
    // }
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    if let Ok(search_dirs) = cli.read_dir() {
        for item in search_dirs {
            eprintln!(
                "{:}",
                item.unwrap()
                    .file_name()
                    .to_str()
                    .unwrap_or("ERROR: OsStr conversion error.")
            );
        }
    }
    println!("{}", strsim::normalized_damerau_levenshtein(" ", ""));
    let mut mock_files = vec!["hello", "there", "how", "are", "you?"];
    fuzzy_search(&mut mock_files, &cli.searcher);
    println!("{:?}", &mock_files);
    Ok(())
}

fn fuzzy_search<'a>(search_space: &mut [&'a str], searcher: &'a str) {
    search_space.sort_by_cached_key(|&item| {
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
