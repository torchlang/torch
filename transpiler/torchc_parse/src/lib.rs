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
    while let Some(token) = script.token(Peek(Feature::Code)) {
        // Function expression.
        if token.is(&Table::Fn) {
            let mut fn_expr: cgen::Expr = cgen::Expr::Fn(None);
            expr::function(script, diagnosis, &mut fn_expr);

            // Illegal token.
        } else {
            //diagnosis.diagnosis("illegal token", token.pos, script);
        }
        script.token(torchc_script::iter::Mode::Next(Feature::Code));
    }
}
