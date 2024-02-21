use async_std::{fs, path::PathBuf};
use colored::Colorize;
use std::panic;
use torchc_diagnosis::Diagnosis;
use torchc_lits::lits;
use torchc_parse::parser;
use torchc_script::{AsScript, Script};

#[async_std::main]
async fn main() {
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

    let mut path: PathBuf = std::env::current_dir().unwrap().into();
    path.push("../onedrive/escritorio/main.t");
    let content: String = fs::read_to_string(&path).await.unwrap();

    let mut script: Script = content.as_script();
    let mut diagnosis: Diagnosis = Diagnosis::new(&path);
    parser(&mut script, &mut diagnosis);
}
