use std::fmt;
use std::convert::From;
use crate::token::Token;

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Star,
    Divide,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

impl From<&Token> for Operator {
    fn from(token: &Token) -> Self {
        match token {
            Token::Minus => Operator::Minus,
            Token::Plus => Operator::Plus,
            Token::Slash => Operator::Divide,
            Token::Star => Operator::Star,
        
            // One or two character tokens.
            Token::Bang => Operator::Bang,
            Token::BangEqual => Operator::BangEqual,
            Token::Equal => Operator::Equal,
            Token::EqualEqual => Operator::EqualEqual,
            Token::Greater => Operator::Greater,
            Token::GreaterEqual => Operator::GreaterEqual,
            Token::Less => Operator::Less,
            Token::LessEqual => Operator::LessEqual,
            _ => panic!("Wrong token type for conversion!"),
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match *self {
            Operator::Plus => "+",
            Operator::Minus => "-",
            Operator::Star => "*",
            Operator::Divide => "/",
            Operator::Bang => "!",
            Operator::BangEqual => "!=",
            Operator::Equal => "=",
            Operator::EqualEqual => "==",
            Operator::Greater => ">",
            Operator::GreaterEqual => ">=",
            Operator::Less => "<",
            Operator::LessEqual => "<=",
        };
        write!(f, "{}", symbol)
    }
}

#[derive(Debug)]
pub enum Primitive {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}

impl fmt::Display for Primitive {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Primitive::String(s) => write!(f, "{}", s),
            Primitive::Number(n) => write!(f, "{}", n),
            Primitive::Boolean(b) => write!(f, "{}", b),
            Primitive::Nil => write!(f, "null"),
        }
    }
}

#[derive(Debug)]
pub enum Expr {
    Binary(Operator, Box<Expr>, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Primitive),
    Unary(Operator, Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Expr::Binary(o, b1, b2) => write!(f, "({} {} {})", o, b1, b2),
            Expr::Grouping(b) => write!(f, "(group {})", b),
            Expr::Literal(p) => write!(f, "{}", p),
            Expr::Unary(o, b) => write!(f, "({} {})", o, b),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ast_print() {
        let pp = format!(
            "{}",
            Expr::Binary(
                Operator::Star,
                Box::new(Expr::Unary(
                    Operator::Minus,
                    Box::new(Expr::Literal(Primitive::Number(123.0)))
                )),
                Box::new(Expr::Grouping(Box::new(Expr::Literal(Primitive::Number(
                    45.67
                ))))),
            )
        );
        assert_eq!(pp, "(* (- 123) (group 45.67))")
    }
}
