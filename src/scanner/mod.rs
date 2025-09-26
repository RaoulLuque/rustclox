use crate::scanner::token::{Token, TokenType};

mod token;

const NEWLINE_CHAR: char = '\n';

pub struct Scanner<'a> {
    source: &'a str,
    line: usize,
    start: usize,
    current: usize,
    tokens: Vec<Token<'a>>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            source,
            line: 1,
            start: 0,
            current: 0,
            tokens: Vec::new(),
        }
    }

    pub fn scan_tokens(mut self) -> Vec<Token<'a>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens
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
                self.add_token(TokenType::Minus);
            }
            '+' => {
                self.add_token(TokenType::Plus);
            }
            ';' => {
                self.add_token(TokenType::Semicolon);
            }
            '*' => {
                self.add_token(TokenType::Star);
            }

            // Possible single character or double character tokens
            '!' => {
                let token_type = if self.match_current('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token_type);
            }
            '=' => {
                let token_type = if self.match_current('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token_type);
            }
            '<' => {
                let token_type = if self.match_current('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token_type);
            }
            '>' => {
                let token_type = if self.match_current('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
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
                    self.add_token(TokenType::Slash);
                }
            }

            // Whitespaces
            ' ' | '\r' | '\t' => {}

            NEWLINE_CHAR => self.line += 1,
            _ => {
                println!("Unexpected character: {}", character);
                todo!("Implement error handling");
            }
        }
    }

    /// Adds a token of the given type to the vec of tokens.
    fn add_token(&mut self, token_type: TokenType) {
        let lexeme = &self.source[self.start..self.current];
        let token = Token::new(token_type, lexeme, self.line);
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
        if self.is_at_end() {
            return None;
        }
        Some(self.source.chars().nth(self.current).unwrap())
    }

    /// Consumes the current character if it matches the expected character.
    /// Returns true if the character was consumed, false otherwise.
    fn match_current(&mut self, expected: char) -> bool {
        if let Some(character) = self.peek() {
            if character == expected {
                self.advance();
                return true;
            }
        }
        false
    }
}
