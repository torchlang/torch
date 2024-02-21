use self::iter::{
    Feature,
    Mode::{Next, Peek},
};
use async_trait::async_trait;
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
    pub async fn token(&mut self, mode: iter::Mode) -> Option<&Token> {
        let mut i: usize = self.i;
        while i < self.tokens.len() {
            i += 1;
            match mode {
                Peek(ft) | Next(ft) => {
                    match mode {
                        Next(_) => {
                            self.i += 1;
                        }
                        Peek(_) => {}
                    }

                    if ft == Feature::Code
                        && (self.tokens[i - 1].is(&Table::Whitespace).await
                            || self.tokens[i - 1].is(&Table::Cmt(None)).await)
                        || ft == Feature::CodeAndCmts && self.tokens[i - 1].is(&Table::Whitespace).await
                    {
                        continue;
                    }
                }
            }
            return Some(&self.tokens[i - 1]);
        }
        None
    }
}

#[async_trait]
pub trait AsScript {
    /// Interprets the script string as a token vector.
    async fn as_script(&self) -> Script;
}
#[async_trait]
impl AsScript for String {
    async fn as_script(&self) -> Script {
        let mut script: Script = {
            let mut script: torchc_lex::Script = self.to_script().await;
            let mut tokens: Vec<Token> = vec![];
            while let Some(token) = lexer(&mut script).await {
                tokens.push(token);
            }
            // Replace `EOF` with `;`.
            if !tokens[tokens.len() - 1].is(&Table::EndOfStmt).await {
                let mut token: Token = Token::new().await;
                token.pos = tokens[tokens.len() - 1].pos;
                token.pos.grapheme = script.pos.grapheme + 1;
                token.lexeme = Table::EndOfStmt;
                tokens.push(token);
            }
            Script { tokens, i: 0 }
        };

        // Retokenizer.

        let mut tokens: Vec<Token> = vec![];

        while let Some(token) = script.token(Next(Feature::Default)).await {
            let mut token: Token = token.clone();

            // Move the following tokens belonging to the comment content into `Cmt(_)`.
            if token.is(&Table::Cmt(None)).await {
                let mut cmt: Vec<Token> = vec![];
                let indent: usize = token.pos.grapheme;

                // Comment content.
                while let Some(token) = script.token(Next(Feature::Default)).await {
                    if token.is(&Table::EndOfStmt).await {
                        // Multiline commentary based on indentation.
                        if let Some(token) = script.token(Peek(Feature::CodeAndCmts)).await {
                            if token.pos.grapheme > indent {
                                continue;
                            }
                        }
                        break;
                    }
                    cmt.push(token.clone());
                }

                // It is `Some(_)` only if content exists.
                if !cmt.is_empty() {
                    token.lexeme = Table::Cmt(Some(cmt));
                }
            }
            tokens.push(token);
        }

        Script { tokens, i: 0 }
    }
}
