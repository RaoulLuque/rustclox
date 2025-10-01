use std::error::Error;

pub use crate::scanner::token::Token;
use crate::scanner::token::{Literal, Operator, UnaryOperator};

pub mod ast_printer;

pub enum Expression<'a> {
    Literal(Literal<'a>),
    Grouping(Box<Expression<'a>>),
    Unary {
        operator: Token<UnaryOperator>,
        right: Box<Expression<'a>>,
    },
    Binary {
        left: Box<Expression<'a>>,
        operator: Token<Operator>,
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
