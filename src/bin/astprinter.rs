use rlox::expr::{Expr, LiteralExpr, UnaryExpr, BinaryExpr, GroupingExpr};
use rlox::ast_pretty_printer::PrettyPrinter;
use rlox::token::{Token, TokenType};

fn main() {
    let expression = Expr::Binary(BinaryExpr::new(
        Expr::Unary(UnaryExpr::new(
            Token::new(TokenType::MINUS, String::from("-"), 1),
            Expr::Literal(LiteralExpr::Number(123f64))
        )),
        Token::new(TokenType::STAR, String::from("*"), 1),
        Expr::Grouping(GroupingExpr::new(
            Expr::Literal(LiteralExpr::Number(45.67f64))
        ))
    ));

    println!("{}", expression.print())
}