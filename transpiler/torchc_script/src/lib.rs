use async_trait::async_trait;
use torchc_lex::{lexer, Table, ToScript, Token};

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

    /// Obtain the next token taking into account all of them.
    pub async fn next_raw_token(&mut self) -> Option<&Token> {
        if self.i < self.tokens.len() {
            self.i += 1;
            Some(&self.tokens[self.i - 1])
        } else {
            None
        }
    }
    /// Get only the next valid token.
    pub async fn next_token(&mut self) -> Option<&Token> {
        while self.i < self.tokens.len() {
            self.i += 1;
            if self.tokens[self.i - 1].is(&Table::Whitespace).await {
                continue;
            }
            return Some(&self.tokens[self.i - 1]);
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
        let mut script: torchc_lex::Script = self.to_script().await;
        let mut tokens: Vec<Token> = vec![];
        while let Some(token) = lexer(&mut script).await {
            tokens.push(token);
        }

        Script { tokens, i: 0 }
    }
}
