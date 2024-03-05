use self::iter::{
    Feature,
    Mode::{Next, Peek},
};
use async_std::{fs, io, path::Path};
use torchc_lex::{lexer, Table, ToScript, Token};

pub mod iter {
    #[derive(Debug)]
    #[repr(u8)]
    pub enum Mode {
        /// Obtain and advance.
        Next(Feature),
        /// Obtain but not advance.
        Peek(Feature),
    }
    #[derive(Debug, PartialEq, Clone, Copy)]
    #[repr(u8)]
    pub enum Feature {
        /// Iterate all tokens.
        Default,
        /// Iterate only the code.
        Code,
        /// Iterate only the code and comments.
        CodeAndCmts,
    }
}

/// Handle the script by language tokens.
#[derive(Debug)]
pub struct Script {
    tokens: Vec<Token>,
    i: usize,
}
impl Script {
    /// Reset the iterator to the beginning.
    pub fn reset(&mut self) {
        self.i = 0;
    }

    /// Iterate according to the selected mode.
    pub fn token(&mut self, mode: iter::Mode) -> Option<&Token> {
        let mut i: usize = self.i;
        while i < self.tokens.len() {
            i += 1;
            match mode {
                Peek(ft) | Next(ft) => {
                    if let Next(_) = mode {
                        self.i += 1;
                    }

                    if ft == Feature::Code
                        && (self.tokens[i - 1].is(&Table::Whitespace)
                            || self.tokens[i - 1].is(&Table::Cmt(None)))
                        || ft == Feature::CodeAndCmts && self.tokens[i - 1].is(&Table::Whitespace)
                    {
                        continue;
                    }
                }
            }
            return Some(&self.tokens[i - 1]);
        }
        None
    }

    /// Interpret the script by tokens.
    pub async fn script(path: &Path) -> io::Result<Script> {
        Ok({
            {
                let contents: String = fs::read_to_string(path).await?;
                let mut script: torchc_lex::Script = contents.to_script();
                let mut tokens: Vec<Token> = vec![];
                while let Some(token) = lexer(&mut script) {
                    tokens.push(token);
                }
                // Replace `EOF` with `\n` (automatic end of statement).
                if !tokens[tokens.len() - 1].is(&Table::EndOfStmt) {
                    let mut token: Token = Token::new();
                    token.pos = tokens[tokens.len() - 1].pos;
                    token.pos.grapheme = script.pos.grapheme + 1;
                    token.lexeme = Table::EndOfStmt;
                    tokens.push(token);
                }
                Script { tokens, i: 0 }
            }
            .retokenizer()
        }
        .filter())
    }
    /// Restructures the most complex tokens.
    fn retokenizer(&mut self) -> Self {
        let mut tokens: Vec<Token> = vec![];
        let mut end_of_multiline_cmt_with_newline: bool = false;
        let mut end_of_stmt: Token = Token::new();

        while let Some(token) = self.token(Next(Feature::Default)) {
            let mut token: Token = token.clone();

            // Move the following tokens belonging to the comment content into `Cmt(_)`.
            if token.is(&Table::Cmt(None)) {
                let mut cmt: Vec<Token> = vec![];
                let indent: usize = token.pos.grapheme;

                // Comment content.
                while let Some(token) = self.token(Next(Feature::Default)) {
                    // Merges all comment tokens, including newlines,
                    // this is for code formatting.
                    cmt.push(token.clone());

                    if token.is(&Table::EndOfStmt) {
                        end_of_stmt = token.clone();

                        // Multiline commentary based on indentation.
                        if let Some(token) = self.token(Peek(Feature::CodeAndCmts)) {
                            if token.pos.grapheme > indent {
                                continue;
                            }
                        }
                        end_of_multiline_cmt_with_newline = true;
                        break;
                    }
                }

                // It is `Some(_)` only if content exists.
                if !cmt.is_empty() {
                    token.lexeme = Table::Cmt(Some(cmt));
                }
            }
            tokens.push(token);
            if end_of_multiline_cmt_with_newline {
                tokens.push(end_of_stmt.clone());
                end_of_multiline_cmt_with_newline = false;
            }
        }

        Script { tokens, i: 0 }
    }
    /// Filter consecutive special tokens.
    fn filter(&mut self) -> Self {
        let mut tokens: Vec<Token> = vec![];

        while let Some(token) = self.token(Next(Feature::Default)) {
            tokens.push(token.clone());
        }

        Script { tokens, i: 0 }
    }
}
