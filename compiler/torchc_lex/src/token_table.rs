use self::Table::*;
use torchc_lits::{lits, Lit};

#[derive(Debug)]
#[repr(u8)]
pub enum Table {
    /// `identifier_name`
    Id(Option<Box<[u8]>>),
    /// `"..."`
    StringLit(Option<Box<[u8]>>),
    /// `'...'`
    CharLit(Option<Box<[u8]>>),
    /// `\n`
    EndOfStmt,
    /// `' '`<br>`\t`
    Whitespace,
    Illegal(Option<Box<[u8]>>),
}
impl Table {
    #[inline]
    pub async fn default() -> Self {
        Self::Illegal(None)
    }

    /// Obtain the token literal.
    #[inline]
    pub async fn lit(&self) -> Option<Lit> {
        Some(match self {
            Id(opt) | Illegal(opt) | CharLit(opt) | StringLit(opt) => match opt {
                Some(lit) => Lit::NonReserved(lit),
                None => return None,
            },
            Whitespace => Lit::Reserved(lits::token_table::SPACE),
            EndOfStmt => Lit::Reserved(lits::token_table::SEMICOLON),
        })
    }
}
