use crate::loxvalue::LoxValue;
use crate::expr::{Expr, UnaryExpr, LiteralExpr, BinaryExpr, GroupingExpr};
use crate::token::{TokenType::*, Token};

pub struct RuntimeError;

impl RuntimeError {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct Interpreter;


impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }
}

pub trait Interpret {
    fn interpret(&self) -> Result<LoxValue, RuntimeError>;
}

impl Interpret for Expr {
    fn interpret(&self) -> Result<LoxValue, RuntimeError> {
        match self {
            Expr::Unary(u) => u.interpret(),
            Expr::Binary(b) => b.interpret(),
            Expr::Literal(l) => l.interpret(),
            Expr::Grouping(g) => g.interpret(),
        }
    }
}

impl Interpret for LiteralExpr {
    fn interpret(&self) -> Result<LoxValue, RuntimeError> {
        match self {
            LiteralExpr::Nil => Ok(LoxValue::LoxNil),
            LiteralExpr::Bool(b) => Ok(LoxValue::LoxBool(*b)),
            LiteralExpr::Number(n) => Ok(LoxValue::LoxNumber(*n)),
            LiteralExpr::String(s) => Ok(LoxValue::LoxString(s.clone()))
        }
    }
}

impl Interpret for UnaryExpr {
    fn interpret(&self) -> Result<LoxValue, RuntimeError> {
        let value = self.operand.interpret()?;

        match self.operator {
            Token { token_type: MINUS, ..} => {
                match value {
                    LoxValue::LoxNumber(n) => Ok(LoxValue::LoxNumber(-1f64 * n)),
                    _ => Err(RuntimeError::new())
                }
            },
            Token { token_type: BANG, ..} => Ok(LoxValue::LoxBool(!is_truthy(value))),
            _ => unreachable!(),
        }
    }
}

impl Interpret for BinaryExpr {
    fn interpret(&self) -> Result<LoxValue, RuntimeError> {
        let left = self.left.interpret()?;
        let right = self.right.interpret()?;

        match self.operator {
            Token { token_type: PLUS, ..} => {
                match (left, right) {
                    (LoxValue::LoxNumber(l), LoxValue::LoxNumber(r)) => Ok(LoxValue::LoxNumber(l + r)),
                    (LoxValue::LoxString(l), LoxValue::LoxString(r)) => Ok(LoxValue::LoxString(format!("{}{}", l, r))),
                    _ => Err(RuntimeError::new()),
                }
            }
            Token { token_type: MINUS, ..} => {
                match (left, right) {
                    (LoxValue::LoxNumber(l), LoxValue::LoxNumber(r)) => Ok(LoxValue::LoxNumber(l - r)),
                    _ => Err(RuntimeError::new())
                }
            },
            Token { token_type: SLASH, ..} => {
                match (left, right) {
                    (LoxValue::LoxNumber(l), LoxValue::LoxNumber(r)) => Ok(LoxValue::LoxNumber(l / r)),
                    _ => Err(RuntimeError::new()),
                }
            },
            Token { token_type: STAR, ..} => {
                match (left, right) {
                    (LoxValue::LoxNumber(l), LoxValue::LoxNumber(r)) => Ok(LoxValue::LoxNumber(l * r)),
                    _ => Err(RuntimeError::new())
                }
            },
            Token { token_type: GREATER, ..} => {
                match (left, right) {
                    (LoxValue::LoxNumber(l), LoxValue::LoxNumber(r)) => Ok(LoxValue::LoxBool(l > r)),
                    _ => Err(RuntimeError::new())
                }
            },
            Token { token_type: GREATER_EQUAL, ..} => {
                match (left, right) {
                    (LoxValue::LoxNumber(l), LoxValue::LoxNumber(r)) => Ok(LoxValue::LoxBool(l >= r)),
                    _ => Err(RuntimeError::new())
                }
            },
            Token { token_type: LESS, ..} => {
                match (left, right) {
                    (LoxValue::LoxNumber(l), LoxValue::LoxNumber(r)) => Ok(LoxValue::LoxBool(l < r)),
                    _ => Err(RuntimeError::new())
                }
            },
            Token { token_type: LESS_EQUAL, ..} => {
                match (left, right) {
                    (LoxValue::LoxNumber(l), LoxValue::LoxNumber(r)) => Ok(LoxValue::LoxBool(l <= r)),
                    _ => Err(RuntimeError::new())
                }
            },
            Token { token_type: EQUAL_EQUAL, ..} => Ok(LoxValue::LoxBool(is_equal(left, right))),
            Token { token_type: BANG_EQUAL, ..} => Ok(LoxValue::LoxBool(!is_equal(left, right))),
            _ => unreachable!()
        }
    }
}


impl Interpret for GroupingExpr {
    fn interpret(&self) -> Result<LoxValue, RuntimeError> {
        (*self.0).interpret()
    }
}


fn is_truthy(v: LoxValue) -> bool {
    match v {
        LoxValue::LoxNil => false,
        LoxValue::LoxBool(b) if b == false => false,
        _ => true,
    }
}

fn is_equal(left: LoxValue, right: LoxValue) -> bool {
    match (left, right) {
        (LoxValue::LoxNil, LoxValue::LoxNil) => true,
        (LoxValue::LoxBool(l), LoxValue::LoxBool(r)) => l == r,
        (LoxValue::LoxNumber(l), LoxValue::LoxNumber(r)) => l == r,
        (LoxValue::LoxString(l), LoxValue::LoxString(r)) => l == r,
        _ => false,
    }
}