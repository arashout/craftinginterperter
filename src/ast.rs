use crate::token::Token;
use std::fmt;
use std::convert::TryFrom;

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

// TODO: Use token wrapper and custom error type
impl TryFrom<&Token> for Operator {
    type Error = String;

    fn try_from(token: &Token) -> Result<Self, Self::Error> {
        match token {
            Token::Minus => Ok(Operator::Minus),
            Token::Plus => Ok(Operator::Plus),
            Token::Slash => Ok(Operator::Divide),
            Token::Star => Ok(Operator::Star),
            // One or two character tokens.
            Token::Bang => Ok(Operator::Bang),
            Token::BangEqual => Ok(Operator::BangEqual),
            Token::Equal => Ok(Operator::Equal),
            Token::EqualEqual => Ok(Operator::EqualEqual),
            Token::Greater => Ok(Operator::Greater),
            Token::GreaterEqual => Ok(Operator::GreaterEqual),
            Token::Less => Ok(Operator::Less),
            Token::LessEqual => Ok(Operator::LessEqual),
            _ => Err(format!("Token variant is not operator: {:?}", token)),
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

impl TryFrom<&Token> for Primitive {
    type Error = String;

    fn try_from(token: &Token) -> Result<Self, Self::Error> {
        match token {
            Token::Number(n) => Ok(Primitive::Number(*n)),
            Token::Nil => Ok(Primitive::Nil),
            Token::String(s) => Ok(Primitive::String(s.to_owned())),
            Token::False => Ok(Primitive::Boolean(false)),
            Token::True => Ok(Primitive::Boolean(true)),
            _ => Err(format!("Could not convert {:?} to Primitive", token)),
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
