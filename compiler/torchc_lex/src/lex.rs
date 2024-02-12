use super::{Script, Table, Token};

/// Get the following token from the script.
///
/// ---
/// _**Lexicographic Analyzer**_
pub async fn lexer<'lexer>(script: &mut Script<'lexer>) -> Option<Token> {
    let mut token: Token = Token::new().await;
    let mut lit: String = String::new();

    while let Some(c) = script.peek_char().await {
        match c {
            // Skip token(s).
            '\r' => {
                script.next_char().await.unwrap();
                continue;
            }

            // Statement separator(s).
            '\n' => {
                script.next_char().await.unwrap();
                token.pos = script.pos.clone();
                token.lexeme = Table::EndOfStmt;

                while let Some(c) = script.peek_char().await {
                    match c {
                        '\r' | '\n' => {
                            script.next_char().await.unwrap();
                        }
                        _ => break,
                    }
                }
            }
            // Whitespaces.
            ' ' | '\t' => {
                script.next_char().await.unwrap();
                token.pos = script.pos.clone();
                token.lexeme = Table::Whitespace;

                while let Some(c) = script.peek_char().await {
                    match c {
                        ' ' | '\t' => {
                            script.next_char().await.unwrap();
                        }
                        _ => break,
                    }
                }
            }

            // Identifier or name.
            'a'..='z' | 'A'..='Z' | '_' => {
                lit.push(script.next_char().await.unwrap());
                token.pos = script.pos.clone();

                while let Some(c) = script.peek_char().await {
                    match c {
                        'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                            lit.push(script.next_char().await.unwrap())
                        }
                        _ => break,
                    }
                }
                token.lexeme = Table::Id(Some(lit.into_bytes().into_boxed_slice()));
            }

            // String literal.
            '"' => {
                script.next_char().await.unwrap();
                token.pos = script.pos.clone();

                while let Some(c) = script.next_char().await {
                    match c {
                        '"' => break,
                        _ => lit.push(c),
                    }
                }
                token.lexeme = Table::StringLit(Some(lit.into_bytes().into_boxed_slice()));
            }
            // Character literal.
            '\'' => {
                script.next_char().await.unwrap();
                token.pos = script.pos.clone();

                while let Some(c) = script.next_char().await {
                    match c {
                        '\'' => break,
                        _ => lit.push(c),
                    }
                }
                token.lexeme = Table::CharLit(Some(lit.into_bytes().into_boxed_slice()));
            }

            // Illegal token.
            _ => {
                token.lexeme = Table::Illegal(Some(
                    script
                        .next_char()
                        .await
                        .unwrap()
                        .to_string()
                        .into_bytes()
                        .into_boxed_slice(),
                ));
                token.pos = script.pos.clone();
            }
        }
        return Some(token);
    }
    None
}
