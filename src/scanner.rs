use std::fmt;

use crate::token::{LocationInfo, Token, TokenWrapper, KEYWORDS};

use crate::utils::char_range_to_string;

#[derive(Debug, Clone)]
pub struct ScannerError {
    line_number: usize,
    source: String,
    message: String,
}

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "Error: {}\n{} | {}",
            self.message, self.line_number, self.source
        )
    }
}

pub struct Scanner {
    characters: Vec<char>,
    // Track which character we are at
    start: usize,
    current: usize,
    //
    current_line: usize,
    //
    tokens_wrappers: Vec<TokenWrapper>,
    errors: Vec<ScannerError>,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            characters: source.chars().collect(),
            start: 0,
            current: 0,
            current_line: 1,
            tokens_wrappers: vec![],
            errors: vec![],
        }
    }
    fn is_empty(&self) -> bool {
        self.current >= (self.characters.len() - 1)
    }

    fn add_token(&mut self, token: Token) {
        self.tokens_wrappers.push(TokenWrapper {
            token,
            location_info: LocationInfo {
                line: self.current_line,
            },
        });
    }

    fn advance(&mut self) -> char {
        if self.is_empty() {
            return '\0';
        }
        self.current += 1;
        *self
            .characters
            .get(self.current)
            .expect("No characters left to scan!")
    }

    fn next(&mut self, expected: &char) -> bool {
        if self.is_empty() {
            return false;
        }
        if self.characters.get(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_empty() {
            return '\0';
        }
        return *self.characters.get(self.current).unwrap();
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.characters.len() {
            return '\0';
        }
        *self
            .characters
            .get(self.current + 1)
            .expect("Expected character at index for peek_next")
    }

    fn consume_string(&mut self) {
        while self.peek() != '"' && !self.is_empty() {
            if self.peek() == '\n' {
                self.current_line += 1;
            }
            self.advance();
        }
        if self.is_empty() {
            self.errors.push(ScannerError {
                line_number: self.current_line,
                message: "Unterminated string".to_owned(),
                source: char_range_to_string(&self.characters, self.start, self.current),
            });
        }
        // Must be closing '"'
        self.advance();
        let value = char_range_to_string(&self.characters, self.start, self.current);
        self.add_token(Token::String(value));
    }

    fn consume_number(&mut self) {
        while self.peek().is_numeric() {
            self.advance();
        }
        // Fractional
        if self.peek() == '.' && self.peek_next().is_numeric() {
            self.advance(); // Consume '.'
            while self.peek().is_numeric() {
                self.advance();
            }
        }
        let value = char_range_to_string(&self.characters, self.start, self.current);
        self.add_token(Token::Number(
            value.parse().expect("Could not parse string into f64"),
        ))
    }

    fn consume_identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }
        let value = char_range_to_string(&self.characters, self.start, self.current);
        if let Some(token) = KEYWORDS.get(&value) {
            self.add_token(token.clone());
        } else {
            self.add_token(Token::Identifier(value));
        }
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(Token::LeftParen),
            ')' => self.add_token(Token::RightParen),
            '{' => self.add_token(Token::LeftBrace),
            '}' => self.add_token(Token::RightBrace),
            ',' => self.add_token(Token::Comma),
            '.' => self.add_token(Token::Dot),
            '-' => self.add_token(Token::Minus),
            '+' => self.add_token(Token::Plus),
            ';' => self.add_token(Token::Semicolon),
            '*' => self.add_token(Token::Star),
            // 2 chars
            '!' => {
                if self.next(&'=') {
                    self.add_token(Token::BangEqual)
                } else {
                    self.add_token(Token::Bang)
                }
            }
            '=' => {
                if self.next(&'=') {
                    self.add_token(Token::EqualEqual)
                } else {
                    self.add_token(Token::Equal)
                }
            }
            '<' => {
                if self.next(&'=') {
                    self.add_token(Token::LessEqual)
                } else {
                    self.add_token(Token::Less)
                }
            }
            '>' => {
                if self.next(&'=') {
                    self.add_token(Token::GreaterEqual)
                } else {
                    self.add_token(Token::Greater)
                }
            }
            '/' => {
                // Is comment
                if self.next(&'/') {
                    // Comment goes until end of line
                    // TODO: Newline and \0?
                    while self.peek() != '\n' && self.is_empty() {
                        self.advance();
                    }
                } else {
                    self.add_token(Token::Slash);
                }
            }
            '\n' => self.current_line += 1,
            '"' => {
                self.consume_string();
            }
            _ => {
                if c.is_numeric() {
                    self.consume_number();
                } else if c.is_alphabetic() {
                    self.consume_identifier();
                } else {
                    self.errors.push(ScannerError {
                        line_number: self.current_line,
                        source: c.to_string(),
                        message: "Unexpected character.".to_owned(),
                    });
                }
            }
        };
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<TokenWrapper>, Vec<ScannerError>> {
        while !self.is_empty() {
            self.start = self.current;
            self.scan_token();
        }
        self.add_token(Token::Eof);
        Ok(self.tokens_wrappers.clone())
    }
}
