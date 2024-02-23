use self::cgen::Expr;

/// It performs the evaluations, optimizations and others; to later generate the
/// C/C++ code of the whole program.
///
/// ---
/// _**Code Generator**_
#[derive(Debug)]
pub struct CGen {
    cgen: Expr,
}

pub mod cgen {
    use torchc_lex::Token;

    /// Language expressions.
    #[derive(Debug)]
    pub enum Expr {
        Fn(Option<Fn>),
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
