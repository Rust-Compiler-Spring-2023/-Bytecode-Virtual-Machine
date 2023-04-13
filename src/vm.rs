use crate::chunk::*;
use crate::debug::*;
use crate::value::*;
use crate::compiler::*;
use std::collections::HashMap;

pub struct VM {
    chunk : Chunk,
    ip : usize,
    stack : Vec<Value>,
    compiler : Compiler,
    globals : HashMap<String, Value>,
}

// This is a way of accessing the globals values with the key
/* 
let key = "key1".to_string();
match map.get(&key) {
    Some(value) => println!("Value for key {}: {}", key, value),
    None => println!("Key {} not found", key),
}
*/

#[derive(Debug,PartialEq)]
pub enum InterpretResult {
    InterpretOk,
    InterpretCompilerError,
    InterpretRuntimeError
}

/*
Helper struct for runtime_error function
Add what types you need to pass for the args parameter in RuntimeError function here
*/
#[derive(Debug)]
pub struct RuntimeErrorValues{
    char: char,
}

impl VM {
    pub fn new() -> Self {
        let code: Vec<u8> = Vec::new();
        let lines: Vec<usize> = Vec::new();
        VM {
            chunk: Chunk::new(),
            ip : 0,
            stack : Vec::new(),
            compiler : Compiler::new(),
            globals : HashMap::new(),
        }
    }

    pub fn free_vm(&mut self) {
        self.stack = Vec::new();
        self.ip = 0;
    }

    // reads the byte currently pointed at by ip and then advances the instruction pointer
    fn read_byte(&mut self, chunk: &Chunk) -> OpCode {
        let curr_ip = self.ip;
        self.ip += 1;

        chunk.code[curr_ip].into()
    }

    fn read_byte_u8(&mut self, chunk: &Chunk) -> u8 {
        let curr_ip = self.ip;
        self.ip += 1;

        chunk.code[curr_ip]
    }

    // reads the next byte from the bytecode, treats the resulting number as an index, 
    // and looks up the corresponding Value in the chunk’s constant table.
    fn read_constant(&mut self, chunk: &Chunk) -> Value {
        let curr_byte: u8 = self.read_byte_u8(chunk);

        chunk.constants[curr_byte as usize].clone()
    }
    // a b
    // 2 5
    // 2 + 5
    pub fn binary_op(&mut self, op: OpCode) -> InterpretResult{
        while self.stack.len() > 1{
            if !is_number(self.peek(0)) || !is_number(self.peek(1)){
                let args: Vec<RuntimeErrorValues> = Vec::new();
                self.runtime_error("Operands must be numbers.".to_string(), args);
                return InterpretResult::InterpretRuntimeError;
            }

            let b : number = self.pop().into();
            let a : number = self.pop().into();
            match op{
                OpCode::OpAdd => self.push(Value::from(a + b)),
                OpCode::OpSubtract => self.push(Value::from(a - b)),
                OpCode::OpMultiply => self.push(Value::from(a * b)),
                OpCode::OpDivide => self.push(Value::from(a / b)),
                OpCode::OpGreater => self.push(Value::from(a > b)),
                OpCode::OpLess => self.push(Value::from(a < b)),
                _ => ()
            }
        }
        return InterpretResult::InterpretOk;
    }

    /*
    Need to pass an arbitrary amount of arguments to this function,
    so made args a vector that hold the struct RuntimeErrorValues.
    You can add what values you may need when calling this function to the struct RuntimeErrorValues above
     */
    pub fn runtime_error(&mut self, format: String, args: Vec<RuntimeErrorValues>){
        if args.len() > 0{
            eprintln!("{} {:?}", format, args);
        }else{
            eprintln!("{}", format)
        }
        if self.chunk.code.len() > 0{
            //need to get index of code that corresponds to where the line of the code is stored in bytecode
            let source_code: usize = usize::try_from(self.chunk.code[0] - 1).unwrap(); //TODO: get correct index for self.chunk.code
            let instruction: usize = self.ip - source_code;
            let line: i32 = self.chunk.lines[instruction].try_into().unwrap();
            eprintln!("[line {}] in script", line);
        }
        
        self.stack.clear();
    }

    fn run(&mut self, chunk: &Chunk) -> InterpretResult {
       loop {
            #[cfg(feature = "debug_trace_execution")]
            self.debug(chunk);

            let instruction: OpCode = self.read_byte(chunk);
            match instruction {
                OpCode::OpConstant => {
                    let constant: Value = self.read_constant(chunk);
                    self.push(constant);
                },
                OpCode::OpNil => {
                    self.push(Value::Nil);
                },
                OpCode::OpTrue => {
                    self.push(Value::from(true));
                },
                OpCode::OpFalse => {
                    self.push(Value::from(false));
                },
                OpCode::OpPop => {
                    self.pop();
                },
                OpCode::OpGetLocal => {
                    let slot = self.read_byte_u8(chunk);
                    self.push(self.stack[slot as usize]);
                },
                OpCode::OpSetLocal => {
                    let slot = self.read_byte_u8(chunk);
                    self.stack[slot as usize] = self.peek(0);
                },
                OpCode::OpGetGlobal => {
                    let name: String = self.read_constant(chunk).to_string();
                    let mut value: Value = Value::Nil;
                    match self.globals.get(&name) {
                        Some(val) => { 
                            value = val.clone();
                            ()
                        },
                        None => {
                            println!("Undefined variable {}.", name);
                            return InterpretResult::InterpretRuntimeError;
                        }
                    }
                    self.push(value);
                },
                OpCode::OpDefineGlobal => { // 21.2
                    let name = self.read_constant(chunk).to_string();
                    let peeked_value = self.peek(0).clone(); 
                    self.globals.insert(name, peeked_value); 
                    self.pop();
                },
                OpCode::OpSetGlobal => {
                    let name: String = self.read_constant(chunk).to_string();
                    match self.globals.get(&name) {
                        Some(val) => {
                            self.globals.insert(name, val.clone()).unwrap();
                            ()
                        },
                        None => {
                            println!("Undefined variable {}", name);
                            return InterpretResult::InterpretRuntimeError;
                        }
                    }
                },
                OpCode::OpEqual => {
                    let b : Value = self.pop();
                    let a : Value = self.pop();
                    self.push(Value::from(b == a));
                }
                OpCode::OpGreater => {
                    self.binary_op(OpCode::OpGreater);
                },
                OpCode::OpLess => {
                    self.binary_op(OpCode::OpLess);
                },
                OpCode::OpAdd => {
                    if is_string(self.peek(0)) && is_string(self.peek(1)) {
                        self.concatenate();
                    }
                    else if is_number(self.peek(0)) && is_number(self.peek(1)) {
                        let b : number = self.pop().into();
                        let a : number = self.pop().into();
                        self.push(Value::from(a+b))
                    }
                    else {
                        let args: Vec<RuntimeErrorValues> = Vec::new();
                        self.runtime_error("Operands must be two numbers or two strings.".to_string(), args);
                        return InterpretResult::InterpretRuntimeError
                    }
                },
                OpCode::OpSubtract => {
                    self.binary_op(OpCode::OpSubtract);
                },
                OpCode::OpMultiply => {
                    self.binary_op(OpCode::OpMultiply);
                },
                OpCode::OpDivide => {
                    self.binary_op(OpCode::OpDivide);
                },
                OpCode::OpNot => {
                    let _pop: Value = self.pop();
                    self.push(Value::from(!_pop.is_falsey()));
                },
                OpCode::OpPrint => {
                    print!("{}",self.pop());
                    println!("");
                },
                OpCode::OpNegate => {
                    if let Value::Number(_num) = self.peek(0){
                        let args: Vec<RuntimeErrorValues> = Vec::new();
                        self.runtime_error("Operand must be a number.".to_string(), args);
                        
                        return InterpretResult::InterpretRuntimeError;
                    }
                    // Pop should be a Value::Number(_)
                    let _pop = self.pop();
                    // gets pushed to the stack<Value> 
                    self.push(_pop);
                },
                OpCode::OpReturn => {
                    return InterpretResult::InterpretOk;
                }
            }
        }
    }

    fn debug(&mut self, chunk: &Chunk) {
        print!("          ");
        let mut copy_stack: Vec<Value> = self.stack.clone();
        while !copy_stack.is_empty(){
            print!("[ ");
            print!("{}",copy_stack.pop().unwrap());
            print!(" ]");
        }
        println!("");
        disassemble_instruction(chunk, self.ip);
    }

    pub fn interpret(&mut self, source: String) -> InterpretResult {
        let mut chunk = Chunk::new();

        // If the compiler encounters an error, compile() returns false and we discard the unusable chunk.
        if !self.compiler.compile(source, &chunk) {
            chunk.free_chunk();
            return InterpretResult::InterpretCompilerError;
        }
        chunk = self.compiler.compiling_chunk.clone();
        let result = self.run(&chunk);
        //println!("vm:interpret(): {:?}", chunk.code);     
        chunk.free_chunk();
         
        result
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }

    pub fn peek(&mut self, distance: usize) -> Value{
        let len = self.stack.len() - 1;

        if distance > len{
            return Value::Nil;
        }
        
        self.stack[len - distance].clone()
    }

    pub fn concatenate(&mut self) {
        let mut b : String = self.pop().into();
        let mut a : String = self.pop().into();

        a.push_str(&b);
        self.push(Value::String(a));
    }

}

