use crate::value::Value;
use crate::scanner::*;
use crate::token_type::TokenType;
use crate::chunk::*;
use crate::debug::*;
use crate::precedence::*;
use crate::value::number_val;

#[derive(Clone)]
struct Parser {
    current: Token,
    previous: Token,
    had_error: bool,
    panic_mode: bool,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            current: Token {
                _type : TokenType::Undefined,
                lexeme: String::new(),
                line: 0,
            },
            previous : Token {
                _type : TokenType::Undefined,
                lexeme: String::new(),
                line: 0
            },
            had_error : false,
            panic_mode: false
        }
    }
}

#[derive(Clone, Copy)]
struct ParseRule {
    prefix: Option<fn(&mut Compiler)>,
    infix: Option<fn(&mut Compiler)>,
    precedence: Precedence
}

pub struct Compiler {
    parser: Parser,
    scanner: Scanner,
    pub compiling_chunk: Chunk,
    rules: Vec<ParseRule>,
}

impl Compiler {
    pub fn new() -> Self{
        let mut rules = vec![
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::PrecNone
            };
            TokenType::Undefined as usize
        ];

        rules[TokenType::TokenLeftParen as usize] = ParseRule{
            // Uses closure to create an object of the struct method
            prefix: Some(|fun| fun.grouping()),
            infix: None,
            precedence: Precedence::PrecNone
        };
        rules[TokenType::TokenRightParen as usize] = ParseRule{
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone
        };
        rules[TokenType::TokenLeftBrace as usize] = ParseRule{
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone
        };
        rules[TokenType::TokenRightBrace as usize] = ParseRule{
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone
        };
        rules[TokenType::TokenComma as usize] = ParseRule{
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone
        };
        rules[TokenType::TokenDot as usize] = ParseRule{
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone
        };
        rules[TokenType::TokenMinus as usize] = ParseRule{
            prefix: Some(|fun| fun.unary()),
            infix: Some(|fun| fun.binary()),
            precedence: Precedence::PrecTerm
        };
        rules[TokenType::TokenPlus as usize] = ParseRule{
            prefix: None,
            infix: Some(|fun| fun.binary()),
            precedence: Precedence::PrecTerm
        };
        rules[TokenType::TokenSemicolon as usize] = ParseRule{
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone
        };
        rules[TokenType::TokenSlash as usize] = ParseRule{
            prefix: None,
            infix: Some(|fun| fun.binary()),
            precedence: Precedence::PrecFactor
        };
        rules[TokenType::TokenStar as usize] = ParseRule{
            prefix: None,
            infix: Some(|fun| fun.binary()),
            precedence: Precedence::PrecFactor
        };
        rules[TokenType::TokenBang as usize] = ParseRule{
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone
        };
        rules[TokenType::TokenBangEquals as usize] = ParseRule{
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone
        };
        rules[TokenType::TokenEqual as usize] = ParseRule{
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone
        };
        rules[TokenType::TokenEqualEqual as usize] = ParseRule{
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone
        };
        rules[TokenType::TokenGreater as usize] = ParseRule{
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone
        };
        rules[TokenType::TokenGreaterEqual as usize] = ParseRule{
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone
        };
        rules[TokenType::TokenLess as usize] = ParseRule{
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone
        };
        rules[TokenType::TokenLessEqual as usize] = ParseRule{
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone
        };
        rules[TokenType::TokenIdentifier as usize] = ParseRule{
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone
        };
        rules[TokenType::TokenString as usize] = ParseRule{
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone
        };
        rules[TokenType::TokenNumber as usize] = ParseRule{
            prefix: Some(|fun| fun.number()),
            infix: None,
            precedence: Precedence::PrecNone
        };
        rules[TokenType::TokenAnd as usize] = ParseRule{
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone
        };
        rules[TokenType::TokenClass as usize] = ParseRule{
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone
        };
        rules[TokenType::TokenElse as usize] = ParseRule{
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone
        };
        rules[TokenType::TokenFalse as usize] = ParseRule{
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone
        };
        rules[TokenType::TokenFor as usize] = ParseRule{
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone
        };
        rules[TokenType::TokenFun as usize] = ParseRule{
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone
        };
        rules[TokenType::TokenIf as usize] = ParseRule{
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone
        };
        rules[TokenType::TokenNil as usize] = ParseRule{
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone
        };
        rules[TokenType::TokenOr as usize] = ParseRule{
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone
        };
        rules[TokenType::TokenPrint as usize] = ParseRule{
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone
        };
        rules[TokenType::TokenReturn as usize] = ParseRule{
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone
        };
        rules[TokenType::TokenSuper as usize] = ParseRule{
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone
        };
        rules[TokenType::TokenThis as usize] = ParseRule{
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone
        };
        rules[TokenType::TokenTrue as usize] = ParseRule{
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone
        };
        rules[TokenType::TokenVar as usize] = ParseRule{
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone
        };
        rules[TokenType::TokenWhile as usize] = ParseRule{
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone
        };
        rules[TokenType::TokenError as usize] = ParseRule{
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone
        };
        rules[TokenType::TokenEOF as usize] = ParseRule{
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone
        };

        Compiler { 
            parser: Parser::new(), 
            scanner: Scanner::new(),
            compiling_chunk: Chunk::new(),
            rules: rules,
        }
    }

    pub fn compile(&mut self, source: String, chunk: &Chunk) -> bool {
        self.scanner.source = source;
        self.compiling_chunk = chunk.clone();
        self.parser.had_error = false;
        self.parser.panic_mode = false;
        self.advance();
        self.expression();
        self.consume(TokenType::TokenEOF, "Expect end of expression.");
        self.end_compiler();
        
        !self.parser.had_error
    }

    fn advance(&mut self) {
        self.parser.previous = self.parser.current.clone();
        loop {
            self.parser.current = self.scanner.scan_token();
            if self.parser.current._type != TokenType::TokenError { break; }

            self.error_at_current(&self.parser.current.lexeme.clone());
        }
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::PrecAssignment)
    }
    
    //  Similiar to advance but validiates that the token has the expected type.
    fn consume(&mut self, _type: TokenType, message: &str) {
        if self.parser.current._type == _type {
            self.advance();
            return;
        }
        self.error_at_current(message);
    }

    fn emit_byte_opcode(&mut self, op_code: OpCode) {
        self.compiling_chunk.write_chunk_opcode(op_code, self.parser.previous.line);
    }

    fn emit_byte_u8(&mut self, byte: u8) {
        self.compiling_chunk.write_chunk_u8(byte, self.parser.previous.line);
    }

    fn emit_bytes_opcode_u8(&mut self, bytes1: OpCode, bytes2: u8) {
        self.emit_byte_opcode(bytes1);
        self.emit_byte_u8(bytes2);
    }

    fn debug_print_code(&mut self) {
        if !self.parser.had_error {
            disassemble_chunk(&self.compiling_chunk, "code");
        }
    }

    fn end_compiler(&mut self) {
        self.emit_return();

        #[cfg(feature="debug_print_code")]
        self.debug_print_code();
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();

        let prefix_rule = self.get_rule(self.parser.previous._type).prefix;
        match prefix_rule {
            Some(rule) => rule(self),
            None => {
                self.error("Expect expression.");
                return
            }
        }

        while precedence <= self.get_rule(self.parser.current._type).precedence {
            self.advance();
            let infix_rule = self.get_rule(self.parser.previous._type).infix.unwrap();
            infix_rule(self);
        }
    }

    fn emit_return(&mut self) {
        self.emit_byte_opcode(OpCode::OpReturn);
    }

    fn make_constant(&mut self, value: Value) -> u8 {
        let constant: u8 = self.compiling_chunk.add_constant(value);
        if constant > u8::MAX {
            self.error("Too many constants in one chunk.");
            return 0;
        }

        constant
    }

    fn emit_constant(&mut self, value: Value) {
        let constant = self.make_constant(value);
        self.emit_bytes_opcode_u8(OpCode::OpConstant, constant);
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(TokenType::TokenRightParen, "Expect ')' after expression.");
    }

    fn number(&mut self) {
        let value:f64 = self.parser.previous.lexeme.parse().unwrap();
        self.emit_constant(number_val(value));
    }

    fn unary(&mut self) {
        let operator_type = self.parser.previous._type.clone();

        // Compile the operand.
        self.parse_precedence(Precedence::PrecUnary);

        // Emit the operator instruction
        match operator_type {
            TokenType::TokenMinus => self.emit_byte_opcode(OpCode::OpNegate),
            _ => return,
        }
    }

    fn binary(&mut self) {
        let operator_type = self.parser.previous._type.clone();
        let rule = self.get_rule(operator_type);
        self.parse_precedence(rule.precedence.next());

        match operator_type {
            TokenType::TokenPlus => self.emit_byte_opcode(OpCode::OpAdd),
            TokenType::TokenMinus => self.emit_byte_opcode(OpCode::OpSubtract),
            TokenType::TokenStar => self.emit_byte_opcode(OpCode::OpMultiply),
            TokenType::TokenSlash => self.emit_byte_opcode(OpCode::OpDivide),
            _ => return // Unreachable
        }
    }

    fn get_rule(&mut self, _type: TokenType) -> ParseRule {
        self.rules[_type as usize]
    }

    // Show an error with the current token
    fn error_at_current(&mut self, message: &str) {
        self.error_at(&self.parser.current.clone(), message);
    } 

    // Show an error with the previous token
    fn error(&mut self, message: &str) {
        self.error_at(&self.parser.previous.clone(), message);
    }

    // Print error
    fn error_at(&mut self, token: &Token, message: &str) {
        if self.parser.panic_mode { return; }
        self.parser.panic_mode = true;
        eprint!("[line {}] Error", token.line);

        if token._type == TokenType::TokenEOF {
            eprint!(" at end ");
        } else if token._type == TokenType::TokenError {
            // Nothing.
        } else {
            eprint!(" at '{}' ", token.lexeme);
        }

        eprintln!(": {}", message);
        self.parser.had_error = true;
    }
}