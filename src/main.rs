use std::env;
use std::fs;
use std::io;

mod scanner;
use crate::scanner::{Scanner, ScannerError};

mod token;

mod utils;

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
    run(source).expect("Failed to run source");
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
    let mut scanner = scanner::Scanner::new(source);
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
