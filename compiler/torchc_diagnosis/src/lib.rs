use colored::Colorize;
use torchc_lex::Token;
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
    pub path: &'diagnosis str,
}
impl<'diagnosis> Diagnosis<'diagnosis> {
    pub async fn new(path: &'diagnosis str) -> Self {
        Self { path }
    }

    /// Launch an error diagnostic and stop the execution.
    pub async fn diagnosis(
        &self,
        msg: &str,
        etoken: Token, /* Error token */
        /*                ^     ^^^^^ etoken */
        script: &mut Script,
    ) {
        let chunk_1: String = format!(
            "{} {} {}\n{}{} {} ",
            msg,
            "→".red().bold(),
            self.path.bold(),
            INDENT_LIT,
            etoken.pos.line,
            "|".bold(),
        );

        let mut chunk_2: String = String::new();
        let mut chunk_3: String = String::new();
        {
            script.reset();
            while let Some(token) = script.next_token().await {
                // Skip the lines before the illegal token line.
                if token.pos.line != etoken.pos.line {
                    continue;
                }

                // Illegal token line.
                let mut after_illegal_token: bool = if token.pos.grapheme == etoken.pos.grapheme {
                    true
                } else {
                    chunk_2.push_str(&format!("{}", token.lit().await.unwrap()));
                    false
                };
                while let Some(token) = script.next_raw_token().await {
                    // Ends after "printing" the illegal token line.
                    if token.pos.line > etoken.pos.line {
                        break;
                        // Illegal token.
                    } else if token.pos.grapheme == etoken.pos.grapheme {
                        after_illegal_token = true;
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
            format!("{}", etoken.lit().await.unwrap()).red().bold(),
            chunk_3,
            // Line 3: `↑ 1`
            INDENT_LIT,
            // `1 | error line`
            //   ^^^ +3
            " ".repeat(
                3 + etoken.pos.line.to_string().len()
                    + if etoken.pos.grapheme > 1 {
                        etoken.pos.grapheme - 1
                    } else {
                        0
                    }
            ),
            "↑".red().bold(),
            etoken.pos.grapheme,
        );
    }
}
