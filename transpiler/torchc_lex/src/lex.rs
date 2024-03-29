use torchc_lits::lits;

use super::{Script, Table, Token};

/// Get the following token from the script.
///
/// ---
/// _**Lexicographic Analyzer**_
pub fn lexer<'lexer>(script: &mut Script<'lexer>) -> Option<Token> {
    let mut token: Token = Token::new();
    let mut lit: String = String::new();
    let mut prer: bool = false; // The previous character is `\r`?

    while let Some(c) = script.peek_char() {
        match c {
            // Skip token(s).
            '\r' => {
                script.next_char().unwrap();
                prer = true;
                continue;
            }

            // Statement separator(s).
            '\n' => {
                token.pos = script.pos;
                if !prer {
                    token.pos.grapheme += 1;
                }
                script.next_char().unwrap();
                token.lexeme = Table::EndOfStmt;

                while let Some(c) = script.peek_char() {
                    match c {
                        '\r' | '\n' => {
                            script.next_char().unwrap();
                        }
                        _ => break,
                    }
                }
            }
            // Whitespaces.
            ' ' | '\t' => {
                script.next_char().unwrap();
                token.pos = script.pos;
                token.lexeme = Table::Whitespace;

                while let Some(c) = script.peek_char() {
                    match c {
                        ' ' | '\t' => {
                            script.next_char().unwrap();
                        }
                        _ => break,
                    }
                }
            }

            // Identifier or name.
            'a'..='z' | 'A'..='Z' | '_' => {
                lit.push(script.next_char().unwrap());
                token.pos = script.pos;

                while let Some(c) = script.peek_char() {
                    match c {
                        'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                            lit.push(script.next_char().unwrap())
                        }
                        _ => break,
                    }
                }

                // Keyword or identifier.
                token.lexeme = match lit.as_str() {
                    lits::token_table::FN => Table::Fn,
                    _ => Table::Id(Some(lit.into_bytes().into_boxed_slice())),
                };
            }

            // String literal.
            '"' => {
                lit.push(script.next_char().unwrap());
                token.pos = script.pos;

                while let Some(c) = script.next_char() {
                    lit.push(c);
                    match c {
                        '"' => break,
                        _ => {}
                    }
                }
                token.lexeme = Table::StringLit(Some(lit.into_bytes().into_boxed_slice()));
            }
            // Character literal.
            '\'' => {
                lit.push(script.next_char().unwrap());
                token.pos = script.pos;

                while let Some(c) = script.next_char() {
                    lit.push(c);
                    match c {
                        '\'' => break,
                        _ => {}
                    }
                }
                token.lexeme = Table::CharLit(Some(lit.into_bytes().into_boxed_slice()));
            }

            // Comment or division symbol.
            '/' => {
                script.next_char().unwrap();
                token.pos = script.pos;

                if let Some(c) = script.peek_char() {
                    // Comment.
                    if *c == '/' {
                        script.next_char().unwrap();
                        token.lexeme = Table::Cmt(None); // Comment tokens are added in
                                                         // the retokenization.
                        return Some(token);
                    }
                }
                token.lexeme = Table::DivisionSym;
            }

            // Illegal token.
            _ => {
                // Advance the position counter (without advancing the iteration of
                // the script) and save the position before getting the grapheme since
                // doing it later could misalign it because it could advance more than
                // 2 positions.
                let c: char = *c;
                token.pos = script.pos;
                token.pos.advance(c);

                token.lexeme = Table::Illegal(Some(script.next_grapheme().unwrap()));
            }
        }
        return Some(token);
    }
    None
}
