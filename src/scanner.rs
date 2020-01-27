use std::fmt;

use crate::token::{LocationInfo, Token, TokenWrapper, KEYWORDS};

use crate::utils::char_range_to_string;

#[derive(Debug, Clone, Default)]
pub struct Cause {
    line: usize,
    source: String,
}

#[derive(Debug, Clone)]
pub enum ScannerError {
    UnclosedBlockComment(Cause),
    UnexpectedCharacter(Cause),
    UnterminatedString(Cause),
}

impl ScannerError {
    fn discriminant(&self) -> usize {
        match *self {
            ScannerError::UnclosedBlockComment(_) => 0,
            ScannerError::UnexpectedCharacter(_) => 1,
            ScannerError::UnterminatedString(_) => 2,
        }
    }
}

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ScannerError::UnclosedBlockComment(ref cause) => write!(
                f,
                "Block comment not closed\nLine: {}\t{}",
                cause.line, cause.source
            ),
            ScannerError::UnexpectedCharacter(ref cause) => write!(
                f,
                "Unexpected character\nLine: {}\t{}",
                cause.line, cause.source
            ),
            ScannerError::UnterminatedString(ref cause) => write!(
                f,
                "String not terminated\nLine: {}\t{}",
                cause.line, cause.source
            ),
        }
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
        self.current >= self.characters.len()
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
            .get(self.current - 1)
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
            self.errors.push(ScannerError::UnterminatedString(Cause {
                line: self.current_line,
                source: char_range_to_string(&self.characters, self.start, self.current),
            }));
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

    fn consume_block_comment(&mut self, depth: usize) {
        // Consume '*'
        self.advance();

        loop {
            if self.is_empty() {
                self.errors.push(ScannerError::UnclosedBlockComment(Cause {
                    line: self.current_line,
                    source: char_range_to_string(&self.characters, self.start, self.current),
                }));
                return;
            }
            let c = self.advance();
            if c == '\n' {
                self.current_line += 1;
            } else if c == '*' && self.peek() == '/' {
                self.advance(); // Consume: '/'
                return;
            }
            // Go deeper
            else if c == '/' && self.peek() == '*' {
                self.consume_block_comment(depth + 1);
            }
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
                    while self.peek() != '\n' && !self.is_empty() {
                        self.advance();
                    }
                } else if self.next(&'*') {
                    self.consume_block_comment(0);
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
                    self.errors.push(ScannerError::UnexpectedCharacter(Cause {
                        line: self.current_line,
                        source: c.to_string(),
                    }));
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
        if !self.errors.is_empty() {
            return Err(self.errors.clone());
        }
        Ok(self.tokens_wrappers.clone())
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::utils::{join_vec_debug, s};

    struct ScanTokensTestCase {
        input: &'static str,
        expected: Vec<Token>,
    }
    struct ScannerErrorTestCase {
        input: &'static str,
        expected: Vec<ScannerError>,
    }

    #[test]
    fn test_scan_tokens() {
        let test_table: Vec<ScanTokensTestCase> = vec![
            ScanTokensTestCase {
                input: "hello;",
                expected: vec![
                    Token::Identifier("hello".to_owned()),
                    Token::Semicolon,
                    Token::Eof,
                ],
            },
            ScanTokensTestCase {
                input: "var k = 10;",
                expected: vec![
                    Token::Var,
                    Token::Identifier(s("k")),
                    Token::Equal,
                    Token::Number(10.0),
                    Token::Semicolon,
                    Token::Eof,
                ],
            },
            ScanTokensTestCase {
                input: "fun hello(){

                };",
                expected: vec![
                    Token::Fun,
                    Token::Identifier(s("hello")),
                    Token::LeftParen,
                    Token::RightParen,
                    Token::LeftBrace,
                    Token::RightBrace,
                    Token::Semicolon,
                    Token::Eof,
                ],
            },
            ScanTokensTestCase {
                input: "// fun comment = hello",
                expected: vec![Token::Eof],
            },
            ScanTokensTestCase {
                input: "/* block comment */",
                expected: vec![Token::Eof],
            },
            ScanTokensTestCase {
                input: "/* /* nested */ block comment */",
                expected: vec![Token::Eof],
            },
        ];
        for tc in test_table {
            let mut scanner = Scanner::new(tc.input.to_string());
            let output = &scanner.scan_tokens().expect("");
            let actual = join_vec_debug(
                &output
                    .iter()
                    .map(|tw| tw.token.clone())
                    .collect::<Vec<Token>>(),
            );
            let expected = join_vec_debug(&tc.expected);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_scanner_errors() {
        let test_table: Vec<ScannerErrorTestCase> = vec![ScannerErrorTestCase {
            input: "/* afdsafdf ",
            expected: vec![ScannerError::UnclosedBlockComment(Default::default())],
        }];
        for tc in test_table {
            let mut scanner = Scanner::new(tc.input.to_owned());
            let errors = scanner
                .scan_tokens()
                .expect_err(&format!("Expected error in test case: {}", tc.input));
            assert_eq!(
                errors.len(),
                tc.expected.len(),
                "Number of errors do not match"
            );
            for (i, error) in errors.iter().enumerate() {
                assert_eq!(
                    error.discriminant(),
                    tc.expected.get(i).unwrap().discriminant()
                );
            }
        }
    }
}
