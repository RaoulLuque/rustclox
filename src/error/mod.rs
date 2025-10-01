use std::{error::Error, fmt::Display};

use crate::scanner::ScannerError;

#[derive(Debug)]
pub enum CloxError {
    ScannerError(ScannerError),
}

impl Display for CloxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CloxError::ScannerError(scanner_error) => write!(f, "{}", scanner_error),
        }
    }
}

impl Error for CloxError {}

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

impl CloxError {
    pub fn report_error(self, source: &str) {
        match self {
            CloxError::ScannerError(scanner_error) => match scanner_error {
                ScannerError::UnknownToken(_, line, current) => {
                    let (line_content, col) = find_location_in_source(source, line, current);
                    eprintln!(
                        "Scanner Error: Unknown Token \n\nline: {line:3} | {}\n          | {}\n          | {}",
                        line_content,
                        " ".repeat(col) + "^",
                        " ".repeat(col) + "Here"
                    );
                }
            },
        }
    }
}
