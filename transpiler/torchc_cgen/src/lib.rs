use async_std::path::{Path, PathBuf};
use cgen::Global;
use torchc_lits::{lits, Lit};

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
    pub fn cgen(&self, script: &Path, i: &mut usize) {
        let mut path: PathBuf = self.dot_target.to_path_buf();
        {
            let mut filename: String = match script.file_stem() {
                Some(filename) => match filename.to_str() {
                    Some(filename) => filename.to_string(),
                    None => {
                        *i += 1;
                        i.to_string()
                    }
                },
                None => {
                    *i += 1;
                    i.to_string()
                }
            };
            filename.push_str(&(".".to_string() + lits::extensions::CPP));
            path.push(filename);
        }

        println!("({})", path.display());
    }
}

pub mod cgen {
    use torchc_lex::Token;

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
