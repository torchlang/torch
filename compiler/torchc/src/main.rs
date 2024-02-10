use async_std::fs;
use colored::Colorize;
use std::panic;
use torchc_lex::{lexer, Script, ToScript};
use torchc_lits::lits;

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

    let content: String = fs::read_to_string("../onedrive/escritorio/main.torch")
        .await
        .unwrap();
    let mut script: Script = content.to_script().await;

    while let Some(token) = lexer(&mut script).await {
        print!("[{}] ", token.lit().await.unwrap());
    }
}
