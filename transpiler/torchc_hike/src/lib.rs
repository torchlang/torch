use async_std::{
    path::{Path, PathBuf},
    stream::StreamExt,
};
use async_walkdir::WalkDir;
use colored::Colorize;
use torchc_lits::lits;

/// Scans the `src/` directory within the current working directory (`cwd`) for
/// `.t`, `.c` and/or `.cpp` files.
pub async fn hike(src: &Path, cwd: &Path) -> Vec<PathBuf> {
    if !src.exists().await {
        panic!(
            "the {} folder does not exist in the {} directory",
            lits::std_resources::SRC.red().bold(),
            match cwd.to_str() {
                Some(cwd) => cwd,
                None => "",
            }
            .bold()
        );
    }

    let mut scripts: Vec<PathBuf> = vec![];
    let mut entries: WalkDir = WalkDir::new(&src);
    loop {
        match entries.next().await {
            Some(Ok(entry)) => scripts.push(entry.path().into()),
            Some(Err(err)) => panic!("{}", err),
            None => break,
        };
    }
    scripts
}
