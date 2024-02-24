use async_std::{
    path::{Path, PathBuf},
    stream::StreamExt,
};
use async_walkdir::WalkDir;
use colored::Colorize;
use std::ffi::OsString;
use torchc_lits::lits;

/// Scans the `src/` directory within the current working directory (`cwd`) for
/// `.t`, `.c` and/or `.cpp` files.
pub async fn hike(src: &Path, cwd: &Path) -> Vec<PathBuf> {
    if !src.exists().await {
        panic!(
            "the {} folder does not exist in the {} directory",
            lits::std_resources::SRC.red().bold(),
            match cwd.to_str() {
                Some(cwd) => cwd.bold(),
                None => lits::CURRENT.normal(),
            }
        );
    }

    let mut scripts: Vec<PathBuf> = vec![];
    let mut entries: WalkDir = WalkDir::new(&src);
    loop {
        match entries.next().await {
            Some(Ok(entry)) => {
                let path: PathBuf = entry.path().into();
                if path.is_file().await {
                    match path.extension() {
                        Some(ext)
                            if {
                                let ext: OsString = ext.to_ascii_lowercase();
                                ext == lits::extensions::T
                                    || ext == lits::extensions::H
                                    || ext == lits::extensions::C
                                    || ext == lits::extensions::CPP
                                    || ext == lits::extensions::CPP2
                            } =>
                        {
                            scripts.push(path);
                        }
                        _ => panic!(
                            "the {} file is illegal in the {} directory",
                            match path.file_name() {
                                Some(filename) => match filename.to_str() {
                                    Some(filename) => filename.bold(),
                                    None => lits::CURRENT.normal(),
                                },
                                None => lits::CURRENT.normal(),
                            },
                            match path.parent() {
                                Some(parent) => match parent.to_str() {
                                    Some(parent) => parent.bold(),
                                    None => lits::CURRENT.normal(),
                                },
                                None => lits::CURRENT.normal(),
                            }
                        ),
                    }
                }
            }
            Some(Err(err)) => panic!("{}", err),
            None => break,
        };
    }
    scripts
}
