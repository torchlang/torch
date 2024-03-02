pub use crate::{lex::lexer, token_table::Table};
use std::{iter::Peekable, str::Chars};
use torchc_lits::Lit;

mod lex;
mod token_table;

#[derive(Debug, Clone)]
pub struct Token {
    pub lexeme: Table,
    pub pos: Pos,
}
impl Token {
    pub fn new() -> Self {
        Self {
            lexeme: Table::default(),
            pos: Pos::default(),
        }
    }

    /// Check what the token identifier is.
    pub fn is(&self, cmp: &Table) -> bool {
        self.lexeme.is(cmp)
    }
    /// Count the clean length of the token.
    pub fn len(&self) -> usize {
        self.lexeme.len()
    }
    /// Obtain the token literal.
    pub fn lit(&self) -> Option<Lit> {
        self.lexeme.lit()
    }
}

/// Script position.
#[derive(Debug, Clone, Copy)]
pub struct Pos {
    pub line: usize,
    pub grapheme: usize,
}
impl Pos {
    pub fn default() -> Self {
        Self {
            line: 1,
            grapheme: 0,
        }
    }

    /// It advances according to the unicode character and returns it.
    pub fn advance(&mut self, c: char) -> char {
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
    pub fn peek_char(&mut self) -> Option<&char> {
        self.buf.peek()
    }
    /// Gets the next character from the script.
    pub fn next_char(&mut self) -> Option<char> {
        match self.buf.next() {
            Some(c) => Some(self.pos.advance(c)),
            None => None,
        }
    }
}
pub trait ToScript {
    fn to_script(&self) -> Script;
}
impl ToScript for String {
    fn to_script(&self) -> Script {
        Script {
            buf: self.chars().peekable(),
            pos: Pos::default(),
        }
    }
}
