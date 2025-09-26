use crate::scanner::token::{Token, TokenType};

mod token;

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

            '\n' => self.line += 1,
            _ => {}
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        let lexeme = &self.source[self.start..self.current];
        let token = Token::new(token_type, lexeme, self.line);
        self.tokens.push(token);
    }

    fn advance(&mut self) -> char {
        let character = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        character
    }
}
