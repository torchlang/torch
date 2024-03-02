use async_std::{fs, path::PathBuf};
use colored::Colorize;
use std::panic;
use torchc_cgen::{cgen, CGen};
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
        }
        if let Some(err) = panic_info.payload().downcast_ref::<String>() {
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

    let mode: cgen::Mode = cgen::Mode::Dev;
    let mut dot_target: PathBuf = cwd.clone();
    {
        dot_target.push(lits::std_resources::dot_target::NAME);
        if mode == cgen::Mode::Dev {
            dot_target.push(lits::std_resources::dot_target::DEV);
        }
        // `.../.target/` or `.../.target/dev/`
        fs::create_dir_all(&dot_target)
            .await
            .unwrap_or_else(|err| panic!("{}", err));
    }

    {
        let mut path: PathBuf = src.clone();
        path.push("main.t");

        let content: String = fs::read_to_string(&path).await.unwrap();

        let mut script: Script = content.as_script();
        let mut diagnosis: Diagnosis = Diagnosis::new(&path, &cwd);
        let mut expr: cgen::Expr = cgen::Expr::Global(None);
        parser(&mut script, &mut diagnosis, &mut expr);

        CGen::new(
            match expr {
                cgen::Expr::Global(global) => match global {
                    Some(program) => program,
                    None => vec![],
                },
                _ => vec![],
            },
            &dot_target,
        )
        .cgen(&path)
        .await;
    }
}
