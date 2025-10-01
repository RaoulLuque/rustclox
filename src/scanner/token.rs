#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Token<T> {
    pub token_type: T,
    pub line: usize,
    pub start_index_in_source: usize,
}

impl<T: Copy> Token<T> {
    pub fn new(token_type: T, line: usize, column: usize) -> Self {
        Token {
            token_type,
            line,
            start_index_in_source: column,
        }
    }
}

impl<'a> Token<TokenType<'a>> {
    pub fn to_token_sub_type<U: TokenSubType<'a, U>>(self, _: &U) -> Option<Token<U>> {
        if let Some(new_token_type) = U::from_token_type(&self.token_type) {
            return Some(Token {
                token_type: new_token_type,
                line: self.line,
                start_index_in_source: self.start_index_in_source,
            });
        }
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType<'a> {
    // Single-character tokens.
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Semicolon,
    Equal,

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

impl TokenType<'_> {
    /// Returns true if the two token types are of the same variant, ignoring any associated data.
    pub fn is_same_type(&self, other: &TokenType) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl<'a> TokenSubType<'a, TokenType<'a>> for TokenType<'a> {
    fn from_token_type(token_type: &TokenType<'a>) -> Option<TokenType<'a>> {
        Some(*token_type)
    }

    fn to_token_type(token_sub_type: TokenType<'a>) -> TokenType<'a> {
        token_sub_type
    }
}

/// A trait for converting between [TokenType] and its subtypes.
pub trait TokenSubType<'a, T>: Copy {
    /// Converts a [TokenType] to the subtype T, if possible.
    fn from_token_type(token_type: &TokenType<'a>) -> Option<T>;

    /// Converts the subtype T to a [TokenType].
    fn to_token_type(token_sub_type: T) -> TokenType<'a>;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Literal<'a> {
    Number(f32),
    Str(&'a str),
    True,
    False,
    Nil,
}

impl<'a> TokenSubType<'a, Literal<'a>> for Literal<'a> {
    fn from_token_type(token_type: &TokenType<'a>) -> Option<Literal<'a>> {
        if let TokenType::Literal(literal) = token_type {
            Some(*literal)
        } else {
            None
        }
    }

    fn to_token_type(token_sub_type: Literal<'a>) -> TokenType<'a> {
        TokenType::Literal(token_sub_type)
    }
}

impl<'a> From<Token<Literal<'a>>> for Token<TokenType<'a>> {
    fn from(token: Token<Literal<'a>>) -> Self {
        Token {
            token_type: TokenType::Literal(token.token_type),
            line: token.line,
            start_index_in_source: token.start_index_in_source,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    EqualEqual,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Plus,
    Minus,
    Star,
    Slash,
}

impl<'a> TokenSubType<'a, Operator> for Operator {
    fn from_token_type(token_type: &TokenType<'a>) -> Option<Operator> {
        if let TokenType::Operator(operator) = token_type {
            Some(*operator)
        } else {
            None
        }
    }

    fn to_token_type(token_sub_type: Operator) -> TokenType<'a> {
        TokenType::Operator(token_sub_type)
    }
}

impl<'a> From<Token<Operator>> for Token<TokenType<'a>> {
    fn from(token: Token<Operator>) -> Self {
        Token {
            token_type: TokenType::Operator(token.token_type),
            line: token.line,
            start_index_in_source: token.start_index_in_source,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOperator {
    Minus(Minus),
    Bang(Bang),
}

impl<'a> TokenSubType<'a, UnaryOperator> for UnaryOperator {
    fn from_token_type(token_type: &TokenType<'a>) -> Option<UnaryOperator> {
        match token_type {
            TokenType::Operator(Operator::Minus) => Some(UnaryOperator::Minus(Minus {})),
            TokenType::Bang => Some(UnaryOperator::Bang(Bang {})),
            _ => None,
        }
    }

    fn to_token_type(token_sub_type: UnaryOperator) -> TokenType<'a> {
        match token_sub_type {
            UnaryOperator::Minus(_) => TokenType::Operator(Operator::Minus),
            UnaryOperator::Bang(_) => TokenType::Bang,
        }
    }
}

impl<'a> From<Token<UnaryOperator>> for Token<TokenType<'a>> {
    fn from(token: Token<UnaryOperator>) -> Self {
        Token {
            token_type: UnaryOperator::to_token_type(token.token_type),
            line: token.line,
            start_index_in_source: token.start_index_in_source,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bang {}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Minus {}
