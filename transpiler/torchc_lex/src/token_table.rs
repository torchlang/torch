use std::{future::Future, pin::Pin};

use super::{Table::*, Token};
use torchc_lits::{lits, Lit, NonReserved};

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
    /// `/`
    DivisionSym,
    /// `//...`
    Cmt(Option<Vec<Token>>),
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
            DivisionSym => match cmp {
                DivisionSym => true,
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
            Cmt(_) => match cmp {
                Cmt(_) => true,
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
                Some(lit) => Lit::NonReserved(NonReserved::Primitive(lit)),
                None => return None,
            },
            Whitespace => Lit::Reserved(lits::token_table::SPACE),
            EndOfStmt => Lit::Reserved(lits::token_table::SEMICOLON_SYMBOL),
            DivisionSym => Lit::Reserved(lits::token_table::DIVISION_SYMBOL),
            Cmt(opt) => match opt {
                Some(tokens) => {
                    let mut lit: String = String::from(lits::token_table::CMT);
                    for token in tokens {
                        let future: Pin<Box<dyn Future<Output = Option<Lit>>>> =
                            Box::pin(token.lit());
                        lit.push_str(&match future.await {
                            Some(lit) => format!("{}", lit),
                            None => String::new(),
                        })
                    }
                    Lit::NonReserved(NonReserved::Pseudo(lit))
                }
                None => Lit::Reserved(lits::token_table::CMT),
            },
        })
    }
}
