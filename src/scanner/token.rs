#[derive(Debug)]
pub struct Token<'a> {
    token_type: TokenType<'a>,
    line: usize,
    column: usize,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType<'a>, line: usize, column: usize) -> Self {
        Token {
            token_type,
            line,
            column,
        }
    }
}

#[derive(Debug)]
pub enum TokenType<'a> {
    // Single-character tokens.
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier(&'a str),
    Str(&'a str),
    Number(f32),

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}
