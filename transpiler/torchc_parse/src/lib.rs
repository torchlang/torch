use torchc_cgen::cgen;
use torchc_diagnosis::panic;
use torchc_lex::Table;
use torchc_script::{
    iter::{
        Feature,
        Mode::{Next, Peek},
    },
    Script,
};

mod stmt;

/// Parse the syntax of the script and obtain the _**cgen data**_.
///
/// ---
/// _**Syntactic Analyzer**_
pub fn parser(
    script: &mut Script,
    diagnosis: &mut panic::Diagnosis<'_>,
    parent_stmt: &cgen::Stmt,
) -> cgen::Stmt {
    let mut globals: Vec<cgen::Stmt> = vec![];

    // Child scope.
    //  - The statement without indentation is added in `globals` and
    //    with indentation it is returned (`return cgen::Stmt`).
    while let Some(token) = script.token(Peek(Feature::Code)) {
        // Skip tokens such as:
        //  - The ends of empty statements (the `EndOfStmt` are automatically added
        //    from newlines).
        if token.is(&Table::EndOfStmt) {
            script.token(Next(Feature::Code));
            continue;
        }

        // Function statement.
        if token.is(&Table::Fn) {
            let fn_stmt: cgen::Stmt = cgen::Stmt::Fn(None);
            if let cgen::Stmt::Global(_) = parent_stmt {
                globals.push(stmt::function(script, diagnosis, &fn_stmt));
            }

            // Illegal token.
        } else {
            diagnosis.diagnosis("illegal token", token.pos, script);
        }
    }

    // Global scope.
    if let cgen::Stmt::Global(_) = parent_stmt {
        cgen::Stmt::Global(Some(globals))
    } else {
        cgen::Stmt::Global(None)
    }
}
