use std::path::PathBuf;

use clap::Parser;
use rustclox::{run_file, run_repl};

/// A simple Lox interpreter and compiler written in Rust.
#[derive(Parser)]
struct Args {
    /// The source file to interpret
    source: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    if let Some(source) = args.source {
        println!("Running File: {:?}", source);
        run_file(&source).unwrap();
    } else {
        println!("Running in REPL mode");
        run_repl();
    }
}
