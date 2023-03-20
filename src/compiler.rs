use crate::scanner::*;
use crate::token_type::TokenType;
use crate::chunk::*;

struct Parser{
    current: Token,
    previous: Token,
    hadError: bool,
}

impl Parser{
    pub fn new() -> Self{
        Parser{
            current: Token{
                _type : TokenType::Undefined,
                lexeme: String::new(),
                line: 0,
            },
            previous : Token {
                _type : TokenType::Undefined,
                lexeme: String::new(),
                line: 0
            },
            hadError : false
        }
    }
}

pub struct Compiler{
    parser : Parser,
    scanner: Scanner,
}

impl Compiler{
    pub fn new() -> Self{
        Compiler { 
            parser: Parser::new(), 
            scanner: Scanner::new(), 
        }
    }

    pub fn compile(&mut self, source : String, chunk: &Chunk) -> bool{
        self.scanner.source = source;
        self.advance();
        expression();
        consume(TokenType::TokenEof, "Expect end of expression.");
        return !self.parser.hadError;
    }

    fn advance(&mut self){
        self.parser.previous = self.parser.current;
        
        loop{
            self.parser.current = self.scanner.scan_token();
            if self.parser.current._type != TokenType::TokenError {break;}

            self.error_at_current(&self.parser.current.lexeme);
        }
    }

    // Show an error with the current token
    fn error_at_current(&mut self, message: &String){
        self.error_at(&self.parser.current, message);
    } 

    // Show an error with the previous token
    fn error(&mut self, message: &String){
        self.error_at(&self.parser.previous, message);
    }

    // Print error
    fn error_at(&mut self, token: &Token, message: &String){
        eprint!("[line {}] Error", token.line);

        if token._type == TokenType::TokenEof {
            eprint!(" at end ");
        } else if token._type == TokenType::TokenError{
            // Nothing.
        } else {
            eprint!(" at '{}' ", token.lexeme);
        }

        eprintln!(": {}", message);
        self.parser.hadError = true;
    }
}


