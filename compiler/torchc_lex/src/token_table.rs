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
    pub async fn default() -> Self {
        Illegal(None)
    }

    /// Check what the token identifier is.
    pub async fn is(&self, cmp: &Self) -> bool {
        match self {
            Id(_) => match cmp {
                Id(_) => true,
                _ => false,
            },
            EndOfStmt => match cmp {
                EndOfStmt => true,
                _ => false,
            },
            CharLit(_) => match cmp {
                CharLit(_) => true,
                _ => false,
            },
            StringLit(_) => match cmp {
                StringLit(_) => true,
                _ => false,
            },
            Whitespace => match cmp {
                Whitespace => true,
                _ => false,
            },
            Illegal(_) => match cmp {
                Illegal(_) => true,
                _ => false,
            },
        }
    }

    /// Obtain the token literal.
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
