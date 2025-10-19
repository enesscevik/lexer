// calisma mantigini kavra
use once_cell::sync::Lazy;
use std::collections::HashSet;

// pub static KEYWORDS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
//     HashSet::from([
//         "let", "op", "if", "else", "while", "for", "loop", "success", "i32", "str", "string",
//         "char", "f32", "bool", "break", "continue", "nret", "data", "and", "or",
//     ])
// });

pub static SYMBOLS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    HashSet::from([
        "=", "\'", "\"", ":", ";", "::", ")", "(", "{", "}", "-", "+", "/", "*", "->", "?", "-<",
        "<-", "-!", ">", ">", "%", "[", "]", "^", "//", "/*", "==", "<=", ">=", "<", ">", ",",
    ])
});

#[derive(Debug, Clone)]
pub enum Token {
    Identifier(String),
    Number(i32),
    Symbol(String),
    StringLiteral(String),
    CharLiteral(char),
    RBrace,
    LBrace,
    Semicolon,
    Colon,
    Equals,
    EqualEqual,
    NotEquals,
    DoubleColon,
    LessEqual,
    GreaterEqual,
    LeftParen,
    RightParen,
    Minus,
    Not,
    Plus,
    Slash,
    Star,
    Question,
    Percent,
    LBracket,
    RBracket,
    Power,
    Arrow,
    ReverseArrow,
    ChainNot,
    ChainBack,
    Less,
    Greater,
    Unknown,
    Let,
    Op,
    If,
    ElseIf,
    Else,
    While,
    For,
    Loop,
    Success,
    IntType(String),
    String,
    Char,
    FloatType(String),
    Bool,
    True,
    False,
    Break,
    Continue,
    Nret,
    Data,
    And,
    Or,
}

pub struct AnalyzedToken {
    pub token: Token,
    pub line: usize,
    pub column: usize,
    pub length: usize,
}

impl std::fmt::Debug for AnalyzedToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AnalyzedToken -> \x1b[033;1m{:?}\x1b[0m [line: {}, column: {}, length: {}]",
            self.token, self.line, self.column, self.length
        )?;
        Ok(())
    }
}
