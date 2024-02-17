use async_std::path::PathBuf;
use colored::Colorize;
use torchc_lex::Pos;
use torchc_script::Script;

const INDENT_LIT: &str = "       ";

/// Handles the diagnosis of language errors.
///
/// ---
/// **Format:**
///
/// `error: message → file`<br>
/// &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
/// `1 | line`<br>
/// &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
/// &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`↑ 1`
#[derive(Debug)]
pub struct Diagnosis<'diagnosis> {
    /// Script path.
    script: &'diagnosis PathBuf,
}
impl<'diagnosis> Diagnosis<'diagnosis> {
    pub async fn new(path: &'diagnosis PathBuf) -> Self {
        Self { script: path }
    }

    /// Launch an error diagnostic and stop the execution.
    pub async fn diagnosis(&self, msg: &str, pos: Pos, script: &mut Script) {
        let chunk_1: String = format!(
            "{} {} {}\n{}{} {} ",
            msg,
            "→".red().bold(),
            //(lits::std_resources::SRC.to_string()+ "/"+
            match self.script.to_str() {
                Some(path) => path,
                None => "",
            }
            .bold(),
            INDENT_LIT,
            pos.line,
            "|".bold(),
        );

        let mut chunk_2: String = String::new();
        let mut chunk_3: String = String::new();
        let mut lit: String = String::new();
        let mut i: usize = pos.grapheme; // Indicator position (`↑`) of the illegal token.
        {
            script.reset();
            while let Some(token) = script.next_token().await {
                // Skip the lines before the illegal token line.
                if token.pos.line != pos.line {
                    continue;
                }

                // Illegal token line.

                // Subtract the indentation from the indicator position, for reasons
                // that the indentation is omitted in the diagnosis.
                if token.pos.grapheme > 1 {
                    i -= token.pos.grapheme;
                }

                // First token on the line.
                let mut after_illegal_token: bool = if token.pos.grapheme == pos.grapheme {
                    lit = format!("{}", token.lit().await.unwrap());
                    true
                } else {
                    chunk_2.push_str(&format!("{}", token.lit().await.unwrap()));
                    false
                };
                while let Some(token) = script.next_raw_token().await {
                    // Ends after "printing" the illegal token line.
                    if token.pos.line > pos.line {
                        break;
                    }

                    // Illegal token.
                    if token.pos.grapheme == pos.grapheme {
                        after_illegal_token = true;
                        lit = format!("{}", token.lit().await.unwrap());
                        continue;
                    }

                    if after_illegal_token {
                        chunk_3.push_str(&format!("{}", token.lit().await.unwrap()));
                    } else {
                        chunk_2.push_str(&format!("{}", token.lit().await.unwrap()));
                    }
                }
                break;
            }
        }

        // Everything is printed until the `panic` is launched.
        panic!(
            "{}{}{}{}\n{}{}{} {}",
            chunk_1,
            chunk_2,
            lit.red().bold(),
            chunk_3,
            // Line 3: `↑ 1`
            INDENT_LIT,
            // `1 | error line`
            //   ^^^ +3
            " ".repeat(3 + pos.line.to_string().len() + if i > 1 { i - 1 } else { 0 }),
            "↑".red().bold(),
            pos.grapheme,
        );
    }
}
