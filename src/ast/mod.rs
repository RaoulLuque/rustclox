pub use crate::scanner::token::Token;
use crate::scanner::token::{Literal, Operator, UnaryOperator};

mod ast_printer;

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
    pub fn accept<T, V: ASTVisitor<T>>(&self, visitor: &V) -> T {
        match self {
            Expression::Literal(_) => visitor.visit_literal(self),
            Expression::Grouping(_) => visitor.visit_grouping(self),
            Expression::Unary { .. } => visitor.visit_unary(self),
            Expression::Binary { .. } => visitor.visit_binary(self),
        }
    }
}

pub trait ASTVisitor<T> {
    fn visit_literal(&self, expr: &Expression) -> T;
    fn visit_grouping(&self, expr: &Expression) -> T;
    fn visit_unary(&self, expr: &Expression) -> T;
    fn visit_binary(&self, expr: &Expression) -> T;
}
