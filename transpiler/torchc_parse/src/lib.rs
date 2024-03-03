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
pub fn parser(
    script: &mut Script,
    diagnosis: &mut Diagnosis<'_>,
    parent_expr: &cgen::Expr,
) -> cgen::Expr {
    let mut globals: Vec<cgen::Expr> = vec![];

    while let Some(token) = script.token(Peek(Feature::Code)) {
        // Function expression (statement or prototype).
        if token.is(&Table::Fn) {
            let fn_expr: cgen::Expr = cgen::Expr::Fn(None);
            if let cgen::Expr::Global(_) = parent_expr {
                globals.push(expr::function(script, diagnosis, &fn_expr));
            }

            // Illegal token.
        } else {
            diagnosis.diagnosis("illegal token", token.pos, script);
        }
    }

    if let cgen::Expr::Global(_) = parent_expr {
        cgen::Expr::Global(Some(globals))
    } else {
        cgen::Expr::Global(None)
    }
}
