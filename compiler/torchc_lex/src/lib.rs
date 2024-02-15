pub use crate::{lex::lexer, token_table::Table};
use async_trait::async_trait;
use std::{iter::Peekable, str::Chars};
use torchc_lits::Lit;

mod lex;
mod token_table;

#[derive(Debug)]
pub struct Token {
    pub lexeme: Table,
    pub pos: Pos,
}
impl Token {
    pub async fn new() -> Self {
        Self {
            lexeme: Table::default().await,
            pos: Pos::default().await,
        }
    }

    /// Check what the token identifier is.
    pub async fn is(&self, cmp: &Table) -> bool {
        self.lexeme.is(cmp).await
    }
    /// Obtain the token literal.
    pub async fn lit(&self) -> Option<Lit> {
        self.lexeme.lit().await
    }
}

/// Script position.
#[derive(Debug, Clone, Copy)]
pub struct Pos {
    pub line: usize,
    pub grapheme: usize,
}
impl Pos {
    pub async fn default() -> Self {
        Self {
            line: 1,
            grapheme: 0,
        }
    }

    /// It advances according to the unicode character and returns it.
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
    /// Gets the next character from the script but does not advance.
    pub async fn peek_char(&mut self) -> Option<&char> {
        self.buf.peek()
    }
    /// Gets the next character from the script.
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
    async fn to_script(&self) -> Script {
        Script {
            buf: self.chars().peekable(),
            pos: Pos::default().await,
        }
    }
}
