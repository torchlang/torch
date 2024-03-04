use async_std::{
    fs::File,
    io::WriteExt,
    path::{Path, PathBuf},
};
use cgen::Stmt;
use std::hash::{DefaultHasher, Hash, Hasher};
use torchc_lits::lits;

/// It performs the evaluations, optimizations and others; to later generate the
/// C/C++ code of the script.
///
/// ---
/// _**Code Generator**_
#[derive(Debug)]
pub struct CGen<'cgen> {
    script: Vec<Stmt>,
    target: &'cgen Path,
}
impl<'cgen> CGen<'cgen> {
    pub fn new(script: Vec<Stmt>, target: &'cgen Path) -> Self {
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

        for stmt in &self.script {
            match stmt {
                Stmt::Fn(opt) => {
                    if let Some(fn_stmt) = opt {
                        let mut cpp_fn: String = String::new();
                        fn_stmt.cgen(&mut cpp_fn);
                        cpp.write_all(cpp_fn.as_bytes())
                            .await
                            .unwrap_or_else(|err| panic!("{}", err));
                    }
                }
                Stmt::Global(_) => {}
            }
        }
    }
}

pub mod cgen {
    use torchc_lex::Token;
    use torchc_lits::lits;

    /// Transpilation mode.
    #[derive(Debug, PartialEq)]
    #[repr(u8)]
    pub enum Mode {
        /// Ready for production.
        Release,
        /// Debugging and/or testing.
        Dev,
    }

    /// Language statements.
    #[derive(Debug)]
    #[repr(u8)]
    pub enum Stmt {
        Global(Option<Vec<Self>>),
        Fn(Option<Fn>),
    }

    /// **Statement:**
    ///
    /// `fn name(var arg1 = type <lit>, arg2 = type <lit>, ...) var type <lit>, type <lit>, ...`<br>
    /// &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`...`
    #[derive(Debug)]
    pub struct Fn {
        pub name: Token,
        pub body: Vec<Stmt>,
    }
    impl Fn {
        pub fn new() -> Self {
            Self {
                name: Token::new(),
                body: vec![],
            }
        }
        /// Generate C/C++ function code.
        pub fn cgen(&self, cpp_fn: &mut String) {
            // Function name.
            match self.name.lit() {
                Some(lit) => cpp_fn.push_str(&format!(" {}", lit)),
                None => {
                    cpp_fn.push(' ');
                    cpp_fn.push_str(lits::cgen::DEFAULT_ID);
                }
            }

            // Function body.
            cpp_fn.push('{');
            for stmt in &self.body {
                match stmt {
                    Stmt::Fn(opt) => match opt {
                        Some(fn_stmt) => {
                            fn_stmt.cgen(cpp_fn);
                        }
                        None => {}
                    },
                    Stmt::Global(_) => {}
                }
            }
            cpp_fn.push('}');
        }
    }
}
