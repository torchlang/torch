use async_trait::async_trait;
use std::slice::Iter;
use torchc_lex::{lexer, Table, ToScript, Token};

/// Handle the script by language tokens.
#[derive(Debug)]
pub struct Script {
    tokens: Vec<Token>,
}
impl Script {
    #[inline]
    pub async fn iter(&mut self) -> Tokens {
        Tokens::new(self.tokens.iter()).await
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

        Script { tokens }
    }
}
#[derive(Debug)]
pub struct Tokens<'tokens> {
    iter: Iter<'tokens, Token>,
}
impl<'tokens> Tokens<'tokens> {
    #[inline]
    pub async fn new(iter: Iter<'tokens, Token>) -> Self {
        Self { iter }
    }
    /// Obtain the next token taking into account all of them.
    #[inline]
    pub async fn next_raw_token(&mut self) -> Option<&Token> {
        self.iter.next()
    }
    /// Get only the next valid token.
    #[inline]
    pub async fn next_token(&mut self) -> Option<&Token> {
        while let Some(token) = self.iter.next() {
            if token.is(&Table::Whitespace).await {
                continue;
            }
            return Some(token);
        }
        None
    }
}
