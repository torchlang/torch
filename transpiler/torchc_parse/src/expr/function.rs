use colored::Colorize;
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
    parent_expr: &mut cgen::Expr,
    kind: &mut cgen::Fn,
    global: &mut Vec<cgen::Global>,
) {
    let mut fn_stmt: cgen::FnStmt = cgen::FnStmt::new();
    let mut protofn: cgen::ProtoFn = cgen::ProtoFn::new();
    let mut pos: Pos = Pos::default();

    // `fn name(var arg1 = type <lit>, arg2 = type <lit>, ...) var type <lit>, type <lit>, ...`
    //  ^^
    match script.token(Peek(Feature::Code)) {
        Some(token) => {
            if token.is(&Table::Fn) {
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
        None => return,
    }

    // `fn name(var arg1 = type <lit>, arg2 = type <lit>, ...) var type <lit>, type <lit>, ...`
    //     ^^^^
    match script.token(Peek(Feature::Code)) {
        Some(token) if !token.is(&Table::EndOfStmt) => {
            if token.is(&Table::Id(None)) {
                pos = token.pos;
                pos.grapheme += token.len();
                match kind {
                    cgen::Fn::FnStmt(_) => fn_stmt.name = token.clone(),
                    cgen::Fn::ProtoFn(_) => protofn.name = token.clone(),
                }
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

    // Save cgen data.
    match parent_expr {
        cgen::Expr::Fn(_) => global.push(cgen::Global::Fn(match kind {
            cgen::Fn::FnStmt(_) => cgen::Fn::FnStmt(Some(fn_stmt)),
            cgen::Fn::ProtoFn(_) => cgen::Fn::ProtoFn(Some(protofn)),
        })),
        _ => {
            #[cfg(debug_assertions)]
            panic!(
                "illegal expression in {}{}{}",
                "match parent_expr { ".bold(),
                "cgen::Expr::Option => {}".red().bold(),
                " }".bold()
            );
            #[cfg(not(debug_assertions))]
            panic!();
        }
    }
}
