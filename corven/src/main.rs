

mod parser;

mod lexer;

mod ast;

use crate::parser::{Expression,Parser};

use crate::lexer::{Scanner, TokenType, Token, Operator};




fn main() {



     let teste = "10*9-7+(8/7-3+2*4)/7*9*8";
    let mut  scanner = Scanner::new(teste);
    let tokens = scanner.scan_tokens();

   // let teste:String = String::from("10*9-7(8/7-3+2*4)/7*9**");
   //
    for token in tokens{
        println!("{:?} '{}' (linha {}) (coluna {})",token.r#type,token.lexemme,token.line,token.column);
    }


    let mut expression = Parser::new(tokens.to_vec());
    let expression_tree = expression.parse();
    

    println!("{:?}",expression_tree);



    

}
