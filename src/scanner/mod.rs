use std::{collections::HashMap, error::Error, fmt::Display, sync::LazyLock};

use crate::scanner::token::{BinaryOperator, Literal, Token, TokenType};

pub mod token;

const NEWLINE_CHAR: char = '\n';

#[allow(clippy::declare_interior_mutable_const)]
const KEYWORDS: LazyLock<HashMap<&str, TokenType>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert("and", TokenType::And);
    m.insert("class", TokenType::Class);
    m.insert("else", TokenType::Else);
    m.insert("false", TokenType::Literal(Literal::False));
    m.insert("for", TokenType::For);
    m.insert("fun", TokenType::Fun);
    m.insert("if", TokenType::If);
    m.insert("nil", TokenType::Literal(Literal::Nil));
    m.insert("or", TokenType::Or);
    m.insert("print", TokenType::Print);
    m.insert("return", TokenType::Return);
    m.insert("super", TokenType::Super);
    m.insert("this", TokenType::This);
    m.insert("true", TokenType::Literal(Literal::True));
    m.insert("var", TokenType::Var);
    m.insert("while", TokenType::While);
    m
});

/// The Scanner is responsible for converting the source code into a series of tokens.
pub struct Scanner<'a> {
    /// The source code to scan.
    source: &'a str,
    /// The current line number in the source code.
    line: usize,
    /// The start index of the current lexeme being scanned.
    start: usize,
    /// The current index in the source code.
    current: usize,
    /// The list of tokens that have been scanned.
    tokens: Vec<Token<TokenType<'a>>>,
    /// Any errors encountered during scanning.
    errors: Vec<ScannerError>,
}

#[derive(Debug)]
pub enum ScannerError {
    /// An unknown character was encountered during scanning. Includes the character, line number, and current number.
    UnknownToken(char, usize, usize),
}

impl Display for ScannerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScannerError::UnknownToken(character, line, current) => {
                write!(
                    f,
                    "[line {}] ScannerError at position {}: Unknown character '{}'",
                    line, current, character
                )
            }
        }
    }
}

impl Error for ScannerError {}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            source,
            line: 1,
            start: 0,
            current: 0,
            tokens: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn scan_tokens(mut self) -> Result<Vec<Token<TokenType<'a>>>, Vec<ScannerError>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        if self.errors.is_empty() {
            Ok(self.tokens)
        } else {
            Err(self.errors)
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let character = self.advance();
        match character {
            // Single-character tokens.
            '(' => {
                self.add_token(TokenType::LeftParenthesis);
            }
            ')' => {
                self.add_token(TokenType::RightParenthesis);
            }
            '{' => {
                self.add_token(TokenType::LeftBrace);
            }
            '}' => {
                self.add_token(TokenType::RightBrace);
            }
            ',' => {
                self.add_token(TokenType::Comma);
            }
            '.' => {
                self.add_token(TokenType::Dot);
            }
            '-' => {
                self.add_token(TokenType::Operator(BinaryOperator::Minus));
            }
            '+' => {
                self.add_token(TokenType::Operator(BinaryOperator::Plus));
            }
            ';' => {
                self.add_token(TokenType::Semicolon);
            }
            '*' => {
                self.add_token(TokenType::Operator(BinaryOperator::Star));
            }

            // Possible single character or double character tokens
            '!' => {
                let token_type = if self.match_current('=') {
                    TokenType::Operator(BinaryOperator::BangEqual)
                } else {
                    TokenType::Bang
                };
                self.add_token(token_type);
            }
            '=' => {
                let token_type = if self.match_current('=') {
                    TokenType::Operator(BinaryOperator::EqualEqual)
                } else {
                    TokenType::Equal
                };
                self.add_token(token_type);
            }
            '<' => {
                let token_type = if self.match_current('=') {
                    TokenType::Operator(BinaryOperator::LessEqual)
                } else {
                    TokenType::Operator(BinaryOperator::Less)
                };
                self.add_token(token_type);
            }
            '>' => {
                let token_type = if self.match_current('=') {
                    TokenType::Operator(BinaryOperator::GreaterEqual)
                } else {
                    TokenType::Operator(BinaryOperator::Greater)
                };
                self.add_token(token_type);
            }
            '/' => {
                if self.match_current('/') {
                    // We are currently scanning a comment and can just discard it
                    while self.peek() != Some(NEWLINE_CHAR) && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Operator(BinaryOperator::Slash));
                }
            }

            // Strings
            '"' => {
                self.scan_string();
            }

            // Digits
            '0'..='9' => self.scan_number(),

            // Alphanumeric
            'a'..='z' | 'A'..='Z' | '_' => self.scan_identifier(),

            // Whitespaces
            ' ' | '\r' | '\t' => {}

            NEWLINE_CHAR => self.line += 1,
            _ => {
                self.errors.push(ScannerError::UnknownToken(
                    character,
                    self.line,
                    self.current,
                ));
            }
        }
    }

    /// Adds a token of the given type to the vec of tokens.
    fn add_token(&mut self, token_type: TokenType<'a>) {
        let token = Token::new(token_type, self.line, self.start);
        self.tokens.push(token);
    }

    /// Consumes the current character and returns it.
    fn advance(&mut self) -> char {
        let character = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        character
    }

    /// Peeks at the current character without consuming it.
    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.current)
    }

    fn peek_next(&self) -> Option<char> {
        self.source.chars().nth(self.current + 1)
    }

    /// Consumes the current character if it matches the expected character.
    /// Returns true if the character was consumed, false otherwise.
    fn match_current(&mut self, expected: char) -> bool {
        if let Some(character) = self.peek()
            && character == expected
        {
            self.advance();
            return true;
        }
        false
    }

    fn scan_string(&mut self) {
        while self.peek() != Some('"') && !self.is_at_end() {
            if self.peek() == Some(NEWLINE_CHAR) {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            todo!("Handle error")
        }

        // The closing "
        self.advance();

        // Trim the surrounding "
        let string_content = &self.source[(self.start + 1)..(self.current - 1)];
        self.add_token(TokenType::Literal(Literal::Str(string_content)));
    }

    fn scan_number(&mut self) {
        while self.peek().is_some_and(|c| c.is_ascii_digit()) {
            self.advance();
        }

        if self.peek() == Some('.') && self.peek_next().is_some_and(|c| c.is_ascii_digit()) {
            // Consume the '.'
            self.advance();
            while self.peek().is_some_and(|c| c.is_ascii_digit()) {
                self.advance();
            }
        }

        let number_value = &self.source[self.start..self.current]
            .parse::<f32>()
            .unwrap();
        self.add_token(TokenType::Literal(Literal::Number(*number_value)));
    }

    fn scan_identifier(&mut self) {
        while self
            .peek()
            .is_some_and(|c| c == '_' || c.is_ascii_alphanumeric())
        {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        #[allow(clippy::borrow_interior_mutable_const)]
        let token_type = if let Some(keyword) = KEYWORDS.get(text) {
            *keyword
        } else {
            TokenType::Identifier(text)
        };
        self.add_token(token_type);
    }
}
