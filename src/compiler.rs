use crate::scanner::*;
use crate::token_type::TokenType;

pub fn compile(source : String) {
    let mut scanner : Scanner = Scanner::new(source);

    let mut line : usize = 0;
    loop{
        let token : Token = scanner.scan_token();
        if token.line != line{
            print!("{:4} ", token.line);
            line = token.line;
        } else{
            print!("   | ")
        }

        print!("{:4?} '{}'\n", token._type, token.lexeme);
        
        if token._type == TokenType::TokenEof {break;}
    }

}