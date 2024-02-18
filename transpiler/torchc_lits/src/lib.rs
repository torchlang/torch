use core::fmt;

/// Global literals.
pub mod lits {
    pub const EPREFIX: &str = "error";
    pub const COLON: &str = ":";

    pub mod std_resources {
        /// Folder name for the code in the language.
        pub const SRC: &str = "src";
    }

    /// Token table literals.
    pub mod token_table {
        pub const SEMICOLON_SYMBOL: &str = ";";
        pub const SPACE: &str = " ";
        pub const DIVISION_SYMBOL: &str = "/";
        /// Commentator's literal.
        pub const CMT: &str = "//";
    }
}

/// Token literal.
///
/// ---
/// > _It is always verified that the token literals are valid unicodes or graphemes
/// at the time of their creation._
#[derive(Debug)]
#[repr(u8)]
pub enum Lit<'lit> {
    /// Literals constructed during tokenization.
    NonReserved(NonReserved<'lit>),
    /// Predefined literals.
    Reserved(&'lit str),
}
impl<'lit> fmt::Display for Lit<'lit> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Lit::Reserved(lit) => f.write_str(lit),
            Lit::NonReserved(lit) => match lit {
                NonReserved::Primitive(lit) => f.write_str(&String::from_utf8_lossy(lit)),
                NonReserved::Pseudo(lit) => f.write_str(lit),
            },
        }
    }
}
#[derive(Debug)]
#[repr(u8)]
pub enum NonReserved<'non_reserved> {
    /// Non-modifiable primitives.
    Primitive(&'non_reserved Box<[u8]>),
    /// Temporary constructions according to the context.
    Pseudo(String),
}
