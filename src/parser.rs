//! # Lox Parser
//! 

use crate::loxerror;
use crate::token::{Token, TokenType::*};
use crate::expr::{Expr, UnaryExpr, LiteralExpr, BinaryExpr, GroupingExpr};


pub struct ParserError {
    token: Option<Token>,
}

impl ParserError {
    pub fn new(token: Option<Token>) -> Self {
        Self { token }
    }
}

impl From<ParserError> for loxerror::LoxError {
    fn from(error: ParserError) -> Self {
        match error.token {
            Some(t) => {
                let msg = format!("Parsing error: Unexpected token '{}'({:?}) at line {}", t.lexeme, t.token_type, t.line);
                loxerror::LoxError::new(&msg)
            },
            None => {
                let msg = format!("Parsing error: Unexpectedly reached end of file");
                loxerror::LoxError::new(&msg)
            }
        }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens,
            current: 0
        }
    }

    pub fn parse(&mut self) -> Result<Expr, ParserError> {
        self.expression() 
    }

    fn expression(&mut self) -> Result<Expr, ParserError> {
        self.equality()
    }
    
    fn equality(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.comparison()?;

        loop {
            match self.current() {
                Some(t) if t.token_type == BANG_EQUAL || t.token_type == EQUAL_EQUAL => {
                    self.advance();
                    let right = self.comparison()?;
                    expr = Expr::Binary(BinaryExpr::new(expr, t, right));
                },
                None => { return Err(ParserError::new(None)) },
                _ => { break; }
            }
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.addition()?;

        loop {
            match self.current() {
                Some(t) if t.token_type == GREATER_EQUAL ||
                            t.token_type == GREATER ||
                            t.token_type == LESS_EQUAL ||
                            t.token_type == LESS => {
                    self.advance();
                    let right = self.addition()?;
                    expr = Expr::Binary(BinaryExpr::new(expr, t, right));
                },
                None => { return Err(ParserError::new(None)); },
                _ => { break; }
            }
        }

        Ok(expr)
    }

    fn addition(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.multiplication()?;

        loop {
            match self.current() {
                Some(t) if t.token_type == MINUS || t.token_type == PLUS => {
                    self.advance();
                    let right = self.multiplication()?;
                    expr = Expr::Binary(BinaryExpr::new(expr, t, right));
                },
                None => { return Err(ParserError::new(None)); },
                _ => { break; }
            }
        }

        Ok(expr)
    }

    fn multiplication(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.unary()?;

        loop {
            match self.current() {
                Some(t) if t.token_type == SLASH || t.token_type == STAR => {
                    self.advance();
                    let right = self.unary()?;
                    expr = Expr::Binary(BinaryExpr::new(expr, t, right));
                },
                None => { return Err(ParserError::new(None)); },
                _ => { break; }
            }
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParserError> {
        match self.current() {
            Some(t) if t.token_type == BANG || t.token_type == MINUS => {
                self.advance();
                let right = self.unary()?;
                Ok(Expr::Unary(UnaryExpr::new(t, right)))
            },
            Some(_) => self.primary(),
            None => Err(ParserError::new(None)),
        }
    }

    fn primary(&mut self) -> Result<Expr, ParserError> {        
        let res = match self.current() {
            Some(Token { token_type: FALSE, .. }) => {
                self.advance();
                Ok(Expr::Literal(LiteralExpr::Bool(false)))
            },
            Some(Token { token_type: TRUE, ..}) => {
                self.advance();
                Ok(Expr::Literal(LiteralExpr::Bool(true)))
            },
            Some(Token { token_type: NIL, ..}) => {
                self.advance();
                Ok(Expr::Literal(LiteralExpr::Nil))
            },
            Some(Token { token_type: NUMBER(n), ..}) => {
                self.advance();
                Ok(Expr::Literal(LiteralExpr::Number(n)))
            },
            Some(Token { token_type: STRING(s), ..}) => {
                self.advance();
                Ok(Expr::Literal(LiteralExpr::String(s)))
            },
            Some(Token { token_type: LEFT_PAREN, ..}) => {
                self.advance();
                let expr = self.expression()?;

                match self.current() {
                    Some(Token { token_type: RIGHT_PAREN, ..}) => { self.advance(); Ok(Expr::Grouping(GroupingExpr::new(expr)))} ,
                    unexpected @ _ => Err(ParserError::new(unexpected)),
                }
            },
            unexpected @ Some(_) => Err(ParserError::new(unexpected)),
            None => Err(ParserError::new(None)),
        };

        res
    }

    fn advance(&mut self) {
        // this prevents us from incrementing the current counter
        // when we reach the end
        if self.current().is_some() {
            self.current += 1;
        }
    }

    fn current(&self) -> Option<Token> {
        self.tokens.get(self.current).cloned()
    }

    fn next(&self) -> Option<Token> {
        self.tokens.get(self.current + 1).cloned()
    }

    fn previous(&self) -> Option<Token> {
        self.tokens.get(self.current - 1).cloned()
    }
}