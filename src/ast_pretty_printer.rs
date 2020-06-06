use crate::expr::{Expr, LiteralExpr, UnaryExpr, BinaryExpr, GroupingExpr};

pub trait PrettyPrinter {
    fn print(&self) -> String;
}

impl PrettyPrinter for Expr {
    fn print(&self) -> String {
        match self {
            Expr::Binary(b) => b.print(),
            Expr::Unary(u) => u.print(),
            Expr::Literal(l) => l.print(),
            Expr::Grouping(g) => g.print(),
        }
    }
}
impl PrettyPrinter for BinaryExpr {
    fn print(&self) -> String {
        let left = self.left.print();
        let right = self.right.print();
        let operator = &self.operator.lexeme;

        format!("({} {} {})", operator, left, right)
    }
}

impl PrettyPrinter for UnaryExpr {
    fn print(&self) -> String {
        let operand = self.operand.print();
        let operator = &self.operator.lexeme;

        format!("({} {})", operator, operand)
    }
}

impl PrettyPrinter for LiteralExpr {
    fn print(&self) -> String {
        match self {
            LiteralExpr::Bool(b) => format!("{}", b),
            LiteralExpr::Number(n) => format!("{}", n),
            LiteralExpr::String(s) => format!("{}", s),
        }
    }
}

impl PrettyPrinter for GroupingExpr {
    fn print(&self) -> String {
        format!("(group {})", self.0.print())
    }
}