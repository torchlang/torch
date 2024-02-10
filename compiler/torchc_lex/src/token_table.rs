use self::Table::*;
use torchc_lits::lits;

#[derive(Debug)]
pub enum Table {
    /// `identifier_name`
    Id(Option<String>),
    /// `\n`
    StmtSep,
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
            Id(opt) | Illegal(opt) => match opt {
                Some(lit) => lit,
                None => return None,
            },
            StmtSep => lits::token_table::NEWLINE,
        })
    }
}