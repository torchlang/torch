use crate::parser;
use torchc_cgen::cgen;
use torchc_diagnosis::Diagnosis;
use torchc_lex::{Pos, Table};
use torchc_lits::lits;
use torchc_script::{
    iter::{
        Feature,
        Mode::{Next, Peek},
    },
    Script,
};

/// It recursively parses the function expression and obtains the _**cgen data**_.
pub fn function(
    script: &mut Script,
    diagnosis: &mut Diagnosis<'_>,
    expr: &cgen::Expr,
) -> cgen::Expr {
    let mut fn_expr: cgen::Fn = cgen::Fn::new();
    let mut indent: usize = 0;
    let mut pos: Pos = Pos::default();

    // `fn name(var arg1 = type <lit>, arg2 = type <lit>, ...) var type <lit>, type <lit>, ...`
    //  ^^
    match script.token(Peek(Feature::Code)) {
        Some(token) => {
            if token.is(&Table::Fn) {
                indent = token.pos.grapheme;
                pos = token.pos;
                pos.grapheme += token.len() + 1; // `+1` == space
                script.token(Next(Feature::Code)).unwrap();
            } else {
                diagnosis.diagnosis(
                    &format!("illegal, keyword '{}' was not found", lits::token_table::FN),
                    token.pos,
                    script,
                );
            }
        }
        None => return cgen::Expr::Fn(None),
    }

    // `fn name(var arg1 = type <lit>, arg2 = type <lit>, ...) var type <lit>, type <lit>, ...`
    //     ^^^^
    match script.token(Peek(Feature::Code)) {
        Some(token) if !token.is(&Table::EndOfStmt) => {
            if token.is(&Table::Id(None)) {
                pos = token.pos;
                pos.grapheme += token.len();
                fn_expr.name = token.clone();
                script.token(Next(Feature::Code)).unwrap();
            } else {
                diagnosis.diagnosis("illegal function name", token.pos, script);
            }
        }
        _ => diagnosis.diagnosis("expecting function name", pos, script),
    }

    // `fn name(var arg1 = type <lit>, arg2 = type <lit>, ...) var type <lit>, type <lit>, ...`
    //                                                                            End of line ^
    match script.token(Peek(Feature::Code)) {
        Some(token) if token.is(&Table::EndOfStmt) => {
            script.token(Next(Feature::Code)).unwrap();
        }
        _ => diagnosis.diagnosis("expecting newline", pos, script),
    }

    // Recursive indentation.
    while let Some(token) = script.token(Peek(Feature::Code)) {
        if token.pos.grapheme <= indent {
            break;
        }

        if token.is(&Table::Fn) {
            diagnosis.diagnosis("illegal indentation", token.pos, script);
        } else {
            fn_expr.body.push(parser(script, diagnosis, expr));
        }
    }

    cgen::Expr::Fn(if let cgen::Expr::Fn(_) = expr {
        Some(fn_expr)
    } else {
        None
    })
}
