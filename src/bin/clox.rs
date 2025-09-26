use std::path::PathBuf;

use clap::Parser;
use rustclox::Lox;

/// A simple Lox interpreter and compiler written in Rust.
#[derive(Parser)]
struct Args {
    /// The source file to interpret
    source: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let mut lox = Lox::new();

    if let Some(source) = args.source {
        lox.run_file(&source);
        return;
    } else {
        lox.run_repl();
        return;
    }
}
