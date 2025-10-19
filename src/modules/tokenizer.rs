use super::analyser::{AnalyzedChar, CharKind};
use super::token_stream::TokenStream;
use super::tokens::{AnalyzedToken, SYMBOLS, Token};
use crate::error_handling::{Error, ErrorType::LexingError, Result};

pub fn tokenize(chars: &[AnalyzedChar], path: &str) -> Result<TokenStream> {
    let mut char_buf: Vec<&AnalyzedChar> = Vec::new();
    let mut string_buf = String::new();
    let mut result_vec: Vec<AnalyzedToken> = Vec::new();

    let mut iter = chars.iter().peekable();

    while let Some(curr) = iter.next() {
        match &curr.kind {
            CharKind::Whitespace | CharKind::Newline => {
                char_buf_clear(&mut char_buf, &mut string_buf, &mut result_vec, path)?;
            }
            CharKind::Char(c) => {
                if *c == '"' {
                    char_buf_clear(&mut char_buf, &mut string_buf, &mut result_vec, path)?;
                    let lit = handle_string_literal(&mut iter, curr.line, curr.column, path)?;
                    result_vec.push(lit);
                } else if *c == '\'' {
                    char_buf_clear(&mut char_buf, &mut string_buf, &mut result_vec, path)?;
                    let lit = handle_char_literal(&mut iter, curr.line, curr.column, path)?;
                    result_vec.push(lit);
                } else if is_symbol_start(*c) {
                    char_buf_clear(&mut char_buf, &mut string_buf, &mut result_vec, path)?;

                    if let Some(next) = iter.peek() {
                        if let CharKind::Char(nc) = next.kind {
                            if let Some(symbol) = try_two_char_symbol(*c, nc) {
                                if symbol == ("//") {
                                    handle_command_line(&mut iter)?;
                                } else {
                                    let mut t = Token::Symbol(symbol.clone());
                                    if symbol == ("==") {
                                        t = Token::Equals;
                                    } else if symbol == ("!=") {
                                        t = Token::NotEquals;
                                    } else if symbol == ("&&") {
                                        t = Token::And;
                                    } else if symbol == ("||") {
                                        t = Token::Or;
                                    } else if symbol == ("<=") {
                                        t = Token::LessEqual;
                                    } else if symbol == (">=") {
                                        t = Token::GreaterEqual;
                                    } else if symbol == ("->") {
                                        t = Token::Arrow;
                                    } else if symbol == ("-<") {
                                        t = Token::ChainBack;
                                    } else if symbol == ("-!") {
                                        t = Token::ChainNot;
                                    } else if symbol == ("<-") {
                                        t = Token::ReverseArrow;
                                    } else if symbol == ("::") {
                                        t = Token::DoubleColon;
                                    }
                                    result_vec.push(AnalyzedToken {
                                        token: t,
                                        line: curr.line,
                                        column: curr.column,
                                        length: 2,
                                    });
                                    iter.next();
                                }
                                continue;
                            }
                        }
                    }
                    let this_token = match c {
                        ';' => Token::Semicolon,
                        '=' => Token::Equals,
                        ':' => Token::Colon,
                        '{' => Token::LBrace,
                        '}' => Token::RBrace,
                        ')' => Token::RightParen,
                        '(' => Token::LeftParen,
                        '[' => Token::LBracket,
                        ']' => Token::RBracket,
                        '+' => Token::Plus,
                        '-' => Token::Minus,
                        '*' => Token::Star,
                        '/' => Token::Slash,
                        '%' => Token::Percent,
                        '^' => Token::Power,
                        '?' => Token::Question,
                        '!' => Token::Not,
                        _ => Token::Symbol(c.to_string()),
                    };
                    result_vec.push(AnalyzedToken {
                        token: this_token,
                        line: curr.line,
                        column: curr.column,
                        length: 1,
                    });
                } else {
                    char_buf.push(curr);
                }
            }
        }
    }
    Ok(TokenStream::new(result_vec))
}

fn final_buffer(
    tokens: &mut Vec<AnalyzedToken>,
    buf: &mut String,
    lin: usize,
    col: usize,
    path: &str,
) -> Result<()> {
    if !buf.is_empty() {
        let t = token_of(buf, lin, col, path)?;
        if let Token::If = t {
            let last_token = tokens.last_mut().unwrap();
            if let Token::Else = last_token.token {
                last_token.token = Token::ElseIf;
                last_token.length = 7;
                buf.clear();
                return Ok(());
            }
        }
        tokens.push(AnalyzedToken {
            token: t,
            line: lin,
            column: col,
            length: buf.len(),
        });

        buf.clear();
    }
    Ok(())
}

fn token_of(st: &str, t_lin: usize, t_col: usize, path: &str) -> Result<Token> {
    if SYMBOLS.contains(st) {
        Ok(Token::Symbol(st.to_string()))
    } else if st.chars().all(|c| c.is_ascii_digit()) {
        Ok(Token::Number(st.parse::<i32>().unwrap()))
    } else {
        match st {
            "let" => Ok(Token::Let),
            "op" => Ok(Token::Op),
            "if" => Ok(Token::If),
            "else" => Ok(Token::Else),
            "while" => Ok(Token::While),
            "for" => Ok(Token::For),
            "loop" => Ok(Token::Loop),
            "success" => Ok(Token::Success),
            "i32" => Ok(Token::IntType("i32".to_string())),
            "string" => Ok(Token::String),
            "char" => Ok(Token::Char),
            "f32" => Ok(Token::FloatType("f32".to_string())),
            "bool" => Ok(Token::Bool),
            "break" => Ok(Token::Break),
            "continue" => Ok(Token::Continue),
            "true" => Ok(Token::True),
            "false" => Ok(Token::False),
            "nret" => Ok(Token::Nret),
            "data" => Ok(Token::Data),
            "and" => Ok(Token::Symbol("&&".to_string())),
            "or" => Ok(Token::Symbol("||".to_string())),
            _ => {
                if is_valid_identifier(st, t_lin, t_col, path)? {
                    Ok(Token::Identifier(st.to_string()))
                } else {
                    Err(Error::new(LexingError)
                        .with_message(format!("Unexpected token! -> '{}'", st))
                        .with_line(t_lin)
                        .with_column(t_col)
                        .with_file_path(path))
                }
            }
        }
    }
}

fn is_valid_identifier(st: &str, lin: usize, col: usize, path: &str) -> Result<bool> {
    if st.is_empty() {
        return Ok(false);
    }
    let first_char = st.chars().next().unwrap();
    if !first_char.is_alphabetic() && first_char != '_' {
        return Err(Error::new(LexingError)
            .with_message(format!(
                "Invalid identifier: Identifier cannot start with a number! -> '{st}'"
            ))
            .with_line(lin)
            .with_column(col)
            .with_file_path(path)
            .with_length(st.len()));
    }
    Ok(st.chars().all(|c| c.is_alphanumeric() || c == '_'))
}

fn is_symbol_start(c: char) -> bool {
    SYMBOLS.contains(c.to_string().as_str())
}
fn try_two_char_symbol(first: char, second: char) -> Option<String> {
    let combined = format!("{}{}", first, second);
    if SYMBOLS.contains(combined.as_str()) {
        Some(combined.to_owned())
    } else {
        None
    }
}

fn handle_string_literal<'a, I>(
    iter: &mut std::iter::Peekable<I>,
    start_line: usize,
    start_col: usize,
    path: &str,
) -> Result<AnalyzedToken>
where
    I: std::iter::Iterator<Item = &'a AnalyzedChar>,
{
    let mut literal_buf = String::new();

    for curr in iter.by_ref() {
        match &curr.kind {
            CharKind::Char('"') => {
                let length = literal_buf.len();
                return Ok(AnalyzedToken {
                    token: Token::StringLiteral(literal_buf),
                    line: start_line,
                    column: start_col,
                    length: length,
                });
            }
            CharKind::Char(c) => {
                literal_buf.push(*c);
            }
            CharKind::Newline => {
                literal_buf.push('\n');
            }
            CharKind::Whitespace => {
                literal_buf.push(' ');
            }
        }
    }
    Err(Error::new(LexingError)
        .with_message("Unclosed string literal!".to_string())
        .with_line(start_line)
        .with_column(start_col)
        .with_file_path(path))
}

fn handle_char_literal<'a, I>(
    iter: &mut std::iter::Peekable<I>,
    start_line: usize,
    start_col: usize,
    path: &str,
) -> Result<AnalyzedToken>
where
    I: Iterator<Item = &'a AnalyzedChar>,
{
    let val: char;

    match iter.next() {
        Some(analyzed) => match analyzed.kind {
            CharKind::Char(c) => val = c,
            CharKind::Whitespace => val = ' ',
            CharKind::Newline => val = '\n',
        },
        None => {
            return Err(Error::new(LexingError)
                .with_message("Unexpected character after single quote!".to_string())
                .with_line(start_line)
                .with_column(start_col)
                .with_file_path(path));
        }
    };
    match iter.next() {
        Some(analyzed) => match analyzed.kind {
            CharKind::Char('\'') => Ok(AnalyzedToken {
                token: Token::CharLiteral(val),
                line: start_line,
                column: start_col,
                length: 1,
            }),
            _ => Err(Error::new(LexingError)
                .with_message("Expected closing single quote!".to_string())
                .with_line(start_line)
                .with_column(start_col)
                .with_file_path(path)),
        },
        None => Err(Error::new(LexingError)
            .with_message("Unexpected EOF while parsing char literal!".to_string())
            .with_line(start_line)
            .with_column(start_col)
            .with_file_path(path)),
    }
}

fn char_buf_clear(
    ch_buf: &mut Vec<&AnalyzedChar>,
    st_buf: &mut String,
    res_vec: &mut Vec<AnalyzedToken>,
    path: &str,
) -> Result<()> {
    if !ch_buf.is_empty() {
        st_buf.clear();
        st_buf.extend(ch_buf.iter().map(|b| match b.kind {
            CharKind::Char(c) => c,
            _ => '\u{FFFD}',
        }));

        let b_lin = ch_buf[0].line;
        let b_col = ch_buf[0].column;
        ch_buf.clear();
        final_buffer(res_vec, st_buf, b_lin, b_col, path)?;
    }
    Ok(())
}

fn handle_command_line<'a, I>(iter: &mut std::iter::Peekable<I>) -> Result<()>
where
    I: Iterator<Item = &'a AnalyzedChar>,
{
    for c in iter.by_ref() {
        match c.kind {
            CharKind::Newline => return Ok(()),
            _ => continue,
        }
    }
    Ok(())
}
