use crate::lexer::{Scanner, TokenType, Token, Operation};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(f64),
    Unary(TokenType, Box<Expression>),
    Binary(Box<Expression>, TokenType, Box<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Expression {
        
        let expr = self.sum();
    if self.peek_type() != TokenType::eof {
        panic!("Exception: unexpected token after expression: {:?}", self.peek_type());
    }
    expr


    }

    pub fn sum(&mut self) -> Expression {
        let mut left = self.multiplication();
        while self.peek_type() == TokenType::operation(Operation::adition)
            || self.peek_type() == TokenType::operation(Operation::subtration)
        {
            let operator = self.advance().r#type;
            let right = self.multiplication();
            left = Expression::Binary(Box::new(left), operator, Box::new(right));
        }
        left
    }

    pub fn multiplication(&mut self) -> Expression {
        let mut left = self.unary();
        while self.peek_type() == TokenType::operation(Operation::multiplication)
            || self.peek_type() == TokenType::operation(Operation::division)
        {
            let operator = self.advance().r#type;
            let right = self.unary();
            left = Expression::Binary(Box::new(left), operator, Box::new(right));
        }
        left
    }

    pub fn unary(&mut self) -> Expression {
        if self.peek_type() == TokenType::operation(Operation::subtration) {
            self.advance();
            let expr = self.unary();
            return Expression::Unary(TokenType::operation(Operation::subtration), Box::new(expr));
        }
        self.primary()
    }

    pub fn primary(&mut self) -> Expression {
        let t = self.advance();
        match t.r#type {
            TokenType::num_literal => Expression::Literal(t.lexemme.parse::<f64>().unwrap()),
            TokenType::opening_bracket => {
                let expr = self.sum();
                if self.peek_type() != TokenType::closing_bracket {
                    panic!("Exception: expected ')' but found {:?}", self.peek_type());
                }
                self.advance();
                expr
            }
            _ => panic!("Exception: was waiting for a number or an '(' but found {:?}", t.r#type),
        }
    }

    pub fn peek_type(&self) -> TokenType {
        self.tokens[self.current].clone().r#type
    }

    pub fn advance(&mut self) -> Token {
        let t = self.tokens[self.current].clone();
        self.current += 1;
        t
    }
}


fn main(){
    println!("corven language parser!");
}
