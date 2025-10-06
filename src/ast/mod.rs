use std::error::Error;

pub use crate::scanner::token::Token;
use crate::scanner::token::{BinaryOperator, Identifier, Literal, UnaryOperator};

pub mod ast_printer;

/// A Declaration in the AST.
pub enum Decl<'a> {
    /// A variable declaration. Is preceded by 'var' and followed by a semicolon ';
    Var {
        name: Token<Identifier<'a>>,
        initializer: Expression<'a>,
    },
    Statement(Stmt<'a>),
}

/// A statement in the AST.
pub enum Stmt<'a> {
    /// An expression statement. Is followed by a semicolon ';'.
    Expression(Expression<'a>),
    /// A print statement. Is preceded by 'print' and followed by a semicolon ';'.
    Print(Expression<'a>),
}

impl<'a> Stmt<'a> {
    pub fn accept<V: StmtVisitor<'a>>(&self, visitor: &V) -> Result<V::Output, V::ErrorType> {
        match self {
            Stmt::Expression(_) => visitor.visit_expression_stmt(self),
            Stmt::Print(_) => visitor.visit_print_stmt(self),
        }
    }
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
    /// An identifier.
    Identifier(Identifier<'a>),
}

impl<'a> Expression<'a> {
    pub fn accept<V: ExprVisitor<'a>>(&self, visitor: &V) -> Result<V::Output, V::ErrorType> {
        match self {
            Expression::Literal(_) => visitor.visit_literal(self),
            Expression::Grouping(_) => visitor.visit_grouping(self),
            Expression::Unary { .. } => visitor.visit_unary(self),
            Expression::Binary { .. } => visitor.visit_binary(self),
        }
    }
}

pub trait StmtVisitor<'a> {
    type Output;
    type ErrorType: Error;

    fn visit_expression_stmt(&self, stmt: &Stmt<'a>) -> Result<Self::Output, Self::ErrorType>;
    fn visit_print_stmt(&self, stmt: &Stmt<'a>) -> Result<Self::Output, Self::ErrorType>;
}

pub trait ExprVisitor<'a> {
    type Output;
    type ErrorType: Error;

    fn visit_literal(&self, expr: &Expression<'a>) -> Result<Self::Output, Self::ErrorType>;
    fn visit_grouping(&self, expr: &Expression<'a>) -> Result<Self::Output, Self::ErrorType>;
    fn visit_unary(&self, expr: &Expression<'a>) -> Result<Self::Output, Self::ErrorType>;
    fn visit_binary(&self, expr: &Expression<'a>) -> Result<Self::Output, Self::ErrorType>;
}
