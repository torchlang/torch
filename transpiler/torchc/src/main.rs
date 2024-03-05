use async_std::{fs, path::PathBuf};
use colored::Colorize;
use torchc_cgen::{cgen, CGen};
use torchc_cli::Cli;
use torchc_diagnosis::panic;
use torchc_hike::hike;
use torchc_lits::lits;
use torchc_parse::parser;
use torchc_script::Script;

#[async_std::main]
async fn main() {
    panic::default();

    let cli: Cli = Cli::parse();
    println!(
        "[cmd: `{}`, subcmd: `{:?}`]",
        String::from_utf8_lossy(&cli.cmd.unwrap()),
        cli.subcmd.unwrap()
    );

    // Find the language files in the `src/` directory.

    let cwd: PathBuf = match std::env::current_dir() {
        Ok(cwd) => cwd.into(),
        Err(err) => panic!("{}", err),
    };
    let mut src: PathBuf = cwd.clone();
    src.push(lits::std_resources::SRC);
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
    let scripts: Vec<PathBuf> = hike(&src).await;

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

        let mut script: Script = Script::script(&path).await.unwrap();
        let mut diagnosis: panic::Diagnosis = panic::Diagnosis::new(&path, &cwd);
        let mut expr: cgen::Stmt = cgen::Stmt::Global(None);
        expr = parser(&mut script, &mut diagnosis, &expr);

        CGen::new(
            match expr {
                cgen::Stmt::Global(global) => match global {
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
