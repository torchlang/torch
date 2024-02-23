use async_std::{fs, path::PathBuf};
use colored::Colorize;
use std::panic;
use torchc_diagnosis::Diagnosis;
use torchc_hike::hike;
use torchc_lits::lits;
use torchc_parse::parser;
use torchc_script::{AsScript, Script};

#[async_std::main]
async fn main() {
    // Configure the general error diagnoser by `panic!(...)`.
    panic::set_hook(Box::new(|panic_info| {
        if let Some(err) = panic_info.payload().downcast_ref::<&str>() {
            return eprintln!(
                "{}{} {}",
                lits::EPREFIX.red().bold(),
                lits::COLON.bold(),
                err
            );
        } else if let Some(err) = panic_info.payload().downcast_ref::<String>() {
            return eprintln!(
                "{}{} {}",
                lits::EPREFIX.red().bold(),
                lits::COLON.bold(),
                err
            );
        }

        #[cfg(debug_assertions)]
        eprintln!(
            "{}{} the type in {}{}{} has no support",
            lits::EPREFIX.red().bold(),
            lits::COLON.bold(),
            "panic_info.payload().downcast_ref::<".bold(),
            "T".red().bold(),
            ">()".bold()
        );
    }));

    // Find the language files in the `src/` directory.

    let cwd: PathBuf = match std::env::current_dir() {
        Ok(cwd) => cwd.into(),
        Err(err) => panic!("{}", err),
    };
    let mut src: PathBuf = cwd.clone();
    src.push(lits::std_resources::SRC);
    let scripts: Vec<PathBuf> = hike(&src, &cwd).await;
    println!("{:?}", scripts);

    let mut path: PathBuf = src.clone().to_path_buf();
    path.push("main.t");

    let content: String = fs::read_to_string(&path).await.unwrap();

    let mut script: Script = content.as_script();
    let mut diagnosis: Diagnosis = Diagnosis::new(&path, &cwd);
    parser(&mut script, &mut diagnosis);
}
