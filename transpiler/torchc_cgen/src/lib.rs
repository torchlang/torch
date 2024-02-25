use async_std::{
    fs,
    path::{Path, PathBuf},
};
use cgen::Global;
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
    dot_target: &'cgen Path,
}
impl<'cgen> CGen<'cgen> {
    pub fn new(script: Vec<Global>, dot_target: &'cgen Path) -> Self {
        Self { script, dot_target }
    }
    /// Generate the C/C++ code of the script (_file-to-file_).
    pub async fn cgen(&self, script: &Path, mode: cgen::Mode) {
        let mut dot_target: PathBuf = self.dot_target.to_path_buf();
        if mode == cgen::Mode::Dev {
            dot_target.push(lits::std_resources::dot_target::DEV);
        }
        // `.../.target/` or `.../.target/dev/`
        fs::create_dir_all(&dot_target)
            .await
            .unwrap_or_else(|err| panic!("{}", err));
        dot_target.push({
            let mut hasher: DefaultHasher = DefaultHasher::new();
            script.hash(&mut hasher);
            // `xxxxxxxxxxxxxxxxxxx.cpp`
            &(hasher.finish().to_string() + lits::DOT + lits::extensions::CPP)
        });

        println!("({})", dot_target.display());
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
        name: Token,
    }
    /// **Prototype:**
    ///
    /// `extern <backend> fn name(var arg1 = type <lit>, arg2 = type <lit>, ...) var type <lit>, type <lit>, ...`
    #[derive(Debug)]
    pub struct ProtoFn {
        name: Token,
    }
}
