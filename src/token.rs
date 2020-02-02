use std::collections::HashMap;
use std::fmt;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref KEYWORDS: HashMap<String, Token> = {
        let mut map = HashMap::new();
        map.insert("and".to_owned(), Token::And);
        map.insert("class".to_owned(), Token::Class);
        map.insert("else".to_owned(), Token::Else);
        map.insert("false".to_owned(), Token::False);
        map.insert("for".to_owned(), Token::For);
        map.insert("fun".to_owned(), Token::Fun);
        map.insert("if".to_owned(), Token::If);
        map.insert("nil".to_owned(), Token::Nil);
        map.insert("or".to_owned(), Token::Or);
        map.insert("print".to_owned(), Token::Print);
        map.insert("return".to_owned(), Token::Return);
        map.insert("super".to_owned(), Token::Super);
        map.insert("this".to_owned(), Token::This);
        map.insert("true".to_owned(), Token::True);
        map.insert("var".to_owned(), Token::Var);
        map.insert("while".to_owned(), Token::While);
        map
    };
}

#[derive(Debug, Clone)]
pub struct LocationInfo {
    // Which line the token was seen
    pub line: usize,
}

#[derive(Debug, Clone)]
pub struct TokenWrapper {
    pub location_info: LocationInfo,
    pub token: Token,
}

impl fmt::Display for TokenWrapper {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        f.write_str(&format!("{:?}", self.token))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals
    Identifier(String),
    String(String),
    Number(f64),
    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    // EOF
    Eof,
}
