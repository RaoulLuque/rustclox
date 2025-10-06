use std::{
    fs,
    io::{self, Write},
};

use crate::{error::CloxError, interpreter::Interpreter, scanner::Scanner};

pub mod ast;
pub mod error;
pub mod interpreter;
pub mod parser;
pub mod scanner;

pub fn run_file(path: &std::path::Path) -> std::io::Result<()> {
    let source = fs::read_to_string(path)?;
    run(&source);
    Ok(())
}

pub fn run_repl() -> std::io::Result<()> {
    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush()?;
        std::io::stdin().read_line(&mut input)?;
        run(&input);
    }
}

pub fn run(source: &str) {
    let scanner = Scanner::new(source);
    let tokens = match scanner.scan_tokens() {
        Ok(tokens) => tokens,
        Err(errors) => {
            for error in errors {
                CloxError::ScannerError(error).report_error(source);
            }
            return;
        }
    };
    // println!("{:#?}", tokens);
    let mut parser = parser::Parser::new(tokens);
    let declarations = parser.parse(source);

    let interpreter = Interpreter::new();

    interpreter.interpret(&declarations);
}
