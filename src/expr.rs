//! # Lox Expressions
//! 

use std::fmt;
use crate::token;

pub struct Ast(Box<Expr>);

pub enum Expr {
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Literal(LiteralExpr),
    Grouping(GroupingExpr),
}

pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: token::Token,
    pub right: Box<Expr>,
}

impl BinaryExpr {
    pub fn new(left: Expr, operator: token::Token, right: Expr) -> Self {
        BinaryExpr {
            left: Box::new(left),
            operator: operator,
            right: Box::new(right),
        }
    }
}

pub struct UnaryExpr {
    pub operator: token::Token,
    pub operand: Box<Expr>,
}

impl UnaryExpr {
    pub fn new(operator: token::Token, operand: Expr) -> Self {
        UnaryExpr {
            operator: operator,
            operand: Box::new(operand),
        }
    }
}

pub enum LiteralExpr {
    Number(f64),
    String(String),
    Bool(bool),
    Nil
}

pub struct GroupingExpr(pub Box<Expr>);

impl GroupingExpr {
    pub fn new(inner: Expr) -> Self {
        GroupingExpr(Box::new(inner))
    }
}

// Trait implementations

// DISPLAY TRAIT
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Binary(b) => b.fmt(f),
            Expr::Unary(u) => u.fmt(f),
            Expr::Literal(l) => l.fmt(f),
            Expr::Grouping(g) => g.fmt(f),
        }
    }
}
impl fmt::Display for BinaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.operator.lexeme, self.left, self.right)
    }
}

impl fmt::Display for UnaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {})", self.operand, self.operator.lexeme)
    }
}

impl fmt::Display for LiteralExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiteralExpr::Bool(b) => write!(f, "{}", b),
            LiteralExpr::Number(n) => write!(f, "{}", n),
            LiteralExpr::String(s) => write!(f, "'{}'", s),
            LiteralExpr::Nil => write!(f, "nil"),
        }
    }
}

impl fmt::Display for GroupingExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(group {})", self.0)
    }
}