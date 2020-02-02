use crate::ast::{Expr, Operator};
use crate::token::{TokenWrapper, Token};

pub struct Parser {
    tokens: Vec<TokenWrapper>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<TokenWrapper>) -> Parser {
        Parser{tokens, current: 0}
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self._match(&vec![Token::BangEqual, Token::EqualEqual]) {
            // At this point we can assume self.previous is an operator Token and can be converted to
            // ast::Operator
            let operator: Operator = Operator::from(&self.previous().token);
            let right = self.comparison();
            expr = Expr::Binary(operator, Box::new(expr), Box::new(right));
        }

        expr
    }
    
    fn comparison(&mut self) -> Expr {
        unimplemented!()
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
        &self.tokens
            .get(self.current)
            .expect(&format!("Tried to peek when 'current' index out of bounds: {} {}", self.current, self.tokens.len()))
            .token   
      }                                
    
    fn previous(&self) -> &TokenWrapper {       
        return self.tokens.get(self.current - 1).expect(&format!("Tried to previous when 'previous' index out of bounds: {} {}", self.current-1, self.tokens.len()))
      }            

}