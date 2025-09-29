#[derive(Debug)]
pub struct Token<T> {
    pub token_type: T,
    pub line: usize,
    pub start_index_in_source: usize,
}

impl<T> Token<T> {
    pub fn new(token_type: T, line: usize, column: usize) -> Self {
        Token {
            token_type,
            line,
            start_index_in_source: column,
        }
    }
}

#[derive(Debug, Clone, Copy)]
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

    // Literals.
    Literal(Literal<'a>),

    // Operators
    Identifier(&'a str),
    Operator(Operator),
    Bang,

    // Keywords.
    And,
    Class,
    Else,
    Fun,
    For,
    If,
    Or,
    Print,
    Return,
    Super,
    This,
    Var,
    While,

    Eof,
}

#[derive(Debug, Clone, Copy)]
pub enum Literal<'a> {
    Number(f32),
    Str(&'a str),
    True,
    False,
    Nil,
}

#[derive(Debug, Clone, Copy)]
pub enum Operator {
    EqualEqual,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Plus,
    Equal,
    Minus,
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOperator {
    Minus(Minus),
    Bang(Bang),
}

#[derive(Debug, Clone, Copy)]
pub struct Bang {}
#[derive(Debug, Clone, Copy)]
pub struct Minus {}
