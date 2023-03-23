use crate::value::Value;
use crate::{scanner::*, chunk};
use crate::token_type::TokenType;
use crate::chunk::*;
use crate::OpCode;
use fast_float;
use std::collections::HashMap;
use std::ptr::null;
use crate::debug::*;

struct Parser{
    current: Token,
    previous: Token,
    hadError: bool,
    panicMode: bool,
}

pub enum Precedence{
    PrecNone,
    PrecAssignment,
    PrecOr,
    PrecAnd,
    PrecEquality,
    PrecComparison,
    PrecTerm,
    PrecUnary,
    PrecCall,
    PrecPrimary
}

pub struct ParseFn{

}
pub struct ParseRule {
    prefix:ParseFn,
    infix:ParseFn,
    precedence:Precedence,
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
            hadError : false,
            panicMode : false
        }
    }
}

pub struct Compiler{
    parser : Parser,
    compilingChunk: Chunk,
    scanner: Scanner,
}

impl Compiler{
    pub fn new() -> Self{
        Compiler { 
            parser: Parser::new(), 
            scanner: Scanner::new(),
            compilingChunk: Chunk::new()
        }
    }

    pub fn compile(&mut self, source : String, chunk: Chunk) -> bool{
        self.scanner.source = source;
        self.compilingChunk = chunk;
        self.parser.hadError = false;
        self.parser.panicMode = false;
        self.advance();
        self.expression();
        self.consume(TokenType::TokenEof, &"Expect end of expression.".to_string());
        self.endCompiler();
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
    
    //  Similiar to advance but validiates that the token has the expected type.
    fn consume(&mut self, _type:TokenType, message:&String){
        if self.parser.current._type == _type{
            self.advance();
            return;
        }
        self.error_at_current(message);
    }

    fn emitByte(&self, byte:u8){
        self.compilingChunk.write_chunk_u8(byte,self.parser.previous.line)
    }

    fn emitBytes(&self, byte1:u8, byte2:u8){
        self.emitByte(byte1);
        self.emitByte(byte2);
    }

    fn emitReturn(&self){
        self.emitByte(OpCode::OpReturn.into());
    }

    fn makeConstant(&self, value:Value) -> u8{
        let constant:i32 = self.compilingChunk.add_constant(value).into();
        if constant > 255{
            self.error(&"Too many constants in one chunk.".to_string());
            return 0;
        }
        let result = constant as u8;
        return result;
    }

    fn emitConstant(&self, value:Value){
        self.emitBytes(OpCode::OpConstant.into(), self.makeConstant(value));
    }

    fn debug(&self, chunk:Chunk){
        if !self.parser.hadError{
            disassemble_chunk(&chunk, "code");
        }
    }

    fn endCompiler(&self){
        self.emitReturn();
        #[cfg(feature = "debug_print_code")]
            self.debug(self.compilingChunk);

    }
   fn parsePrecedence(&self, precedence:Precedence){
        self.advance();
        let prefixRule = self.getRule(self.parser.previous._type);
        if prefixRule.len() == 0 {
            self.error(&"Expect expression".to_string());
            return;
        }
        prefixRule();
        while precedence <= self.getRule(self.parser.current._type){
            self.advance();
            let infixRule:&Vec<*const i32> = self.getRule(self.parser.previous._type);
            return infixRule;
        }
   }

    fn binary(&self){
        let operatorType:TokenType = self.parser.previous._type;
        let rule: &ParseRule = self.getRule(operatorType);
        self.parsePrecedence(self.rule.precendence+1);
        match operatorType {
            TokenType::TokenPlus => self.emitByte(OpCode::OpAdd.into()),
            TokenType::TokenMinus => self.emitByte(OpCode::OpSubtract.into()),
            TokenType::TokenStar => self.emitByte(OpCode::OpMultiply.into()),
            TokenType::TokenSlash => self.emitByte(OpCode::OpDivide.into()),
            // break ?? 
            _=> return
        }

    }

    fn grouping(&self){
        self.expression();
        self.consume(TokenType::TokenRightParen, &"Expect ')' after expression.".to_string());
    }

    fn number(&self){
        // Convert string to float, using fast_float, alternative to strtod. 
        // Parses a 64 bit floating number
        let value:f64 = fast_float::parse(self.parser.previous.lexeme).unwrap();
        self.emitConstant(value);
    }
    fn unary(&self){
        let operatorType:TokenType = self.parser.previous._type;
        // Compile the operand
        self.parsePrecedence(Precedence::PrecUnary);

        self.expression();

        // Emit the operator instruction
        match operatorType {
            TokenType::TokenMinus => self.emitByte(OpCode::OpNegate.into()),
            _=> return
        }
    }

    pub fn parseRule(&self) ->&HashMap<TokenType, Vec<*const i32>>{
        // Parse rule using a hash map so we can index using TokenType, beginnings.
        let mut parse_rule:HashMap<TokenType, Vec<*const i32>>= HashMap::new();
        parse_rule.insert(TokenType::TokenLeftParen, vec![null(), null(), null()]);
        parse_rule.insert(TokenType::TokenLeftParen, vec![null(),null(),null()]);
        parse_rule.insert(TokenType::TokenRightParen, vec![null(),null(),null()]);
        parse_rule.insert(TokenType::TokenLeftBrace, vec![null(),null(),null()]);
        parse_rule.insert(TokenType::TokenLeftBrace, vec![null(),null(),null()]);
        parse_rule.insert(TokenType::TokenRightBrace, vec![null(),null(),null()]);
        parse_rule.insert(TokenType::TokenComma, vec![null(),null(),null()]);
        parse_rule.insert(TokenType::TokenDot, vec![null(),null(),null()]);
        parse_rule.insert(TokenType::TokenMinus, vec![null(),null(),null()]);
        parse_rule.insert(TokenType::TokenPlus, vec![null(), null(), null()]);
        parse_rule.insert(TokenType::TokenSemicolon, vec![null(), null(), null()]);
        parse_rule.insert(TokenType::TokenSlash, vec![null(), null(), null()]);
        parse_rule.insert(TokenType::TokenStar, vec![null(), null(), null()]);
        parse_rule.insert(TokenType::TokenBang, vec![null(), null(), null()]);
        parse_rule.insert(TokenType::TokenBangEquals, vec![null(), null(), null()]);
        parse_rule.insert(TokenType::TokenEqual, vec![null(), null(), null()]);
        parse_rule.insert(TokenType::TokenEqualEqual, vec![null(), null(), null()]);
        parse_rule.insert(TokenType::TokenGreater, vec![null(), null(), null()]);
        parse_rule.insert(TokenType::TokenGreaterEqual, vec![null(), null(), null()]);
        parse_rule.insert(TokenType::TokenLess, vec![null(), null(), null()]);
        parse_rule.insert(TokenType::TokenLessEqual, vec![null(), null(), null()]);
        parse_rule.insert(TokenType::TokenIdentifier, vec![null(), null(), null()]);
        parse_rule.insert(TokenType::TokenString, vec![null(), null(), null()]);
        parse_rule.insert(TokenType::TokenNumber, vec![null(), null(), null()]);
        parse_rule.insert(TokenType::TokenAnd, vec![null(), null(), null()]);
        parse_rule.insert(TokenType::TokenClass, vec![null(), null(), null()]);
        parse_rule.insert(TokenType::TokenElse, vec![null(), null(), null()]);
        parse_rule.insert(TokenType::TokenFalse, vec![null(), null(), null()]);
        parse_rule.insert(TokenType::TokenFor, vec![null(), null(), null()]);
        parse_rule.insert(TokenType::TokenFun, vec![null(), null(), null()]);
        parse_rule.insert(TokenType::TokenIf, vec![null(), null(), null()]);
        parse_rule.insert(TokenType::TokenNil, vec![null(), null(), null()]);
        parse_rule.insert(TokenType::TokenOr, vec![null(), null(), null()]);
        parse_rule.insert(TokenType::TokenPrint, vec![null(), null(), null()]);
        parse_rule.insert(TokenType::TokenReturn, vec![null(), null(), null()]);
        parse_rule.insert(TokenType::TokenSuper, vec![null(), null(), null()]);
        parse_rule.insert(TokenType::TokenThis, vec![null(), null(), null()]);
        parse_rule.insert(TokenType::TokenTrue, vec![null(), null(), null()]);
        parse_rule.insert(TokenType::TokenVar, vec![null(), null(), null()]);
        parse_rule.insert(TokenType::TokenWhile, vec![null(), null(), null()]);
        parse_rule.insert(TokenType::TokenError, vec![null(), null(), null()]);
        parse_rule.insert(TokenType::TokenEof, vec![null(), null(), null()]);
        return &parse_rule;
    }

    fn getRule(&self, _type:TokenType) -> &Vec<*const i32>{
        return &self.parseRule()[&_type];
    }

    fn expression(&self){
        self.parsePrecedence(Precedence::PrecAssignment);
    }

    // Show an error with the current token
    fn error_at_current(&mut self, message: &String){
        self.error_at(&self.parser.current, message);
    } 

    // Show an error with the previous token
    fn error(&mut self, message: &String){
        self.error_at(&self.parser.previous, message);
    }

    fn currentChunk(&self) -> &Chunk{
        return &self.compilingChunk
    }

    // Print error
    fn error_at(&mut self, token: &Token, message: &String){
        // Error so panic flag.
        if self.parser.panicMode {return;}
        self.parser.panicMode = true;
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


