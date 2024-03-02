use torchc_cgen::cgen;
use torchc_diagnosis::Diagnosis;
use torchc_lex::Table;
use torchc_script::{
    iter::{Feature, Mode::Peek},
    Script,
};

mod expr;

/// Parse the syntax of the script and obtain the _**cgen data**_.
///
/// ---
/// _**Syntactic Analyzer**_
pub fn parser(script: &mut Script, diagnosis: &mut Diagnosis<'_>, parent_expr: &mut cgen::Expr) {
    let mut global: Vec<cgen::Global> = vec![];

    while let Some(token) = script.token(Peek(Feature::Code)) {
        // Function expression (statement or prototype).
        if token.is(&Table::Fn) {
            let mut fn_expr: cgen::Expr = cgen::Expr::Fn(None);
            let mut fn_stmt: cgen::Fn = cgen::Fn::FnStmt(None);
            expr::function(script, diagnosis, &mut fn_expr, &mut fn_stmt, &mut global);

            // Illegal token.
        } else {
            diagnosis.diagnosis("illegal token", token.pos, script);
        }
    }

    // Save cgen data.
    match parent_expr {
        cgen::Expr::Global(_) => *parent_expr = cgen::Expr::Global(Some(global)),
        cgen::Expr::Fn(_) => {}
    }
}
