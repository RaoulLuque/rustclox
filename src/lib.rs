use std::{
    fs,
    io::{self, Write},
};

pub struct Lox;

impl Lox {
    pub fn new() -> Self {
        Lox {}
    }

    pub fn run_file(&mut self, path: &std::path::Path) -> std::io::Result<()> {
        let source = fs::read_to_string(path)?;
        self.run(&source);
        Ok(())
    }

    pub fn run_repl(&mut self) -> std::io::Result<()> {
        loop {
            let mut input = String::new();
            print!("> ");
            io::stdout().flush()?;
            std::io::stdin().read_line(&mut input)?;
            self.run(&input);
        }
    }

    pub fn run(&mut self, source: &str) {}
}
