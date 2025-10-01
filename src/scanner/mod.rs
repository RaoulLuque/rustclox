use std::{collections::HashMap, sync::LazyLock};

use crate::scanner::token::{Literal, Operator, Token, TokenType};

pub mod token;

const NEWLINE_CHAR: char = '\n';

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

pub struct Scanner<'a> {
    source: &'a str,
    line: usize,
    start: usize,
    current: usize,
    tokens: Vec<Token<TokenType<'a>>>,
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

    pub fn scan_tokens(mut self) -> Vec<Token<TokenType<'a>>> {
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
                self.add_token(TokenType::Operator(Operator::Minus));
            }
            '+' => {
                self.add_token(TokenType::Operator(Operator::Plus));
            }
            ';' => {
                self.add_token(TokenType::Semicolon);
            }
            '*' => {
                self.add_token(TokenType::Operator(Operator::Star));
            }

            // Possible single character or double character tokens
            '!' => {
                let token_type = if self.match_current('=') {
                    TokenType::Operator(Operator::BangEqual)
                } else {
                    TokenType::Bang
                };
                self.add_token(token_type);
            }
            '=' => {
                let token_type = if self.match_current('=') {
                    TokenType::Operator(Operator::EqualEqual)
                } else {
                    TokenType::Equal
                };
                self.add_token(token_type);
            }
            '<' => {
                let token_type = if self.match_current('=') {
                    TokenType::Operator(Operator::LessEqual)
                } else {
                    TokenType::Operator(Operator::Less)
                };
                self.add_token(token_type);
            }
            '>' => {
                let token_type = if self.match_current('=') {
                    TokenType::Operator(Operator::GreaterEqual)
                } else {
                    TokenType::Operator(Operator::Greater)
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
                    self.add_token(TokenType::Operator(Operator::Slash));
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
                println!("Unexpected character: {}", character);
                todo!("Implement error handling");
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
        if let Some(character) = self.peek() {
            if character == expected {
                self.advance();
                return true;
            }
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
        while self.peek().is_some_and(|c| is_digit(c)) {
            self.advance();
        }

        if self.peek() == Some('.') && self.peek_next().is_some_and(|c| is_digit(c)) {
            // Consume the '.'
            self.advance();
            while self.peek().is_some_and(|c| is_digit(c)) {
                self.advance();
            }
        }

        let number_value = &self.source[self.start..self.current]
            .parse::<f32>()
            .unwrap();
        self.add_token(TokenType::Literal(Literal::Number(*number_value)));
    }

    fn scan_identifier(&mut self) {
        while self.peek().is_some_and(|c| is_alphanumeric(c)) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let token_type = if let Some(keyword) = KEYWORDS.get(text) {
            *keyword
        } else {
            TokenType::Identifier(text)
        };
        self.add_token(token_type);
    }
}

fn is_digit(character: char) -> bool {
    matches!(character, '0'..='9')
}

fn is_alphanumeric(character: char) -> bool {
    matches!(character, 'a'..='z' | 'A'..='Z' | '_') || is_digit(character)
}
