use crate::ast::{ASTVisitor, Expression};

/// ASTPrinter is a visitor that converts an AST into a parenthesized, Lisp-like string representation.
pub struct ASTPrinter {}

impl ASTPrinter {
    pub fn new() -> Self {
        ASTPrinter {}
    }

    pub fn print(&self, expr: &Expression) -> String {
        expr.accept(self)
    }

    fn parenthesize(&self, name: &str, exprs: &[&Expression]) -> String {
        let mut result = String::new();
        result.push('(');
        result.push_str(name);
        for expr in exprs {
            result.push(' ');
            result.push_str(&expr.accept(self));
        }
        result.push(')');
        result
    }
}

impl ASTVisitor<String> for ASTPrinter {
    fn visit_literal(&self, expr: &Expression) -> String {
        if let Expression::Literal(literal) = expr {
            format!("{:?}", literal)
        } else {
            unreachable!()
        }
    }

    fn visit_grouping(&self, expr: &Expression) -> String {
        if let Expression::Grouping(inner) = expr {
            format!("(group {})", inner.accept(self))
        } else {
            unreachable!()
        }
    }

    fn visit_unary(&self, expr: &Expression) -> String {
        if let Expression::Unary { operator, right } = expr {
            format!("({:?} {})", operator.token_type, right.accept(self))
        } else {
            unreachable!()
        }
    }

    fn visit_binary(&self, expr: &Expression) -> String {
        if let Expression::Binary {
            left,
            operator,
            right,
        } = expr
        {
            format!(
                "({:?} {} {})",
                operator.token_type,
                left.accept(self),
                right.accept(self)
            )
        } else {
            unreachable!()
        }
    }
}
