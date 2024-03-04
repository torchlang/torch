use super::{Table::*, Token};
use torchc_lits::{lits, Lit, NonReserved};

#[derive(Debug, Clone)]
#[repr(u8)]
pub enum Table {
    /// `identifier_name`
    Id(Option<Box<[u8]>>),
    /// `fn`
    Fn,
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
    pub fn default() -> Self {
        Illegal(None)
    }

    /// Check what the token identifier is.
    pub fn is(&self, cmp: &Self) -> bool {
        match self {
            Id(_) => match cmp {
                Id(_) => true,
                _ => false,
            },
            Fn => match cmp {
                Fn => true,
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
    pub fn lit(&self) -> Option<Lit> {
        Some(match self {
            Id(opt) | Illegal(opt) | CharLit(opt) | StringLit(opt) => match opt {
                Some(lit) => Lit::NonReserved(NonReserved::Primitive(lit)),
                None => return None,
            },
            Fn => Lit::Reserved(lits::token_table::FN),
            Whitespace => Lit::Reserved(lits::token_table::SPACE),
            EndOfStmt => Lit::Reserved(lits::token_table::SEMICOLON_SYMBOL),
            DivisionSym => Lit::Reserved(lits::token_table::DIVISION_SYMBOL),
            Cmt(opt) => match opt {
                Some(tokens) => {
                    if !tokens.is_empty() {
                        let mut cmt: String = String::from(lits::token_table::CMT);
                        for token in tokens {
                            if let Some(lit) = token.lit() {
                                cmt.push_str(&format!("{}", lit));
                            }
                        }
                        Lit::NonReserved(NonReserved::Pseudo(cmt.into_bytes().into_boxed_slice()))
                    } else {
                        Lit::Reserved(lits::token_table::CMT)
                    }
                }
                None => Lit::Reserved(lits::token_table::CMT),
            },
        })
    }
    /// Count the clean length of the token.
    pub fn len(&self) -> usize {
        match self.lit() {
            Some(lit) => format!("{}", lit).len(),
            None => 0,
        }
    }
}
