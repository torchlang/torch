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
        pub const SEMICOLON: &str = ";";
        pub const SPACE: &str = " ";
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
    NonReserved(&'lit Box<[u8]>),
    Reserved(&'lit str),
}
impl<'lit> fmt::Display for Lit<'lit> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Lit::Reserved(lit) => f.write_str(lit),
            Lit::NonReserved(lit) => f.write_str(&String::from_utf8_lossy(lit)),
        }
    }
}
