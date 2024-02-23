use async_std::path::Path;
use colored::Colorize;
use pathdiff::diff_paths;
use torchc_lex::{Pos, Table};
use torchc_lits::lits;
use torchc_script::{
    iter::{Feature, Mode::Next},
    Script,
};

/// Handles the diagnosis of language errors.
///
/// ---
/// **Format:**
///
/// `error: message → src/file`<br>
/// &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
/// `1 | line`<br>
/// &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
/// &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`↑ 1`
#[derive(Debug)]
pub struct Diagnosis<'diagnosis> {
    /// Script path.
    script: &'diagnosis Path,
    cwd: &'diagnosis Path,
}
impl<'diagnosis> Diagnosis<'diagnosis> {
    pub fn new(path: &'diagnosis Path, cwd: &'diagnosis Path) -> Self {
        Self { script: path, cwd }
    }

    /// Launch an error diagnostic and stop the execution.
    pub fn diagnosis(&self, msg: &str, pos: Pos, script: &mut Script) {
        // `error: message → src/file`
        //       ^^ +2
        let mut indent: String = " ".repeat(lits::EPREFIX.len() + 2);

        let chunk_1: String = format!(
            "{} {} {}\n{}{} {} ",
            msg,
            "→".red().bold(),
            match diff_paths(self.script, self.cwd) {
                Some(src) => match src.to_str() {
                    Some(path) => String::from(path),
                    None => String::new(),
                },
                None => String::from(lits::std_resources::SRC),
            }
            .bold(),
            indent,
            pos.line,
            "|".bold(),
        );

        let mut chunk_2: String = String::new();
        let mut chunk_3: String = String::new();
        let mut lit: String = String::new();
        let mut i: usize = pos.grapheme; // Indicator position (`↑`) of the illegal token.
        {
            script.reset();
            while let Some(token) = script.token(Next(Feature::Code)) {
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
                    lit = format!("{}", token.lit().unwrap());
                    true
                } else {
                    chunk_2.push_str(&format!("{}", token.lit().unwrap()));
                    false
                };
                while let Some(token) = script.token(Next(Feature::Default)) {
                    // Ends after "printing" the illegal token line.
                    if token.is(&Table::EndOfStmt) || token.pos.line > pos.line {
                        break;
                    }

                    // Illegal token.
                    if token.pos.grapheme == pos.grapheme {
                        after_illegal_token = true;
                        lit = format!("{}", token.lit().unwrap());
                        continue;
                    }

                    if after_illegal_token {
                        chunk_3.push_str(&format!("{}", token.lit().unwrap()));
                    } else {
                        chunk_2.push_str(&format!("{}", token.lit().unwrap()));
                    }
                }
                break;
            }
        }

        // `1 | error line`
        //   ^^^ +3
        indent
            .push_str(&" ".repeat(3 + pos.line.to_string().len() + if i > 1 { i - 1 } else { 0 }));

        // Everything is printed until the `panic` is launched.
        panic!(
            "{}{}{}{}\n{}{} {}",
            chunk_1,
            chunk_2,
            lit.red().bold(),
            chunk_3,
            // Line 3: `↑ 1`
            indent,
            "↑".red().bold(),
            pos.grapheme,
        );
    }
}
