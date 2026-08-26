
use crate::lexer::{Scanner,TokenType,Token,Operation};



#[derive(Debug,Clone,PartialEq)]
pub struct Expression{
    Literal(f64),
    Unary(TokenType,Box<Expression>),
    Binary(Box<Expression>,TokenType,Box<Expression>)
}

#[derive(Debug,Clone,PartialEq)]
pub struct Parser{
    tokens: Vec<Token>,
    current: usize
}



pub impl Parser{
    pub fn new(&self,tokens:Vec<Token>) -> Self{
        Parser{
            tokens
            current:0
        }
    }

    pub fn parse(&self) -> Expression{
        sum()
    }

    pub fn sum() -> Expression{
        let left =  self.multiplication();

        while self.peek_type == TokenType::operation(Operation::adition) || self.peek_type == TokenType::operation(Operation::subtration){
            let operator = self.advance().r#type;
            let right = self.multiplication();

            left = Expression {
                Binary(left,operator,right)
            };

            left


        }



    }
    
    pub fn multiplication(&self) -> Expression{
        let left = self.unary()

        while self.peek_type() == TokenType::operation(Operation::multiplication) || self.peek_type() == TokenType::operation(Operation::division) {
            let operator = self.advance().r#type;
            let right = self.unary();

            left = Expression::Binary(left,operator,right)

        } 

        left
    }

    pub fn literal_value(&self) -> Expression {

        todo!();

    }

    pub fn look_ahead(){
        todo!();
    }

    pub fn peek_type(&self) -> TokenType {
        self.tokens[self.current].clone().r#type
    }

    pub fn advance(&self) -> Token{
            
        let t = self.tokens[self.current].clone();
        self.current += 1;
        t

    }

}


fn main(){  
    
    println!("corven language parser!");
}
