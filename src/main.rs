use std::env;
use std::fs;
use std::io;

use std::fmt;

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

fn run(source: String) -> Result<&'static str, ScannerError> {
    let scanner = Scanner::new(source);
    match scanner.scan_tokens() {
        Ok(tokens) => {
            for token in tokens {
                // println!("{:?}", token);
            }
            return Ok("result of last line");
        }
        Err(vec_scanner_errs) => Err(vec_scanner_errs.get(0).expect("No error in vector").clone()),
    }
}

#[derive(Debug)]
struct LocationInfo {
    // Which line the token was seen
    line: usize,
}

#[derive(Debug)]
enum Token {
    // Single-character tokens.
    LeftParen(LocationInfo),
    RightParen(LocationInfo),
    LeftBrace(LocationInfo),
    RightBrace(LocationInfo),
    Comma(LocationInfo),
    Dot(LocationInfo),
    Minus(LocationInfo),
    Plus(LocationInfo),
    Semicolon(LocationInfo),
    Slash(LocationInfo),
    Star(LocationInfo),

    // One or two character tokens.
    Bang(LocationInfo),
    BangEqual(LocationInfo),
    Equal(LocationInfo),
    EqualEqual(LocationInfo),
    Greater(LocationInfo),
    GreaterEqual(LocationInfo),
    Less(LocationInfo),
    LessEqual(LocationInfo),
    // Literals
    Identifier(LocationInfo),
    String(LocationInfo),
    Number(LocationInfo),
    // Keywords
    And(LocationInfo),
    Class(LocationInfo),
    Else(LocationInfo),
    False(LocationInfo),
    Fun(LocationInfo),
    For(LocationInfo),
    If(LocationInfo),
    Nil(LocationInfo),
    Or(LocationInfo),
    Print(LocationInfo),
    Return(LocationInfo),
    Super(LocationInfo),
    This(LocationInfo),
    True(LocationInfo),
    Var(LocationInfo),
    While(LocationInfo),
    // EOF
    Eof(LocationInfo),
}

struct Scanner {
    characters: Vec<char>,
    // Track which character we are at
    start: usize,
    current: usize,
    // 
    current_line: usize,
}

impl Scanner {
    fn new(source: String) -> Scanner {
        Scanner { 
            characters: source.chars().collect(),
            start: 0,
            current: 0,
            current_line: 1,
        }
    }

    fn scan_tokens(&self) -> Result<Vec<Token>, Vec<ScannerError>> {
        let split: Vec<&str> = self.source.split_ascii_whitespace().collect();
        Ok(split
            .into_iter()
            .map(|s: &str| Token::Word(s.to_owned()))
            .collect())
    }

    fn advance(&mut self) -> Option<&char> {
        self.current += 1;
        self.characters.get(self.current)
    }

    fn scan_token(&mut self){
        
    }
}
