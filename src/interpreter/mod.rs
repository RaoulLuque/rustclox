use std::{error::Error, fmt::Display};

use crate::{
    ast::{Decl, ExprVisitor, Expression, Stmt, StmtVisitor, Token},
    scanner::token::{BinaryOperator, Literal, TokenType, UnaryOperator},
};

#[derive(PartialEq, Debug)]
pub enum LoxObject {
    Number(f32),
    Str(String),
    Boolean(bool),
    Nil,
}

#[derive(Debug)]
pub enum RuntimeError<'a> {
    TypeError(String, Token<TokenType<'a>>),
    UndefinedVariable(String),
}

// TODO: Pretty print the error message
impl Display for RuntimeError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::TypeError(msg, token) => {
                write!(
                    f,
                    "[line {}] RuntimeError at '{:?}': {}",
                    token.line, token, msg
                )
            }
        }
    }
}

impl Error for RuntimeError<'_> {}

pub struct Interpreter {}

impl Interpreter {
    /// Creates a new Interpreter instance.
    pub fn new() -> Self {
        Interpreter {}
    }

    /// Interprets an expression by evaluating it and printing the result.
    pub fn interpret(&self, declarations: &[Decl]) {
        for declaration in declarations {
            // TODO: Properly handle error here
            self.execute(declaration).unwrap();
        }
    }

    /// Executes a statement.
    fn execute<'a>(&self, stmt: &Stmt<'a>) -> Result<(), RuntimeError<'a>> {
        stmt.accept(self)
    }

    /// Evaluates an expression and returns the resulting LoxObject.
    fn evaluate<'a>(&self, expr: &Expression<'a>) -> Result<LoxObject, RuntimeError<'a>> {
        expr.accept(self)
    }

    /// Determines the "truthiness" of a LoxObject.
    /// In Lox, `false` and `nil` are falsey. Everything else is truthy.
    fn is_truthy(&self, obj: LoxObject) -> bool {
        match obj {
            LoxObject::Nil => false,
            LoxObject::Boolean(b) => b,
            _ => true,
        }
    }

    /// Converts a LoxObject to a simple string representation.
    fn stringify(&self, obj: LoxObject) -> String {
        match obj {
            LoxObject::Number(n) => n.to_string(),
            LoxObject::Str(s) => s,
            LoxObject::Boolean(b) => b.to_string(),
            LoxObject::Nil => "nil".to_string(),
        }
    }
}

impl<'a> StmtVisitor<'a> for Interpreter {
    type Output = ();
    type ErrorType = RuntimeError<'a>;

    fn visit_expression_stmt(&self, stmt: &Stmt<'a>) -> Result<Self::Output, Self::ErrorType> {
        if let Stmt::Expression(expr) = stmt {
            let _ = self.evaluate(expr)?;
            Ok(())
        } else {
            panic!("Expected Expression statement");
        }
    }

    fn visit_print_stmt(&self, stmt: &Stmt<'a>) -> Result<Self::Output, Self::ErrorType> {
        if let Stmt::Print(expr) = stmt {
            let value = self.evaluate(expr)?;
            println!("{}", self.stringify(value));
            Ok(())
        } else {
            panic!("Expected Print statement");
        }
    }
}

impl<'a> ExprVisitor<'a> for Interpreter {
    type Output = LoxObject;
    type ErrorType = RuntimeError<'a>;

    fn visit_literal(&self, value: &Expression<'a>) -> Result<Self::Output, Self::ErrorType> {
        match value {
            Expression::Literal(Literal::Number(n)) => Ok(LoxObject::Number(*n)),
            Expression::Literal(Literal::Str(s)) => Ok(LoxObject::Str(s.to_string())),
            Expression::Literal(Literal::True) => Ok(LoxObject::Boolean(true)),
            Expression::Literal(Literal::False) => Ok(LoxObject::Boolean(false)),
            Expression::Literal(Literal::Nil) => Ok(LoxObject::Nil),
            _ => panic!("Expected literal type"),
        }
    }

    fn visit_grouping(&self, expr: &Expression<'a>) -> Result<Self::Output, Self::ErrorType> {
        if let Expression::Grouping(inner) = expr {
            self.evaluate(inner)
        } else {
            panic!("Expected Grouping expression");
        }
    }

    fn visit_unary(&self, expr: &Expression<'a>) -> Result<Self::Output, Self::ErrorType> {
        if let Expression::Unary { operator, right } = expr {
            let right_val = self.evaluate(right)?;
            match (operator.token_type, right_val) {
                (UnaryOperator::Minus(_), LoxObject::Number(n)) => Ok(LoxObject::Number(-n)),
                (UnaryOperator::Minus(_), _) => Err(RuntimeError::TypeError(
                    "Operand must be a number.".to_string(),
                    (*operator).into(),
                )),
                (UnaryOperator::Bang(_), right_val) => {
                    Ok(LoxObject::Boolean(!self.is_truthy(right_val)))
                }
            }
        } else {
            panic!("Expected Unary expression");
        }
    }

    // Evaluates a binary expression. In particular, operands are evaluated left-to-right.
    fn visit_binary(&self, expr: &Expression<'a>) -> Result<Self::Output, Self::ErrorType> {
        if let Expression::Binary {
            left,
            operator,
            right,
        } = expr
        {
            let left_val = self.evaluate(left)?;
            let right_val = self.evaluate(right)?;
            match (left_val, operator.token_type, right_val) {
                // Computation operators (-, +, *, /)
                (LoxObject::Number(l), BinaryOperator::Minus, LoxObject::Number(r)) => {
                    Ok(LoxObject::Number(l - r))
                }
                (_, BinaryOperator::Minus, _) => Err(RuntimeError::TypeError(
                    "Operands to Minus need to be numbers.".to_string(),
                    (*operator).into(),
                )),

                (LoxObject::Number(l), BinaryOperator::Plus, LoxObject::Number(r)) => {
                    Ok(LoxObject::Number(l + r))
                }
                (LoxObject::Str(l), BinaryOperator::Plus, LoxObject::Str(r)) => {
                    Ok(LoxObject::Str(l + &r))
                }
                (_, BinaryOperator::Plus, _) => Err(RuntimeError::TypeError(
                    "Operands to Plus need to be both numbers or both strings.".to_string(),
                    (*operator).into(),
                )),

                (LoxObject::Number(l), BinaryOperator::Star, LoxObject::Number(r)) => {
                    Ok(LoxObject::Number(l * r))
                }
                (_, BinaryOperator::Star, _) => Err(RuntimeError::TypeError(
                    "Operands to Star need to be numbers.".to_string(),
                    (*operator).into(),
                )),

                (LoxObject::Number(l), BinaryOperator::Slash, LoxObject::Number(r)) => {
                    Ok(LoxObject::Number(l / r))
                }
                (_, BinaryOperator::Slash, _) => Err(RuntimeError::TypeError(
                    "Operands to Slash need to be numbers.".to_string(),
                    (*operator).into(),
                )),

                // Comparison operators (>, >=, <, <=)
                (LoxObject::Number(l), BinaryOperator::Greater, LoxObject::Number(r)) => {
                    Ok(LoxObject::Boolean(l > r))
                }
                (_, BinaryOperator::Greater, _) => Err(RuntimeError::TypeError(
                    "Operands to Greater need to be numbers.".to_string(),
                    (*operator).into(),
                )),

                (LoxObject::Number(l), BinaryOperator::GreaterEqual, LoxObject::Number(r)) => {
                    Ok(LoxObject::Boolean(l >= r))
                }
                (_, BinaryOperator::GreaterEqual, _) => Err(RuntimeError::TypeError(
                    "Operands to GreaterEqual need to be numbers.".to_string(),
                    (*operator).into(),
                )),

                (LoxObject::Number(l), BinaryOperator::Less, LoxObject::Number(r)) => {
                    Ok(LoxObject::Boolean(l < r))
                }
                (_, BinaryOperator::Less, _) => Err(RuntimeError::TypeError(
                    "Operands to Less need to be numbers.".to_string(),
                    (*operator).into(),
                )),

                (LoxObject::Number(l), BinaryOperator::LessEqual, LoxObject::Number(r)) => {
                    Ok(LoxObject::Boolean(l <= r))
                }
                (_, BinaryOperator::LessEqual, _) => Err(RuntimeError::TypeError(
                    "Operands to LessEqual need to be numbers.".to_string(),
                    (*operator).into(),
                )),

                // Equality operators (==, !=)
                (l, BinaryOperator::EqualEqual, r) => Ok(LoxObject::Boolean(l == r)),
                (l, BinaryOperator::BangEqual, r) => Ok(LoxObject::Boolean(l != r)),
            }
        } else {
            panic!("Expected Binary expression");
        }
    }
}
