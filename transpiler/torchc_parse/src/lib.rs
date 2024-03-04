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
    parent_stmt: &cgen::Stmt,
) -> cgen::Stmt {
    let mut globals: Vec<cgen::Stmt> = vec![];

    // Child scope.
    //  - The statement without indentation is added in `globals` and
    //    with indentation it is returned (`return cgen::Stmt`).
    while let Some(token) = script.token(Peek(Feature::Code)) {
        // Function statement.
        if token.is(&Table::Fn) {
            let fn_expr: cgen::Stmt = cgen::Stmt::Fn(None);
            if let cgen::Stmt::Global(_) = parent_stmt {
                globals.push(expr::function(script, diagnosis, &fn_expr));
            }

            // Illegal token.
        } else {
            diagnosis.diagnosis("illegal token", token.pos, script);
        }
    }

    // Global scope.
    let parent_expr = cgen::Stmt::Fn(None);
    if let cgen::Stmt::Global(_) = parent_expr {
        cgen::Stmt::Global(Some(globals))
    } else {
        cgen::Stmt::Global(None)
    }
}
