use core::fmt;

/// Global literals.
pub mod lits {
    pub const EPREFIX: &str = "error";
    pub const COLON: &str = ":";
    pub const DOT: &str = ".";
    pub const CURRENT: &str = "current";

    pub mod extensions {
        /// `file.t`
        pub const T: &str = "t";
        /// `file.h`
        pub const H: &str = "h";
        /// `file.c`
        pub const C: &str = "c";
        /// `file.cpp`
        pub const CPP: &str = "cpp";
        /// `file.c++`
        pub const CPP2: &str = "c++";
    }
    pub mod std_resources {
        /// Folder name for the code in the language.
        pub const SRC: &str = "src";

        /// Hidden compilation folder.
        pub mod dot_target {
            /// Compilation hidden folder name.
            pub const NAME: &str = ".target";
            /// Folder name for the development compilation.
            pub const DEV: &str = "dev";
        }
    }

    /// Token table literals.
    pub mod token_table {
        pub const SEMICOLON_SYMBOL: &str = ";";
        pub const SPACE: &str = " ";
        pub const DIVISION_SYMBOL: &str = "/";
        /// Commentator's literal.
        pub const CMT: &str = "//";
        pub const FN: &str = "fn";
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
