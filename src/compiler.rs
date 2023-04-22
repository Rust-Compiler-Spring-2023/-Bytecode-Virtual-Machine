use std::borrow::Borrow;
// A mutable memory location with dynamically checked borrow rules
use std::cell::RefCell;

use crate::value::*;
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

#[derive(PartialEq,Clone, Copy)]
enum FunctionType{
    TypeFunction,
    TypeScript,
}

struct CurrCompiler{
    function: RefCell<Function>,
    locals: RefCell<Vec<Local>>,
    fun_type: FunctionType,
    scope_depth: RefCell<usize>,
}

impl CurrCompiler{
    fn new(fun_type: FunctionType) -> Self{
        CurrCompiler {
            function: RefCell::new(Function::new()),
            locals: RefCell::new(Vec::new()),
            fun_type: fun_type,
            scope_depth: RefCell::new(0),
        }
    }

    fn set_local_scope(&self){
        let last = self.locals.borrow().len() - 1;
        let mut locals = self.locals.borrow_mut();
        locals[last].depth = Some(*self.scope_depth.borrow());
    }

    fn in_scope(&self) -> bool{
        *self.scope_depth.borrow() != 0
    }
}


pub struct Compiler {
    parser: Parser,
    scanner: Scanner,
    rules: Vec<ParseRule>,
    curr_compiler: RefCell<CurrCompiler>,
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
            precedence: Precedence::PrecCall
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
            infix: Some(Compiler::and_),
            precedence: Precedence::PrecAnd
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
            infix: Some(Compiler::or_),
            precedence: Precedence::PrecOr
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
            curr_compiler: RefCell::new(CurrCompiler::new(FunctionType::TypeScript)),
            rules: rules,
        }
    }

    pub fn compile(&mut self, source: String) -> Option<Function> {
        self.scanner.source = source;
        self.parser.had_error = false;
        self.parser.panic_mode = false;

        self.curr_compiler.borrow_mut().locals.borrow_mut().push(Local { name: Token { _type: TokenType::Undefined, lexeme: "".to_string(), line: 0 }, depth: Some(0) });

        self.advance();

        while !self.matching(TokenType::TokenEOF) {
            self.declaration();
        }

        let function : Function = self.end_compiler();
        
        if self.parser.had_error{
            return None;
        }

        Some(function)
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
        while !self.check(TokenType::TokenRightBrace) && !self.check(TokenType::TokenEOF){
            self.declaration();
        }
        self.consume(TokenType::TokenRightBrace, "Expect '}' after block.");
    }

    fn function(&mut self, _type: FunctionType){
        let fun_type = _type.clone();
        let _prev_compiler: CurrCompiler = self.curr_compiler.replace(CurrCompiler::new(_type));
        if fun_type != FunctionType::TypeScript{
            self.curr_compiler.borrow_mut().function.borrow_mut().name = Some(self.parser.previous.lexeme.clone());
        }

        self.begin_scope();

        self.consume(TokenType::TokenLeftParen, "Expect '(' after function name.");

        if !self.check(TokenType::TokenRightParen){
            loop{
                self.curr_compiler.borrow_mut().function.borrow_mut().arity += 1;
                if self.curr_compiler.borrow().function.borrow().arity > 255 {
                    self.error_at_current("Can't have more than 255 paramenters.");
                }
                let _constant = self.parse_variable("Expect parameter name.");
                self.define_variable(_constant);

                if !self.matching(TokenType::TokenComma) {break;}
            }
            
        }

        self.consume(TokenType::TokenRightParen, "Expect ')' after parameters.");
        self.consume(TokenType::TokenLeftBrace, "Expect '{' after function body.");
        self.block();

        let function = self.end_compiler();
    
        let _result = self.curr_compiler.replace(_prev_compiler);
        let _function = _result.function.replace(Function::new());

    

        let fun_constant = self.make_constant(Value::Fun(function));
        self.emit_bytes_opcode_u8(OpCode::OpConstant, fun_constant);
        
    }

    fn fun_declaration(&mut self){
        let global : u8 = self.parse_variable("Expect function name.");
        self.mark_initialized();
        self.function(FunctionType::TypeFunction);
        self.define_variable(global);
    }

    fn var_declaration(&mut self) {
        let global = self.parse_variable("Expect variable name.");
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

    fn for_statement(&mut self){
        self.begin_scope();
        
        // Initializer clause
        self.consume(TokenType::TokenLeftParen, "Expect '(' after 'for'.");
        if self.matching(TokenType::TokenSemicolon){
            // No initializer
        } else if self.matching(TokenType::TokenVar){
            self.var_declaration();
        } else {
            self.expression_statement();
        }

        let mut loop_start: usize = self.curr_compiler.borrow().function.borrow().chunk.lines.len();
        // chunk.lines.len();

        // Condition clause
        let mut exit_jump: Option<usize> = None;
        if !self.matching(TokenType::TokenSemicolon){
            self.expression();
            self.consume(TokenType::TokenSemicolon, "Expect ';' after loop condition.");

            // Jump out of the loop if the condition is false
            exit_jump = Some(self.emit_jump(OpCode::OpJumpIfFalse));
            self.emit_byte_opcode(OpCode::OpPop);
        }

        // Increment clause
        if !self.matching(TokenType::TokenRightParen){
            let body_jump: usize = self.emit_jump(OpCode::OpJump);
            let increment_start: usize = self.curr_compiler.borrow().function.borrow().chunk.lines.len();
            self.expression();
            self.emit_byte_opcode(OpCode::OpPop);
            self.consume(TokenType::TokenRightParen, "Expect ')' after for clauses.");

            self.emit_loop(loop_start);
            loop_start = increment_start;
            self.patch_jump(body_jump);

        }

        self.statement();
        self.emit_loop(loop_start);

        if let Some(exit) = exit_jump {
            self.patch_jump(exit);
            self.emit_byte_opcode(OpCode::OpPop);
        }

        self.end_scope();
    }

    fn if_statement(&mut self){
        self.consume(TokenType::TokenLeftParen, "Expect '(' after 'if'.");
        self.expression();
        self.consume(TokenType::TokenRightParen, "Expect ')' after condition");

        let then_jump : usize = self.emit_jump(OpCode::OpJumpIfFalse);
        self.emit_byte_opcode(OpCode::OpPop);
        self.statement();

        let else_jump : usize = self.emit_jump(OpCode::OpJump);

        self.patch_jump(then_jump);
        self.emit_byte_opcode(OpCode::OpPop);
        if self.matching(TokenType::TokenElse) {
            self.statement();
        }
        self.patch_jump(else_jump);
    }

    fn print_statement(&mut self) {
        self.expression();
        self.consume(TokenType::TokenSemicolon, "Expect ';' after value.");
        self.emit_byte_opcode(OpCode::OpPrint);
    }

    fn while_statement(&mut self){
        let loop_start = self.curr_compiler.borrow().function.borrow().chunk.lines.len();
        self.consume(TokenType::TokenLeftParen, "Expect '(' after 'while'.");
        self.expression();
        self.consume(TokenType::TokenRightParen, "Expect ')' after condition");
        
        let exit_jump: usize = self.emit_jump(OpCode::OpJumpIfFalse);
        self.emit_byte_opcode(OpCode::OpPop);
        self.statement();

        // Needs to know how far back to jump 
        self.emit_loop(loop_start);

        self.patch_jump(exit_jump);
        self.emit_byte_opcode(OpCode::OpPop);
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
        if self.matching(TokenType::TokenFun){
            self.fun_declaration();
        }else if self.matching(TokenType::TokenVar) {
            self.var_declaration();
        } else {
            self.statement();
        }
        if self.parser.panic_mode { self.synchronize(); }
    }

    fn statement(&mut self) {
        if self.matching(TokenType::TokenPrint) {
            self.print_statement()
        } else if self.matching(TokenType::TokenFor) {
            self.for_statement();
        } else if self.matching(TokenType::TokenIf){
            self.if_statement();
        } else if self.matching(TokenType::TokenWhile) {
            self.while_statement();
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
        self.curr_compiler.borrow_mut().function.borrow_mut().chunk.write_chunk_opcode(op_code, self.parser.previous.line);
    }

    fn emit_byte_u8(&mut self, byte: u8) {
        self.curr_compiler.borrow_mut().function.borrow_mut().chunk.write_chunk_u8(byte, self.parser.previous.line);
    }

    fn emit_bytes_opcode_u8(&mut self, bytes1: OpCode, bytes2: u8) {
        self.emit_byte_opcode(bytes1);
        self.emit_byte_u8(bytes2);
    }

    fn emit_bytes_opcode(&mut self, bytes1: OpCode, bytes2: OpCode) {
        self.emit_byte_opcode(bytes1);
        self.emit_byte_opcode(bytes2);
    }

    fn emit_loop(&mut self, loop_start: usize){
        self.emit_byte_opcode(OpCode::OpLoop);

        let offset = self.curr_compiler.borrow().function.borrow().chunk.lines.len() - loop_start + 2;
        if offset > u16::MAX.into() {
            self.error("Loop body too large.");
        }

        self.emit_byte_u8(((offset>> 8) & 0xff) as u8);
        self.emit_byte_u8((offset & 0xff) as u8);
    }

    fn emit_jump(&mut self, instruction: OpCode) -> usize{
        self.emit_byte_opcode(instruction);
        // We use two bytes for the jump offset operand. 
        // A 16-bit offset lets us jump over up to 65,535 bytes of code, which should be plenty for our needs. 
        self.emit_byte_u8(0xff); // 0xff a byte of 255
        self.emit_byte_u8(0xff); // 0xff a byte of 255

        return self.curr_compiler.borrow().function.borrow().chunk.lines.len() - 2; 
    }
    
    fn debug_print_code(&mut self) {
        if !self.parser.had_error {

            let fun_name: String;
            
            match &self.curr_compiler.borrow().function.borrow().name {
                Some(name) => fun_name = name.to_string(),
                None => fun_name = "<script>".to_string()
            }
            //println!("function(): bytecode = {:?}", self.curr_compiler.borrow().function.borrow().chunk.code);
            #[cfg(feature = "debug_print_code")]
            disassemble_chunk(&self.curr_compiler.borrow().function.borrow().chunk, &fun_name);
        }
    }

    fn end_compiler(&mut self) -> Function {
        self.emit_return();

        let function : Function = self.curr_compiler.borrow().function.borrow().clone();

        #[cfg(feature="debug_print_code")]
        self.debug_print_code();

        function
    }

    fn begin_scope(&mut self){
        *self.curr_compiler.borrow_mut().scope_depth.borrow_mut() += 1;
    }

    fn end_scope(&mut self){
        *self.curr_compiler.borrow_mut().scope_depth.borrow_mut() -= 1;
        let scope_depth = *self.curr_compiler.borrow_mut().scope_depth.borrow();
        let depth = self.curr_compiler.borrow_mut().locals.borrow().len();
        while depth > 0 && self.curr_compiler.borrow_mut().locals.borrow().last().unwrap().depth.unwrap() > scope_depth{
            self.emit_byte_opcode(OpCode::OpPop);
            self.curr_compiler.borrow_mut().locals.borrow_mut().pop();
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
        if self.curr_compiler.borrow_mut().locals.borrow().len() > 256 {
            self.error("Too many local variables in function");
            return;
        }
        let local = Local {
            depth : None,
            name: name
        };
        self.curr_compiler.borrow_mut().locals.borrow_mut().push(local)
    }

    fn identifier_equal(&mut self, a: &Token, b: &Token) -> bool {
        if a.lexeme.len() != b.lexeme.len() {
            return false;
        }
        return a.lexeme == b.lexeme;
    }

    fn resolve_local(&mut self, name: &Token) -> Option<usize> {
        let length = self.curr_compiler.borrow_mut().locals.borrow().len();
        for i in (0..length).rev() {
            let local = self.curr_compiler.borrow_mut().locals.borrow()[i].clone();
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
        if !self.curr_compiler.borrow().in_scope() {
            return
        }
        let name = self.parser.previous.clone();
        let length = self.curr_compiler.borrow_mut().locals.borrow().len();
        for i in (0.. length).rev(){
            // Get the local at position i
            let local = &self.curr_compiler.borrow_mut().locals.borrow()[i].clone();

            if local.depth != None && local.depth.unwrap() < *self.curr_compiler.borrow_mut().scope_depth.borrow() {
                break;
            }

            if self.identifier_equal(&name, &local.name){
                self.error("Already variable with name in this scope.");
            }
        }

        self.add_local(name);
    }

    fn parse_variable(&mut self, error_message: &str) -> u8 {
        self.consume(TokenType::TokenIdentifier, error_message);
        self.declare_variable();
        if !self.curr_compiler.borrow().in_scope() {
            self.identifier_constant(self.parser.previous.clone())
        } else{
            0
        } 
    }

    fn mark_initialized(&mut self) {
        if *self.curr_compiler.borrow().scope_depth.borrow() != 0 {
            self.curr_compiler.borrow().set_local_scope();
        }
    }

    fn define_variable(&mut self, global: u8) {
        if !self.curr_compiler.borrow().in_scope() {
            self.emit_bytes_opcode_u8(OpCode::OpDefineGlobal, global);
        } else {
            self.mark_initialized();
        }
    }

    fn and_(&mut self, can_assign: bool){
        let end_jump: usize = self.emit_jump(OpCode::OpJumpIfFalse);

        self.emit_byte_opcode(OpCode::OpPop);
        self.parse_precedence(Precedence::PrecAnd);

        self.patch_jump(end_jump);
    }

    fn emit_return(&mut self) {
        self.emit_byte_opcode(OpCode::OpReturn);
    }

    fn make_constant(&mut self, value: Value) -> u8 {
        let constant: u8 = self.curr_compiler.borrow_mut().function.borrow_mut().chunk.add_constant(value);
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

    fn patch_jump(&mut self, offset: usize){
        // -2 to adjust for the bytecode for the jump offset itself
        let jump : usize = self.curr_compiler.borrow().function.borrow().chunk.lines.len() - offset - 2;
        
        if jump > u16::MAX.into() {
            self.error("Too much code to jump over.");
        }

        self.curr_compiler.borrow_mut().function.borrow_mut().chunk.code[offset] = ((jump >> 8) & 0xff) as u8;
        self.curr_compiler.borrow_mut().function.borrow_mut().chunk.code[offset+1] = (jump & 0xff) as u8;
    }


    fn grouping(&mut self, can_assign: bool) {
        self.expression();
        self.consume(TokenType::TokenRightParen, "Expect ')' after expression.");
    }

    fn number(&mut self, can_assign: bool) {
        let _number:f64 = self.parser.previous.lexeme.parse().unwrap();
        self.emit_constant(Value::Number(_number));
    }

    fn or_(&mut self, can_assign: bool){
        let else_jump = self.emit_jump(OpCode::OpJumpIfFalse);
        let end_jump = self.emit_jump(OpCode::OpJump);

        self.patch_jump(else_jump);
        self.emit_byte_opcode(OpCode::OpPop);

        self.parse_precedence(Precedence::PrecOr);
        self.patch_jump(end_jump);
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