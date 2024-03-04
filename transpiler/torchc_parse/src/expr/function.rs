use super::IllegalIndentAccordingTo;
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

/// It recursively parses the function statement and obtains the _**cgen data**_.
pub fn function(
    script: &mut Script,
    diagnosis: &mut Diagnosis<'_>,
    stmt: &cgen::Stmt,
) -> cgen::Stmt {
    let mut fn_stmt: cgen::Fn = cgen::Fn::new();
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
        None => return cgen::Stmt::Fn(None),
    }

    // `fn name(var arg1 = type <lit>, arg2 = type <lit>, ...) var type <lit>, type <lit>, ...`
    //     ^^^^
    match script.token(Peek(Feature::Code)) {
        Some(token) if !token.is(&Table::EndOfStmt) => {
            if token.is(&Table::Id(None)) {
                pos = token.pos;
                pos.grapheme += token.len();
                fn_stmt.name = token.clone();
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

        // Checks for valid statements within the function.
        if token.is(&Table::Fn) {
            diagnosis.diagnosis(
                "illegal indentation"
                    .to_string()
                    .illegal_indent_according_to(stmt, &cgen::Stmt::Fn(None)),
                token.pos,
                script,
            );
        }

        fn_stmt.body.push(parser(script, diagnosis, stmt));
    }

    cgen::Stmt::Fn(if let cgen::Stmt::Fn(_) = stmt {
        Some(fn_stmt)
    } else {
        None
    })
}
