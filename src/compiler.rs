// A mutable memory location with dynamically checked borrow rules
use std::cell::RefCell;

use crate::value::*;
use crate::scanner::*;
use crate::token_type::TokenType;
use crate::chunk::*;
use crate::precedence::*;
use crate::debug::*;

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

/*
    Using closures, pass function as object and execute function when needing specific parsing rules.
*/
#[derive(Clone, Copy)]
struct ParseRule {
    prefix: Option<fn(&mut Compiler, bool)>,
    infix: Option<fn(&mut Compiler, bool)>,
    precedence: Precedence
}

/*
    Locals represent the local variables
*/
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

/*
    Holds the necessary fields a compiler needs 
*/
struct CurrCompiler{
    function: RefCell<Function>,
    locals: RefCell<Vec<Local>>,
    fun_type: FunctionType,
    scope_depth: RefCell<usize>,
}

impl CurrCompiler{
    fn new(fun_type: FunctionType) -> Self{
        CurrCompiler {
            function: RefCell::new(Function::new(0, Chunk::new(), None)),
            locals: RefCell::new(Vec::new()),
            fun_type: fun_type,
            scope_depth: RefCell::new(0),
        }
    }
}


pub struct Compiler {
    parser: Parser,
    scanner: Scanner,
    rules: Vec<ParseRule>,
    // CITE: Learned to use RefCell by UncleScientist lox-bytecode repo in Github
    // CITE: https://github.com/UncleScientist/lox-bytecode
    curr_compiler: RefCell<CurrCompiler>,
}

impl Compiler {
    pub fn new() -> Self{
        /* Create all the parser rules */
        // CITE: https://github.com/UncleScientist/lox-bytecode
        // CITE: To be able to write rules and pass function as varibales
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
            infix: Some(Compiler::call),
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
        rules[TokenType::TokenMinusEqual as usize] = ParseRule{
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: Precedence::PrecNone
        };
        rules[TokenType::TokenPlus as usize] = ParseRule{
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: Precedence::PrecTerm
        };
        rules[TokenType::TokenPlusEqual as usize] = ParseRule{
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: Precedence::PrecNone
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
        rules[TokenType::TokenSlashEqual as usize] = ParseRule{
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: Precedence::PrecNone
        };
        rules[TokenType::TokenStar as usize] = ParseRule{
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: Precedence::PrecFactor
        };
        rules[TokenType::TokenStarEqual as usize] = ParseRule{
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: Precedence::PrecNone
        };
        rules[TokenType::TokenCarat as usize] = ParseRule{
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: Precedence::PrecFactor
        };
        rules[TokenType::TokenCaratEqual as usize] = ParseRule{
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: Precedence::PrecNone
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

    /**
     * Entry point of the compiler
    */
    pub fn compile(&mut self, source: String) -> Option<Function> {
        self.scanner.source = source;
        self.parser.had_error = false;
        self.parser.panic_mode = false;

        /* the compiler implicitly claims stack slot zero for the VM’s own internal use */
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

    /* 
    Scanner looks at the current character and makes a token.
    Token is stored in the parser as the "current" field 
    */
    fn advance(&mut self) {
        self.parser.previous = self.parser.current.clone();
        loop {
            self.parser.current = self.scanner.scan_token();
            if self.parser.current._type != TokenType::TokenError { break; }

            self.error_at_current(&self.parser.current.lexeme.clone());
        }
    }

    /*
    Parse the lowest precedence level to absorb all of the higher level ones too  
    */
    fn expression(&mut self) {
        self.parse_precedence(Precedence::PrecAssignment)
    }

    // Helper function to compile the rest of the block
    fn block(&mut self){
        while !self.check(TokenType::TokenRightBrace) && !self.check(TokenType::TokenEOF){
            self.declaration();
        }
        self.consume(TokenType::TokenRightBrace, "Expect '}' after block.");
    }

    // Create and execute a function declaration
    fn function(&mut self, _type: FunctionType){
        let fun_type = _type.clone();
        let _prev_compiler: CurrCompiler = self.curr_compiler.replace(CurrCompiler::new(_type));

        // If function is not the main "script" function, assign name to that function using previous lexeme
        if fun_type != FunctionType::TypeScript{
            self.curr_compiler.borrow_mut().function.borrow_mut().name = Some(self.parser.previous.lexeme.clone());
        }

        self.begin_scope();

        self.consume(TokenType::TokenLeftParen, "Expect '(' after function name.");

        if !self.check(TokenType::TokenRightParen){
            loop{
                // Increase the amount of parameters
                self.curr_compiler.borrow_mut().function.borrow_mut().arity += 1;
                // Function can't have more than 255 parameters
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

        self.end_compiler();
        
        // Get the previous compiler
        let _result = self.curr_compiler.replace(_prev_compiler);

        let arity = self.curr_compiler.borrow().function.borrow().arity;
        let chunk = self.curr_compiler.borrow().function.borrow().chunk.clone();
        let name = self.curr_compiler.borrow().function.borrow().name.clone();
        let _function = _result.function.replace(Function::new(arity, chunk, name));

    

        let fun_constant = self.make_constant(Value::Fun(_function));
        self.emit_bytes(OpCode::OpConstant as u8, fun_constant);
        
    }

    // Creates a function declaration
    fn fun_declaration(&mut self){
        let global : u8 = self.parse_variable("Expect function name.");
        self.mark_initialized();
        self.function(FunctionType::TypeFunction);
        self.define_variable(global);
    }

    // Creates a variable declaration
    fn var_declaration(&mut self) {
        let global = self.parse_variable("Expect variable name.");
        if self.matching(TokenType::TokenEqual) {
            self.expression();
        } else {
            self.emit_byte(OpCode::OpNil as u8);
        }
        self.consume(TokenType::TokenSemicolon, "Expect ';' after variable declaration.");
        
        self.define_variable(global);
    }

    // Checks that an expression is followed by a semicolon
    fn expression_statement(&mut self) {
        self.expression();
        self.consume(TokenType::TokenSemicolon, "Expect ';' after value.");
        self.emit_byte(OpCode::OpPop as u8);
    }

    // Creates for statement declaration
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

        // Condition clause
        let mut exit_jump: Option<usize> = None;
        if !self.matching(TokenType::TokenSemicolon){
            self.expression();
            self.consume(TokenType::TokenSemicolon, "Expect ';' after loop condition.");

            // Jump out of the loop if the condition is false
            exit_jump = Some(self.emit_jump(OpCode::OpJumpIfFalse as u8));
            self.emit_byte(OpCode::OpPop as u8);
        }

        // Increment clause
        if !self.matching(TokenType::TokenRightParen){
            let body_jump: usize = self.emit_jump(OpCode::OpJump as u8);
            let increment_start: usize = self.curr_compiler.borrow().function.borrow().chunk.lines.len();
            self.expression();
            self.emit_byte(OpCode::OpPop as u8);
            self.consume(TokenType::TokenRightParen, "Expect ')' after for clauses.");

            self.emit_loop(loop_start);
            loop_start = increment_start;
            self.patch_jump(body_jump);

        }

        self.statement();
        self.emit_loop(loop_start);

        if let Some(exit) = exit_jump {
            self.patch_jump(exit);
            self.emit_byte(OpCode::OpPop as u8);
        }

        self.end_scope();
    }

    // Creates if statement declaration
    fn if_statement(&mut self){
        // Compile if statement
        self.consume(TokenType::TokenLeftParen, "Expect '(' after 'if'.");
        self.expression();
        self.consume(TokenType::TokenRightParen, "Expect ')' after condition");

        // Get offset if if statement is false
        let then_jump : usize = self.emit_jump(OpCode::OpJumpIfFalse as u8);
        self.emit_byte(OpCode::OpPop as u8);
        // Compile then section
        self.statement();

        // Get offset if both if and then are false 
        let else_jump : usize = self.emit_jump(OpCode::OpJump as u8);

        self.patch_jump(then_jump);
        self.emit_byte(OpCode::OpPop as u8);
        if self.matching(TokenType::TokenElse) {
            self.statement();
        }
        self.patch_jump(else_jump);
    }

    // Creates print statement declaration
    fn print_statement(&mut self) {
        self.expression();
        self.consume(TokenType::TokenSemicolon, "Expect ';' after value.");
        self.emit_byte(OpCode::OpPrint as u8);
    }

    // Creates return statement declaration
    fn return_statement(&mut self){
        if self.curr_compiler.borrow().fun_type == FunctionType::TypeScript{
            self.error("Can't return from top-level code.");
        }

        // Return Nil implicitly if no expression is given with an OpReturn instruction
        if self.matching(TokenType::TokenSemicolon){
            self.emit_return();
        } else { // Otherwise, compile the return value expression and return it with an OpInstruction
            self.expression();
            self.consume(TokenType::TokenSemicolon, "Expect ';' after return value.");
            self.emit_byte(OpCode::OpReturn as u8);
        }
    }

    // Creates while loop statement declaration
    fn while_statement(&mut self){
        // Get position where loop starts
        let loop_start = self.curr_compiler.borrow().function.borrow().chunk.lines.len();
        self.consume(TokenType::TokenLeftParen, "Expect '(' after 'while'.");
        self.expression();
        self.consume(TokenType::TokenRightParen, "Expect ')' after condition");
        
        // Jump if condition is false
        let exit_jump: usize = self.emit_jump(OpCode::OpJumpIfFalse as u8);
        self.emit_byte(OpCode::OpPop as u8);
        self.statement();

        // Needs to know how far back to jump 
        self.emit_loop(loop_start);

        self.patch_jump(exit_jump);
        self.emit_byte(OpCode::OpPop as u8);
    }

    // Skip tokens indiscriminately until we reach something that looks like a statement boundary
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

    /* Determine what kind of declaration it is */
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

    /* Determine what kind of statement it is */
    fn statement(&mut self) {
        if self.matching(TokenType::TokenPrint) {
            self.print_statement()
        } else if self.matching(TokenType::TokenFor) {
            self.for_statement();
        } else if self.matching(TokenType::TokenIf){
            self.if_statement();
        } else if self.matching(TokenType::TokenReturn){
            self.return_statement();
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
    
    //  Makes a token but first validiates that the token has the expected type.
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

    // Adds a byte into the chunk of the current compiler
    fn emit_byte(&mut self, byte: u8) {
        self.curr_compiler.borrow_mut().function.borrow_mut().chunk.write_chunk(byte, self.parser.previous.line);
    }

    // Adds two bytes into the chunk of the current compiler
    fn emit_bytes(&mut self, bytes1: u8, bytes2: u8) {
        self.emit_byte(bytes1);
        self.emit_byte(bytes2);
    }

    // Emits a new loop instruction, which unconditionally jumps backwards by a given offset.
    fn emit_loop(&mut self, loop_start: usize){
        self.emit_byte(OpCode::OpLoop as u8);

        let offset = self.curr_compiler.borrow().function.borrow().chunk.lines.len() - loop_start + 2;
        if offset > u16::MAX.into() {
            self.error("Loop body too large.");
        }

        self.emit_byte(((offset>> 8) & 0xff) as u8);
        self.emit_byte((offset & 0xff) as u8);
    }

    /*
    The first emits a bytecode instruction and writes a placeholder operand for the jump offset. 
    We pass in the opcode as an argument because later we’ll have two different instructions that use this helper.
    */
    fn emit_jump(&mut self, instruction: u8) -> usize{
        self.emit_byte(instruction);
        // We use two bytes for the jump offset operand. 
        // A 16-bit offset lets us jump over up to 65,535 bytes of code, which should be plenty for our needs. 
        self.emit_byte(0xff); // 0xff a byte of 255
        self.emit_byte(0xff); // 0xff a byte of 255

        return self.curr_compiler.borrow().function.borrow().chunk.lines.len() - 2; 
    }
    
    // Activate debug_print_code feature to print a chunk log for debugging
    #[allow(unused)]
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

    // End current compiler, return the function that it held (which includes the chunk)
    fn end_compiler(&mut self) -> Function {
        self.emit_return();
    
        let function : Function = self.curr_compiler.borrow().function.borrow().clone();
        
        #[cfg(feature="debug_print_code")]
        self.debug_print_code();

        function
    }

    // Since new scope is entered, increase scope depth by 1
    fn begin_scope(&mut self){
        *self.curr_compiler.borrow_mut().scope_depth.borrow_mut() += 1;
    }

    /* 
    Since scope is finished, decrease scope depth by 1
    */
    fn end_scope(&mut self){
        *self.curr_compiler.borrow_mut().scope_depth.borrow_mut() -= 1;
        let scope_depth = *self.curr_compiler.borrow_mut().scope_depth.borrow();
        let depth = self.curr_compiler.borrow_mut().locals.borrow().len();
        // Pop any local variables declared at the scope depth we just left
        while depth > 0 && self.curr_compiler.borrow_mut().locals.borrow().last().unwrap().depth.unwrap() > scope_depth{
            self.emit_byte(OpCode::OpPop as u8);
            self.curr_compiler.borrow_mut().locals.borrow_mut().pop();
        }
    }

    /*
    Starts at the current token and parses any expression at the given precedence level or higher
    */
    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();

        let prefix_rule = self.get_rule(self.parser.previous._type).prefix;
        
        match prefix_rule {
            Some(rule) => {
                /*
                Since assignment is the lowest-precedence expression, 
                the only time we allow an assignment is when parsing an assignment expression 
                or top-level expression like in an expression statement.
                */
                let _can_assign = precedence as u8 <= Precedence::PrecAssignment as u8;
                rule(self, _can_assign);
            },
            None => {
                self.error("Expect expression.");
                return
            }
        }
        let _can_assign: bool = true;

        /*
        Keep checking until token has too low precedence. 
        */
        while precedence <= self.get_rule(self.parser.current._type).precedence {
            self.advance();
            let infix_rule = self.get_rule(self.parser.previous._type).infix.unwrap();
            // consume the operator and hand off control to the infix parser we found
            infix_rule(self, _can_assign);
        }

        if _can_assign && self.matching(TokenType::TokenEqual) {
            self.error("Invalid assignment target.");
        }
    }


    /*
    Takes the given token and adds its lexeme to the chunk’s constant vector as a string.
    It then returns the index of that constant in the constant vector.
    */
    fn identifier_constant(&mut self, name: Token) -> u8 {
        self.make_constant(Value::String(name.lexeme.clone()))
    }

    // Adds a local varibale to the locals vector
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

    // Checks if two tokens are equal
    fn identifier_equal(&mut self, a: &Token, b: &Token) -> bool {
        if a.lexeme.len() != b.lexeme.len() {
            return false;
        }
        return a.lexeme == b.lexeme;
    }

    /**
     * Walk the list of locals that are currently in scope. 
     * If one has the same name as the identifier token, the identifier must refer to that variable.
     * Ensures that inner local variables correctly shadow locals with the same name in surrounding scopes.
     */
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

    /*
    Declares local variable, if not previously created
    */
    fn declare_variable(&mut self) {
        // Global variables are implicitly declared
        if *self.curr_compiler.borrow().scope_depth.borrow() == 0 {
            return
        }
        let name = self.parser.previous.clone();
        let length = self.curr_compiler.borrow_mut().locals.borrow().len();
        // Checks if variable already exists in this scope
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

    /*
    Creates variable and adds it to local array if not a global variable
    returns index of local varibale, 0 if global
    */
    fn parse_variable(&mut self, error_message: &str) -> u8 {
        self.consume(TokenType::TokenIdentifier, error_message);
        self.declare_variable();
        if *self.curr_compiler.borrow().scope_depth.borrow() == 0 {
            return self.identifier_constant(self.parser.previous.clone())
        } else{
            0
        } 
    }

    // Marks the depth of the last local in the vector
    fn mark_initialized(&mut self) {
        if *self.curr_compiler.borrow().scope_depth.borrow() == 0 { return; }
        
        let last  = self.curr_compiler.borrow().locals.borrow().len() - 1;
        let binding = self.curr_compiler.borrow();
        let mut locals = binding.locals.borrow_mut();
        locals[last].depth = Some(*self.curr_compiler.borrow().scope_depth.borrow());  
    }

    /*
    Defines a variable
    Either global if scope depth is 0
    Or mark the depth (initialize) of the last local in the Local vector
    */
    fn define_variable(&mut self, global: u8) {
        if *self.curr_compiler.borrow().scope_depth.borrow() == 0 {
            self.emit_bytes(OpCode::OpDefineGlobal as u8, global);
        } else {
            self.mark_initialized();
        }
    }

    // Returns the number of arguments it compiled
    fn argument_list(&mut self) -> u8 {
        let mut arg_count: u8 = 0;
        if !self.check(TokenType::TokenRightParen) {
            loop{
                self.expression();
                if arg_count == 255 {
                    self.error("Can't have more than 255 arguments.");
                }
                arg_count += 1;
                if !self.matching(TokenType::TokenComma) {break;}
            }
        }
        self.consume(TokenType::TokenRightParen, "Expect ')' after arguments.");
        arg_count
    }

    /**
     * At the point this is called, the left-hand side expression has already been compiled.
     * That means at runtime, its value will be on top of the stack. 
     * If that value is falsey, then we know the entire and must be false, so we skip the right operand
     * and leave the left-hand side value as the result of the entire expression.
     */
    fn and_(&mut self, _can_assign: bool){
        let end_jump: usize = self.emit_jump(OpCode::OpJumpIfFalse as u8);

        self.emit_byte(OpCode::OpPop as u8);
        self.parse_precedence(Precedence::PrecAnd);

        self.patch_jump(end_jump);
    }

    /*
    Emits operations Nil and Return
    Nil is emited to satisfy logic regarding functions
    */
    fn emit_return(&mut self) {
        self.emit_byte(OpCode::OpNil as u8);
        self.emit_byte(OpCode::OpReturn as u8);
    }

    /*
    Silimar to add_constant(), just checks that there aren't too many constants in the chunk
    */
    fn make_constant(&mut self, value: Value) -> u8 {
        let constant: u8 = self.curr_compiler.borrow_mut().function.borrow_mut().chunk.add_constant(value);
        if constant > u8::MAX {
            self.error("Too many constants in one chunk.");
            return 0;
        }

        constant
    }

    // Generate the code to load a value
    fn emit_constant(&mut self, value: Value) {
        let constant = self.make_constant(value);
        self.emit_bytes(OpCode::OpConstant as u8, constant);
    }

    // Goes back into the bytecode and replaces the operand at the given location with the calculated jump offset.
    fn patch_jump(&mut self, offset: usize){
        // -2 to adjust for the bytecode for the jump offset itself
        let jump : usize = self.curr_compiler.borrow().function.borrow().chunk.lines.len() - offset - 2;
        
        if jump > u16::MAX.into() {
            self.error("Too much code to jump over.");
        }

        self.curr_compiler.borrow_mut().function.borrow_mut().chunk.code[offset] = ((jump >> 8) & 0xff) as u8;
        self.curr_compiler.borrow_mut().function.borrow_mut().chunk.code[offset+1] = (jump & 0xff) as u8;
    }

    /**
     * we assume the initial ( has already been consumed.
     * We recursively call back into expression() to compile the expression between the parentheses,
     * then parse the closing ) at the end.
     */
    fn grouping(&mut self, _can_assign: bool) {
        self.expression();
        self.consume(TokenType::TokenRightParen, "Expect ')' after expression.");
    }

    /**
     * We assume the token for the number literal has already been consumed and is stored in previous.
     * We take that lexeme and use the Rust parse() method to convert it to an Option<f64>, and unwrap() to make it an f64.
     */
    fn number(&mut self, _can_assign: bool) {
        let _number:f64 = self.parser.previous.lexeme.parse().unwrap();
        self.emit_constant(Value::Number(_number));
    }

    fn or_(&mut self, _can_assign: bool){
        let else_jump = self.emit_jump(OpCode::OpJumpIfFalse as u8);
        let end_jump = self.emit_jump(OpCode::OpJump as u8);

        self.patch_jump(else_jump);
        self.emit_byte(OpCode::OpPop as u8);

        self.parse_precedence(Precedence::PrecOr);
        self.patch_jump(end_jump);
    }

    /**
     * Takes the string’s characters directly from the lexeme
     * Uses that string to wrap it in a Value, and stuffs it into the constant table.
     */
    fn string(&mut self, _can_assign: bool) {
        let end_index = self.parser.previous.lexeme.len() - 1;
        let _string:String = self.parser.previous.lexeme.substring(1, end_index);
        self.emit_constant(Value::from(_string));
    }

    /**
     * Checks whether the variable should be local or global
     * Adds the name of the varibale to the table
     */
    fn named_variable(&mut self, name: Token, _can_assign: bool) {
        let (get_op, set_op): (u8, u8);
        let mut arg = self.resolve_local(&name);

        if arg != None {
            get_op = OpCode::OpGetLocal as u8;
            set_op = OpCode::OpSetLocal as u8;
        } else {
            arg = Some(self.identifier_constant(name) as usize);
            get_op = OpCode::OpGetGlobal as u8;
            set_op = OpCode::OpSetGlobal as u8;
        }
        if _can_assign && self.matching(TokenType::TokenEqual) {
            self.expression();
            self.emit_bytes(set_op, arg.unwrap() as u8);
        } else {
            self.emit_bytes(get_op, arg.unwrap() as u8);
        }
    }

    // Variable parser function 
    fn variable(&mut self, _can_assign: bool) {
        self.named_variable(self.parser.previous.clone(), _can_assign);
    }

    // Unary parser function
    fn unary(&mut self, _can_assign: bool) {
        let operator_type = self.parser.previous._type.clone();

        // Compile the operand.
        self.parse_precedence(Precedence::PrecUnary);

        // Emit the operator instruction
        match operator_type {
            TokenType::TokenBang => self.emit_byte(OpCode::OpNot as u8),
            TokenType::TokenMinus => self.emit_byte(OpCode::OpNegate as u8),
            _ => return,
        }
    }

    /**
     * Binary parser function
     */
    fn binary(&mut self, _can_assign: bool) {
        let operator_type = self.parser.previous._type.clone();
        let rule = self.get_rule(operator_type);
        self.parse_precedence(rule.precedence.next());

        match operator_type {
            TokenType::TokenBangEqual => self.emit_bytes(OpCode::OpEqual as u8, OpCode::OpNot as u8),
            TokenType::TokenEqualEqual => self.emit_byte(OpCode::OpEqual as u8),
            TokenType::TokenGreater => self.emit_byte(OpCode::OpGreater as u8),
            TokenType::TokenGreaterEqual => self.emit_bytes(OpCode::OpLess as u8, OpCode::OpNot as u8),
            TokenType::TokenLess => self.emit_byte(OpCode::OpLess as u8),
            TokenType::TokenLessEqual => self.emit_bytes(OpCode::OpGreater as u8, OpCode::OpNot as u8),
            TokenType::TokenPlus => self.emit_byte(OpCode::OpAdd as u8),
            TokenType::TokenMinus => self.emit_byte(OpCode::OpSubtract as u8),
            TokenType::TokenStar => self.emit_byte(OpCode::OpMultiply as u8),
            TokenType::TokenSlash => self.emit_byte(OpCode::OpDivide as u8),
            TokenType::TokenCarat => self.emit_byte(OpCode::OpExponent as u8),
            _ => return // Unreachable
        }
    }

    // Call parser function
    fn call(&mut self, _can_assign: bool){
        let arg_count: u8 = self.argument_list();
        self.emit_bytes(OpCode::OpCall as u8, arg_count);
    }

    // When the parser encounters false, nil, or true, in prefix position, it calls this literal parser function 
    fn literal(&mut self, _can_assign: bool) {
        match self.parser.previous._type {
            TokenType::TokenFalse => self.emit_byte(OpCode::OpFalse as u8),
            TokenType::TokenNil => self.emit_byte(OpCode::OpNil as u8),
            TokenType::TokenTrue => self.emit_byte(OpCode::OpTrue as u8),
            _ => return
        }
    }

    // Returns the corresponding rule given a TokenType
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