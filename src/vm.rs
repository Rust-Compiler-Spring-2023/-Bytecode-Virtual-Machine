use crate::chunk::*;
use crate::debug::*;
use crate::value::*;
use crate::compiler::*;
pub struct VM {
    chunk : Chunk,
    ip : usize,
    stack : Vec<Value>,
    compiler : Compiler,
}

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
            chunk: Chunk { code: code, constants: ValueArray::new(), lines: lines },
            ip : 0,
            stack : Vec::new(),
            compiler : Compiler::new(),
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

        chunk.constants.values[curr_byte as usize]
    }

    pub fn binary_op(&mut self, value_type: fn(f64) -> Value, op: OpCode) -> InterpretResult{
        loop{
            if !is_number(self.peek(0)) || !is_number(self.peek(1)){
                let args: Vec<RuntimeErrorValues> = Vec::new();
                self.runtime_error("Operands must be numbers.".to_string(), args);
                return InterpretResult::InterpretRuntimeError;
            }
            let b = as_number(self.pop());
            let a = as_number(self.pop());
            match op{
                OpCode::OpAdd => self.push(value_type(a + b)),
                OpCode::OpSubtract => self.push(value_type(a - b)),
                OpCode::OpMultiply => self.push(value_type(a * b)),
                OpCode::OpDivide => self.push(value_type(a / b)),
                _ => ()
            }
        }
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
        
        //need to get index of code that corresponds to where the line of the code is stored in bytecode
        let source_code: usize = usize::try_from(self.chunk.code[0] - 1).unwrap(); //TODO: get correct index for self.chunk.code
        let instruction: usize = self.ip - source_code;
        let line: i32 = self.chunk.lines[instruction].try_into().unwrap();
        eprintln!("[line {}] in script", line);
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
                    // break ?
                },
                OpCode::OpNil => {
                    self.push(nil_val());
                },
                OpCode::OpTrue => {
                    self.push(bool_val(true));
                },
                OpCode::OpFalse => {
                    self.push(bool_val(false));
                },
                OpCode::OpEqual => {
                    let a : Value = self.pop();
                    let b : Value = self.pop();
                    self.push(bool_val(values_equal(a, b)));
                }
                OpCode::OpGreater => {
                    let a : f64 = as_number(self.pop());
                    let b : f64 = as_number(self.pop());
                    self.push(bool_val(a > b));
                },
                OpCode::OpLess => {
                    let a : f64 = as_number(self.pop());
                    let b : f64 = as_number(self.pop());
                    self.push(bool_val(a < b));
                },
                OpCode::OpAdd => {
                    self.binary_op(number_val, OpCode::OpAdd);
                },
                OpCode::OpSubtract => {
                    self.binary_op(number_val, OpCode::OpSubtract);
                },
                OpCode::OpMultiply => {
                    self.binary_op(number_val, OpCode::OpMultiply);
                },
                OpCode::OpDivide => {
                    self.binary_op(number_val, OpCode::OpDivide);
                },
                OpCode::OpNot => {
                    let _pop = self.pop();
                    self.push(bool_val(is_falsey(_pop)));
                },
                OpCode::OpNegate => {
                    if !is_number(self.peek(0)){
                        let args: Vec<RuntimeErrorValues> = Vec::new();
                        self.runtime_error("Operand must be a number.".to_string(), args);
                        
                        return InterpretResult::InterpretRuntimeError;
                    }
                    let _pop = self.pop();
                    self.push(number_val(- as_number(_pop)));
                },
                OpCode::OpReturn => {
                    print_value(self.pop());
                    println!("");
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
            print_value(copy_stack.pop().unwrap());
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
        println!("vm:interpret(): {:?}", chunk.code);     
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
        self.stack[len - distance]
    }
}