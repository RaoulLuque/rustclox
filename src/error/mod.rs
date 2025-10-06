use std::{error::Error, fmt::Display};

use colored::Colorize;

use crate::{parser::ParserError, scanner::ScannerError};

#[derive(Debug)]
pub enum CloxError<'a> {
    ScannerError(ScannerError),
    ParserError(ParserError<'a>),
}

impl Display for CloxError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CloxError::ScannerError(scanner_error) => write!(f, "{}", scanner_error),
            CloxError::ParserError(parser_error) => write!(f, "{}", parser_error),
        }
    }
}

impl Error for CloxError<'_> {}

/// Finds the specific line and column in the source code based on the provided line number and current index.
/// Returns a tuple containing the line content and the column number in that line (0-indexed).
pub fn find_location_in_source(source: &str, line: usize, current_index: usize) -> (&str, usize) {
    let lines: Vec<&str> = source.lines().collect();
    if line == 0 || line > lines.len() {
        return ("", 0);
    }
    let target_line = lines[line - 1];
    let col = current_index - lines[..line - 1].iter().map(|l| l.len() + 1).sum::<usize>();
    (target_line, col - 1)
}

impl CloxError<'_> {
    pub fn report_error(self, source: &str) {
        match self {
            CloxError::ScannerError(scanner_error) => match scanner_error {
                ScannerError::UnknownToken(char, line, current) => {
                    let (line_content, col) = find_location_in_source(source, line, current);
                    eprintln!(
                        "{} \n\nline: {line:3} | {}\n          | {}\n          | {}",
                        format!("Scanner Error: Unknown Token: \"{}\"", char).red(),
                        line_content,
                        format!("{}{}", " ".repeat(col), "^".yellow()),
                        format!("{}{}", " ".repeat(col), "Here".yellow())
                    );
                }
            },
            CloxError::ParserError(parser_error) => match parser_error {
                ParserError::UnexpectedToken { expected, found } => {
                    let line = found.line;
                    let current = found.start_index_in_source;
                    let (line_content, col) = find_location_in_source(source, line, current);
                    eprintln!(
                        "{} \n\nline: {line:3} | {}\n          | {}\n          | {}",
                        format!(
                            "Parser Error: Unexpected Token: found '{:?}', expected '{:?}'",
                            found.token_type, expected
                        )
                        .red(),
                        line_content,
                        format!("{}{}", " ".repeat(col), "^".yellow()),
                        format!("{}{}", " ".repeat(col), "Here".yellow())
                    );
                }
            },
        }
    }
}
