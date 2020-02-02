use crate::ast::{Expr, Operator, Primitive};
use crate::token::{Token, TokenWrapper};
use std::convert::TryFrom;

pub struct Parser {
    tokens: Vec<TokenWrapper>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<TokenWrapper>) -> Parser {
        Parser { tokens, current: 0 }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self._match(&vec![Token::BangEqual, Token::EqualEqual]) {
            let operator: Operator =
                Operator::try_from(&self.previous().token).expect("Expected operator");
            let right = self.comparison();
            expr = Expr::Binary(operator, Box::new(expr), Box::new(right));
        }

        expr
    }
    fn comparison(&mut self) -> Expr {
        let mut expr = self.addition();

        while self._match(&vec![
            Token::Greater,
            Token::GreaterEqual,
            Token::Less,
            Token::LessEqual,
        ]) {
            let operator: Operator =
                Operator::try_from(&self.previous().token).expect("Expected operator");
            let right = self.addition();
            expr = Expr::Binary(operator, Box::new(expr), Box::new(right));
        }

        expr
    }

    fn addition(&mut self) -> Expr {
        let mut expr = self.multiplication();

        while self._match(&vec![Token::Minus, Token::Plus]) {
            let operator: Operator =
                Operator::try_from(&self.previous().token).expect("Expected operator");
            let right = self.multiplication();
            expr = Expr::Binary(operator, Box::new(expr), Box::new(right));
        }

        expr
    }

    fn multiplication(&mut self) -> Expr {
        let mut expr = self.unary();

        while self._match(&vec![Token::Star, Token::Slash]) {
            let operator: Operator =
                Operator::try_from(&self.previous().token).expect("Expected operator");
            let right = self.unary();
            expr = Expr::Binary(operator, Box::new(expr), Box::new(right));
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self._match(&vec![Token::Bang, Token::Minus]) {
            let operator: Operator =
                Operator::try_from(&self.previous().token).expect("Expected operator");
            let right = self.unary();
            Expr::Unary(operator, Box::new(right))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Expr {
        let p = Primitive::try_from(self.advance()).expect("Not a primitive");
        // TODO: Try to match grouping if primitive match fails
        Expr::Literal(p)
    }

    fn _match(&mut self, needles: &Vec<Token>) -> bool {
        for needle in needles.iter() {
            if self.check(needle) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, needle: &Token) -> bool {
        if self.is_at_end() {
            return false;
        }
        return self.peek() == needle;
    }

    fn advance(&mut self) -> &Token {
        if (!self.is_at_end()) {
            self.current += 1;
        }
        return &self.previous().token;
    }

    fn is_at_end(&self) -> bool {
        return self.peek() == &Token::Eof;
    }

    fn peek(&self) -> &Token {
        &self
            .tokens
            .get(self.current)
            .expect(&format!(
                "Tried to peek when 'current' index out of bounds: {} {}",
                self.current,
                self.tokens.len()
            ))
            .token
    }

    fn previous(&self) -> &TokenWrapper {
        return self.tokens.get(self.current - 1).expect(&format!(
            "Tried to previous when 'previous' index out of bounds: {} {}",
            self.current - 1,
            self.tokens.len()
        ));
    }
}
