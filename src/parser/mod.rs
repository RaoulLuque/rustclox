use std::{error::Error, fmt::Display};

use crate::{
    ast::{Decl, Expression, Stmt, Token},
    error::CloxError,
    scanner::token::{
        Bang, BinaryOperator, Identifier, Literal, Minus, TokenSubType, TokenType, UnaryOperator,
    },
};

#[derive(Debug)]
pub enum ParserError<'a> {
    UnexpectedToken {
        expected: Vec<TokenType<'a>>,
        found: Token<TokenType<'a>>,
    },
}

// TODO: Pretty print the error message
impl Display for ParserError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::UnexpectedToken { expected, found } => {
                write!(
                    f,
                    "ParserError: Expected token {:?}, but found {:?}",
                    expected, found
                )
            }
        }
    }
}

impl Error for ParserError<'_> {}

/// A recursive descent parser for the Lox programming language.
pub struct Parser<'a> {
    /// The list of tokens to parse.
    tokens: Vec<Token<TokenType<'a>>>,
    /// The index of the current token being parsed in the vec of tokens.
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<TokenType<'a>>>) -> Self {
        Parser { tokens, current: 0 }
    }

    /// Parses the list of tokens and returns a vector of declarations representing the AST.
    /// Synchronizes the parser if an error is encountered.
    pub fn parse(&mut self, source: &str) -> Vec<Decl<'a>> {
        // Initialize with a rough estimate TODO: Possibly optimize this
        let mut declarations = Vec::with_capacity(self.tokens.len() / 10 + 1);
        while !self.is_at_end() {
            match self.parse_declaration() {
                Ok(decl) => declarations.push(decl),
                Err(err) => {
                    self.synchronize();
                    // Report the error
                    CloxError::ParserError(err).report_error(source);
                }
            }
        }
        declarations
    }

    /// Parses a declaration and returns the resulting AST node.
    /// Synchronizes the parser if an error is encountered.
    ///
    /// The BNF rules are:
    /// declaration    → varDecl | statement ;
    fn parse_declaration(&mut self) -> Result<Decl<'a>, ParserError<'a>> {
        if self.match_token(&[TokenType::Var]).is_some() {
            self.parse_var_declaration()
        } else {
            Ok(Decl::Statement(self.parse_statement()?))
        }
    }

    /// Parses a variable declaration and returns the resulting AST node.
    ///
    /// The BNF rule is:
    /// varDecl        → "var" IDENTIFIER ( "=" expression )? ";" ;
    fn parse_var_declaration(&mut self) -> Result<Decl<'a>, ParserError<'a>> {
        let name_token = self
            .consume(TokenType::Identifier(Identifier { name: "" }))?
            .to_token_sub_type(&Identifier { name: "" })
            .unwrap(); // We just consumed an identifier, so this is safe

        let initializer = if self.match_token(&[TokenType::Equal]).is_some() {
            self.parse_expression()?
        } else {
            Expression::Literal(Literal::Nil)
        };

        self.consume(TokenType::Semicolon)?;

        Ok(Decl::Var {
            name: name_token,
            initializer,
        })
    }

    /// Parses a statement and returns the resulting AST node.
    ///
    /// The BNF rules are:
    /// statement      → exprStmt | printStmt ;
    fn parse_statement(&mut self) -> Result<Stmt<'a>, ParserError<'a>> {
        if self.match_token(&[TokenType::Print]).is_some() {
            self.parse_print_statement()
        } else {
            self.parse_expression_statement()
        }
    }

    /// Parses a print statement and returns the resulting AST node.
    ///
    /// The BNF rule is:
    /// printStmt      → "print" expression ";" ;
    fn parse_print_statement(&mut self) -> Result<Stmt<'a>, ParserError<'a>> {
        let value = self.parse_expression()?;
        self.consume(TokenType::Semicolon)?;
        Ok(Stmt::Print(value))
    }

    /// Parses an expression statement and returns the resulting AST node.
    ///
    /// The BNF rule is:
    /// exprStmt       → expression ";" ;
    fn parse_expression_statement(&mut self) -> Result<Stmt<'a>, ParserError<'a>> {
        let expr = self.parse_expression()?;
        self.consume(TokenType::Semicolon)?;
        Ok(Stmt::Expression(expr))
    }

    /// Parses an expression and returns the resulting AST node.
    ///
    /// The BNF rule is:
    /// expression     → equality ;
    fn parse_expression(&mut self) -> Result<Expression<'a>, ParserError<'a>> {
        self.parse_equality()
    }

    /// Parses an equality expression.
    ///
    /// The BNF rule is:
    /// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
    ///
    /// Returns a ParserError if the current token is not a valid equality expression.
    fn parse_equality(&mut self) -> Result<Expression<'a>, ParserError<'a>> {
        let mut expr = self.parse_comparison()?;

        while let Some(operator) =
            self.match_token(&[BinaryOperator::BangEqual, BinaryOperator::EqualEqual])
        {
            let right = self.parse_comparison()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    /// Parses a comparison expression.
    ///
    /// The BNF rule is:
    /// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    ///
    /// Returns a ParserError if the current token is not a valid comparison expression.
    fn parse_comparison(&mut self) -> Result<Expression<'a>, ParserError<'a>> {
        let mut expr = self.parse_term()?;

        while let Some(operator) = self.match_token(&[
            BinaryOperator::Greater,
            BinaryOperator::GreaterEqual,
            BinaryOperator::Less,
            BinaryOperator::LessEqual,
        ]) {
            let right = self.parse_term()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    /// Parses a term expression.
    ///
    /// The BNF rule is:
    /// term           → factor ( ( "-" | "+" ) factor )* ;
    ///
    /// Returns a ParserError if the current token is not a valid term expression.
    fn parse_term(&mut self) -> Result<Expression<'a>, ParserError<'a>> {
        let mut expr = self.parse_factor()?;

        while let Some(operator) = self.match_token(&[BinaryOperator::Minus, BinaryOperator::Plus])
        {
            let right = self.parse_factor()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    /// Parses a factor expression.
    ///
    /// The BNF rule is:
    /// factor         → unary ( ( "/" | "*" ) unary )* ;
    ///
    /// Returns a ParserError if the current token is not a valid factor expression.
    fn parse_factor(&mut self) -> Result<Expression<'a>, ParserError<'a>> {
        let mut expr = self.parse_unary()?;

        while let Some(operator) = self.match_token(&[BinaryOperator::Star, BinaryOperator::Slash])
        {
            let right = self.parse_unary()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    /// Parses a unary expression.
    ///
    /// The BNF rule is:
    /// unary          → ( "!" | "-" ) unary
    ///                | primary ;
    ///
    /// Returns a ParserError if the current token is not a valid unary expression.
    fn parse_unary(&mut self) -> Result<Expression<'a>, ParserError<'a>> {
        if let Some(operator) =
            self.match_token(&[UnaryOperator::Minus(Minus {}), UnaryOperator::Bang(Bang {})])
        {
            let right = self.parse_unary()?;
            Ok(Expression::Unary {
                operator,
                right: Box::new(right),
            })
        } else {
            self.parse_primary()
        }
    }

    /// Parses a primary expression.
    ///
    /// The BNF rule is:
    /// primary        → "true" | "false" | "nil"
    ///               | NUMBER | STRING
    ///              | "(" expression ")" ;
    ///
    /// Returns a ParserError if the current token is not a valid primary expression.
    fn parse_primary(&mut self) -> Result<Expression<'a>, ParserError<'a>> {
        if let Some(literal_token) =
            self.match_token(&[Literal::False, Literal::True, Literal::Nil])
        {
            return Ok(Expression::Literal(literal_token.token_type));
        }

        if let Some(number_token) = self.match_token(&[Literal::Number(0.0)]) {
            return Ok(Expression::Literal(number_token.token_type));
        }

        if let Some(string_token) = self.match_token(&[Literal::Str("")]) {
            return Ok(Expression::Literal(string_token.token_type));
        }

        if let Some(identifier) = self.match_token(&[Identifier { name: "" }]) {
            return Ok(Expression::Identifier(Identifier {
                name: identifier.token_type.name,
            }));
        }

        if self.match_token(&[TokenType::LeftParenthesis]).is_some() {
            let expr = self.parse_expression()?;
            self.consume(TokenType::RightParenthesis)?;
            return Ok(Expression::Grouping(Box::new(expr)));
        }

        Err(ParserError::UnexpectedToken {
            expected: vec![
                TokenType::Literal(Literal::False),
                TokenType::Literal(Literal::True),
                TokenType::Literal(Literal::Nil),
                TokenType::Literal(Literal::Number(0.0)),
                TokenType::Literal(Literal::Str("")),
                TokenType::LeftParenthesis,
            ],
            found: *self.peek(),
        })
    }

    /// Checks if the current token's type matches any of the given types. If so, consumes the current token and returns true.
    /// Otherwise, returns false.
    ///
    /// In particular, the value or associated data of the token is ignored when matching.
    fn match_token<T: TokenSubType<'a, T>>(&mut self, types: &[T]) -> Option<Token<T>> {
        for token_type in types {
            if self.check(token_type) {
                // This branch always returns Some because we just checked that the token is of the given type.
                return self.advance().to_token_sub_type(token_type);
            }
        }
        None
    }

    /// Checks if the current token is of the given type.
    fn check<T: TokenSubType<'a, T>>(&self, token_type: &T) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek()
            .token_type
            .is_same_type(&T::to_token_type(*token_type))
    }

    /// Consumes the current token and returns it.
    fn advance(&mut self) -> Token<TokenType<'a>> {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    /// Returns true if the current token is the end of file token.
    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    /// Returns the current token without consuming it.
    fn peek(&self) -> &Token<TokenType<'a>> {
        &self.tokens[self.current]
    }

    /// Returns the previous token.
    fn previous(&self) -> Token<TokenType<'a>> {
        self.tokens[self.current - 1]
    }

    /// Consumes the current token if it matches the expected type. Otherwise, returns a ParserError.
    /// This is used for tokens that must be present, such as closing parentheses.
    fn consume(
        &mut self,
        expected: TokenType<'a>,
    ) -> Result<Token<TokenType<'a>>, ParserError<'a>> {
        if self.check(&expected) {
            Ok(self.advance())
        } else {
            Err(ParserError::UnexpectedToken {
                expected: vec![expected],
                found: *self.peek(),
            })
        }
    }

    /// Synchronizes the parser after an error. This is done by discarding tokens until we reach a (heuristically determined) statement boundary.
    /// That is, we consider a semicolon or keywords (such as `class`, `fun`, `var`, `for`, `if`, `while`, `print`, `return`) as a statement boundary.
    /// This is a heuristic, because we could hit a semicolon separating clauses in a for loop for example.
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {}
            }

            self.advance();
        }
    }
}
