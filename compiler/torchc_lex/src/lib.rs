pub use crate::{lex::lexer, token_table::Table};
use async_trait::async_trait;
use std::{iter::Peekable, slice::Iter, str::Chars};
use torchc_lits::Lit;

mod lex;
mod token_table;

#[derive(Debug)]
pub struct Token {
    pub lexeme: Table,
    pub pos: Pos,
}
impl Token {
    #[inline]
    pub async fn new() -> Self {
        Self {
            lexeme: Table::default().await,
            pos: Pos::default().await,
        }
    }

    /// Check what the token identifier is.
    #[inline]
    pub async fn is(&self, cmp: &Table) -> bool {
        self.lexeme.is(cmp).await
    }
    /// Obtain the token literal.
    #[inline]
    pub async fn lit(&self) -> Option<Lit> {
        self.lexeme.lit().await
    }
}

/// Script position.
#[derive(Debug, Clone)]
pub struct Pos {
    pub line: usize,
    pub grapheme: usize,
}
impl Pos {
    #[inline]
    pub async fn default() -> Self {
        Self {
            line: 1,
            grapheme: 0,
        }
    }

    /// It advances according to the unicode character and returns it.
    #[inline]
    pub async fn advance(&mut self, c: char) -> char {
        if c == '\n' {
            self.line += 1;
            self.grapheme = 0;
        } else {
            self.grapheme += 1;
        }
        c
    }
}

#[derive(Debug)]
pub struct Script<'script> {
    buf: Peekable<Chars<'script>>,
    pub pos: Pos,
}
impl<'script> Script<'script> {
    #[inline]
    pub async fn peek_char(&mut self) -> Option<&char> {
        self.buf.peek()
    }

    #[inline]
    pub async fn next_char(&mut self) -> Option<char> {
        match self.buf.next() {
            Some(c) => Some(self.pos.advance(c).await),
            None => None,
        }
    }
}
#[async_trait]
pub trait ToScript {
    async fn to_script(&self) -> Script;
}
#[async_trait]
impl ToScript for String {
    #[inline]
    async fn to_script(&self) -> Script {
        Script {
            buf: self.chars().peekable(),
            pos: Pos::default().await,
        }
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
