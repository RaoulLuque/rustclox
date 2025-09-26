use std::path::PathBuf;

use clap::Parser;

/// A simple Lox interpreter and compiler written in Rust.
#[derive(Parser)]
struct Args {
    /// The source file to interpret
    source: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    if let Some(source) = args.source {
        println!("Interpreting source file: {:?}", source);
        // Here you would add the logic to read and interpret the source file.
        return;
    } else {
        println!("No source file provided. Entering REPL mode...");
        // Here you would add the logic to start a REPL (Read-Eval-Print Loop).
        return;
    }
}
