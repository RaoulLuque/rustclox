use core::panic;

use crate::ast::{ExprVisitor, Expression};

/// ASTPrinter is a visitor that converts an AST into a parenthesized, Lisp-like string representation.
pub struct ASTPrinter {}

impl ASTPrinter {
    pub fn new() -> Self {
        ASTPrinter {}
    }

    pub fn print(&self, expr: &Expression) -> String {
        expr.accept(self)
            .expect("This should never panic as the error type is Infallible")
    }

    fn parenthesize(&self, name: &str, exprs: &[&Expression]) -> String {
        let mut result = String::new();
        result.push('(');
        result.push_str(name);
        for expr in exprs {
            result.push(' ');
            result.push_str(
                &expr
                    .accept(self)
                    .expect("This should never panic as the error type is Infallible"),
            );
        }
        result.push(')');
        result
    }
}

impl ExprVisitor<'_> for ASTPrinter {
    type Output = String;
    type ErrorType = core::convert::Infallible;

    fn visit_literal(&self, expr: &Expression) -> Result<String, Self::ErrorType> {
        if let Expression::Literal(literal) = expr {
            Ok(format!("{:?}", literal))
        } else {
            panic!("Expected Literal expression");
        }
    }

    fn visit_grouping(&self, expr: &Expression) -> Result<String, Self::ErrorType> {
        if let Expression::Grouping(inner) = expr {
            Ok(format!("(group {})", inner.accept(self).unwrap()))
        } else {
            panic!("Expected Grouping expression");
        }
    }

    fn visit_unary(&self, expr: &Expression) -> Result<String, Self::ErrorType> {
        if let Expression::Unary { operator, right } = expr {
            Ok(format!(
                "({:?} {})",
                operator.token_type,
                right.accept(self).unwrap()
            ))
        } else {
            panic!("Expected Unary expression");
        }
    }

    fn visit_binary(&self, expr: &Expression) -> Result<String, Self::ErrorType> {
        if let Expression::Binary {
            left,
            operator,
            right,
        } = expr
        {
            Ok(format!(
                "({:?} {} {})",
                operator.token_type,
                left.accept(self).unwrap(),
                right.accept(self).unwrap()
            ))
        } else {
            panic!("Expected Binary expression");
        }
    }
}
