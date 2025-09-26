use std::{
    fs,
    io::{self, Write},
};

use crate::scanner::Scanner;

mod scanner;

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
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }
}
