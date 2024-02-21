use async_std::path::PathBuf;

/// Scans the `src/` directory within the current working directory (`cwd`) for
/// `.t`, `.c` and/or `.cpp` files.
pub async fn walkdir(cwd: PathBuf) -> Vec<Box<[u8]>> {
    let scripts: Vec<Box<[u8]>> = vec![];
    scripts
}
