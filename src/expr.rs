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
}

pub struct GroupingExpr(pub Box<Expr>);

impl GroupingExpr {
    pub fn new(inner: Expr) -> Self {
        GroupingExpr(Box::new(inner))
    }
}