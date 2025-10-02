use std::error::Error;

pub use crate::scanner::token::Token;
use crate::scanner::token::{Literal, BinaryOperator, UnaryOperator};

pub mod ast_printer;

/// A statement in the AST.
pub enum Stmt<'a> {
    /// An expression statement. Is followed by a semicolon ';'.
    Expression(Expression<'a>),
    /// A print statement. Is preceded by 'print' and followed by a semicolon ';'.
    Print(Expression<'a>),
}

/// An expression in the AST.
pub enum Expression<'a> {
    /// A literal value.
    Literal(Literal<'a>),
    /// A grouping of expressions, enclosed in parentheses '(' - grouping here - ')'.
    Grouping(Box<Expression<'a>>),
    /// A unary operation of Operation type [UnaryOperation].
    Unary {
        operator: Token<UnaryOperator>,
        right: Box<Expression<'a>>,
    },
    /// A binary operation of Operation type [Operator].
    Binary {
        left: Box<Expression<'a>>,
        operator: Token<BinaryOperator>,
        right: Box<Expression<'a>>,
    },
}

impl<'a> Expression<'a> {
    pub fn accept<V: ASTVisitor<'a>>(&self, visitor: &V) -> Result<V::Output, V::ErrorType> {
        match self {
            Expression::Literal(_) => visitor.visit_literal(self),
            Expression::Grouping(_) => visitor.visit_grouping(self),
            Expression::Unary { .. } => visitor.visit_unary(self),
            Expression::Binary { .. } => visitor.visit_binary(self),
        }
    }
}

pub trait ASTVisitor<'a> {
    type Output;
    type ErrorType: Error;

    fn visit_literal(&self, expr: &Expression<'a>) -> Result<Self::Output, Self::ErrorType>;
    fn visit_grouping(&self, expr: &Expression<'a>) -> Result<Self::Output, Self::ErrorType>;
    fn visit_unary(&self, expr: &Expression<'a>) -> Result<Self::Output, Self::ErrorType>;
    fn visit_binary(&self, expr: &Expression<'a>) -> Result<Self::Output, Self::ErrorType>;
}
