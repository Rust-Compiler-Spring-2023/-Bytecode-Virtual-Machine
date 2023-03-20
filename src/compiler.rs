use crate::scanner::*;
use crate::token_type::TokenType;
use crate::chunk::*;

pub fn compile(source : String, chunk: &Chunk) -> bool{
    let mut scanner : Scanner = Scanner::new(source);
    advance();
    expression();
    consume(TokenType::TokenEof, "Expect end of expression.");
}