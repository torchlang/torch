use colored::Colorize;
use torchc_lex::Token;

const INDENT_LIT: &str = "  ";

/// Handles the diagnosis of language errors.
///
/// ---
/// **Format:**
///
/// `error: message → file`<br>
/// &nbsp;&nbsp;&nbsp;&nbsp;`1 | line`<br>
/// &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`↑ 1`
#[derive(Debug)]
pub struct Diagnosis<'diagnosis> {
    /// Script path.
    pub path: &'diagnosis str,
}
impl<'diagnosis> Diagnosis<'diagnosis> {
    pub async fn new(path: &'diagnosis str) -> Self {
        Self { path }
    }

    /// Launch an error diagnostic and stop the execution.
    pub async fn diagnosis(&self, msg: &str, token: &Token) {
        panic!(
            "{} {} {}\n{}{}",
            msg,
            "→".red().bold(),
            self.path,
            INDENT_LIT,
            if let Some(lit) = token.lit().await {
                format!("{}", lit)
            } else {
                String::new()
            }
        );
    }
}
