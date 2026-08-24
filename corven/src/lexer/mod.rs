
#[derive(Debug,Clone,Copy,PartialEq)]
pub enum TokenType {
    collon,
    equal,
    greater_than,
    less_than,
    greater_equal,
    less_equal,
    opening_bracket,
    closing_bracket,
    opening_braces,
    closing_braces,
    semicolon,
    comma,
    r#true,
    r#false,
    point,
    slash,
    star,
    bang,
    bang_equal,
    r#while,
    r#for,
    Number,
    r#let,
    string,
    string_literal,
    num_literal,
    bool,
    identifier,
    operation(Operation),
    eof
}

#[derive(Debug,Clone,Copy,PartialEq)]
pub enum Operation{
    adition,
    subtration,
    division,
    multiplication

}

#[derive(Debug,Clone)]
pub struct Token{
    r#type: TokenType,
    lexemme: String,
    line: usize,
    column: usize
}


#[derive(Debug)]
pub struct Scanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    line_current: usize,
    line: usize,
    column:usize,
    tokens: Vec<Token>

}

impl Scanner{
    pub fn new(source: &str) -> Self {
        Scanner{
            source: source.chars().collect(),
            start: 0,
            current: 0,
            line_current: 1,
            line:1,
            column:1,
            tokens: Vec::new()
        }
    }

    
    
    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.end_reached(){
            self.start = self.current;
            self.scan_token();

        };
        self.tokens.push(Token{
             r#type:TokenType::eof,
             lexemme: String::new(),
             line: self.line,
             column: 1
        });
        &self.tokens
    }

    pub fn add_token(&mut self,tokentype:TokenType)  {
        let token:String = self.source[self.start..self.current].iter().collect();
        self.tokens.push( Token{
            r#type:tokentype,
            lexemme:token.clone(),
            line: self.line,
            column: self.line_current - token.len()
            }
        );
        }

    pub fn scan_token(&mut self){
        let character = self.advance();

        match character {
            ')' => self.add_token(TokenType::closing_bracket),
            '(' => self.add_token(TokenType::opening_bracket),
            '!' => if self.peek() == '=' {
                self.advance();
                self.add_token(TokenType::bang_equal)
            }else { 
                self.add_token(TokenType::bang)
            },
            '/' => self.add_token(TokenType::operation(Operation::division)),
            '=' => self.add_token(TokenType::equal),
            '*' => self.add_token(TokenType::operation(Operation::multiplication)),
            '.' => self.add_token(TokenType::point),
            ',' => self.add_token(TokenType::comma),
            '+' => self.add_token(TokenType::operation(Operation::adition)),
            '-' => self.add_token(TokenType::operation(Operation::subtration)),
            ':' => self.add_token(TokenType::collon),
            ';' => self.add_token(TokenType::semicolon),
            '>' => if self.peek() == '='{
                self.advance();
                self.add_token(TokenType::greater_equal)
            }else{
                self.add_token(TokenType::greater_than)
            },
            '<' => if self.peek() == '=' {
                self.advance();
                self.add_token(TokenType::less_equal)
            }else {
                self.add_token(TokenType::less_than)
            },
            '\r'|' '|'\t' => {},
            '"' => self.string(),
            '\n'=> {
                self.line +=1;
                self.line_current =1;
            },
             _ =>  if character.is_ascii_digit(){
                    self.number()
             }else if character.is_alphabetic(){
                 self.identifier()
             }
        }
    }

    pub fn advance(&mut self) -> char{
        let character = self.source[self.current];
        self.current += 1;
        self.line_current += 1;
        character

    }

    pub fn peek(&self) -> char{
        if self.end_reached(){
            '\0'
        }else {
            
            self.source[self.current]
        }
    }

    pub fn peek_ahead(&self) -> char{
        if self.end_reached(){
            '\0'
        }else{
            self.source[self.current + 1]
        }
    }

    pub fn number(&mut self){
        
        while self.peek().is_ascii_digit(){

            self.advance();
            
        }
        if self.peek() == '.' && self.peek_ahead().is_ascii_digit(){
            self.advance();
             
            while self.peek().is_ascii_digit(){
                self.advance();
            }
        }

        self.add_token(TokenType::num_literal);
    }

    pub fn string(&mut self){
        self.advance();
        while self.peek() != '"'  && !self.end_reached(){
            self.advance();
        }
        self.advance();
        self.add_token(TokenType::string_literal);
        
    }

    pub fn identifier(&mut self){
        while self.peek().is_alphanumeric(){
            self.advance();
        }
        let value:String = self.source[self.start..self.current].iter().collect();
        let Type = match value.as_str() {
            "let" => self.add_token(TokenType::r#let),
            "bool"=> self.add_token(TokenType::bool),
            "Number"=> self.add_token(TokenType::Number),
            "for"=> self.add_token(TokenType::r#for),
            "while"=> self.add_token(TokenType::r#while),
            "string"=> self.add_token(TokenType::string),
            "true"=> self.add_token(TokenType::r#true),
            "false"=> self.add_token(TokenType::r#false),
            _ => self.add_token(TokenType::identifier)
        };
    }
    

    pub fn end_reached(&self) -> bool {
        self.current >= self.source.len()
    }

}

fn main(){
    let sample = "let x = 9;
    let y = 7.9,z=0.8()


    string = \"ola\" 
    bool = true
    bool compa/ring = 9.4 >= 0;
    let first = 8 > 8;
    7!= 0;
    8+9/7*2-6;


        " ;
    println!("corven language lexer");
    let mut  scanner = Scanner::new(sample);
    let tokens = scanner.scan_tokens();
    for token in tokens{
        println!("{:?} '{}' (linha {}) (coluna {})",token.r#type,token.lexemme,token.line,token.column);
    }
}
