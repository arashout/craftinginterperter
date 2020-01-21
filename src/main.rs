use std::env;
use std::fs;
use std::io;

use std::fmt;

use std::collections::HashMap;

// lazy_static! {
//     static ref PRIVILEGES: HashMap<&'static str, Vec<&'static str>> = {
//         let mut map = HashMap::new();
//         map.insert("James", vec!["user", "admin"]);
//         map.insert("Jim", vec!["user"]);
//         map
//     };
// }

#[derive(Debug, Clone)]
struct ScannerError {
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

fn main() {
    if env::args().len() > 2 {
        println!("Usage: jlox [script]");
    } else if env::args().len() == 2 {
        run_script(&env::args().last().expect("Expected 2nd argument"));
    } else {
        // TODO: Add sigterm handler
        run_prompt();
    }
}

fn run_script(file_path: &String) {
    println!("Running script: {}", file_path);

    let source =
        fs::read_to_string(file_path).expect(&format!("Cannot read file at {}", file_path));
    run(source);
}

fn run_prompt() {
    let stdin = io::stdin();
    let input = &mut String::new();
    let mut source_acc: Vec<String> = Vec::new();
    loop {
        input.clear();
        stdin.read_line(input).expect("Could not read line");
        source_acc.push(input.to_owned());
        match run(source_acc.join("\n")) {
            Ok(r) => println!("{}", r),
            Err(scanner_err) => {
                println!("{:?}", scanner_err);
                source_acc.pop().expect("Could not pop last input");
            }
        }
    }
}

fn run(source: String) -> Result<String, ScannerError> {
    let mut scanner = Scanner::new(source);
    match scanner.scan_tokens() {
        Ok(wrappers) => {
            let mut output = String::new();
            for wrapper in wrappers {
                output.push_str(&format!("{:?}\n", wrapper));
            }
            return Ok(output.to_string());
        }
        Err(vec_scanner_errs) => Err(vec_scanner_errs.get(0).expect("No error in vector").clone()),
    }
}

#[derive(Debug, Clone)]
struct LocationInfo {
    // Which line the token was seen
    line: usize,
}

#[derive(Debug, Clone)]
struct TokenWrapper {
    location_info: LocationInfo,
    token: Token,
}

impl fmt::Display for TokenWrapper {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        f.write_str(&format!("{:?}", self.token))
    }
}

#[derive(Debug, Clone)]
enum Token {
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

struct Scanner {
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
    fn new(source: String) -> Scanner {
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
                source: self.characters[self.start..self.current]
                    .iter()
                    .cloned()
                    .collect::<String>(),
                // TODO: Take slice from start to current
                // source: self.characters[self.start..self.current],
            });
        }
        // Must be closing '"'
        self.advance();

        let value = self.characters[self.start + 1..self.current - 1]
            .iter()
            .cloned()
            .collect::<String>();
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
        let value = self.characters[self.start..self.current]
            .iter()
            .cloned()
            .collect::<String>();
        self.add_token(Token::Number(
            value.parse().expect("Could not parse string into f64"),
        ))
    }

    fn consume_identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }
        let value = 
        self.add_token()
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

    fn scan_tokens(&mut self) -> Result<Vec<TokenWrapper>, Vec<ScannerError>> {
        while !self.is_empty() {
            self.start = self.current;
            self.scan_token();
        }
        self.add_token(Token::Eof);
        Ok(self.tokens_wrappers.clone())
    }
}

fn collect_slice<T>(source: Vec<T>, from: usize, to: usize) -> Vec<T> {
    source
}