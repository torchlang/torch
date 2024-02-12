use self::Table::*;
use torchc_lits::lits;

#[derive(Debug)]
#[repr(u8)]
pub enum Table {
    /// `identifier_name`
    Id(Option<String>),
    /// `"..."`
    StringLit(Option<String>),
    /// `'...'`
    CharLit(Option<String>),
    /// `\n`
    StmtSep,
    /// `' '`<br>`\t`
    Whitespace,
    Illegal(Option<String>),
}
impl Table {
    #[inline]
    pub async fn default() -> Self {
        Self::Illegal(None)
    }

    /// Obtain the token literal.
    #[inline]
    pub async fn lit(&self) -> Option<&str> {
        Some(match self {
            Id(opt) | Illegal(opt) | CharLit(opt) | StringLit(opt) => match opt {
                Some(lit) => lit,
                None => return None,
            },
            Whitespace => lits::token_table::SPACE,
            StmtSep => lits::token_table::NEWLINE,
        })
    }
}
