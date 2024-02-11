use async_trait::async_trait;
use std::slice::Iter;
use torchc_lex::{lexer, ToScript, Token};

/// Handle the script by language tokens.
#[derive(Debug)]
pub struct Script {
    tokens: Vec<Token>,
}
impl Script {
    #[inline]
    pub async fn iter(&mut self) -> Iter<Token> {
        self.tokens.iter()
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
