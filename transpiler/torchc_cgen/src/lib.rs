use async_std::{
    fs::File,
    io::WriteExt,
    path::{Path, PathBuf},
};
use cgen::{Fn, Global};
use std::hash::{DefaultHasher, Hash, Hasher};
use torchc_lits::lits;

/// It performs the evaluations, optimizations and others; to later generate the
/// C/C++ code of the script.
///
/// ---
/// _**Code Generator**_
#[derive(Debug)]
pub struct CGen<'cgen> {
    script: Vec<Global>,
    target: &'cgen Path,
}
impl<'cgen> CGen<'cgen> {
    pub fn new(script: Vec<Global>, target: &'cgen Path) -> Self {
        Self { script, target }
    }
    /// Generate the C/C++ code of the script (_file-to-file_).
    pub async fn cgen(&self, script: &Path) {
        let mut path: PathBuf = self.target.to_path_buf();
        path.push({
            let mut hasher: DefaultHasher = DefaultHasher::new();
            script.hash(&mut hasher);
            // `xxxxxxxxxxxxxxxxxxx.cpp`
            &(hasher.finish().to_string() + lits::DOT + lits::extensions::CPP)
        });
        let mut cpp: File = async_std::fs::File::create(&path)
            .await
            .unwrap_or_else(|err| panic!("{}", err));

        for expr in &self.script {
            match expr {
                Global::Fn(fn_expr) => match fn_expr {
                    Fn::FnStmt(opt) => {
                        if let Some(fn_stmt) = opt {
                            let mut cpp_fn: String = String::new();

                            match fn_stmt.name.lit() {
                                Some(lit) => cpp_fn.push_str(&format!(" {}", lit)),
                                None => {
                                    cpp_fn.push(' ');
                                    cpp_fn.push_str(lits::cgen::DEFAULT_ID);
                                }
                            }

                            cpp.write_all(cpp_fn.as_bytes())
                                .await
                                .unwrap_or_else(|err| panic!("{}", err));
                        }
                    }
                    Fn::ProtoFn(opt) => {
                        if let Some(protofn) = opt {
                            let mut cpp_protofn: String = String::new();

                            match protofn.name.lit() {
                                Some(lit) => cpp_protofn.push_str(&format!(" {}", lit)),
                                None => {
                                    cpp_protofn.push(' ');
                                    cpp_protofn.push_str(lits::cgen::DEFAULT_ID);
                                }
                            }

                            cpp.write_all(cpp_protofn.as_bytes())
                                .await
                                .unwrap_or_else(|err| panic!("{}", err));
                        }
                    }
                },
            };
        }
    }
}

pub mod cgen {
    use torchc_lex::Token;

    /// Transpilation mode.
    #[derive(Debug, PartialEq)]
    pub enum Mode {
        /// Ready for production.
        Release,
        /// Debugging and/or testing.
        Dev,
    }

    /// Language expressions.
    #[derive(Debug)]
    pub enum Expr {
        Global(Option<Vec<Global>>),
        Fn(Option<Fn>),
    }

    #[derive(Debug)]
    pub enum Global {
        Fn(Fn),
    }

    #[derive(Debug)]
    pub enum Fn {
        FnStmt(Option<FnStmt>),
        ProtoFn(Option<ProtoFn>),
    }
    /// **Statement:**
    ///
    /// `fn name(var arg1 = type <lit>, arg2 = type <lit>, ...) var type <lit>, type <lit>, ...`<br>
    /// &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`...`
    #[derive(Debug)]
    pub struct FnStmt {
        pub name: Token,
    }
    impl FnStmt {
        pub fn new() -> Self {
            Self { name: Token::new() }
        }
    }
    /// **Prototype:**
    ///
    /// `extern <backend> fn name(var arg1 = type <lit>, arg2 = type <lit>, ...) var type <lit>, type <lit>, ...`
    #[derive(Debug)]
    pub struct ProtoFn {
        pub name: Token,
    }
    impl ProtoFn {
        pub fn new() -> Self {
            Self { name: Token::new() }
        }
    }
}
