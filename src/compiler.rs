use crate::value::Value;
use crate::scanner::*;
use crate::token_type::TokenType;
use crate::chunk::*;
use crate::debug::*;
use crate::precedence::*;

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
    prefix: Option<fn(&mut Compiler, bool)>,
    infix: Option<fn(&mut Compiler, bool)>,
    precedence: Precedence
}

#[derive(Clone,Debug)]
struct Local{
    name: Token,
    depth: Option<usize>
}

pub struct Compiler {
    parser: Parser,
    scanner: Scanner,
    pub compiling_chunk: Chunk,
    rules: Vec<ParseRule>,
    locals: Vec<Local>,
    scope_depth: usize
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
            prefix: Some(Compiler::grouping),
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
            prefix: Some(Compiler::unary),
            infix: Some(Compiler::binary),
            precedence: Precedence::PrecTerm
        };
        rules[TokenType::TokenPlus as usize] = ParseRule{
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: Precedence::PrecTerm
        };
        rules[TokenType::TokenSemicolon as usize] = ParseRule{
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone
        };
        rules[TokenType::TokenSlash as usize] = ParseRule{
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: Precedence::PrecFactor
        };
        rules[TokenType::TokenStar as usize] = ParseRule{
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: Precedence::PrecFactor
        };
        rules[TokenType::TokenBang as usize] = ParseRule{
            prefix: Some(Compiler::unary),
            infix: None,
            precedence: Precedence::PrecNone
        };
        rules[TokenType::TokenBangEqual as usize] = ParseRule{
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: Precedence::PrecEquality
        };
        rules[TokenType::TokenEqual as usize] = ParseRule{
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone
        };
        rules[TokenType::TokenEqualEqual as usize] = ParseRule{
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: Precedence::PrecEquality
        };
        rules[TokenType::TokenGreater as usize] = ParseRule{
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: Precedence::PrecComparison
        };
        rules[TokenType::TokenGreaterEqual as usize] = ParseRule{
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: Precedence::PrecComparison
        };
        rules[TokenType::TokenLess as usize] = ParseRule{
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: Precedence::PrecComparison
        };
        rules[TokenType::TokenLessEqual as usize] = ParseRule{
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: Precedence::PrecComparison
        };
        rules[TokenType::TokenIdentifier as usize] = ParseRule{
            prefix: Some(Compiler::variable),
            infix: None,
            precedence: Precedence::PrecNone
        };
        rules[TokenType::TokenString as usize] = ParseRule{
            prefix: Some(Compiler::string),
            infix: None,
            precedence: Precedence::PrecNone
        };
        rules[TokenType::TokenNumber as usize] = ParseRule{
            prefix: Some(Compiler::number),
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
            prefix: Some(Compiler::literal),
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
            prefix: Some(Compiler::literal),
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
            prefix: Some(Compiler::literal),
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
            locals: Vec::new(),
            scope_depth: 0,
        }
    }

    pub fn compile(&mut self, source: String, chunk: &Chunk) -> bool {
        self.scanner.source = source;
        self.compiling_chunk = chunk.clone();
        self.parser.had_error = false;
        self.parser.panic_mode = false;
        self.advance();

        while !self.matching(TokenType::TokenEOF) {
            self.declaration();
        }

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

    fn block(&mut self){
        while !self.check(TokenType::TokenRightBrace) && ! self.check(TokenType::TokenEOF){
            self.declaration();
        }
        self.consume(TokenType::TokenRightBrace, "Expect '}' after block.");
    }

    fn var_declaration(&mut self) {
        let global = self.parser_variable("Expect variable name.");
        
        if self.matching(TokenType::TokenEqual) {
            self.expression();
        } else {
            self.emit_byte_opcode(OpCode::OpNil);
        }
        self.consume(TokenType::TokenSemicolon, "Expect ';' after variable declaration.");
        
        self.define_variable(global);
    }

    fn expression_statement(&mut self) {
        self.expression();
        self.consume(TokenType::TokenSemicolon, "Expect ';' after value.");
        self.emit_byte_opcode(OpCode::OpPop);
    }

    fn print_statement(&mut self) {
        self.expression();
        self.consume(TokenType::TokenSemicolon, "Expect ';' after value.");
        self.emit_byte_opcode(OpCode::OpPrint);
    }

    // 21.1.3
    fn synchronize(&mut self) {
        self.parser.panic_mode = false;

        while self.parser.current._type != TokenType::TokenEOF {
            if self.parser.previous._type == TokenType::TokenSemicolon { return; }
            match self.parser.current._type {
                TokenType::TokenClass => (),
                TokenType::TokenFun => (),
                TokenType::TokenVar => (),
                TokenType::TokenFor => (),
                TokenType::TokenIf => (),
                TokenType::TokenWhile => (),
                TokenType::TokenPrint => (),
                TokenType::TokenReturn => return,
                _ => (),
            }
        }

        self.advance();
    }

    fn declaration(&mut self) {
        if self.matching(TokenType::TokenVar) {
            self.var_declaration();
        } else {
            self.statement();
        }
        if self.parser.panic_mode { self.synchronize(); }
    }

    fn statement(&mut self) {
        if self.matching(TokenType::TokenPrint) {
            self.print_statement()
        } else if self.matching(TokenType::TokenLeftBrace){
            self.begin_scope();
            self.block();
            self.end_scope();
        } else {
            self.expression_statement()
        }
    }
    
    //  Similiar to advance but validiates that the token has the expected type.
    fn consume(&mut self, _type: TokenType, message: &str) {
        if self.parser.current._type == _type {
            self.advance();
            return;
        }
        self.error_at_current(message);
    }

    fn check(&mut self, token_type: TokenType) -> bool {
        self.parser.current._type == token_type
    }

    fn matching(&mut self, token_type: TokenType) -> bool {
        if !self.check(token_type) { return false; }
        self.advance();

        true
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

    fn emit_bytes_opcode(&mut self, bytes1: OpCode, bytes2: OpCode) {
        self.emit_byte_opcode(bytes1);
        self.emit_byte_opcode(bytes2);
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

    fn begin_scope(&mut self){
        self.scope_depth += 1;
    }

    fn end_scope(&mut self){
        self.scope_depth -= 1;
        
        
        while self.locals.len() > 0 && self.locals.last().unwrap().depth.unwrap() > self.scope_depth{
            self.emit_byte_opcode(OpCode::OpPop);
            self.locals.pop();
        }
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();

        let prefix_rule = self.get_rule(self.parser.previous._type).prefix;
        let mut can_assign: bool = true;
        match prefix_rule {
            Some(rule) => {
                can_assign = precedence as u8 <= Precedence::PrecAssignment as u8;
                rule(self, can_assign);
            },
            None => {
                self.error("Expect expression.");
                return
            }
        }

        while precedence <= self.get_rule(self.parser.current._type).precedence {
            self.advance();
            let infix_rule = self.get_rule(self.parser.previous._type).infix.unwrap();
            infix_rule(self, can_assign);
        }

        if can_assign && self.matching(TokenType::TokenEqual) {
            self.error("Invalid assignment target.");
        }
    }

    fn identifier_constant(&mut self, name: Token) -> u8 {
        self.make_constant(Value::String(name.lexeme.clone()))
    }
    fn add_local(&mut self, name: Token) {
        if self.locals.len() > 256 {
            self.error("Too many local variables in function");
            return;
        }
        let local = Local {
            depth : None,
            name: name
        };
        self.locals.push(local)
    }

    fn identifier_equal(&mut self, a: &Token, b: &Token) -> bool {
        if a.lexeme.len() != b.lexeme.len() {
            return false;
        }
        return a.lexeme == b.lexeme;
    }

    fn resolve_local(&mut self, name: &Token) -> Option<usize> {
        for i in (0..self.locals.len()).rev() {
            let local = self.locals[i].clone();
            // println!("compiler.rs:resolve_local(): local = {:?}", local);
            if self.identifier_equal(name, &local.name) {
                if local.depth == None{
                    self.error("Can't read local variable in its own initializer");
                }
                return Some(i);
            }
        }
        return None;
    }

    fn declare_variable(&mut self) {
        // Global variables are implicitly declared
        if self.scope_depth == 0 {
            return
        }
        let name = self.parser.previous.clone();

        for i in (0..self.locals.len()).rev(){
            // Get the local at position i
            let local = &self.locals[i].clone();

            if local.depth != Some(usize::MAX) && local.depth.unwrap() < self.scope_depth {
                break;
            }

            if self.identifier_equal(&name, &local.name){
                self.error("Already varibale with name in this scope.");
            }
        }


        self.add_local(name);
    }

    fn parser_variable(&mut self, error_message: &str) -> u8 {
        self.consume(TokenType::TokenIdentifier, error_message);
        self.declare_variable();
        if self.scope_depth > 0 {
            return 0;
        }
        self.identifier_constant(self.parser.previous.clone())
    }

    fn mark_initialized(&mut self) {
        let local_count = self.locals.len() - 1;
        self.locals[local_count].depth = Some(self.scope_depth);
    }

    fn define_variable(&mut self, global: u8) {
        if self.scope_depth > 0 {
            self.mark_initialized();
            return;
        }
        self.emit_bytes_opcode_u8(OpCode::OpDefineGlobal, global);
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


    fn grouping(&mut self, can_assign: bool) {
        self.expression();
        self.consume(TokenType::TokenRightParen, "Expect ')' after expression.");
    }

    fn number(&mut self, can_assign: bool) {
        let _number:f64 = self.parser.previous.lexeme.parse().unwrap();
        self.emit_constant(Value::Number(_number));
    }

    fn string(&mut self, can_assign: bool) {
        let end_index = self.parser.previous.lexeme.len() - 1;
        let _string:String = self.parser.previous.lexeme.substring(1, end_index);
        self.emit_constant(Value::from(_string));
    }

    fn named_variable(&mut self, name: Token, can_assign: bool) {
        let (get_op, set_op): (OpCode, OpCode);
        let mut arg = self.resolve_local(&name);

        if arg != None {
            get_op = OpCode::OpGetLocal;
            set_op = OpCode::OpSetLocal;
        } else {
            arg = Some(self.identifier_constant(name) as usize);
            get_op = OpCode::OpGetGlobal;
            set_op = OpCode::OpSetGlobal;
        }
        if can_assign && self.matching(TokenType::TokenEqual) {
            self.expression();
            self.emit_bytes_opcode_u8(set_op, arg.unwrap() as u8);
        } else {
            self.emit_bytes_opcode_u8(get_op, arg.unwrap() as u8);
        }
    }


    fn variable(&mut self, can_assign: bool) {
        self.named_variable(self.parser.previous.clone(), can_assign);
    }

    fn unary(&mut self, can_assign: bool) {
        let operator_type = self.parser.previous._type.clone();

        // Compile the operand.
        self.parse_precedence(Precedence::PrecUnary);

        // Emit the operator instruction
        match operator_type {
            TokenType::TokenBang => self.emit_byte_opcode(OpCode::OpNot),
            TokenType::TokenMinus => self.emit_byte_opcode(OpCode::OpNegate),
            _ => return,
        }
    }

    fn binary(&mut self, can_assign: bool) {
        let operator_type = self.parser.previous._type.clone();
        let rule = self.get_rule(operator_type);
        self.parse_precedence(rule.precedence.next());

        match operator_type {
            TokenType::TokenBangEqual => self.emit_bytes_opcode(OpCode::OpEqual, OpCode::OpNot),
            TokenType::TokenEqualEqual => self.emit_byte_opcode(OpCode::OpEqual),
            TokenType::TokenGreater => self.emit_byte_opcode(OpCode::OpGreater),
            TokenType::TokenGreaterEqual => self.emit_bytes_opcode(OpCode::OpLess, OpCode::OpNot),
            TokenType::TokenLess => self.emit_byte_opcode(OpCode::OpLess),
            TokenType::TokenLessEqual => self.emit_bytes_opcode(OpCode::OpGreater, OpCode::OpNot),
            TokenType::TokenPlus => self.emit_byte_opcode(OpCode::OpAdd),
            TokenType::TokenMinus => self.emit_byte_opcode(OpCode::OpSubtract),
            TokenType::TokenStar => self.emit_byte_opcode(OpCode::OpMultiply),
            TokenType::TokenSlash => self.emit_byte_opcode(OpCode::OpDivide),
            _ => return // Unreachable
        }
    }

    fn literal(&mut self, can_assign: bool) {
        match self.parser.previous._type {
            TokenType::TokenFalse => self.emit_byte_opcode(OpCode::OpFalse),
            TokenType::TokenNil => self.emit_byte_opcode(OpCode::OpNil),
            TokenType::TokenTrue => self.emit_byte_opcode(OpCode::OpTrue),
            _ => return
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

trait StringUtils {
    // Trait and implementation for a method for String that returns
    // a substring, which begins at the specified begin_index and extends
    // to the character at index end_index - 1
    fn substring(&self, begin_index: usize, end_index: usize) -> Self;
    // Gets the character in a position
    fn char_at(&mut self, index_pos: usize) -> char;
}


impl StringUtils for String {
    fn substring(&self, begin_index: usize, end_index: usize) -> Self {
        if begin_index + (end_index - begin_index) > self.len() {
            panic!("substring(): index out of bounds");
        }
        self.chars().skip(begin_index).take(end_index - begin_index).collect()
    }

    fn char_at(&mut self, index_pos: usize) -> char {
        let curr_source_char : char =  match self.chars().nth(index_pos) {
            Some(x) => x,
            None => {
                println!("advance(): char not accessible by index. Returning empty space. Index: {}", index_pos);
                return ' '
            }
        };

        curr_source_char
    }
}